use std::io;
use git2;

error_chain! {
    foreign_links {
        io::Error, Io;
        git2::Error, Git;
    }
}
