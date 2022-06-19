use serde_json::{Result, Value};

use crate::slack_interface;

pub struct Message {
    text: String,
    channel: String,
    username: String,
    ts: u64,
    // icon_emoji: String,
    // attachments: Vec<attachment>,
}

pub fn get_channel_messages(channel_id: &str, oauth_token: &str) -> Result<Vec<Message>> {
    let url = "https://slack.com/api/channels.history";
    let mut params: Vec<String> = Vec::new();
    params.push(channel_id.to_string());
    let json_res = slack_interface::get(url, oauth_token, Some(params)).expect("Get channel messages expect");
    let res = parse_messages(&json_res).expect("parse messages expect");

    Ok(res)
}

fn parse_messages(json_rsp: &Value) -> Result<Vec<Message>>{
    let messages = json_rsp["messages"].as_array().unwrap();
    let messages = messages.iter().map(|message| {
        let text = message["text"].as_str().unwrap();
        let channel = message["channel"].as_str().unwrap();
        let username = message["username"].as_str().unwrap();
        let ts = message["ts"].as_str().unwrap();
        // let icon_emoji = message["icon_emoji"].as_str().unwrap();
        // let attachments = message["attachments"].as_array().unwrap();
        Message{
            text: text.to_string(),
            channel: channel.to_string(),
            username: username.to_string(),
            ts: ts.to_string().parse::<u64>().unwrap(),
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