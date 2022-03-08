use crate::constants;
use ferrisgram::error::{GroupIteration, Result};
use ferrisgram::ext::Context;
use ferrisgram::Bot;

pub async fn start(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let msg = ctx.effective_message.unwrap();
    bot.send_message(
        msg.chat.id,
        constants::START
            .replace("{user}", msg.from.unwrap().first_name.as_str())
            .replace("{bot}", bot.user.first_name.as_str()),
    )
    .parse_mode("html".to_string())
    .disable_web_page_preview(true)
    .reply_markup(constants::get_main_buttons())
    .send()
    .await?;

    Ok(GroupIteration::EndGroups)
}

pub async fn help(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let msg = ctx.effective_message.unwrap();
    bot.send_message(msg.chat.id, constants::HELP.to_string())
        .parse_mode("html".to_string())
        .disable_web_page_preview(true)
        .send()
        .await?;

    Ok(GroupIteration::EndGroups)
}

pub async fn about(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let msg = ctx.effective_message.unwrap();
    bot.send_message(msg.chat.id, constants::ABOUT.to_string())
        .parse_mode("html".to_string())
        .disable_web_page_preview(true)
        .send()
        .await?;

    Ok(GroupIteration::EndGroups)
}
