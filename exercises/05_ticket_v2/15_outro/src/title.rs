// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

#[derive(Debug,PartialEq,Clone)]
pub struct TicketTitle(String);


#[derive(Debug,thiserror::Error)]
pub enum TicketError{
    #[error("The title cannot be empty")]
    TitleEmpty,
    #[error("The title cannot be longer than 50 bytes")]
    TitleTooLong
}

pub fn validate_title(value: &str) -> Result<(), TicketError>{
        if value.is_empty() {
            return Err(TicketError::TitleEmpty)
        }
        if value.len()>50 {
            return Err(TicketError::TitleTooLong)
        }
        Ok(())
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate_title(value)?;
        Ok(TicketTitle(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
