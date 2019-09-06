use reqwest;
use serde_json;
use std::env;
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let directory = env::args()
        .nth(1)
        .expect("no directory with database exports specified");
    let paths = fs::read_dir(&directory).expect("could not list files in directory");

    let player = env::args()
        .nth(2)
        .expect("no player name specified")
        .to_lowercase();

    let mut white: String = "".to_string();
    let mut black: String = "".to_string();
    let mut white_elo: i32 = 0;
    let mut black_elo: i32 = 0;
    let mut white_rating_diff: i32 = 0;
    let mut black_rating_diff: i32 = 0;
    let mut utc_date: String = "".to_string();
    let mut utc_time: String = "".to_string();
    let mut game_url: String = "".to_string();

    let mut latest_rating: i32 = -1;
    let mut latest_rating_time: (i32, i32, i32, i32, i32, i32) = (0, 0, 0, 0, 0, 0);
    let mut latest_rating_game_url: String = "".to_string();
    let mut highest_rating: i32 = -1;
    let mut highest_rating_game_url: String = "".to_string();

    for path in paths {
        let file_name1 = path.unwrap().file_name();
        let file_name2 = file_name1.to_str().unwrap();
        println!("Opening {}", &file_name2);
        let file = File::open(directory.clone() + "/" + file_name2).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            if line != "" {
                let mut split_quotes = line.split('"');
                if line.starts_with('1') {
                    if white == player || black == player {
                        let rating_after = if white == player {
                            white_elo + white_rating_diff
                        } else {
                            black_elo + black_rating_diff
                        };

                        if rating_after > highest_rating {
                            highest_rating = rating_after;
                            highest_rating_game_url = game_url.clone();
                        }

                        let mut date_split = utc_date.split('.');
                        let mut time_split = utc_time.split(':');
                        let game_time: (i32, i32, i32, i32, i32, i32) = (
                            date_split.next().unwrap().parse().unwrap(),
                            date_split.next().unwrap().parse().unwrap(),
                            date_split.next().unwrap().parse().unwrap(),
                            time_split.next().unwrap().parse().unwrap(),
                            time_split.next().unwrap().parse().unwrap(),
                            time_split.next().unwrap().parse().unwrap(),
                        );

                        if game_time > latest_rating_time {
                            latest_rating_time = game_time;
                            latest_rating = rating_after;
                            latest_rating_game_url = game_url.clone();
                        }
                    }
                } else if line.starts_with("[Site ") {
                    game_url = split_quotes.nth(1).unwrap().to_string();
                } else if line.starts_with("[White ") {
                    white = split_quotes.nth(1).unwrap().to_lowercase();
                } else if line.starts_with("[Black ") {
                    black = split_quotes.nth(1).unwrap().to_lowercase();
                } else if line.starts_with("[WhiteElo ") {
                    white_elo = split_quotes.nth(1).unwrap().parse().unwrap();
                } else if line.starts_with("[BlackElo ") {
                    black_elo = split_quotes.nth(1).unwrap().parse().unwrap();
                } else if line.starts_with("[WhiteRatingDiff ") {
                    white_rating_diff = split_quotes.nth(1).unwrap().parse().unwrap();
                } else if line.starts_with("[BlackRatingDiff ") {
                    black_rating_diff = split_quotes.nth(1).unwrap().parse().unwrap();
                } else if line.starts_with("[UTCDate ") {
                    utc_date = split_quotes.nth(1).unwrap().to_string();
                } else if line.starts_with("[UTCTime ") {
                    utc_time = split_quotes.nth(1).unwrap().to_string();
                }
            }
        }
    }

    if latest_rating == -1 {
        // no games played in time period
        let api_resp: serde_json::Value =
            reqwest::get(&format!("https://lichess.org/api/user/{}", player))
                .expect("API request failed")
                .json()
                .expect("API request invalid JSON");
        let anti_perf: i32 = api_resp["perfs"]["antichess"]["rating"]
            .as_i64()
            .expect("could not read antichess rating from API") as i32;
        latest_rating = anti_perf;
        highest_rating = anti_perf;
        latest_rating_game_url = String::from("API response");
        highest_rating_game_url = String::from("API response");
    }

    println!("Stats for {}", player);
    println!(
        "Rating at cutoff: {} from {}",
        latest_rating, latest_rating_game_url
    );
    println!(
        "Highest rating: {} from {}",
        highest_rating, highest_rating_game_url
    );
    println!("This took {} ms.", start.elapsed().as_millis());
}
