use std::fmt::Display;

pub struct TrivialGraphFormat(String);

impl From<String> for TrivialGraphFormat {
    fn from(value: String) -> Self {
        TrivialGraphFormat(value)
    }
}

impl<'a> From<&'a str> for TrivialGraphFormat {
    fn from(value: &'a str) -> TrivialGraphFormat {
        TrivialGraphFormat(String::from(value))
    }
}

impl Into<String> for TrivialGraphFormat {
    fn into(self) -> String {
        self.0
    }
}

impl Display for TrivialGraphFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
