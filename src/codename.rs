use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Error)]
pub enum CodenameParseError {
    #[error(display = "unknown codename string")]
    NotFound,
}

/// The codename associated with an Ubuntu version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Codename {
    Bionic,
    Cosmic,
    Disco,
}

impl Display for Codename {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str(<&'static str>::from(*self)) }
}

impl FromStr for Codename {
    type Err = CodenameParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let release = match input {
            "bionic" => Codename::Bionic,
            "cosmic" => Codename::Cosmic,
            "disco" => Codename::Disco,
            _ => return Err(CodenameParseError::NotFound),
        };

        Ok(release)
    }
}

impl From<Codename> for &'static str {
    fn from(codename: Codename) -> Self {
        match codename {
            Codename::Bionic => "bionic",
            Codename::Cosmic => "cosmic",
            Codename::Disco => "disco",
        }
    }
}
