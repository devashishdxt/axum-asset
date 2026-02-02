use std::{fs, path::Path, time::UNIX_EPOCH};

use proc_macro2::Span;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

/// Collect all files from a directory.
pub fn collect_files(span: Span, dir: &Path) -> Result<Vec<FileInfo>, syn::Error> {
    if !dir.exists() {
        return Err(syn::Error::new(
            span,
            format!("Directory does not exist: {}", dir.display()),
        ));
    }

    if !dir.is_dir() {
        return Err(syn::Error::new(
            span,
            format!("Path is not a directory: {}", dir.display()),
        ));
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(dir).follow_links(true) {
        let entry = entry.map_err(|e| {
            syn::Error::new(
                span,
                format!("Error walking directory {}: {}", dir.display(), e),
            )
        })?;

        // Skip directories
        if entry.file_type().is_dir() {
            continue;
        }

        let file_info = FileInfo::load(span, dir, entry.path())?;
        files.push(file_info);
    }

    // Sort by path for deterministic output
    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    Ok(files)
}

/// Information about a single embedded file.
#[derive(Debug)]
pub struct FileInfo {
    /// Path relative to the asset directory (with forward slashes).
    pub relative_path: String,

    /// Raw file contents.
    pub contents: Vec<u8>,

    /// SHA-256 hash of the contents (hex-encoded).
    pub content_hash: String,

    /// Unix timestamp of last modification.
    pub last_modified: u64,

    /// MIME type.
    pub mime_type: String,
}

impl FileInfo {
    /// Load a file and compute its metadata.
    fn load(span: Span, base_dir: &Path, file_path: &Path) -> Result<Self, syn::Error> {
        // Read file contents
        let contents = fs::read(file_path).map_err(|e| {
            syn::Error::new(
                span,
                format!("Failed to read {}: {}", file_path.display(), e),
            )
        })?;

        // Compute SHA-256 hash
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        let hash_bytes = hasher.finalize();
        let content_hash = hex::encode(hash_bytes);

        // Get last modified time
        let metadata = fs::metadata(file_path).map_err(|e| {
            syn::Error::new(
                span,
                format!("Failed to get metadata for {}: {}", file_path.display(), e),
            )
        })?;
        let last_modified = metadata
            .modified()
            .map_err(|e| {
                syn::Error::new(
                    span,
                    format!("Failed to get mtime for {}: {}", file_path.display(), e),
                )
            })?
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Compute relative path with forward slashes
        let relative_path = file_path
            .strip_prefix(base_dir)
            .map_err(|e| syn::Error::new(span, format!("Failed to compute relative path: {}", e)))?
            .to_string_lossy()
            .replace('\\', "/");

        // Guess MIME type from extension
        let mime_type = mime_guess::from_path(file_path)
            .first_or_octet_stream()
            .to_string();

        Ok(FileInfo {
            relative_path,
            contents,
            content_hash,
            last_modified,
            mime_type,
        })
    }
}
