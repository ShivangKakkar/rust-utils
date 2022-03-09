use crate::ferrisgram;
use ferrisgram::error::{GroupIteration, Result};
use ferrisgram::ext::Context;
use ferrisgram::Bot;

pub async fn test(bot: Bot, ctx: Context) -> Result<GroupIteration> {
    let cb = ctx.update.callback_query.unwrap();
    let msg = cb.message.unwrap();
    bot.edit_message_text("Another test".to_string())
        .message_id(msg.message_id)
        .chat_id(msg.chat.id)
        .parse_mode("html".to_string())
        .disable_web_page_preview(true)
        .send()
        .await?;
    Ok(GroupIteration::EndGroups)
}
