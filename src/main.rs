fn main() {
    println!("Hello, world!");

    if let Err(e) = uniqr::get_args().and_then(uniqr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
