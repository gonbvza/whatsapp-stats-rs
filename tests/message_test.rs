#[cfg(test)]
mod tests {
    use whatsapp_stats::message::Message;

    #[test]
    fn test_parse_one_message() {
        let input = "[3/5/22, 20:37] test_user: Hello";
        let expected = Message {
            owner: "test_user".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:37".to_string(),
            text: "Hello".to_string(),
        };
        let parsed = Message::new(input).unwrap();
        assert_eq!(expected, parsed);
    }
}
