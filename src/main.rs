use std::error::Error;
use std::net::Ipv4Addr;
use std::str::from_utf8;
use std::{env, fs::read_to_string, path::Path, process::exit};

enum ParseData {
    S,
}

fn parse_line(line: &[u8]) -> Vec<ParseData> {
    let mut cur = 0;
    let mut seen = 0;

    let mut v = vec![];

    while cur < line.len() {
        let (inc, data) = parse(line, cur, seen);
        cur += inc;

        if data.is_some() {
            seen += 1;
            unsafe {
                v.push(data.unwrap_unchecked());
            }
        }
    }

    v
}

fn parse(line: &[u8], i: usize, seen: u32) -> (usize, Option<ParseData>) {
    match line[i] as char {
        '\"' => {
            let mut end = i;

            if (seen % 2) == 0 {
                for b in &line[(i + 1)..] {
                    if (*b) as char == '\"' {
                        print!("{} ", from_utf8(&line[(i + 1)..(end+1)]).unwrap());
                        break;
                    }
                    end += 1;
                }
            }

            (end - i + 1, Some(ParseData::S))
        }
        _ => (1, None),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() == 0 {
        eprintln!("file argument missing");
        exit(-1);
    }

    let file = read_to_string(&args[0])?;

    file.lines().for_each(|x| {
        // for faster reading we are just going to use the number of quotes so far
        parse_line(x.as_bytes());
        println!("");
    });

    Ok(())
}
