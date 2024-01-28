use std::{
    fs::read_dir,
    io::{self, StdoutLock, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use rayon::prelude::*;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

#[derive(Debug)]
pub struct Match<'a> {
    pub col: Vec<usize>,
    pub path: &'a [u8],
}

pub fn search_par_write(
    path: &Path,
    pat: &[u8],
    pat_len: usize,
    buf: &Arc<Mutex<termcolor::Buffer>>,
) {
    if path.is_dir() {
        if let Ok(d) = read_dir(path) {
            d.into_iter().into_iter().par_bridge().for_each(|entry| {
                let path = match entry {
                    Ok(e) => e.path(),
                    _ => return,
                };
                search_par_write(&path, pat, pat_len, buf);

                let path = path.to_str().unwrap_or("").as_bytes();
                if pat_len > path.len() {
                    return;
                }
                let mut col: Vec<usize> = Vec::new();
                let mut i = 0;
                while i <= path.len() - pat_len {
                    if &path[i..i + pat_len] == pat {
                        col.push(i + 1);
                    }
                    i += 1;
                }
                if col.is_empty() {
                    return;
                }
                let _ = write_match(Match { col, path }, pat_len, buf);
            });
        }
    };
}

pub fn search_par_print(path: &Path, pat: &[u8], pat_len: usize) {
    if path.is_dir() {
        if let Ok(d) = read_dir(path) {
            d.into_iter().into_iter().par_bridge().for_each(|entry| {
                let path = match entry {
                    Ok(e) => e.path(),
                    _ => return,
                };
                search_par_print(&path, pat, pat_len);

                let path = path.to_str().unwrap_or("").as_bytes();
                if pat_len > path.len() {
                    return;
                }
                let mut col: Vec<usize> = Vec::new();
                let mut i = 0;
                while i <= path.len() - pat_len {
                    if &path[i..i + pat_len] == pat {
                        col.push(i + 1);
                    }
                    i += 1;
                }
                if col.is_empty() {
                    return;
                }
                let _ = print_match(Match { col, path }, pat_len);
            });
        }
    };
}

pub fn print_all_paths(path: &Path, handle: &mut StdoutLock) {
    if path.is_dir() {
        let entries: Vec<_> = read_dir(path)
            .unwrap()
            .par_bridge()
            .filter_map(|entry| entry.ok())
            .collect();

        entries.into_iter().for_each(|i| {
            let path = i.path();
            handle
                .write(path.to_str().unwrap_or("").as_bytes())
                .unwrap_or_default();
            write!(handle, "\n").unwrap_or_default();
            print_all_paths(&path, handle);
        });
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
