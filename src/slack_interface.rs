use curl::easy::{Easy, List};
use serde_json::{Result, Value};

pub mod user_interface;
pub mod channel_interface;

// pub struct User{
//     id: String,
//     pub name: String,
// }

pub fn get(url: &str, token: &str) -> Result<(Value)> {
    // Send request to Slack API
    let mut handle = Easy::new();
    handle.url(url).unwrap();

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

    Ok(json_rsp)
}

pub fn get_channel_list(token: &str) -> Result<()> {
    // Send request to Slack API
    let mut handle = Easy::new();
    handle.url("https://slack.com/api/conversations.list").unwrap();

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
    println!("{:?}", json_rsp);

    Ok(())
}