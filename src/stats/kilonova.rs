use crate::stats::Stats;

pub fn get_stats(username: String) -> Result<Stats, Box<dyn std::error::Error>> {
    // Basic placeholder
    return Ok(Stats {
        solved_today: 0,
        solved_this_month: 0,
    });
}
