use std::collections::HashMap;

pub fn print_hashmap(stats: HashMap<String, i64>) {
    let mut sorted: Vec<_> = stats.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    for (owner, count) in sorted {
        println!("{}: {}", owner, count);
    }
}

pub fn pretty_print_top_speakers(top_speakers: &HashMap<String, String>) {
    println!("Top speaker per hour:");
    let mut entries: Vec<_> = top_speakers.iter().collect();
    entries.sort_by_key(|(hour, _)| hour.parse::<i64>().unwrap_or(0));

    for (hour, user) in entries {
        println!("{} -> {}", hour, user);
    }
}
