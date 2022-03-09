mod callback;
mod command;

extern crate wildfire_ferris;

use ferrisgram::ext::filters::callback_query::Equal as CBEqual;
use ferrisgram::ext::handlers::{CallbackQueryHandler, CommandHandler};
use ferrisgram::ext::Dispatcher;
use std::collections::BTreeMap;
use wildfire_ferris::ferrisgram;
use wildfire_ferris::run;
use wildfire_ferris::tokio;
use wildfire_ferris::utils;

#[tokio::main]
async fn main() {
    let work = /* Telegram Bot to */ "do anything";
    let link = /* https://github.com/StarkBotsIndustries/ */ "Some-Bot";
    let mut cmds = BTreeMap::new();
    cmds.insert("test", "Test the Bot");
    utils::info(work, link, work);
    run(add_handlers, &mut cmds).await;
}

fn add_handlers(dispatcher: &mut Dispatcher) {
    // Command
    dispatcher.add_handler(CommandHandler::new("test", command::test));
    // Callback
    dispatcher.add_handler(CallbackQueryHandler::new(
        callback::test,
        CBEqual::filter("test".to_string()),
    ));
}
