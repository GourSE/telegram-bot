use crate::config::{BOT_TOKEN};
use reqwest;
use colored::Colorize;

#[tokio::main]
pub async fn get_me() -> Result<(String), reqwest::Error> {

    let url = format!("https://api.telegram.org/bot{}/getMe", &BOT_TOKEN);

    let res = reqwest::get(&url).await?
    .text()
    .await?;

    Ok((res))
}

#[tokio::main]
pub async fn get_updates() -> Result<(String), reqwest::Error> {

    let url = format!("https://api.telegram.org/bot{}/getUpdates", &BOT_TOKEN);

    let res = reqwest::get(&url).await?
    .text()
    .await?;

    println!("{}", res);
    
    Ok((res))
}

#[tokio::main]
pub async fn send_message(chat_id: &str, text: &str, reply_to_message_id: &str, is_markdown: bool) -> Result<(), reqwest::Error> {

    let mut reply: String = "".to_string();
    let mut markdown: &str = "";

    if reply_to_message_id != "" {
        reply = format!("&reply_to_message_id={}&allow_sending_without_reply=True", reply_to_message_id);
    }

    if is_markdown {
        markdown = "&parse_mode=MarkdownV2";
    }

    let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}{}{}", &BOT_TOKEN, chat_id, text, reply, markdown);

    let res = reqwest::get(&url).await?
    .text()
    .await?;    
    
    Ok(())
}

#[tokio::main]
pub async fn forward_message(chat_id: &str, from_chat_id: &str, message_id: &str) -> Result<(), reqwest::Error> {

    let url = format!("https://api.telegram.org/bot{}/forwardMessage?chat_id={}form_chat_id={}&message_id={}", &BOT_TOKEN, chat_id, from_chat_id, message_id);

    let res = reqwest::get(&url).await?
    .text()
    .await?;
    
    Ok(())
}

#[tokio::main]
pub async fn copy_message(
        chat_id: &str, 
        from_chat_id: &str, 
        message_id: &str, 
        caption: &str, 
        is_markdown: bool, 
        reply_to_message_id: &str
    ) -> Result<(), reqwest::Error> {

    let mut reply: String = "".to_string();
    let mut markdown: &str = "";
    let mut _caption: String = "".to_string();

    if reply_to_message_id != "" {
        reply = format!("&reply_to_message_id={}&allow_sending_without_reply=True", reply_to_message_id);
    }

    if is_markdown {
        markdown = "&parse_mode=MarkdownV2";
    }

    if caption != "" {
        _caption = format!("&caption={}", &caption);
    }

    let url = format!(
        "https://api.telegram.org/bot{}/copyMessage?chat_id={}form_chat_id={}&message_id={}{}{}{}", 
        &BOT_TOKEN, 
        chat_id, 
        from_chat_id, 
        message_id, 
        caption, 
        markdown, 
        _caption
    );

    let res = reqwest::get(&url).await?
    .text()
    .await?;    
    
    Ok(())
}