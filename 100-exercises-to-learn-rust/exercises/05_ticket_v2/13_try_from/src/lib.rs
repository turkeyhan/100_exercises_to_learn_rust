// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status{
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lower_value = value.to_lowercase();
        if lower_value == "todo".to_string(){
            Ok(Status::ToDo)
        }
        else if lower_value == "inprogress".to_string(){
            Ok(Status::InProgress)
        }
        else if lower_value == "done".to_string(){
            Ok(Status::Done)
        }
        else{
            return Err("Error!".to_string());
        }
    }
}
impl TryFrom<&str> for Status{
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error>{
        let lower_value = value.to_lowercase();
        if lower_value == "todo".to_string(){
            Ok(Status::ToDo)
        }
        else if lower_value == "inprogress".to_string(){
            Ok(Status::InProgress)
        }
        else if lower_value == "done".to_string(){
            Ok(Status::Done)
        }
        else{
            return Err("Error!".to_string());
        }
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
