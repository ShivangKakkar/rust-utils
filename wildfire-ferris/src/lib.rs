pub mod basics;
pub mod callbacks;
pub mod constants;
pub mod utils;
pub extern crate ferrisgram;
pub extern crate tokio;

use dotenv::dotenv;
use ferrisgram::ext::filters::callback_query::Equal;
use ferrisgram::ext::handlers::{CallbackQueryHandler, CommandHandler};
use ferrisgram::ext::Dispatcher;
use ferrisgram::ext::Updater;
use ferrisgram::Bot;
use std::collections::BTreeMap;

pub async fn init_bot() -> Bot {
    dotenv().ok();
    std::env::set_var("RUST_BACKTRACE", "full"); /* Good or Bad for production? */
    let bot = match Bot::new((*utils::TOKEN).as_str()).await {
        Ok(bot) => bot,
        Err(error) => {
            panic!("failed to create bot: {}", error)
        }
    };
    bot
}

// Why pub?
pub fn default_handlers<'a>(dispatcher: &'a mut Dispatcher<'a>) -> &'a mut Dispatcher<'a> {
    dispatcher.add_error_handler(utils::error_handler);
    dispatcher.add_handler(CommandHandler::new("start", basics::start));
    dispatcher.add_handler(CommandHandler::new("help", basics::help));
    dispatcher.add_handler(CommandHandler::new("about", basics::about));
    dispatcher.add_handler(CallbackQueryHandler::new(
        callbacks::help,
        Equal::filter("help".to_string()),
    ));
    dispatcher.add_handler(CallbackQueryHandler::new(
        callbacks::about,
        Equal::filter("about".to_string()),
    ));
    dispatcher.add_handler(CallbackQueryHandler::new(
        callbacks::home,
        Equal::filter("home".to_string()),
    ));
    dispatcher
}

type HandlersFunc = fn(&mut Dispatcher) -> ();

pub async fn run(func: HandlersFunc, dict: &mut BTreeMap<&str, &str>) {
    let bot = init_bot().await;
    println!("Starting...");
    utils::commands(dict, &bot).await;
    let dispatcher = &mut Dispatcher::new(&bot);
    let dispatcher = default_handlers(dispatcher);
    func(dispatcher);
    let mut updater = Updater::new(&bot, dispatcher);
    println!("{} is now running...", bot.clone().user.username.unwrap());
    updater.start_polling(true).await.ok();
}
