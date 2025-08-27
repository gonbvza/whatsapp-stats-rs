use crate::{errors::DatabaseError, message::Message, utils::time_utils::get_hour};
use std::collections::HashMap;

/// Counts how many times a specific word appears per user across all messages.
/// The comparison is case-insensitive and matches whole words only.
pub fn count_word_per_user(
    messages: &[Message],
    target: &str,
) -> Result<HashMap<String, i64>, DatabaseError> {
    let mut counts: HashMap<String, i64> = HashMap::new();
    let target_lower = target.to_lowercase();
    for message in messages {
        if message
            .text
            .to_lowercase()
            .split_whitespace()
            .any(|word| word == target_lower)
        {
            *counts.entry(message.owner.clone()).or_insert(0) += 1;
        }
    }
    Ok(counts)
}

/// Counts how many times a specific phrase appears per user across all messages.
/// The comparison is case-insensitive and matches substrings.
pub fn count_phrase_per_user(
    messages: &[Message],
    target: &str,
) -> Result<HashMap<String, i64>, DatabaseError> {
    let mut counts: HashMap<String, i64> = HashMap::new();
    let target_lower = target.to_lowercase();
    for message in messages {
        if message.text.to_lowercase().contains(&target_lower) {
            *counts.entry(message.owner.clone()).or_insert(0) += 1;
        }
    }
    Ok(counts)
}

/// Builds a frequency map of all words across all messages.
/// Words are converted to lowercase to normalize the results.
pub fn total_word_count(messages: &[Message]) -> Result<HashMap<String, i64>, DatabaseError> {
    let mut counts: HashMap<String, i64> = HashMap::new();
    for message in messages {
        for word in message.text.split_whitespace() {
            *counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }
    Ok(counts)
}

/// Counts how many times a specific word appears in the entire message set.
/// The comparison is case-insensitive and matches whole words only.
pub fn extract_word_count(messages: &[Message], target: &str) -> Result<usize, DatabaseError> {
    let mut count = 0;
    for message in messages {
        for word in message.text.split_whitespace() {
            if word.eq_ignore_ascii_case(target) {
                count += 1;
            }
        }
    }
    Ok(count)
}

/// Finds the top speaker for each hour by counting user messages per hour.
/// Returns a map of hour (HH format) to the username who spoke the most in that hour.
pub fn top_speaker_per_hour(
    messages: &[Message],
) -> Result<HashMap<String, String>, DatabaseError> {
    let mut hour_to_user_counts: HashMap<String, HashMap<String, i64>> = HashMap::new();
    for message in messages {
        if let Some(hour) = message.hour.split(':').next() {
            let user_counts = hour_to_user_counts
                .entry(hour.to_lowercase())
                .or_insert_with(HashMap::new);
            *user_counts.entry(message.owner.clone()).or_insert(0) += 1;
        }
    }
    let mut top_speakers: HashMap<String, String> = HashMap::new();
    for (hour, user_counts) in hour_to_user_counts {
        if let Some((user, _)) = user_counts.into_iter().max_by_key(|(_, count)| *count) {
            top_speakers.insert(hour, user);
        }
    }
    Ok(top_speakers)
}

/// Counts the total number of words sent across all messages in the dataset.
/// Uses whitespace splitting to account for multiple spaces.
pub fn words_sent(messages: &[Message]) -> Result<usize, DatabaseError> {
    let total = messages
        .iter()
        .map(|m| m.text.split_whitespace().count())
        .sum();
    Ok(total)
}

/// Finds the hour of the day with the highest number of messages sent.
/// Returns the hour in lowercase string form (e.g. "14" for 2 PM).
pub fn most_active_hour(messages: &[Message]) -> Result<String, DatabaseError> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for message in messages {
        let hour = get_hour(&message.hour);
        *counts.entry(hour.to_lowercase()).or_insert(0) += 1;
    }
    if let Some((peak_hour, _)) = counts.iter().max_by_key(|(_, count)| *count) {
        Ok(peak_hour.to_owned())
    } else {
        Ok(String::new())
    }
}

/// Returns the length (in words) of the single longest message in the dataset.
/// If no messages are provided, returns 0.
pub fn longest_message_length(messages: &[Message]) -> Result<usize, DatabaseError> {
    if let Some((_, word_count)) = messages
        .iter()
        .map(|m| (m, m.text.split_whitespace().count()))
        .max_by_key(|&(_, count)| count)
    {
        Ok(word_count)
    } else {
        Ok(0)
    }
}

/// Computes the average number of words per message across all users.
/// Returns 0.0 if there are no messages.
pub fn average_words_per_message(messages: &[Message]) -> Result<f64, DatabaseError> {
    if messages.is_empty() {
        return Ok(0.0);
    }
    let total_words: usize = messages
        .iter()
        .map(|m| m.text.split_whitespace().count())
        .sum();
    Ok(total_words as f64 / messages.len() as f64)
}

/// Computes the average number of messages sent per user across all messages.
/// Returns 0.0 if no messages are provided.
pub fn average_messages_per_user(messages: &[Message]) -> Result<f64, DatabaseError> {
    if messages.is_empty() {
        return Ok(0.0);
    }
    let mut user_counts: HashMap<&str, usize> = HashMap::new();
    for message in messages {
        *user_counts.entry(&message.owner).or_insert(0) += 1;
    }
    let total_messages = messages.len();
    let unique_users = user_counts.len();
    Ok(total_messages as f64 / unique_users as f64)
}

// Counts the total number of messages sent by each user.
pub fn messages_per_user(messages: &[Message]) -> Result<HashMap<String, i64>, DatabaseError> {
    let mut counts: HashMap<String, i64> = HashMap::new();
    for message in messages {
        *counts.entry(message.owner.clone()).or_insert(0) += 1;
    }
    Ok(counts)
}
