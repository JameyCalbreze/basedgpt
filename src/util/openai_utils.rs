use log;

// Call this method to remove first two words in the string
pub fn remove_message_prefix(message: &String) -> String {
    log::debug!("Entered: remove_message_prefix. message: {}", message);
    let mut trimmed_message: String = String::from(message);

    // Quick and dirty way to pull out the first two words
    for _ in 0..2 {
        let first_space = trimmed_message.find(" ");
        trimmed_message = match first_space {
            Some(pos) => String::from(&trimmed_message[(pos + 1)..trimmed_message.len()]),
            None => trimmed_message,
        };
    }

    log::debug!("trimmed_message: {}", trimmed_message);

    String::from(trimmed_message)
}

#[cfg(test)]
mod tests {
    use crate::util::openai_utils::remove_message_prefix;

    #[test]
    fn remove_message_prefix_with_an_empty_string() {
        let empty_string = "".to_string();
        let trimmed_string = remove_message_prefix(&empty_string);
        let result = trimmed_string.eq(&empty_string);
        assert!(result);
    }

    #[test]
    fn remove_message_prefix_with_one_word() {
        let example = String::from("OneWord");
        let trimmed_string = remove_message_prefix(&example);
        let result = trimmed_string.eq(&example);
        assert!(result);
    }
}
