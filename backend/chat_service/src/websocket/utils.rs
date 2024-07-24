use log::info;

/// Parses the query string and extracts session data.
///
/// # Arguments
///
/// * `query_string` - The query string containing the session data.
///
/// # Returns
///
/// A tuple containing the sender ID, conversation member IDs, and conversation ID.
///
/// # Panics
///
/// This function will panic if the user IDs in the query string are invalid or if the
/// conversation IDs are invalid or missing.
///
/// # Examples
///
/// ```
/// let query_string = "sender_id=1&receiver_id=2&conversation_id=3";
/// let (sender_id, conversation_member_ids, conversation_id) = get_session_data(query_string);
/// assert_eq!(sender_id, 1);
/// assert_eq!(conversation_member_ids, vec![1, 2]);
/// assert_eq!(conversation_id, Some(3));
/// ```

pub fn get_session_data(query_string: &str) -> (i32, Vec<i32>, Option<i32>) {
    let params = serde_urlencoded::from_str::<Vec<(String, String)>>(query_string)
        .expect("Failed to parse query string");
    info!("Query: {}", query_string);
    info!("Params: {:?}", params);

    let mut sender_id = 0;
    let mut conversation_member_ids: Vec<i32> = Vec::new();
    let mut conversation_id: Option<i32> = None;

    for (key, value) in params {
        if key == "sender_id" {
            sender_id = value.parse::<i32>().expect("Invalid user ID");
            conversation_member_ids.push(sender_id);
        } else if key == "receiver_id" {
            let user_id: i32 = value.parse().expect("Invalid receiver ID");
            conversation_member_ids.push(user_id);
        } else if key == "conversation_id" {
            conversation_id = match value.parse() {
                Ok(id) => Some(id),
                Err(_) => None,
            };
        }
    }
    info!(
        "Sender ID: {}, Conversation Member IDs: {:?}, Conversation ID: {:?}",
        sender_id, conversation_member_ids, conversation_id
    );

    if sender_id == 0 {
        panic!("Invalid user IDs in query string");
    } else if conversation_member_ids.len() < 2 && conversation_id.is_none() {
        panic!("Invalid user IDs or conversation ID in query string");
    }

    (sender_id, conversation_member_ids, conversation_id)
}
