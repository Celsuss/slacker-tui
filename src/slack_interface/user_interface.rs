use curl::easy::{Easy, List};
use serde_json::{Result, Value};

use crate::slack_interface;

pub struct User{
    id: String,
    pub name: String,
}

pub fn get_user_list(token: &str) -> Result<Vec<User>> {
    // Send request to Slack API
    // TODO: Handle errors
    let json_res = slack_interface::get("https://slack.com/api/users.list", token).expect("Get user list expect");
    let json_res_parsed = parse_user_list(&json_res).expect("parse user list expect");

    Ok(json_res_parsed)
}

fn parse_user_list(json_rsp: &Value) -> Result<Vec<User>> {
    let members = json_rsp["members"].as_array().unwrap();
    let users = members.iter().map(|member| {
        let id = member["id"].as_str().unwrap();
        let name = member["name"].as_str().unwrap();
        User{
            id: id.to_string(),
            name: name.to_string(),
        }
    }).collect::<Vec<User>>();

    Ok(users)
}