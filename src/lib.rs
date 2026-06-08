//! # step-id
//!
//! Stable, deterministic IDs for agent steps. Same `(run_id, index,
//! kind)` always produces the same id — useful for joining trace
//! records across reruns or storage backends.
//!
//! 16-char hex (first 64 bits of SHA-256).
//!
//! ## Example
//!
//! ```
//! use step_id::id;
//! let a = id("run-1", 0, "tool_call");
//! let b = id("run-1", 0, "tool_call");
//! assert_eq!(a, b);
//! assert_eq!(a.len(), 16);
//! ```

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod sha256;

/// 16-char hex id (first 64 bits of SHA-256).
pub fn id(run_id: &str, index: u64, kind: &str) -> String {
    let mut buf = String::with_capacity(run_id.len() + kind.len() + 24);
    buf.push_str(run_id);
    buf.push('\0');
    buf.push_str(&index.to_string());
    buf.push('\0');
    buf.push_str(kind);
    let full = sha256::hex(buf.as_bytes());
    full[..16].to_string()
}
