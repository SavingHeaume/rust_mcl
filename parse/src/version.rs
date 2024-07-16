use crate::Parse;
use model::version::*;

impl Parse<&str> for Version {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Version>(value)
    }
}

impl Parse<&str> for Download {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Download>(value)
    }
}

impl Parse<&str> for Client {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Client>(value)
    }
}

