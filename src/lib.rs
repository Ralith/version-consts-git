#![recursion_limit = "1024"]   // for error-chain
#[macro_use]
extern crate error_chain;
extern crate git2;

mod errors;

use std::{io, env};
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

use git2::{Repository, Oid};

use errors::*;

fn format_commit<W: Write>(out: &mut W, commit: Oid) -> io::Result<()> {
    try!(out.write_all(b"pub const COMMIT: [u8; 20] = ["));
    for b in commit.as_bytes() {
        try!(write!(out, "{:#04x},", b));
    }
    try!(out.write_all(b"];\n"));
    Ok(())
}

pub fn generate() -> Result<()> {
    let repo = try!(Repository::open(".").chain_err(|| "couldn't open git repository"));
    let path = PathBuf::from(try!(env::var("OUT_DIR").chain_err(|| "couldn't get OUT_DIR"))).join("version.rs");
    let mut file = try!(File::create(&path).chain_err(|| format!("couldn't create {}", path.to_string_lossy())));
    
    let head = try!(repo.head().chain_err(|| "couldn't get git HEAD"));
    let commit = try!(head.resolve().chain_err(|| "couldn't resolve git HEAD")).target().unwrap();
    try!(format_commit(&mut file, commit).chain_err(|| format!("couldn't write to {}", path.to_string_lossy())));
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use git2::Oid;

    #[test]
    fn format_commit() {
        let mut buf = Vec::new();
        ::format_commit(&mut buf, Oid::from_bytes(&[0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f,0x10,0x11,0x12,0x13,]).unwrap()).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(),
                   "pub const COMMIT: [u8; 20] = [0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f,0x10,0x11,0x12,0x13,];\n");
    }
}
