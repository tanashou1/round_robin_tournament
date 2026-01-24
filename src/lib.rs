use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TournamentResult {
    pub rounds: Vec<Round>,
}

#[derive(Serialize, Deserialize)]
pub struct Round {
    pub round_number: usize,
    pub matches: Vec<Match>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub court_number: usize,
    pub team1: String,
    pub team2: String,
}

#[wasm_bindgen]
pub fn generate_tournament(teams_str: &str, court_num: usize) -> String {
    // Parse team names from newline-separated string
    let teams: Vec<String> = teams_str
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    
    if teams.is_empty() {
        return serde_json::to_string(&TournamentResult { rounds: vec![] }).unwrap();
    }
    
    let team_num = teams.len();
    
    // Create name map
    let mut name_map = HashMap::new();
    for (i, name) in teams.iter().enumerate() {
        name_map.insert(i + 1, name.clone());
    }
    
    // Generate all games
    let games = generate_all_games(team_num);
    
    // Adjust court number
    let court_num = court_num.min(team_num / 2).max(1);
    
    // Generate games at once
    let games_at_once = generate_games_at_once(&games, court_num);
    
    // Format output
    let mut rounds = vec![];
    for (round_idx, round_games) in games_at_once.iter().enumerate() {
        let mut matches = vec![];
        for (court_idx, (team1_id, team2_id)) in round_games.iter().enumerate() {
            let team1 = name_map.get(team1_id.min(team2_id)).cloned().unwrap_or_default();
            let team2 = name_map.get(team1_id.max(team2_id)).cloned().unwrap_or_default();
            matches.push(Match {
                court_number: court_idx + 1,
                team1,
                team2,
            });
        }
        rounds.push(Round {
            round_number: round_idx + 1,
            matches,
        });
    }
    
    let result = TournamentResult { rounds };
    serde_json::to_string(&result).unwrap()
}

fn generate_games_at_once(games: &[(usize, usize)], court_num: usize) -> Vec<Vec<(usize, usize)>> {
    let mut games_at_once = vec![];

    let mut start = 0;
    let mut end = court_num;

    let games = games
        .iter()
        .copied()
        .filter(|(t1, t2)| t1 != &0 && t2 != &0)
        .collect::<Vec<(usize, usize)>>();

    while start < games.len() {
        games_at_once.push(games[start..end.min(games.len())].to_vec());
        start += court_num;
        end += court_num;
    }
    games_at_once
}

fn generate_all_games(team_num: usize) -> Vec<(usize, usize)> {
    // 奇数の場合、0を休憩として入れる
    // 先頭を1にするので0は末尾に入れる
    let teams: Vec<usize> = if team_num % 2 == 1 {
        (1..=team_num).chain([0]).collect()
    } else {
        (1..=team_num).collect()
    };

    let mut games: Vec<(usize, usize)> = vec![];

    // チーム1は固定するので飛ばす
    let mut group1: VecDeque<usize> = VecDeque::from_iter(teams.iter().step_by(2).skip(1).copied());

    let mut group2: VecDeque<usize> = VecDeque::from_iter(teams.iter().skip(1).step_by(2).copied());

    for _ in 0..teams.len() - 1 {
        for (&g1, &g2) in [1].iter().chain(&group1).zip(&group2) {
            games.push((g1, g2));
        }
        group2.push_front(group1.pop_front().expect("予期せぬエラーです"));
        group1.push_back(group2.pop_back().expect("予期せぬエラーです"));
    }

    games
}
