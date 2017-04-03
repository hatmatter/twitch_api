extern crate chrono;
extern crate serde_json;

use super::TwitchClient;
use super::response::TwitchResult;


/// Gets a list of Twitch ingest servers
///
/// The Twitch ingesting system is the first stop for
/// a broadcast stream. An ingest server receives your
/// stream, and the ingesting system authorizes and
/// registers streams, then prepares them for viewers.
///
/// #### Authentication: `None`
///
pub fn servers(c: &TwitchClient)
        -> TwitchResult<IngestServerList> {
    let r = try!(c.get::<IngestServerList>("/ingests"));
    Ok(r)
}

///////////////////////////////////////
// GetIngestServerList
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct IngestServerList {
    pub ingests: Vec<IngestServer>,
}

#[derive(Deserialize, Debug)]
pub struct IngestServer {
    pub _id: i64,
    pub availability: f32,
    pub default: bool,
    pub name: String,
    pub url_template: String,
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
    fn servers() {
        let c = new(String::from(CLIENTID));
        match super::servers(&c) {
            Ok(r)  => assert!(r.ingests.len() > 0),
            Err(r) => { println!("{:?}", r); assert!(false); },
        }
    }
}