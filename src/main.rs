use clap::{arg, Arg, ArgAction, Command};
use std::env;
use std::{collections::HashMap, path::Path, usize};

use whatsapp_stats::{
    db::DatabaseHandler,
    display::{pretty_print_top_speakers, print_hashmap},
    message::Message,
    parser::Parser,
    stats::{
        count_phrase_per_user, count_word_per_user, extract_word_count, top_speaker_per_hour,
        total_word_count,
    },
};

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .about("Does awesome things")
        .arg(arg!(--file <VALUE>).required(true))
        .arg(arg!(--word <VALUE>).required(false))
        .arg(arg!(--phrase <VALUE>).required(false))
        .arg(arg!(--word_count <VALUE>).required(false))
        .arg(
            Arg::new("message_amount")
                .short('a')
                .long("message_amount")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("time")
                .short('t')
                .long("time")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("total_word_count")
                .short('w')
                .long("total_word_count")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let file_path: &str = matches.get_one::<String>("file").expect("required");

    let db = DatabaseHandler::new().unwrap();
    let parser = Parser::new(Path::new(file_path));

    let messages: Vec<Message> = parser.parse().unwrap();
    db.insert_messages(&messages).unwrap();

    if let Some(word) = matches.get_one::<String>("word") {
        let stats = count_word_per_user(&messages, word).unwrap();
        println!("The times \"{}\" was said is: ", word);

        print_hashmap(stats);
    }

    if let Some(word) = matches.get_one::<String>("phrase") {
        let stats = count_phrase_per_user(&messages, word).unwrap();
        println!("The times \"{}\" was said is: ", word);
        print_hashmap(stats);
    }

    if let Some(word) = matches.get_one::<String>("word_count") {
        extract_word_count(&messages, word).unwrap();
    }

    if matches.get_flag("time") {
        let hour_speakers = top_speaker_per_hour(&messages[..]).unwrap();
        pretty_print_top_speakers(&hour_speakers);
    }

    if matches.get_flag("total_word_count") {
        let total_word_count = total_word_count(&messages[..]).unwrap();
        print_hashmap(total_word_count);
    }
}
