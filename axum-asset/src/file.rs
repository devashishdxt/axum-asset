/// Metadata about an embedded file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmbeddedFileMetadata {
    /// SHA-256 hash of the file contents, used for ETag.
    pub content_hash: &'static str,

    /// Unix timestamp of last modification.
    pub last_modified: u64,

    /// MIME type derived from file extension.
    pub mime_type: &'static str,
}

/// A file embedded at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmbeddedFile {
    /// Route for the file.
    pub route: &'static str,

    /// Path relative to the embedded directory.
    pub path: &'static str,

    /// Raw file contents.
    pub contents: &'static [u8],

    /// File metadata.
    pub metadata: EmbeddedFileMetadata,
}
