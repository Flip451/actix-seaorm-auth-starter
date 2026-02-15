use chrono::{DateTime, Utc};
use uuid::{NoContext, Timestamp, Uuid};

pub fn generate_uuid_v7(now: DateTime<Utc>) -> Uuid {
    // UUID v7 timestamps are 48-bit unsigned integers representing milliseconds since Unix epoch.
    // The maximum value is 2^48 - 1 (0xFFFFFFFFFFFF), which is approx. year 10889.
    const MAX_UUID_V7_MILLIS: i64 = 0x0000_FFFF_FFFF_FFFF;

    let now_millis = now.timestamp_millis();
    debug_assert!(
        (0..=MAX_UUID_V7_MILLIS).contains(&now_millis),
        "generate_uuid_v7: `now` ({}) is outside the representable UUID v7 range [0, {}]",
        now_millis,
        MAX_UUID_V7_MILLIS
    );

    let millis = now_millis.clamp(0, MAX_UUID_V7_MILLIS);
    let seconds = (millis / 1000) as u64;
    let nanos = ((millis % 1000) as u32) * 1_000_000;

    let ts = Timestamp::from_unix(NoContext, seconds, nanos);
    Uuid::new_v7(ts)
}
