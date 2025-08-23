use std::collections::HashMap;

use crate::{errors::DatabaseError, message::Message};

pub fn count_word_per_user(
    messages_array: &[Message],
    target: &str,
) -> Result<HashMap<String, i64>, DatabaseError> {
    let mut count_hashmap: HashMap<String, i64> = HashMap::new();
    let target_lower = target.to_lowercase();

    for message in messages_array {
        if message
            .text
            .to_lowercase()
            .split_whitespace()
            .any(|word| word == target_lower)
        {
            *count_hashmap.entry(message.owner.clone()).or_insert(0) += 1;
        }
    }

    let mut vec: Vec<(&String, &i64)> = count_hashmap.iter().collect();

    vec.sort_by(|a, b| b.1.cmp(a.1));

    Ok(count_hashmap)
}

pub fn count_phrase_per_user(
    messages_array: &[Message],
    target: &str,
) -> Result<HashMap<String, i64>, DatabaseError> {
    let mut count_hashmap: HashMap<String, i64> = HashMap::new();
    let target_lower = target.to_lowercase();

    for message in messages_array {
        if message.text.to_lowercase().contains(&target_lower) {
            *count_hashmap.entry(message.owner.clone()).or_insert(0) += 1;
        }
    }

    let mut vec: Vec<(&String, &i64)> = count_hashmap.iter().collect();

    vec.sort_by(|a, b| b.1.cmp(a.1));

    Ok(count_hashmap)
}

pub fn total_word_count(messages_array: &[Message]) -> Result<HashMap<String, i64>, DatabaseError> {
    let mut count_hashmap: HashMap<String, i64> = HashMap::new();
    for message in messages_array {
        message
            .text
            .split(" ")
            .for_each(|x| *count_hashmap.entry(x.to_lowercase()).or_insert(0) += 1);
    }

    //Ok(count_hashmap)
    Ok(count_hashmap)
}

pub fn extract_word_count(
    messages_array: &[Message],
    target: &str,
) -> Result<usize, DatabaseError> {
    let mut count: usize = 0;
    for message in messages_array {
        message.text.split(" ").for_each(|x| {
            if x.to_lowercase() == target {
                count += 1
            }
        });
    }

    println!(
        "The word {} has been said {} times",
        target.to_string(),
        count
    );

    //Ok(count_hashmap)
    Ok(count)
}

pub fn top_speaker_per_hour(
    messages_array: &[Message],
) -> Result<HashMap<String, String>, DatabaseError> {
    // Outer hashmap groups by hour
    let mut hour_to_user_counts: HashMap<String, HashMap<String, i64>> = HashMap::new();

    for message in messages_array {
        let hour = message.hour.split(':').next().unwrap();
        let owner = message.owner.clone();

        // Get or insert the inner hashmap for this hour
        let user_counts = hour_to_user_counts
            .entry(hour.to_lowercase())
            .or_insert_with(HashMap::new);

        // Increase the count for this user in that hour
        *user_counts.entry(owner).or_insert(0) += 1;
    }

    // Now, for each hour, pick the user with the max count
    let mut top_speakers: HashMap<String, String> = HashMap::new();

    for (hour, user_counts) in hour_to_user_counts {
        if let Some((user, _)) = user_counts.into_iter().max_by_key(|(_, count)| *count) {
            top_speakers.insert(hour, user);
        }
    }

    Ok(top_speakers)
}
