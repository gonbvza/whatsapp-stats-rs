use crate::errors::ParseError;

/// A WhatsApp message with date, time, sender, and content.
#[derive(Debug)]
pub struct Message {
    pub date: String,
    pub hour: String,
    pub owner: String,
    pub text: String,
}

impl Message {
    /// Parses a line like `[date, time] owner: message` into a `Message`.
    ///
    /// Returns `Err(ParseError::NoSplitter)` if the line is malformed.
    pub fn new(line: &str) -> Result<Self, ParseError> {
        let line = line.trim();
        if !line.starts_with('[') {
            return Err(ParseError::NoSplitter);
        }

        let end_bracket = line.find(']').ok_or(ParseError::NoSplitter)?;
        let datetime_str = &line[1..end_bracket];
        let rest = line[end_bracket + 1..].trim();

        let mut dt_parts = datetime_str.split(", ");
        let date = dt_parts.next().ok_or(ParseError::NoSplitter)?.to_string();
        let hour = dt_parts.next().ok_or(ParseError::NoSplitter)?.to_string();

        let mut rest_parts = rest.splitn(2, ':');
        let owner = rest_parts
            .next()
            .ok_or(ParseError::NoSplitter)?
            .trim()
            .to_string();
        let text = rest_parts
            .next()
            .ok_or(ParseError::NoSplitter)?
            .trim()
            .to_string();

        Ok(Message {
            date,
            hour,
            owner,
            text,
        })
    }

    /// Prints the message to the screen
    pub fn print(&self) {
        println!(
            "{} {} | {}: {}",
            self.date, self.hour, self.owner, self.text
        );
    }
}

impl PartialEq for Message {
    /// Implements the Eq trait for the message. Implemented for testing purposes
    ///
    /// Returns true if all the attributes are the same
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.text == other.text
            && self.date == other.date
            && self.hour == other.hour
    }
}
