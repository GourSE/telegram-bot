#![allow(dead_code)]
// basic default settings

// bot token created by bot father in telegram
pub const BOT_TOKEN: &str = "TOKEN";

// should be chat ID for admin
pub const ADMIN_ID: &str = "ID";

// default chat to interact with(sending messages etc)
pub const DEFAULT_CHAT: &str = "ID";

// send typing will be true or false, which sends typing vefore sending costum message
pub const SEND_TYPING: bool = true;

// use markdown for costum messages
pub const USE_MARKDOWN: bool = true;

/* 
to pass message to admin chat
list of options
0: off
1: mention only
2: all messages
*/
pub const ECHO: i32 = 0;