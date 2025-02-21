mod stats;
use stats::{get_all_stats, Identity, Stats};

fn main() {
    let s: Stats = get_all_stats(Identity {
        codeforces_username: "<username>".to_string(),
        infoarena_username: "<username>".to_string(),
        kilonova_username: "<username>".to_string(),
        pbinfo_username: "<username>".to_string(),
    });
    println!("You have solved: {} problems today", s.solved_today);
    println!(
        "You have solved: {} problems this month",
        s.solved_this_month
    )
}
