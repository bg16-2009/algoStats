pub mod codeforces;
pub mod infoarena;
pub mod kilonova;
pub mod pbinfo;

#[derive(Debug)]
pub struct Identity {
    pub codeforces_username: String,
    pub infoarena_username: String,
    pub kilonova_username: String,
    pub pbinfo_username: String,
}

#[derive(Debug)]
pub struct Stats {
    pub solved_today: i32,
    pub solved_this_month: i32,
}

impl std::ops::Add for Stats {
    type Output = Stats;
    fn add(self, other: Stats) -> Stats {
        return Stats {
            solved_today: self.solved_today + other.solved_today,
            solved_this_month: self.solved_this_month + other.solved_this_month,
        };
    }
}

pub fn get_all_stats(user: Identity) -> Result<Stats, Box<dyn std::error::Error>> {
    return Ok(codeforces::get_stats(user.codeforces_username)?
        + infoarena::get_stats(user.infoarena_username)?
        + kilonova::get_stats(user.kilonova_username)?
        + pbinfo::get_stats(user.pbinfo_username)?);
}
