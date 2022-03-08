use ferrisgram::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use lazy_static;

lazy_static::lazy_static! {
    pub static ref START: String = {
        let start: String;
        match std::env::var("START_MESSAGE") {
            Ok(val) => start = val,
            Err(_e) => {
                println!("START_MESSAGE not found!");
                start = "#".to_string()
            },
        };
        // ToDo: Detailed?
        "Hey {user} \n\nWelcome to {bot} \n\nI can {}. For more info tap \"How to Use?\" button below".replace("{}", &start)
    };
    pub static ref HELP: String = {
        let help;
        match std::env::var("HELP_MESSAGE") {
            Ok(val) => help = val,
            Err(_e) => {
                println!("HELP_MESSAGE not found!");
                help = "#".to_string()
            },
        };
        "<b>Available Commands</b>\n\n{}\n/start - Start the Bot \n/help - Help Message \n/about - About this bot".replace("{}", &help)
    };
    pub static ref ABOUT: String = {
        let about;
        match std::env::var("ABOUT_MESSAGE") {
            Ok(val) => about = val,
            Err(_e) => {
                println!("ABOUT_MESSAGE not found!");
                about = "#".to_string()
        },
        };
        let link;
        match std::env::var("LINK_MESSAGE") {
            Ok(val) => link = val,
            Err(_e) => {
                println!("LINK_MESSAGE not found!");
                link = "#".to_string()
            },
        };
        "
Telegram bot to {} by @StarkBots

<b>Language</b> - <a href=\"https://rust-lang.org\">Rust</a>
        
<b>Source Code</b> - <a href=\"https://github.com/StarkBotsIndustries/{link}\">GitHub Repository</a>
        
<b>Telegram Library</b> - <a href=\"https://github.com/ferrisgram/ferrisgram\">ferrisgram</a>

Developed with ❤️ by @StarkProgrammer
        ".replace("{}", &about).replace("{link}", &link)
    };
}

// ----------------- BUTTONS -------------------- //

pub(crate) fn get_main_buttons() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![InlineKeyboardButton {
                text: "✨ Bot Status and More Bots ✨".to_string(),
                url: Option::from("https://t.me/StarkBots/7".to_string()),
                login_url: None,
                callback_data: None,
                switch_inline_query: None,
                switch_inline_query_current_chat: None,
                callback_game: None,
                pay: None,
            }],
            vec![
                InlineKeyboardButton {
                    text: "How to Use ❔".to_string(),
                    callback_data: Option::from("help".to_string()),
                    url: None,
                    login_url: None,
                    switch_inline_query: None,
                    switch_inline_query_current_chat: None,
                    callback_game: None,
                    pay: None,
                },
                InlineKeyboardButton {
                    text: "🎪 About 🎪".to_string(),
                    callback_data: Option::from("about".to_string()),
                    url: None,
                    login_url: None,
                    switch_inline_query: None,
                    switch_inline_query_current_chat: None,
                    callback_game: None,
                    pay: None,
                },
            ],
            vec![InlineKeyboardButton {
                text: "♥ More Amazing bots ♥".to_string(),
                url: Option::from("https://t.me/StarkBots".to_string()),
                login_url: None,
                callback_data: None,
                switch_inline_query: None,
                switch_inline_query_current_chat: None,
                callback_game: None,
                pay: None,
            }],
        ],
    }
}

pub(crate) fn get_home_buttons() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![vec![InlineKeyboardButton {
            text: "🏠 Return Home 🏠".to_string(),
            callback_data: Option::from("home".to_string()),
            url: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }]],
    }
}
