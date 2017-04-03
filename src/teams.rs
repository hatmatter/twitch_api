extern crate chrono;
extern crate serde_json;
extern crate urlparse;

use std;
use std::io::Write;

use self::chrono::prelude::*;

use super::TwitchClient;
use super::response::TwitchResult;
use super::users::User;

/// Gets all active teams
///
/// #### Authentication: `None`
///
pub fn get_all<'c>(c: &'c TwitchClient)
        -> TwitchResult<TeamIterator<'c>> {
    let iter = TeamIterator { client: c, cur: None, offset: 0 };
    Ok(iter)
}

/// Gets a specified team object
///
/// #### Authentication: `None`
///
pub fn get(c: &TwitchClient, team_name: &str)
        -> TwitchResult<Team> {
    let r = try!(c.get::<Team>(&format!("/teams/{}", team_name)));
    Ok(r)
}

///////////////////////////////////////
// GetAllTeams
///////////////////////////////////////
pub struct TeamIterator<'c> {
    client: &'c TwitchClient,
    cur: Option<SerdeAllTeams>,
    offset: i32,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    #[serde(rename = "_id")]
    pub id: i64,
    pub background: Option<String>,
    pub banner: String,
    pub created_at: DateTime<UTC>,
    pub display_name: String,
    pub info: String,
    pub logo: String,
    pub name: String,
    pub updated_at: DateTime<UTC>,
    pub users: Option<Vec<User>>,
}

#[derive(Deserialize, Debug)]
struct SerdeAllTeams {
    teams: Vec<Team>,
}

impl<'c> Iterator for TeamIterator<'c> {
    type Item = Team;

    fn next(&mut self) -> Option<Team> {
        let url = &format!("/teams?limit=100&offset={}", self.offset);
        next_result!(self, &url, SerdeAllTeams, teams)
    }
}

///////////////////////////////////////
// TESTS
///////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::super::new;
    use super::super::response;
    use super::super::tests::{CLIENTID, TOKEN, CHANID};

    #[test]
    fn get_all() {
        let c = new(String::from(CLIENTID));
        match super::get_all(&c) {
            Ok(mut r)  => match r.next() {
                Some(team) => assert_ne!(team.id, 0),
                None       => assert!(false)
            },
            Err(r) => { println!("{:?}", r); assert!(false); }
        }
    }
}
