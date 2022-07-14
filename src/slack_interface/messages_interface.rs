use serde_json::{Result, Value};

use crate::slack_interface;

pub struct Message {
    pub text: String,
    // channel: String,
    pub username: String,
    pub message_type: String,
    pub ts: String,
    // icon_emoji: String,
    // attachments: Vec<attachment>,
}

pub fn get_channel_messages(channel_id: &str, oauth_token: &str) -> Result<Vec<Message>> {
    let url = "https://slack.com/api/conversations.history?channel=".to_string() + channel_id;
    let json_res = slack_interface::get(&url, oauth_token).expect("Get channel messages expect");
    let res = parse_messages(&json_res).expect("parse messages expect");

    Ok(res)
}

fn parse_messages(json_rsp: &Value) -> Result<Vec<Message>>{
    //  TODO: Make sure to check if json_rsp is not None when unwrap or it will panic
    let messages = json_rsp["messages"].as_array().unwrap();
    let messages = messages.iter().map(|message| {
        let text = message["text"].as_str().unwrap();
        let username = message["user"].as_str().unwrap();
        let ts = message["ts"].as_str().unwrap();
        let message_type = message["type"].as_str().unwrap();
        // let icon_emoji = message["icon_emoji"].as_str().unwrap();
        // let attachments = message["attachments"].as_array().unwrap();
        Message{
            text: text.to_string(),
            username: username.to_string(),
            ts: ts.to_string(),
            message_type: message_type.to_string(),
            // icon_emoji: icon_emoji.to_string(),
            // attachments: attachments,
        }
    }).collect::<Vec<Message>>();
    
    Ok(messages)
}

// Example response:
// {
//     "ok": true,
//     "messages": [
//         {
//             "type": "message",
//             "user": "U012AB3CDE",
//             "text": "I find you punny and would like to smell your nose letter",
//             "ts": "1512085950.000216"
//         },
//         {
//             "type": "message",
//             "user": "U061F7AUR",
//             "text": "What, you want to smell my shoes better?",
//             "ts": "1512104434.000490"
//         }
//     ],
//     "has_more": true,
//     "pin_count": 0,
//     "response_metadata": {
//         "next_cursor": "bmV4dF90czoxNTEyMDg1ODYxMDAwNTQz"
//     }
// }

pub fn send_channel_message(text: &str, channel_id: &str, oauth_token: &str) -> Result<bool>{
    // let formatted_text = format!("\"{}\"", text);    // Do this to handle spaces
    let url = format!(
        "https://slack.com/api/chat.postMessage?channel={}&text={}",
        channel_id, text.replace(" ", "+"));

    let json_res = slack_interface::get(&url, oauth_token)
        .expect("Get channel messages expect");
    let res = parse_send_channel_message(&json_res).expect("parse messages expect");

    return Ok(res)
}

fn parse_send_channel_message(json_rsp: &Value) -> Result<bool>{
    let rsp = json_rsp["ok"].as_bool().unwrap();
    Ok(rsp)
}

// Example sucess response
// {
//     "ok": true,
//     "channel": "C123456",
//     "ts": "1503435956.000247",
//     "message": {
//         "text": "Here's a message for you",
//         "username": "ecto1",
//         "bot_id": "B123456",
//         "attachments": [
//             {
//                 "text": "This is an attachment",
//                 "id": 1,
//                 "fallback": "This is an attachment's fallback"
//             }
//         ],
//         "type": "message",
//         "subtype": "bot_message",
//         "ts": "1503435956.000247"
//     }
// }