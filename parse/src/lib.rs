pub mod version_manifest;
pub mod version;
pub mod asset;
pub mod library;

pub trait Parse<T>: Sized {
    type Error;
    fn parse(value: T) -> Result<Self, Self::Error>;
}

