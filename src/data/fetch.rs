use anyhow::anyhow;
use lazy_static::lazy_static;
use reqwest::{self};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    pub static ref RACE_MAPPING: HashMap<i64, &'static str> = [
        (0, "random"),
        (1, "human"),
        (2, "orc"),
        (4, "night elf"),
        (8, "undead"),
    ]
    .iter()
    .cloned()
    .collect();
}

#[derive(Debug)]
pub struct Data {
    pub user: Option<User>,
    pub opponent: Option<User>,
}

#[derive(Default, Debug)]
pub struct User {
    pub user_id: String,
    pub stats: Vec<Stat>,
}
#[derive(Debug)]
pub struct Stat {
    pub race: String,
    pub winrate: f64,
    pub ranking_point: i64,
}

impl Data {
    pub fn new() -> Self {
        Data {
            user: Data::fetch_player_profile("Genê#1875"),
            opponent: None,
        }
    }

    pub fn fetch_player_profile(id: &str) -> Option<User> {
        let id = urlencoding::encode(id);
        if let Ok(resp) = reqwest::blocking::get(format!("https://statistic-service.w3champions.com/api/players/{}/game-mode-stats?gateway=20&season=7", &id)) {
            if let Ok(user_json) = resp.json::<Value>() {
                let stats_json = user_json.as_array()?;
                let mut user = User::default();
                user.user_id = urlencoding::decode(&id).expect("decode fail");
                for stat_json in stats_json {
                    let stat = Stat{
                        race: {
                            if stat_json.get("race")?.as_i64().is_none() {
                                continue; 
                            } else {
                                RACE_MAPPING.get(&stat_json.get("race")?.as_i64()?)?.to_string()
                            }
                        },
                        winrate: stat_json.get("winrate")?.as_f64()?,
                        ranking_point: stat_json.get("rankingPoints")?.as_i64()?,
                    };
                    user.stats.push(stat);
                }
                Some(user)
            } else {
                println!("JSON PARSE FAILED");
                None
            }
        } else {
            println!("resp failed");
            None
        }
    }

    pub fn fetch_ongoing_match(&mut self) -> anyhow::Result<()> {
        let user = self.user.as_ref().expect("user did not existed");
        if let Ok(resp) = reqwest::blocking::get(format!(
            "https://statistic-service.w3champions.com/api/matches/ongoing/{}",
            urlencoding::encode(&user.user_id)
        )) {
            let resp_json: Value = resp.json()?;
            self.opponent = Data::inner_fetch_ongoing_match(&user.user_id, &resp_json);
            Ok(())
        } else {
            Err(anyhow!("did not have json"))
        }
    }

    fn inner_fetch_ongoing_match(user_id: &String, resp_json: &Value) -> Option<User> {
        let teams = resp_json.get("teams")?.as_array()?;
        for team in teams {
            let players = team.get("players")?.as_array()?;
            for player in players {
                let id = player.get("battleTag")?.as_str()?;
                if user_id != id {
                    let opponent = Data::fetch_player_profile(id);
                    return opponent;
                }
            }
        }

        None
    }
}

mod tests {
    #[test]
    fn get_player_profile() {
        use super::Data;
        let data = Data::new();
        println!("{:?}", data.user);
    }

    #[test]
    fn fetch_ongoing_match() {
        use super::Data;
        let mut data = Data::new();
        data.fetch_ongoing_match();
        println!("{:?}", data);
    }
}
