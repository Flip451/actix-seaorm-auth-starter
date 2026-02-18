use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::{ClockSequence, Timestamp, Uuid};

/// UUID v7 timestamps are 48-bit unsigned integers representing milliseconds since Unix epoch.
/// The maximum value is 2^48 - 1 (0xFFFFFFFFFFFF), which is approx. year 10889.
const MAX_UUID_V7_MILLIS: i64 = 0x0000_FFFF_FFFF_FFFF;

#[derive(Debug, Error)]
pub enum UuidError {
    #[error(
        "タイムスタンプの範囲外: `now` {0} は表現可能なUUID v7の範囲 [0, {MAX_UUID_V7_MILLIS}] を超えています"
    )]
    TimestampOutOfRange(i64),
}

pub fn calculate_v7_timestamp_parts(now: DateTime<Utc>) -> Result<(u64, u32), UuidError> {
    let now_millis = now.timestamp_millis();
    if !(0..=MAX_UUID_V7_MILLIS).contains(&now_millis) {
        return Err(UuidError::TimestampOutOfRange(now_millis));
    }

    let millis = now_millis as u64;
    let seconds = millis / 1000;
    let nanos = ((millis % 1000) as u32) * 1_000_000;

    Ok((seconds, nanos))
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
) -> Result<Uuid, UuidError> {
    let (seconds, nanos) = calculate_v7_timestamp_parts(now)?;
    Ok(generate_uuid_v7_with_parts(context, seconds, nanos))
}
