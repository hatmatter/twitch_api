use std::collections::HashMap;

use super::TwitchClient;
use super::response::TwitchResult;

/// Gets a list of badges that can be used in chat for a specified channel
///
/// #### Authentication: `None`
///
pub fn get_badges(c: &TwitchClient, chan_id: &str)
        -> TwitchResult<BadgeSet> {
    let r = try!(c.get::<BadgeSet>(&format!("/chat/{}/badges", chan_id)));
    Ok(r)
}

/// Gets all chat emoticons (not including their images) in one or more specified sets
///
/// #### Authentication: `None`
///
/// # Remarks
/// Caution: When not specifying the emotesets parameter,
/// this endpoint returns a large amount of data.
///
pub fn get_emote_sets(c: &TwitchClient, sets: &[&str])
        -> TwitchResult<EmotesBySet> {
    let r = try!(c.get::<EmotesBySet>(&format!("/chat/emoticon_images?emotesets={}", sets.join(","))));
    Ok(r)
}

/// Gets all chat emoticons (including their images)
///
/// #### Authentication: `None`
///
/// # Remarks
/// Caution: This endpoint returns a large amount of data.
///
pub fn get_emotes(c: &TwitchClient)
        -> TwitchResult<ChatEmotes> {
    let r = try!(c.get::<ChatEmotes>("/chat/emoticons"));
    Ok(r)
}

///////////////////////////////////////
// GetChatBadgesByChannel
///////////////////////////////////////
pub type BadgeSet = HashMap<String, Option<Badge>>;

#[derive(Deserialize, Debug)]
pub struct Badge {
    pub alpha: String,
    pub image: String,
    pub svg: String,
}

///////////////////////////////////////
// GetChatEmotesBySets
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct EmotesBySet {
    pub emoticon_sets: HashMap<String, Vec<EmoteSet>>,
}

#[derive(Deserialize, Debug)]
pub struct EmoteSet {
    pub id: i64,
    pub code: String,
}

///////////////////////////////////////
// GetAllChatEmotes
///////////////////////////////////////
#[derive(Deserialize, Debug)]
pub struct ChatEmotes {
    pub emoticons: Vec<ChatEmote>,
}

#[derive(Deserialize, Debug)]
pub struct ChatEmote {
    pub regex: String,
    pub images: Vec<ChatEmoteImage>,
}

#[derive(Deserialize, Debug)]
pub struct ChatEmoteImage {
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub emoticon_set: Option<i64>,
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
    fn get_badges() {
        let c = new(String::from(CLIENTID));

        match super::get_badges(&c, "12826") {
            Ok(r)  => assert!(r.contains_key("global_mod")),
            Err(r) => { println!("{:?}", r); assert!(false); },
        }
    }

    #[test]
    fn get_emote_sets() {
        let c = new(String::from(CLIENTID));

        match super::get_emote_sets(&c, &["19151"]) {
            Ok(r)  => assert!(r.emoticon_sets.contains_key("19151")),
            Err(r) => { println!("{:?}", r); assert!(false); },
        }
    }

    #[test]
    fn get_emotes() {
        let c = new(String::from(CLIENTID));

        match super::get_emotes(&c) {
            Ok(r)  => assert!(r.emoticons.len() > 0),
            Err(r) => { println!("{:?}", r); assert!(false); },
        }
    }
}