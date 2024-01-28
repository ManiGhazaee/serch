use std::{
    io::{self, stdout, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

#[derive(Debug)]
pub struct Match<'a> {
    pub col: Vec<usize>,
    pub path: &'a [u8],
}

pub fn scan_print_end(path: &Path, pat: &[u8], pat_len: usize) {
    let bw = BufferWriter::stdout(ColorChoice::Always);
    let b = Arc::new(Mutex::new(bw.buffer()));
    for entry in jwalk::WalkDir::new(Path::new(path)) {
        let entry = match entry {
            Ok(e) => e,
            _ => continue,
        };
        let path = entry.path();
        let path = path.to_str().unwrap_or("").as_bytes();
        let path_len = path.len();
        if pat_len > path_len {
            continue;
        }
        let mut col: Vec<usize> = Vec::new();
        let mut i = 0;
        while i <= path_len - pat_len {
            if &path[i..i + pat_len] == pat {
                col.push(i + 1);
                i += path_len;
                continue;
            }
            i += 1;
        }
        if col.is_empty() {
            continue;
        }
        let _ = write_match(Match { col, path }, pat_len, &b);
    }
    bw.print(&b.lock().unwrap()).unwrap();
}

pub fn scan_print(path: &Path, pat: &[u8], pat_len: usize) {
    for entry in jwalk::WalkDir::new(Path::new(path)) {
        let entry = match entry {
            Ok(e) => e,
            _ => continue,
        };
        let path = entry.path();
        let path = path.to_str().unwrap_or("").as_bytes();
        let path_len = path.len();
        if pat_len > path_len {
            continue;
        }
        let mut col: Vec<usize> = Vec::new();
        let mut i = 0;
        while i <= path_len - pat_len {
            if &path[i..i + pat_len] == pat {
                col.push(i + 1);
                i += pat_len;
                continue;
            }
            i += 1;
        }
        if col.is_empty() {
            continue;
        }
        let _ = print_match(Match { col, path }, pat_len);
    }
}

pub fn print_all_paths(path: &Path) {
    let mut handle = stdout().lock();
    for entry in jwalk::WalkDir::new(Path::new(path)) {
        let entry = match entry {
            Ok(e) => e,
            _ => continue,
        };
        let path = entry.path();
        let path = match path.to_str() {
            Some(p) => p,
            _ => continue,
        };
        let _ = writeln!(handle, "{}", path);
    }
}

pub fn write_match(m: Match, pat_len: usize, b: &Arc<Mutex<termcolor::Buffer>>) -> io::Result<()> {
    let mut binding = ColorSpec::new();
    let red = binding.set_fg(Some(Color::Red));
    let mut b = b.lock().unwrap();
    let mut j = 0;
    for i in m.col.iter() {
        let col = i - 1;
        b.write(&m.path[j..col])?;
        b.set_color(&red)?;
        b.write(&m.path[col..col + pat_len])?;
        b.reset()?;
        j = col + pat_len;
    }
    b.write(&m.path[m.col[m.col.len() - 1] - 1 + pat_len..])?;
    write!(b, "\n")?;

    Ok(())
}

pub fn print_match(m: Match, pat_len: usize) -> io::Result<()> {
    let mut binding = ColorSpec::new();
    let red = binding.set_fg(Some(Color::Red));
    let bw = BufferWriter::stdout(ColorChoice::Always);
    let mut b = bw.buffer();
    let mut j = 0;
    for i in m.col.iter() {
        let col = i - 1;
        b.write(&m.path[j..col])?;
        b.set_color(&red)?;
        b.write(&m.path[col..col + pat_len])?;
        b.reset()?;
        j = col + pat_len;
    }
    b.write(&m.path[m.col[m.col.len() - 1] - 1 + pat_len..])?;
    write!(b, "\n")?;
    bw.print(&b).unwrap();

    Ok(())
}
