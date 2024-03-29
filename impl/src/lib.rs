extern crate proc_macro;

use std::env;

use git2::Repository;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;

struct Version {
    commit: [u8; 20],
    dirty: bool,
}

#[proc_macro_hack]
pub fn version(tokens: TokenStream) -> TokenStream {
    assert!(tokens.is_empty(), "no arguments expected");
    let none = TokenStream::from(quote! { None::<version_consts_git::Version> });
    let repo = match Repository::discover(env::var("CARGO_MANIFEST_DIR").unwrap()) {
        Ok(x) => x,
        Err(_) => return none,
    };
    let version = from_repo(&repo);
    let dep_path = repo.path().join("logs/HEAD");
    if !dep_path.exists() {
        // No initial commit
        return none;
    }
    let dep_path = dep_path.to_str();
    let version = match version {
        None => quote! { None },
        Some(Version { commit, dirty }) => quote! {
            Some(version_consts_git::Version {
                commit: version_consts_git::Commit([#(#commit),*]),
                dirty: #dirty,
            })
        },
    };
    TokenStream::from(quote! {
        {
            const _GIT_HEAD: &[u8] = include_bytes!(#dep_path);
            #version
        }
    })
}

fn from_repo(repo: &Repository) -> Option<Version> {
    // Guard against repositories in initial state
    if repo.is_empty().ok()? {
        return None;
    }

    let head = repo.head().ok()?;
    let commit = head.resolve().ok()?.target().unwrap();

    let diff = repo
        .diff_tree_to_workdir_with_index(
            Some(
                &repo
                    .find_tree(repo.revparse_single("HEAD^{tree}").ok()?.id())
                    .ok()?,
            ),
            None,
        )
        .ok()?;

    let mut commit_bytes = [0; 20];
    commit_bytes.copy_from_slice(commit.as_bytes());
    Some(Version {
        commit: commit_bytes,
        dirty: diff.deltas().len() != 0,
    })
}
