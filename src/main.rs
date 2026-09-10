use std::env;
use std::fs;
use std::io::{self, Read};
use std::thread;

#[derive(Default, Debug, Clone, Copy)]
struct Count {
    lines: usize,
    words: usize,
    bytes: usize,
}

fn count(buf: &[u8]) -> Count {
    let mut c = Count { bytes: buf.len(), ..Default::default() };
    let mut in_word = false;
    for &b in buf {
        match b {
            b'\n' => c.lines += 1,
            b' ' | b'\t' | b'\r' => in_word = false,
            _ => {
                if !in_word {
                    c.words += 1;
                    in_word = true;
                }
            }
        }
    }
    c
}

fn report(name: &str, c: Count) {
    println!("{:>8} {:>8} {:>8} {}", c.lines, c.words, c.bytes, name);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        report("-", count(&buf));
        return Ok(());
    }
    let mut handles = Vec::new();
    for path in args {
        handles.push(thread::spawn(move || {
            let data = fs::read(&path).unwrap_or_default();
            (path, count(&data))
        }));
    }
    let mut total = Count::default();
    for h in handles {
        if let Ok((path, c)) = h.join() {
            total.lines += c.lines;
            total.words += c.words;
            total.bytes += c.bytes;
            report(&path, c);
        }
    }
    report("total", total);
    Ok(())
}
