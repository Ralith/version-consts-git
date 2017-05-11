use std::io;
use git2;

error_chain! {
    foreign_links {
        Io(io::Error);
        Git(git2::Error);
    }
}
