use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Invalid feed URL: {0}")]
    InvalidFeedUrl(String),

    #[error("Sparkle initialization failed: {0}")]
    SparkleInit(String),

    #[error("Updater not ready")]
    UpdaterNotReady,

    #[error("Sparkle doesn't allow automatic downloads: with automatic checks off, set SUAllowsAutomaticUpdates in Info.plist")]
    AutomaticDownloadsNotAllowed,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
