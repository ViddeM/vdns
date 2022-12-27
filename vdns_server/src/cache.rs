use mobc::Pool;
use mobc_redis::{redis::AsyncCommands, RedisConnectionManager};
use vdns_lib::{
    common::{domain_name::DomainName, rr_type::RRType},
    messages::{message::Message, resource_record::resource_record::ResourceRecord},
};

#[inline(always)]
fn get_id(name: &DomainName, rr_type: &RRType) -> String {
    let lower_name = name.to_string().to_lowercase(); // Domain names should be compared case-insensitively
    format!("{rr_type}@{lower_name}")
}

pub async fn cache_response(redis_pool: &Pool<RedisConnectionManager>, response: &Message) {
    let mut redis_conn = redis_pool
        .get()
        .await
        .expect("Failed to get redis connection");

    // Cache all answers
    for answer in response.answer.iter() {
        let (name, rr_type) = answer.get_query_name_type();
        let id = get_id(&name, &rr_type);

        let answer_json = serde_json::to_string(answer).expect("Failed to convert answer to json");

        let seconds_left = answer.seconds_until_expiration();
        if seconds_left == 0 {
            // Don't cache...
            continue;
        }

        redis_conn
            .set_ex::<String, String, String>(id, answer_json, seconds_left)
            .await
            .expect("Failed to insert to cache");
    }
}

pub async fn lookup_cached(
    redis_pool: &Pool<RedisConnectionManager>,
    domain_name: &DomainName,
    rr_type: &RRType,
) -> Option<ResourceRecord> {
    let mut redis_conn = redis_pool
        .get()
        .await
        .expect("Failed to get redis connection");

    let id = get_id(domain_name, rr_type);

    let ans = match redis_conn
        .get::<String, Option<String>>(id.clone())
        .await
        .expect("Failed to retrieve cached request from cache")
    {
        Some(val) => val,
        None => return None,
    };

    let record_ttl: usize = redis_conn
        .ttl(id)
        .await
        .expect("Failed to get TTL for record");

    let mut deserialized = serde_json::from_str::<ResourceRecord>(&ans)
        .expect("Failed to deserialize the cached answer");

    deserialized.set_ttl(record_ttl);

    Some(deserialized)
}
