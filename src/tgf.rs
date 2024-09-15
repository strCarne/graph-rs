use std::fmt::Display;

/// TrivialGraphFormat is a format for graphs.
/// See <https://en.wikipedia.org/wiki/Trivial_Graph_Format#:~:text=Trivial%20Graph%20Format%20(TGF)%20is,used%20because%20of%20its%20simplicity>.
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
