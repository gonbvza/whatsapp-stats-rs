#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use whatsapp_stats::{
        message::Message,
        stats::{
            count_phrase_per_user, count_word_per_user, extract_word_count, top_speaker_per_hour,
            total_word_count,
        },
    };

    #[test]
    fn test_count_words() {
        let message_1 = Message {
            owner: "test_user".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:37".to_string(),
            text: "World Hello".to_string(),
        };
        let message_2 = Message {
            owner: "test_user".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:37".to_string(),
            text: "Hello sir".to_string(),
        };
        let messages_array = vec![message_1, message_2];
        let count: HashMap<String, i64> =
            count_word_per_user(&messages_array[..], "Hello").unwrap();
        assert_eq!(count.get("test_user"), Some(&(2 as i64)));
    }

    #[test]
    fn test_count_phrase_per_user() {
        let message_1 = Message {
            owner: "Alice".to_string(),
            date: "3/5/22".to_string(),
            hour: "10:00".to_string(),
            text: "Rust is awesome".to_string(),
        };
        let message_2 = Message {
            owner: "Bob".to_string(),
            date: "3/5/22".to_string(),
            hour: "10:05".to_string(),
            text: "I think Rust is awesome too".to_string(),
        };
        let messages_array = vec![message_1, message_2];

        let counts = count_phrase_per_user(&messages_array[..], "Rust is awesome").unwrap();

        assert_eq!(counts.get("Alice"), Some(&(1 as i64)));
        assert_eq!(counts.get("Bob"), Some(&(1 as i64)));
    }

    #[test]
    fn test_total_word_count() {
        let message_1 = Message {
            owner: "Alice".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:37".to_string(),
            text: "Hello world".to_string(),
        };
        let message_2 = Message {
            owner: "Bob".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:38".to_string(),
            text: "hello Rust".to_string(),
        };
        let messages_array = vec![message_1, message_2];

        let counts = total_word_count(&messages_array[..]).unwrap();

        assert_eq!(counts.get("hello"), Some(&(2 as i64))); // "Hello" and "hello" counted together
        assert_eq!(counts.get("world"), Some(&(1 as i64)));
        assert_eq!(counts.get("rust"), Some(&(1 as i64)));
    }

    #[test]
    fn test_extract_word_count() {
        let message_1 = Message {
            owner: "Alice".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:37".to_string(),
            text: "Hello world hello".to_string(),
        };
        let message_2 = Message {
            owner: "Bob".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:38".to_string(),
            text: "hello Rust".to_string(),
        };
        let messages_array = vec![message_1, message_2];

        let count = extract_word_count(&messages_array[..], "hello").unwrap();

        assert_eq!(count, 3); // "Hello" + "hello" + "hello"
    }

    #[test]
    fn test_top_speaker_per_hour() {
        let message_1 = Message {
            owner: "Alice".to_string(),
            date: "3/5/22".to_string(),
            hour: "09:00".to_string(),
            text: "Hi there".to_string(),
        };
        let message_2 = Message {
            owner: "Bob".to_string(),
            date: "3/5/22".to_string(),
            hour: "09:15".to_string(),
            text: "Hello Alice".to_string(),
        };
        let message_3 = Message {
            owner: "Alice".to_string(),
            date: "3/5/22".to_string(),
            hour: "09:30".to_string(),
            text: "How are you?".to_string(),
        };
        let message_4 = Message {
            owner: "Charlie".to_string(),
            date: "3/5/22".to_string(),
            hour: "10:00".to_string(),
            text: "Good morning".to_string(),
        };

        let messages_array = vec![message_1, message_2, message_3, message_4];

        let top_speakers = top_speaker_per_hour(&messages_array[..]).unwrap();

        assert_eq!(top_speakers.get("09"), Some(&"Alice".to_string())); // Alice spoke 2x at 09
        assert_eq!(top_speakers.get("10"), Some(&"Charlie".to_string()));
    }
}
