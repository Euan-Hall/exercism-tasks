use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

const GIGASECOND: i64 = 1000000000;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(GIGASECOND.seconds())
}
