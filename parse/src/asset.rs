use model::asset::*;

use crate::Parse;

impl Parse<&str> for AssetIndex {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<AssetIndex>(value)
    }
}

impl Parse<&str> for Index {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Index>(value)
    }
}

impl Parse<&str> for Object {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Object>(value)
    }
}

