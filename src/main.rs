use std::{env, path::Path, time::Instant};

use serch::{print_all_paths, scan_print};

fn main() {
    let inst = Instant::now();
    let args: Vec<String> = env::args().collect();
    let path = match args.get(2) {
        Some(path) => path,
        _ => "./",
    };
    match args.get(1) {
        Some(pat) if pat != "-" && !pat.is_empty() => {
            let pat = pat.as_bytes();
            let pat_len = pat.len();
            scan_print(Path::new(path), pat, pat_len);
        }
        _ => {
            print_all_paths(Path::new(path));
        }
    };

    println!("{}ms", inst.elapsed().as_millis());
}
