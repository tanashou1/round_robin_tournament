#[cfg(feature = "cli")]
use calamine::{Reader, Xlsx};
#[cfg(feature = "cli")]
use core::panic;
#[cfg(feature = "cli")]
use std::collections::{HashMap, VecDeque};

#[cfg(feature = "cli")]
use dialoguer::Input;

#[cfg(feature = "cli")]
use std::fs::File;
#[cfg(feature = "cli")]
use std::io::{prelude::*, BufWriter};
#[cfg(feature = "cli")]
use std::path::Path;

#[cfg(feature = "cli")]
fn main() {
    let file_name = Input::<String>::new()
        .with_prompt("チーム数か、入力のExcelファイルの名前を入力してください。")
        .interact()
        .expect("エラーです。整数を入力してください");
    let (team_num, name_map) = if let Ok(team_num) = file_name.parse::<usize>() {
        (team_num, create_number_map(team_num))
    } else {
        let name_map = create_name_map(&file_name);
        (name_map.len(), name_map)
    };

    let court_num = Input::<usize>::new()
        .with_prompt("コート数を入力してください")
        .interact()
        .expect("エラーです。整数を入力してください");

    let games = generate_all_games(team_num);

    // コート数がチーム数の半分よりも大きければ、毎回全試合できる
    let court_num = court_num.min(team_num / 2);
    let games_at_once = generate_games_at_once(&games, court_num);
    output(games_at_once, court_num, name_map);
}

#[cfg(feature = "cli")]

fn output(
    games_at_once: Vec<Vec<(usize, usize)>>,
    court_num: usize,
    name_map: HashMap<usize, String>,
) {
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
                .map(|(a, b)| format!("{}-{}", name_map[a.min(b)], name_map[a.max(b)]))
                .collect::<Vec<String>>()
                .join(",")
        );
        writeln!(file, "{line}").unwrap();
    }

    file.flush().unwrap();
}

#[cfg(feature = "cli")]

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

#[cfg(feature = "cli")]

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

#[cfg(feature = "cli")]

fn read_excel(file: &str) -> Vec<String> {
    let mut workbook: Xlsx<_> = calamine::open_workbook(file).unwrap();

    // Sheet1 という名前のワークシートを読み込む
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        let s = range.rows().map(|r| format!("{}", r[0])).collect();
        s
    } else {
        panic!()
    }
}

#[cfg(feature = "cli")]

fn create_name_map(file_name: &str) -> HashMap<usize, String> {
    let names = read_excel(file_name);
    let mut map = HashMap::new();
    for (_i, n) in names.into_iter().enumerate() {
        let i = _i + 1;
        map.insert(i, n);
    }
    map
}

#[cfg(feature = "cli")]
fn create_number_map(team_num: usize) -> HashMap<usize, String> {
    let mut map = HashMap::new();
    for i in 1..=team_num {
        map.insert(i, i.to_string());
    }
    map
}
