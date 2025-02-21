use crate::stats::Stats;
use chrono::{Duration, Utc};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Member {
    handle: String,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
struct Party {
    contestId: Option<i32>,
    members: Vec<Member>,
    participantType: String,
    teamId: Option<i32>,
    teamName: Option<String>,
    ghost: bool,
    room: Option<i32>,
    startTimeSeconds: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
struct Problem {
    contestId: Option<i32>,
    problemsetName: Option<String>,
    index: String,
    name: String,
    r#type: String,
    points: Option<f64>,
    rating: Option<i32>,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
struct Submission {
    id: i32,
    contestId: Option<i32>,
    creationTimeSeconds: i64,
    relativeTimeSeconds: i64,
    problem: Problem,
    author: Party,
    programmingLanguage: String,
    verdict: Option<String>,
    testset: String,
    passedTestCount: i32,
    timeConsumedMillis: i32,
    memoryConsumedBytes: i32,
    points: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct UserStatusResponse {
    status: String,
    comment: Option<String>,
    result: Option<Vec<Submission>>,
}

pub fn get_stats(username: String) -> Result<Stats, Box<dyn std::error::Error>> {
    let mut solved_today: HashSet<String> = HashSet::new();
    let mut solved_this_month: HashSet<String> = HashSet::new();
    let data: UserStatusResponse = reqwest::blocking::get(
        "https://codeforces.com/api/user.status?handle=".to_string() + &username,
    )?
    .json::<UserStatusResponse>()?;

    if data.status != "OK" {
        return Err(Box::<dyn std::error::Error>::from(data.comment.unwrap()));
    }
    for submission in data.result.unwrap() {
        if submission.verdict.as_deref() == Some("OK") {
            if submission.creationTimeSeconds >= (Utc::now() - Duration::days(1)).timestamp() {
                solved_today.insert(submission.problem.index.clone() + &submission.problem.name);
            }
            if submission.creationTimeSeconds >= (Utc::now() - Duration::days(30)).timestamp() {
                solved_this_month
                    .insert(submission.problem.index.clone() + &submission.problem.name);
            }
        }
    }

    return Ok(Stats {
        solved_today: solved_today.len() as i32,
        solved_this_month: solved_this_month.len() as i32,
    });
}
