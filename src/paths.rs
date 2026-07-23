//! Path safety for plugin/audio/output arguments.

use crate::error::RenderError;
use std::path::{Component, Path};

/// Reject paths that contain `..` components (path traversal).
pub fn deny_parent_dirs(path: &Path, label: &str) -> Result<(), RenderError> {
    for c in path.components() {
        if matches!(c, Component::ParentDir) {
            return Err(RenderError::Job(format!(
                "{label} must not contain '..' path components"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn accepts_plain() {
        assert!(deny_parent_dirs(Path::new("/tmp/out.mkv"), "output").is_ok());
    }

    #[test]
    fn rejects_dotdot() {
        assert!(deny_parent_dirs(Path::new("../evil.so"), "plugin").is_err());
        assert!(deny_parent_dirs(Path::new("/tmp/../etc/passwd"), "audio").is_err());
    }
}
