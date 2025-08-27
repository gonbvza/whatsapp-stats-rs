use clap::{Arg, Command};
use std::path::Path;
use whatsapp_stats::{
    display::{pretty_print_top_speakers, print_hashmap},
    html::html::generate_html,
    message::Message,
    parser::Parser,
    stats::{
        count_phrase_per_user, count_word_per_user, extract_word_count, messages_per_user,
        top_speaker_per_hour, total_word_count,
    },
};

fn main() {
    let matches = Command::new("whatsapp-stats")
        .version("1.0")
        .about("Blazingly fast WhatsApp chat analyzer in Rust 🚀")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Path to WhatsApp export file")
                .required(true),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("word")
                .about("Count occurrences of a single word per user")
                .arg(Arg::new("target").required(false)),
        )
        .subcommand(
            Command::new("phrase")
                .about("Count occurrences of a phrase per user")
                .arg(Arg::new("target").required(false)),
        )
        .subcommand(
            Command::new("word-count")
                .about("Count occurrences of a word across all messages")
                .arg(Arg::new("target").required(false)),
        )
        .subcommand(
            Command::new("total-word-count").about("Count total word frequency across the chat"),
        )
        .subcommand(Command::new("top-speakers").about("Find top speakers per hour"))
        .subcommand(Command::new("user-activity").about("Messages sent by each user"))
        .subcommand(Command::new("html").about("Generate html"))
        .get_matches();

    let file_path: &str = matches.get_one::<String>("file").unwrap();
    let parser = Parser::new(Path::new(file_path));
    let messages: Vec<Message> = parser.parse().unwrap();

    match matches.subcommand() {
        Some(("word", sub)) => {
            let word = sub.get_one::<String>("target").unwrap();
            let stats = count_word_per_user(&messages, word).unwrap();
            println!("The times \"{}\" was said is:", word);
            print_hashmap(stats);
        }
        Some(("phrase", sub)) => {
            let phrase = sub.get_one::<String>("target").unwrap();
            let stats = count_phrase_per_user(&messages, phrase).unwrap();
            println!("The times \"{}\" was said is:", phrase);
            print_hashmap(stats);
        }
        Some(("word-count", sub)) => {
            let word = sub.get_one::<String>("target").unwrap();
            extract_word_count(&messages, word).unwrap();
        }
        Some(("total-word-count", _)) => {
            let total = total_word_count(&messages).unwrap();
            print_hashmap(total);
        }
        Some(("top-speakers", _)) => {
            let hour_speakers = top_speaker_per_hour(&messages).unwrap();
            pretty_print_top_speakers(&hour_speakers);
        }
        Some(("user-activity", _)) => {
            let user_activity = messages_per_user(&messages).unwrap();
            print_hashmap(user_activity);
        }
        Some(("html", _)) => {
            generate_html(&messages[..]).unwrap();
        }

        _ => unreachable!(),
    }
}
