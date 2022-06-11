use curl::easy::{Easy, List};
use serde_json::{Result, Value};

pub struct User{
    id: String,
    pub name: String,
}

pub fn get_user_list(token: &str) -> Result<Vec<User>> {
    // Send request to Slack API
    let mut handle = Easy::new();
    handle.url("https://slack.com/api/users.list").unwrap();

    let mut list = List::new();
    list.append(&("Authorization: Bearer ".to_string() + token)).unwrap();
    handle.http_headers(list).unwrap(); 
    
    // TODO: Handle errors
    // Handle response
    let mut rsp = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            rsp.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    // Check response
    let response = handle.response_code().unwrap();
    match response {
        200 => {
            // println!("{}", response);
        },
        _ => {
            println!("Error");
        }
    }
    
    // Parse response
    let json_rsp: Value = serde_json::from_slice(&rsp).unwrap();

    Ok(parse_user_list(&json_rsp).expect("parse user list expect"))
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