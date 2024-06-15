use time::{PrimitiveDateTime, Duration };

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    return start + Duration::seconds(1_000_000_000);
}
