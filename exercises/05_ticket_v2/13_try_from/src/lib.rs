#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(thiserror::Error, Debug)]
enum StatusParseError {
    #[error(":(")]
    Error,
}

impl TryFrom<String> for Status {
    type Error = StatusParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let normalized = s.to_lowercase();
        if normalized.trim() == "todo" {
            Ok(Status::ToDo)
        } else if normalized.trim() == "inprogress" {
            Ok(Status::InProgress)
        } else if normalized.trim() == "done" {
            Ok(Status::Done)
        } else {
            Err(StatusParseError::Error {})
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusParseError;
    fn try_from(ps: &str) -> Result<Self, Self::Error> {
        Status::try_from(ps.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
