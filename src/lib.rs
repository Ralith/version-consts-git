use std::fmt;

use proc_macro_hack::proc_macro_hack;

/// Determine properties of the git working tree at the crate root, if any
///
/// # Example
/// ```
/// use version_consts_git::version;
/// fn main() {
///     match version!() {
///         None => eprintln!("not built from git"),
///         Some(x) => {
///             print!("{}", x.commit);
///             if x.dirty {
///                 println!(" (dirty)");
///             } else {
///                 println!();
///             }
///         }
///     }
/// }
/// ```

#[proc_macro_hack]
pub use version_consts_git_impl::version;

/// A concrete git commit
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Commit(pub [u8; 20]);

impl std::ops::Deref for Commit {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in &self.0 {
            write!(f, "{:02x}", x)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Version {
    /// The current git commit
    pub commit: Commit,
    /// Whether there were uncommited changes
    pub dirty: bool,
}
