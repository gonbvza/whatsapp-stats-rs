use crate::errors::ParseError;
use crate::message::Message;
use std::fs;
use std::path::Path;
use std::str::Split;

pub struct Parser<'a> {
    filepath: &'a Path,
}

impl<'a> Parser<'a> {
    pub fn new(filepath: &'a Path) -> Self {
        Parser { filepath }
    }

    pub fn read_file(&self) -> String {
        let contents =
            fs::read_to_string(&self.filepath).expect("Should have been able to read the file");
        contents
    }

    pub fn parse(&self) -> Result<Vec<Message>, ParseError> {
        let content: String = self.read_file();
        let file_rows: Split<'_, &str> = content.split("\n");

        let mut messages_array: Vec<Message> = Vec::new();
        for row in file_rows {
            let message = Message::new(row);
            match message {
                Ok(message) => {
                    messages_array.push(message);
                }
                Err(_) => {
                    continue;
                }
            }
        }

        Ok(messages_array)
    }

}
