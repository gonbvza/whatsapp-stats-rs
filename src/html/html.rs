use askama::Template;
use std::fs;

use crate::{
    message::Message,
    stats::{
        average_messages_per_user, average_words_per_message, longest_message_length,
        messages_per_user, most_active_hour, top_speaker_per_hour, total_word_count, words_sent,
    },
};

/// Template context for rendering the dashboard.
/// This struct maps directly to the variables available inside `index.html`.
#[derive(Template)]
#[template(path = "index.html")]
struct DashboardTemplate<'a> {
    names: &'a [String],
    values: &'a [i32],
    words: &'a [String],
    words_count: &'a [i64],
    left_schedule: &'a [(String, String)],
    right_schedule: &'a [(String, String)],
    words_sent: &'a usize,
    messages_sent: &'a usize,
    active_user: &'a String,
    active_hour: &'a String,
    longest_message: &'a usize,
    average_word: &'a f64,
    average_message: &'a f64,
}

/// Prepares sorted user activity data (names and counts).
fn prepare_user_activity(messages: &[Message]) -> (Vec<String>, Vec<i32>) {
    let user_activity = messages_per_user(&messages).unwrap();
    let mut data: Vec<(String, i32)> = user_activity
        .into_iter()
        .map(|(k, v)| (k, v as i32))
        .collect();
    data.sort_by(|a, b| b.1.cmp(&a.1));
    data.into_iter().unzip()
}

/// Prepares the top-N most used words and their counts.
fn prepare_word_frequencies(messages: &[Message], top_n: usize) -> (Vec<String>, Vec<i64>) {
    let mut word_counts: Vec<_> = total_word_count(messages).unwrap().into_iter().collect();
    word_counts.sort_by(|a, b| b.1.cmp(&a.1));
    word_counts
        .into_iter()
        .take(top_n)
        .map(|(word, count)| (word, count))
        .unzip()
}

/// Prepares two halves (0–11, 12–23) of the top speaker schedule by hour.
fn prepare_top_speaker_schedule(
    messages: &[Message],
) -> (Vec<(String, String)>, Vec<(String, String)>) {
    let mut entries: Vec<_> = top_speaker_per_hour(messages)
        .unwrap()
        .into_iter()
        .collect();
    entries.sort_by_key(|(hour, _)| hour.parse::<i64>().unwrap_or(0));
    let left = entries[0..12].to_vec();
    let right = entries[12..24].to_vec();
    (left, right)
}

/// Generates the dashboard HTML and writes it to `output/index.html`.
/// This function aggregates statistics, prepares the template context,
/// and renders the final dashboard using Askama.
pub fn generate_html(messages: &[Message]) -> Result<(), Box<dyn std::error::Error>> {
    let (names, values) = prepare_user_activity(messages);
    let (words, words_count) = prepare_word_frequencies(messages, 20);
    let (left_schedule, right_schedule) = prepare_top_speaker_schedule(messages);

    let words_sent = words_sent(messages).unwrap();
    let messages_sent = messages.len();
    let active_user = &names[0];
    let active_hour = most_active_hour(messages).unwrap();
    let average_message: f64 = average_messages_per_user(messages).unwrap().round();
    let average_word: f64 = average_words_per_message(messages).unwrap().round();
    let longest_message: usize = longest_message_length(messages).unwrap();

    let template = DashboardTemplate {
        names: &names,
        values: &values,
        words: &words,
        words_count: &words_count,
        left_schedule: &left_schedule,
        right_schedule: &right_schedule,
        words_sent: &words_sent,
        messages_sent: &messages_sent,
        active_user,
        active_hour: &active_hour,
        average_message: &average_message,
        average_word: &average_word,
        longest_message: &longest_message,
    };

    fs::create_dir_all("./output")?;
    fs::write("output/index.html", template.render()?)?;
    Ok(())
}
