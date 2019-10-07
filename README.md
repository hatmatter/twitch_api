# Twitch API library
A Rust library for the Twitch API.

# Contributing
Help for this project is highly appreciated. This was built against an older version of the Twitch API. 
Some updates are necessary to work with the latest version of the API. 
Take a look into the Issues if you want to contribute to the project.

Fork it, implement your changes and make a Pull-Request against the `feature-dev` branch of this repo. 

# Usage
```
use twitch_api;
use twitch_api::users;

...

let mut c = twitch_api::new(String::from(CLIENTID));
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

# License
GNU AGPLv3 or later; see copying.md and legal/AGPL-v3.
