pub const PLAN_GRACE_PERIOD_DAYS: u32 = 7;

pub fn validate_plan(plan: &str) -> Result<(), &'static str> {
    if plan != "pro" {
        return Err("INVALID_PLAN");
    }
    Ok(())
}
