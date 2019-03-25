use std::error::Error;

pub struct ParseError{
    message: String
}

impl ParseError{
    pub fn create(message: String) -> ParseError{
        return ParseError{message: message};
    }

    pub fn from_error(error: std::io::Error) -> ParseError{
        return ParseError{
            message: String::from(error.description())
        }
    }

    pub fn to_string(&self) -> &String{
        return &self.message;
    }
}