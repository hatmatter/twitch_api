# libtwitch-rs [![crates.io version][1]][2] ![GNU AGPLv3][agpl-logo]
A Rust library for the Twitch API.

# Contributing
Help for this project is highly appreciated. This was built against an older version of the Twitch API. 
Some updates are necessary to work with the latest version of the API. 
Take a look into the [issues](https://github.com/simonsan/libtwitch-rs/issues) if you want to contribute to the project.

Fork it, implement your changes and make a Pull-Request against the `feature-dev` branch of this repo. 

# Usage
```
use libtwitch_rs;
use libtwitch_rs::users;

...

let mut c = libtwitch_rs::new(String::from(CLIENTID));
c.set_oauth_token(TOKEN);

if let Some(user) = match users::get(&c) {
    Ok(r)  => { assert!(r.email.is_some()); Some(r) },
    Err(r) => { println!("{:?}", r); assert!(false); None }
    } {
    let user_id = user.id.to_string();

    match users::get_by_id(&c, &user_id) {
        Ok(r)  => assert_eq!(r.name, user.name),
        Err(r) => { println!("{:?}", r); assert!(false); }
    }
}
```

# Supported API versions

Endpoints         | Kraken/v5          |  Helix/v6 [WIP]  |
:----------------:|:------------------:|:----------------:|
Analytics         | :heavy_minus_sign: | :x:              |
Bits              | :heavy_check_mark: | :x:              |
Channels          | :heavy_check_mark: | :x:              |
Chat              | :heavy_check_mark: | :x:              |
Clips             | :heavy_check_mark: | :x:              |
Collections       | :heavy_check_mark: | :x:              |
Entitlements      | :heavy_minus_sign: | :x:              |
Games             | :heavy_check_mark: | :x:              |
Ingests           | :heavy_check_mark: | :x:              |
Search            | :heavy_check_mark: | :x:              |
Streams           | :heavy_check_mark: | :x:              |
Tags              | :heavy_minus_sign: | :x:              |
Teams             | :heavy_check_mark: | :x:              |
Users             | :heavy_check_mark: | :x:              |
Videos            | :heavy_check_mark: | :x:              |
Webhooks          | :heavy_minus_sign: | :x:              |


# Links
## Helpful Documentation
- [Token test, infos incl. Channel_ID](https://codepen.io/Alca/pen/VwwazOK)
- [Achieve Authorization for a bot](http://web.archive.org/web/20191016034229/https://d-fischer.github.io/twitch-chat-client/docs/examples/basic-bot.html)
- [Twitch API Guide](https://dev.twitch.tv/docs/api/guide)

## Token Generation
- [Token generator](https://twitchtokengenerator.com/)
- [Twitch IRC OAuth generator](https://twitchapps.com/tmi/)

## Repositories:
- [Twitch library (C#)](https://github.com/TwitchLib/TwitchLib)
- [Twitch library (PHP)](https://github.com/Dkamps18/PHP-Twitch-lib/)
- [Python Twitch client](https://github.com/tsifrer/python-twitch-client)
- [TwitchIO, Twtich Bot/API wrapper (Python)](https://github.com/TwitchIO/TwitchIO)


# License
GNU AGPL-3.0-or-later; see [copying.md](copying.md) and [legal/AGPL-v3](legal/AGPL-v3).


[1]: https://img.shields.io/crates/v/libtwitch-rs.svg?style=flat-square
[2]: https://crates.io/crates/libtwitch-rs
[agpl-logo]: https://www.gnu.org/graphics/agplv3-88x31.png
