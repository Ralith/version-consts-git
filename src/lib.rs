#![recursion_limit = "1024"]   // for error-chain
#[macro_use]
extern crate error_chain;
extern crate git2;

mod errors;

use std::{io, env};
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use std::path::Path;

use git2::{Repository, Oid};

use errors::*;

#[cfg(unix)]
fn write_path<W: Write, P: AsRef<Path>>(out: &mut W, path: P) -> io::Result<usize> {
    use std::os::unix::ffi::OsStrExt;

    let path = path.as_ref().as_os_str().as_bytes();
    out.write(path)
}

#[cfg(not(unix))]
fn write_path<W: Write, P: AsRef<Path>>(out: &mut W, path: P) -> io::Result<usize> {
    out.write(path.as_ref().to_str().expect("non-unicode path handling is unimplemented").as_bytes())
}

fn format_commit<W: Write>(out: &mut W, commit: Oid) -> io::Result<()> {
    try!(out.write_all(b"["));
    for b in commit.as_bytes() {
        try!(write!(out, "{:#04x},", b));
    }
    out.write_all(b"]")
}

fn generate_<W: Write>(out: &mut W, repo: &Repository) -> Result<()> {
    // Guard against repositories in initial state
    if try!(repo.is_empty()) { return Ok(()); }

    let head = try!(repo.head());
    let commit = try!(head.resolve().chain_err(|| "couldn't resolve git HEAD")).target().unwrap();
    try!(write!(out, "pub const COMMIT: [u8; 20] = "));
    try!(format_commit(out, commit));
    try!(write!(out, ";\n"));

    let diff = try!(repo.diff_tree_to_workdir_with_index(Some(&repo.find_tree(repo.revparse_single("HEAD^{tree}")?.id())?), None));
    try!(write!(out, "pub const DIRTY: bool = {};\n", diff.deltas().len() != 0));

    Ok(())
}

pub fn generate() -> Result<()> {
    let repo = try!(Repository::open(".").chain_err(|| "couldn't open git repository"));
    let path = PathBuf::from(try!(env::var("OUT_DIR").chain_err(|| "couldn't get OUT_DIR"))).join("version.rs");
    let mut file = try!(File::create(&path).chain_err(|| format!("couldn't create {}", path.to_string_lossy())));
    try!(generate_(&mut file, &repo));

    let mut paths = Vec::with_capacity(2);
    paths.push(repo.path().join("HEAD"));
    if let Ok(reference) = repo.head() {
        if reference.is_branch() {
            let name = String::from_utf8(reference.name_bytes().to_vec()).expect("non-unicode path handling is unimplemented");
            paths.push(repo.path().join(name));
        }
    }

    for path in &paths {
        print!("cargo:rerun-if-changed=");
        try!(write_path(&mut io::stdout(), path));
        print!("\n");
    }

    Ok(())
}
