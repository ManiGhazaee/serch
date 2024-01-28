use std::{
    env, io::stdout, path::Path, sync::{Arc, Mutex}, time::Instant
};

use pscan::{print_all_paths, search_par_print, search_par_write};
use termcolor::{BufferWriter, ColorChoice};

fn main() {
    let inst = Instant::now();
    let args: Vec<String> = env::args().collect();
    let path = match args.get(2) {
        Some(path) => path,
        _ => "./",
    };
    let pat = match args.get(1) {
        Some(pat) if pat != "-" && !pat.is_empty() => pat.as_bytes(),
        _ => {
            let mut handle = stdout().lock();
            print_all_paths(Path::new(path), &mut handle);
            return;
        }
    };
    let pat_len = pat.len();

    // let bw = BufferWriter::stdout(ColorChoice::Always);
    // let mut b = Arc::new(Mutex::new(bw.buffer()));
    // search_par_write(Path::new(path), pat, pat_len, &mut b);
    // bw.print(&b.lock().unwrap()).unwrap();

    search_par_print(Path::new(path), pat, pat_len);
    println!("{}ms", inst.elapsed().as_millis());
}
