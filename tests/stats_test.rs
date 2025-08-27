#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use whatsapp_stats::{
        message::Message,
        stats::{
            average_messages_per_user, average_words_per_message, count_phrase_per_user,
            count_word_per_user, extract_word_count, longest_message_length, most_active_hour,
            top_speaker_per_hour, total_word_count, words_sent,
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
        assert_eq!(counts.get("hello"), Some(&(2 as i64))); // "Hello" + "hello"
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
        assert_eq!(count, 3);
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
        assert_eq!(top_speakers.get("09"), Some(&"Alice".to_string()));
        assert_eq!(top_speakers.get("10"), Some(&"Charlie".to_string()));
    }

    #[test]
    fn test_words_sent() {
        let messages = vec![
            Message {
                owner: "Alice".to_string(),
                date: "1/1/22".to_string(),
                hour: "12:00".to_string(),
                text: "Hello world".to_string(),
            },
            Message {
                owner: "Bob".to_string(),
                date: "1/1/22".to_string(),
                hour: "12:10".to_string(),
                text: "Hi Alice, how are you?".to_string(),
            },
        ];
        let total = words_sent(&messages[..]).unwrap();
        assert_eq!(total, 7);
    }

    #[test]
    fn test_most_active_hour() {
        let messages = vec![
            Message {
                owner: "Alice".to_string(),
                date: "1/1/22".to_string(),
                hour: "09:05".to_string(),
                text: "Morning".to_string(),
            },
            Message {
                owner: "Bob".to_string(),
                date: "1/1/22".to_string(),
                hour: "09:15".to_string(),
                text: "Hello".to_string(),
            },
            Message {
                owner: "Alice".to_string(),
                date: "1/1/22".to_string(),
                hour: "10:00".to_string(),
                text: "Later".to_string(),
            },
        ];
        let peak = most_active_hour(&messages[..]).unwrap();
        assert_eq!(peak, "09".to_string());
    }

    #[test]
    fn test_longest_message_length() {
        let messages = vec![
            Message {
                owner: "Alice".to_string(),
                date: "1/1/22".to_string(),
                hour: "11:00".to_string(),
                text: "Short msg".to_string(),
            },
            Message {
                owner: "Bob".to_string(),
                date: "1/1/22".to_string(),
                hour: "11:10".to_string(),
                text: "This is a much longer message than the first one".to_string(),
            },
        ];
        let longest = longest_message_length(&messages[..]).unwrap();
        assert_eq!(longest, 10);
    }

    #[test]
    fn test_average_words_per_message() {
        let messages = vec![
            Message {
                owner: "Alice".to_string(),
                date: "1/1/22".to_string(),
                hour: "12:00".to_string(),
                text: "Hi".to_string(),
            },
            Message {
                owner: "Bob".to_string(),
                date: "1/1/22".to_string(),
                hour: "12:10".to_string(),
                text: "Hello world!".to_string(),
            },
        ];
        let avg = average_words_per_message(&messages[..]).unwrap();
        assert!((avg - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_average_messages_per_user() {
        let messages = vec![
            Message {
                owner: "Alice".to_string(),
                date: "1/1/22".to_string(),
                hour: "13:00".to_string(),
                text: "Hi".to_string(),
            },
            Message {
                owner: "Bob".to_string(),
                date: "1/1/22".to_string(),
                hour: "13:10".to_string(),
                text: "Hey".to_string(),
            },
            Message {
                owner: "Alice".to_string(),
                date: "1/1/22".to_string(),
                hour: "13:20".to_string(),
                text: "How are you?".to_string(),
            },
        ];
        let avg = average_messages_per_user(&messages[..]).unwrap();
        // Alice has 2, Bob has 1 → total 3 / 2 users = 1.5
        assert!((avg - 1.5).abs() < f64::EPSILON);
    }
}
