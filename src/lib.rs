//                   __  __               __
//      ____  ____ _/ /_/ /_  _______  __/ /_
//     / __ \/ __  / __/ __ \/ ___/ / / / __ \
//    / /_/ / /_/ / /_/ / / (__  / /_/ / /_/ /
//   / ____/\____/\__/_/ /_/____/\____/_____/
//  /_/
//
//! # pathsub
//!
//! Subtract one path from another, returns subtraction difference not the relative path
//!
//! ## Example:
//! ```rust
//! use pathsub::sub_paths;
//! use std::path::Path;
//!
//! let a = Path::new("foo/bar");
//! let b = Path::new("foo");
//!
//! sub_paths(a, b); // Some("bar")
//! ```
//! Made with <3 by Dervex

use std::path::{Path, PathBuf};

/// Subtract `other` from `path` (in this order)
///
/// ```rust
/// use pathsub::sub_paths;
/// use std::path::{Path, PathBuf};
///
/// let a = Path::new("foo/bar");
/// let b = Path::new("foo");
/// let c = Path::new("baz");
/// let d = Path::new(".").canonicalize().unwrap();
///
/// assert_eq!(sub_paths(a, b), Some(PathBuf::from("bar")));
/// assert_eq!(sub_paths(b, a), Some(PathBuf::from("")));
/// assert_eq!(sub_paths(a, c), None);
/// assert_eq!(sub_paths(a, &d), None);
/// ```
pub fn sub_paths(path: &Path, other: &Path) -> Option<PathBuf> {
	let mut a_comps = path.components();
	let mut b_comps = other.components();
	let mut output = vec![];

	if path.is_absolute() != other.is_absolute() {
		return None;
	}

	loop {
		match (a_comps.next(), b_comps.next()) {
			(Some(a), Some(b)) => {
				if a != b {
					return None;
				}
			}
			(Some(a), None) => output.push(a),
			_ => break,
		}
	}

	Some(output.iter().map(|c| c.as_os_str()).collect())
}

#[cfg(test)]
mod tests {
	use crate::sub_paths;
	use std::path::{Path, PathBuf};

	#[test]
	fn absolute() {
		let root = Path::new(".").canonicalize().unwrap();

		let a = root.join("foo/bar");
		let b = root.join("foo");
		let c = root.join("baz");
		let d = Path::new("foo/bar");

		assert_eq!(sub_paths(&a, &b), Some(PathBuf::from("bar")));
		assert_eq!(sub_paths(&b, &a), Some(PathBuf::from("")));
		assert_eq!(sub_paths(&a, &c), None);
		assert_eq!(sub_paths(&a, d), None);
	}

	#[test]
	fn relative() {
		let a = Path::new("foo/bar");
		let b = Path::new("foo");
		let c = Path::new("baz");
		let d = Path::new(".").canonicalize().unwrap().join("foo/bar");

		assert_eq!(sub_paths(a, b), Some(PathBuf::from("bar")));
		assert_eq!(sub_paths(b, a), Some(PathBuf::from("")));
		assert_eq!(sub_paths(a, c), None);
		assert_eq!(sub_paths(a, &d), None);
	}

	#[test]
	fn empty() {
		let a = Path::new("");
		let b = Path::new("");
		let c = Path::new("foo");

		assert_eq!(sub_paths(a, b), Some(PathBuf::from("")));
		assert_eq!(sub_paths(a, c), Some(PathBuf::from("")));
		assert_eq!(sub_paths(c, a), Some(PathBuf::from("foo")));
	}
}
