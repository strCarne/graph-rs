use std::fmt::Display;

/// TrivialGraphFormat is a format for graphs.
/// See https://shorturl.at/qS3MA
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

/// TgfConvertible is a trait that allows converting from and to TrivialGraphFormat.
pub trait TgfConvertible {
    fn to_tgf(&self) -> TrivialGraphFormat;

    fn from_tgf(tgf: TrivialGraphFormat) -> Result<Self, String>
    where
        Self: Sized;
}

impl Display for TrivialGraphFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
