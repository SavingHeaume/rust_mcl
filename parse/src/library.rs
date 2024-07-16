use model::library::*;

use crate::Parse;

impl Parse<&str> for Library {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Library>(value)
    }
}

impl Parse<&str> for Rule {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Rule>(value)
    }
}

impl Parse<&str> for Os {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Os>(value)
    }
}

impl Parse<&str> for Download {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Download>(value)
    }
}

impl Parse<&str> for Artifact {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Artifact>(value)
    }
}

