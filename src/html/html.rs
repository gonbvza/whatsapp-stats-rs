use askama::Template;
use std::{collections::HashMap, fs, iter::zip};

use crate::{
    message::Message,
    stats::{
        average_messages_per_user, average_words_per_message, longest_message_length,
        most_active_hour, top_speaker_per_hour, total_word_count, words_sent,
    },
};

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

pub fn generate_html(
    user_activity: HashMap<String, i64>,
    messages_array: &[Message],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data: Vec<(String, i32)> = user_activity
        .into_iter()
        .map(|(k, v)| (k, v as i32))
        .collect();
    data.sort_by(|a, b| b.1.cmp(&a.1));
    let (names, values): (Vec<String>, Vec<i32>) = data.into_iter().unzip();

    let total_word_count: HashMap<String, i64> = total_word_count(&messages_array).unwrap();
    let mut total_word_count: Vec<_> = total_word_count.iter().collect();
    total_word_count.sort_by(|a, b| b.1.cmp(&a.1));

    let (words, words_count): (Vec<String>, Vec<i64>) = total_word_count[0..20]
        .iter()
        .map(|(words, words_count)| ((*words).clone(), (*words_count).clone()))
        .collect();

    let top_speakers = top_speaker_per_hour(messages_array).unwrap();
    let mut entries: Vec<_> = top_speakers.iter().collect();
    entries.sort_by_key(|(hour, _)| hour.parse::<i64>().unwrap_or(0));

    let left_schedule: Vec<(String, String)> = entries[0..12]
        .iter()
        .map(|(hour, name)| ((*hour).clone(), (*name).clone()))
        .collect();
    let right_schedule: Vec<(String, String)> = entries[12..24]
        .iter()
        .map(|(hour, name)| ((*hour).clone(), (*name).clone()))
        .collect();

    let words_sent = words_sent(&messages_array).unwrap();
    let messages_sent = messages_array.len();
    let active_user = &names[0];
    let active_hour = most_active_hour(messages_array).unwrap();
    let average_message: f64 = average_messages_per_user(messages_array).unwrap().round();
    let average_word: f64 = average_words_per_message(messages_array).unwrap().round();
    let longest_message: usize = longest_message_length(messages_array).unwrap();

    let template = DashboardTemplate {
        names: &names,
        values: &values,
        words: &words,
        words_count: &words_count,
        left_schedule: &left_schedule[..],
        right_schedule: &right_schedule[..],
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
