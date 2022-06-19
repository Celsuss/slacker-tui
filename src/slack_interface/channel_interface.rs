use curl::easy::{Easy, List};
use serde_json::{Result, Value};

use crate::slack_interface;

pub struct Channel{
    pub id: String,
    pub name: String,
}

pub fn get_channel_list(token: &str) -> Result<Vec<Channel>> {
    // Send request to Slack API
    let rsp = slack_interface::get("https://slack.com/api/conversations.list", token, None).expect("Get channel list expect");
    let json_res = parse_channel_list(&rsp).expect("parse channel list expect");

    Ok(json_res)
}

fn parse_channel_list(json_rsp: &Value) -> Result<Vec<Channel>> {
    let channels_json = json_rsp["channels"].as_array().unwrap();
    let channels = channels_json.iter().map(|channel| {
        let id = channel["id"].as_str().unwrap();
        let name = channel["name"].as_str().unwrap();
        Channel{
            id: id.to_string(),
            name: name.to_string(),
        }
    }).collect::<Vec<Channel>>();

    Ok(channels)
}

// Example response:
// {
//     "ok": true,
//     "channels": [
//         {
//             "id": "C012AB3CD",
//             "name": "general",
//             "is_channel": true,
//             "is_group": false,
//             "is_im": false,
//             "created": 1449252889,
//             "creator": "U012A3CDE",
//             "is_archived": false,
//             "is_general": true,
//             "unlinked": 0,
//             "name_normalized": "general",
//             "is_shared": false,
//             "is_ext_shared": false,
//             "is_org_shared": false,
//             "pending_shared": [],
//             "is_pending_ext_shared": false,
//             "is_member": true,
//             "is_private": false,
//             "is_mpim": false,
//             "topic": {
//                 "value": "Company-wide announcements and work-based matters",
//                 "creator": "",
//                 "last_set": 0
//             },
//             "purpose": {
//                 "value": "This channel is for team-wide communication and announcements. All team members are in this channel.",
//                 "creator": "",
//                 "last_set": 0
//             },
//             "previous_names": [],
//             "num_members": 4
//         },
//         {
//             "id": "C061EG9T2",
//             "name": "random",
//             "is_channel": true,
//             "is_group": false,
//             "is_im": false,
//             "created": 1449252889,
//             "creator": "U061F7AUR",
//             "is_archived": false,
//             "is_general": false,
//             "unlinked": 0,
//             "name_normalized": "random",
//             "is_shared": false,
//             "is_ext_shared": false,
//             "is_org_shared": false,
//             "pending_shared": [],
//             "is_pending_ext_shared": false,
//             "is_member": true,
//             "is_private": false,
//             "is_mpim": false,
//             "topic": {
//                 "value": "Non-work banter and water cooler conversation",
//                 "creator": "",
//                 "last_set": 0
//             },
//             "purpose": {
//                 "value": "A place for non-work-related flimflam, faffing, hodge-podge or jibber-jabber you'd prefer to keep out of more focused work-related channels.",
//                 "creator": "",
//                 "last_set": 0
//             },
//             "previous_names": [],
//             "num_members": 4
//         }
//     ],
//     "response_metadata": {
//         "next_cursor": "dGVhbTpDMDYxRkE1UEI="
//     }
// }