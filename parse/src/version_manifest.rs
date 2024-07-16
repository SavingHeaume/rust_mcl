use model::version_manifest::*;

use crate::Parse;

impl Parse<&str> for VersionManifest {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<VersionManifest>(value)
    }
}

impl Parse<&str> for Latest {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Latest>(value)
    }
}

impl Parse<&str> for Version {
    type Error = serde_json::Error;

    fn parse(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Version>(value)
    }
}

