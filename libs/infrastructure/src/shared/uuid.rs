use chrono::{DateTime, Utc};
use uuid::{ClockSequence, Timestamp, Uuid};

/// UUID v7 timestamps are 48-bit unsigned integers representing milliseconds since Unix epoch.
/// The maximum value is 2^48 - 1 (0xFFFFFFFFFFFF), which is approx. year 10889.
const MAX_UUID_V7_MILLIS: i64 = 0x0000_FFFF_FFFF_FFFF;

pub fn calculate_v7_timestamp_parts(now: DateTime<Utc>) -> (u64, u32) {
    let now_millis = now.timestamp_millis();
    assert!(
        (0..=MAX_UUID_V7_MILLIS).contains(&now_millis),
        "calculate_v7_timestamp_parts: `now` ({}) is outside the representable UUID v7 range [0, {}]",
        now_millis,
        MAX_UUID_V7_MILLIS
    );

    let millis = now_millis;
    let seconds = (millis / 1000) as u64;
    let nanos = ((millis % 1000) as u32) * 1_000_000;

    (seconds, nanos)
}

pub fn generate_uuid_v7_with_parts(
    context: impl ClockSequence<Output = impl Into<u128>>,
    seconds: u64,
    nanos: u32,
) -> Uuid {
    let ts = Timestamp::from_unix(context, seconds, nanos);
    Uuid::new_v7(ts)
}

pub fn generate_uuid_v7(
    now: DateTime<Utc>,
    context: impl ClockSequence<Output = impl Into<u128>>,
) -> Uuid {
    let (seconds, nanos) = calculate_v7_timestamp_parts(now);
    generate_uuid_v7_with_parts(context, seconds, nanos)
}
