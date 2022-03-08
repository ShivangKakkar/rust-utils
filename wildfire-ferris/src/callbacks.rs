use ferrisgram::error::{GroupIteration, Result};
use ferrisgram::ext::Context;
use ferrisgram::Bot;

use crate::constants;

pub async fn help(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let cb = ctx.update.callback_query.unwrap();
    let msg = cb.message.unwrap();
    bot.edit_message_text(constants::HELP.to_string())
        .message_id(msg.message_id)
        .chat_id(msg.chat.id)
        .parse_mode("html".to_string())
        .disable_web_page_preview(true)
        .reply_markup(constants::get_home_buttons())
        .send()
        .await?;
    Ok(GroupIteration::EndGroups)
}

pub async fn about(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let cb = ctx.update.callback_query.unwrap();
    let msg = cb.message.unwrap();
    bot.edit_message_text(constants::ABOUT.to_string())
        .message_id(msg.message_id)
        .chat_id(msg.chat.id)
        .parse_mode("html".to_string())
        .disable_web_page_preview(true)
        .reply_markup(constants::get_home_buttons())
        .send()
        .await?;
    Ok(GroupIteration::EndGroups)
}

pub async fn home(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let cb = ctx.update.callback_query.unwrap();
    let msg = cb.message.unwrap();
    bot.edit_message_text(
        constants::START
            .replace("{user}", msg.from.unwrap().first_name.as_str())
            .replace("{bot}", bot.user.first_name.as_str()),
    )
    .message_id(msg.message_id)
    .chat_id(msg.chat.id)
    .parse_mode("html".to_string())
    .disable_web_page_preview(true)
    .reply_markup(constants::get_main_buttons())
    .send()
    .await?;
    Ok(GroupIteration::EndGroups)
}
