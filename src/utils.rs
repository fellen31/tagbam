/// Validate that the tag has a maximum length of 2 characters
pub fn validate_tag(input: &str) -> Result<(), String> {
    if input.is_empty() || input.len() > 2 {
        Err("Input must be between 1 and 2 characters long".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tag() {
        assert_eq!(validate_tag("A"), Ok(()));
        assert_eq!(validate_tag("AB"), Ok(()));
    }

    #[test]
    fn test_invalid_tag() {
        assert_eq!(
            validate_tag("ABC"),
            Err("Input must be between 1 and 2 characters long".to_string())
        );
        assert_eq!(
            validate_tag(""),
            Err("Input must be between 1 and 2 characters long".to_string())
        ); // Empty string is valid
    }
}
