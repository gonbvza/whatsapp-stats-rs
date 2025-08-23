use std::path::Path;
use whatsapp_stats::parser::Parser;

#[cfg(test)]
mod tests {
    use whatsapp_stats::message::Message;

    use super::*;

    #[test]
    fn read_file_test() {
        let expected_test: String = String::from(
            "Lorem Ipsum is simply dummy text of the\nprinting and typesetting industry. Lorem\n",
        );

        let parser = Parser::new(Path::new("./tests/mocks/test.txt"));
        let content = parser.read_file();
        assert_eq!(expected_test, content)
    }

    #[test]
    fn test_parse() {
        let test_message_1 = Message {
            owner: "test_user".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:37".to_string(),
            text: "Hello".to_string(),
        };
        let test_message_2 = Message {
            owner: "test_user".to_string(),
            date: "3/5/22".to_string(),
            hour: "20:37".to_string(),
            text: "GoodBye".to_string(),
        };

        let expected_test: Vec<Message> = vec![test_message_1, test_message_2];
        let parser = Parser::new(Path::new("./tests/mocks/mock_messages.txt"));

        let parsed_array: Vec<Message> = parser.parse().unwrap();
        assert_eq!(expected_test, parsed_array)
    }
}
