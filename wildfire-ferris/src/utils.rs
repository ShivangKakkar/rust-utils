use ferrisgram::error::Error;
use ferrisgram::error::GroupIteration;
use ferrisgram::error::GroupIteration::ContinueGroups;
use ferrisgram::ext::Context;
use ferrisgram::Bot;
use std::env;

extern crate lazy_static;

lazy_static::lazy_static! {
    pub static ref TOKEN: String = {
        let token;
        match env::var("TOKEN") {
            Ok(val) => token = val,
            Err(_e) => panic!("bot token not found! Exiting..."),
        };
        token
    };
    pub static ref LOG_CHAT_ID: i64 = {
        let log_chat;
        match env::var("LOG_CHAT_ID") {
            Ok(val) => log_chat = val,
            Err(_e) => log_chat = "".to_string(),
        }
        let log_chat_id: i64;
        if !log_chat.is_empty() {
            if log_chat.chars().next().unwrap().is_numeric() {
                log_chat_id = log_chat.parse().unwrap();
            } else {
                panic!("var LOG_CHAT_ID is not an integer and therefore an invalid chat id");
            }
        } else {
            log_chat_id = 0;
        };
        log_chat_id
    };
    pub static ref OWNER_ID: i64 = {
        let owner;
        match env::var("OWNER_ID") {
            Ok(val) => owner = val,
            Err(_e) => owner = "".to_string(),
        }
        let owner_id: i64;
        if !owner.is_empty() {
            if owner.chars().next().unwrap().is_numeric() {
                owner_id = owner.parse().unwrap();
            } else {
                panic!("var OWNER_ID is not an integer and therefore an invalid id");
            }
        } else {
            owner_id = 0;
        };
        owner_id
    };
}

pub fn error_handler(bot: &Bot, _: &Context, err: Error) -> GroupIteration {
    // let lci = *LOG_CHAT_ID;
    let e = format!("an error occurred: {}", err);
    if *LOG_CHAT_ID == 0 {
        println!("{}", e);
    } else {
        let b = bot.clone();
        tokio::spawn(async move {
            let r = b
                .send_message(*LOG_CHAT_ID, e)
                .parse_mode("html".to_string())
                .send()
                .await;
            match r {
                Ok(_) => (),
                Err(e) => println!("two error occurred: \n  Can't send message in log chat: {} \n  Handler Error: {}", err, e),
            }
        });
    };
    ContinueGroups
}

pub fn info(start: &str, link: &str, about: Option<&str>) {
    std::env::set_var("START_MESSAGE", start);
    match about {
        Some(a) => std::env::set_var("ABOUT_MESSAGE", a),
        None => std::env::set_var("ABOUT_MESSAGE", start),
    }
    std::env::set_var("LINK_MESSAGE", link);
}
