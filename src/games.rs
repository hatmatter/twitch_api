extern crate chrono;
extern crate serde_json;

use std::collections::HashMap;
use std;
use std::io::Write;

use super::TwitchClient;
use super::response::TwitchResult;

/// Gets games sorted by number of current viewers on Twitch, most popular first
///
/// #### Authentication: `None`
///
pub fn top<'c>(c: &'c TwitchClient)
        -> TwitchResult<TopGames<'c>> {
    let iter = TopGames { client: c,
                          cur: None,
                          offset: 0 };
    Ok(iter)
}

///////////////////////////////////////
// GetTopGames
///////////////////////////////////////
pub struct TopGames<'c> {
    client: &'c TwitchClient,
    cur: Option<SerdeTopGames>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
pub struct TopGame {
    pub channels: i32,
    pub viewers: i32,
    pub game: Game,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: i64,
    #[serde(rename = "box")]
    pub _box: HashMap<String, String>,
    pub giantbomb_id: i64,
    pub logo: HashMap<String, String>,
    pub name: String,
    pub popularity: i32,
}

#[derive(Deserialize, Debug)]
struct SerdeTopGames {
    top: Vec<TopGame>,
}

impl<'c> Iterator for TopGames<'c> {
    type Item = TopGame;

    fn next(&mut self) -> Option<TopGame> {
        let url = &format!("/games/top?limit=100&offset={}", self.offset);
        next_result!(self, &url, SerdeTopGames, top)
    }
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::response;
    use super::super::tests::CLIENTID;

    #[test]
    fn top() {
        let c = new(String::from(CLIENTID));
        let mut r = super::top(&c).unwrap();
        r.next();
    }
}