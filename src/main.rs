use std::collections::VecDeque;

use dialoguer::Input;

use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::Path;

fn main() {
    let team_num = Input::<usize>::new()
        .with_prompt("チーム数を入力してください")
        .interact()
        .expect("エラーです。整数を入力してください");

    let court_num = Input::<usize>::new()
        .with_prompt("コート数を入力してください")
        .interact()
        .expect("エラーです。整数を入力してください");

    let games = generate_all_games(team_num);

    // コート数がチーム数の半分よりも大きければ、毎回全試合できる
    let court_num = court_num.min(team_num / 2);
    let games_at_once = generate_games_at_once(&games, court_num);
    output(games_at_once, court_num);
}

fn output(games_at_once: Vec<Vec<(usize, usize)>>, court_num: usize) {
    let path = Path::new("組合せ結果.csv");
    let mut file = BufWriter::new(
        File::create(path).expect("ファイルを開けませんでした。開いている場合は閉じてください。"),
    );

    const BOM: &[u8; 3] = &[0xEF, 0xBB, 0xBF]; // UTF-8
    file.write_all(BOM).unwrap();

    // ヘッダー
    let header = format!(
        ",{}",
        (1..=court_num)
            .map(|n| format!("第{n}コート"))
            .collect::<Vec<String>>()
            .join(",")
    );
    writeln!(file, "{header}").unwrap();

    for (_i, games) in games_at_once.iter().enumerate() {
        let i = _i + 1;
        let line = format!(
            "第{i}試合,{}",
            games
                .iter()
                .map(|(a, b)| format!("{}-{}", a.min(b), a.max(b)))
                .collect::<Vec<String>>()
                .join(",")
        );
        writeln!(file, "{line}").unwrap();
    }

    file.flush().unwrap();
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
