use crate::ferrisgram;
use ferrisgram::error::{GroupIteration, Result};
use ferrisgram::ext::Context;
use ferrisgram::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use ferrisgram::Bot;

pub async fn test(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let msg = ctx.effective_message.unwrap();
    bot.send_message(msg.chat.id, "A simple test".to_string())
        .parse_mode("html".to_string())
        .disable_web_page_preview(true)
        .reply_markup(InlineKeyboardMarkup {
            inline_keyboard: vec![vec![
                InlineKeyboardButton {
                    text: "XKCD".to_string(),
                    url: Option::from("https://xkcd.com".to_string()),
                    login_url: None,
                    callback_data: None,
                    switch_inline_query: None,
                    switch_inline_query_current_chat: None,
                    callback_game: None,
                    pay: None,
                },
                InlineKeyboardButton {
                    text: "Callback".to_string(),
                    url: None,
                    login_url: None,
                    callback_data: Option::from("test".to_string()),
                    switch_inline_query: None,
                    switch_inline_query_current_chat: None,
                    callback_game: None,
                    pay: None,
                },
            ]],
        })
        .send()
        .await?;

    Ok(GroupIteration::EndGroups)
}
