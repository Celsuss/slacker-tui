use curl::easy::{Easy, List};
use serde_json::{Result, Value};

use crate::slack_interface;

pub fn get_channel_list(token: &str) -> Result<Vec<String>> {
    // Send request to Slack API
    let rsp = slack_interface::get("https://slack.com/api/conversations.list", token).expect("Get channel list expect");
    let json_res = parse_channel_list(&rsp).expect("parse channel list expect");

    Ok(json_res)
}

fn parse_channel_list(json_rsp: &Value) -> Result<Vec<String>> {
    let channels = json_rsp["channels"].as_array().unwrap();
    let channel_names = channels.iter().map(|channel| {
        let name = channel["name"].as_str().unwrap();
        name.to_string()
    }).collect::<Vec<String>>();

    Ok(channel_names)
}