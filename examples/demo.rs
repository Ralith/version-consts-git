use version_consts_git::version;

fn main() {
    match version!() {
        None => eprintln!("not built from git"),
        Some(x) => {
            println!("{:?}", x);
        }
    }
}
