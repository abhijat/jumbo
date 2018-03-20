use std::env;
use std::fs;
use std::io;
use std::path;

fn cumulative_size(path: &path::Path, mut dir_sizes: &mut Vec<(String, u64)>) -> io::Result<u64> {
    if path.is_dir() {
        let mut total_size: u64 = 0;
        let entries = path.read_dir();

        if entries.is_err() {
            println!("failed to read path {:?}", path);
            return Ok(0);
        }

        let entries = entries.unwrap();
        for e in entries {
            let e = e?;
            if e.path().is_file() {
                total_size += e.path().metadata()?.len();
            } else {
                total_size += cumulative_size(&e.path(), &mut dir_sizes)?;
            }
        }

        dir_sizes.push((path.to_str().unwrap().to_owned(), total_size));
        Ok(total_size)
    } else if path.is_file() {
        let size = fs::metadata(path)?.len();
        dir_sizes.push((path.to_str().unwrap().to_owned(), size));
        Ok(size)
    } else {
        Ok(0)
    }
}

fn bytes_to_mb(size: u64) -> u64 {
    size / 1024 / 1024
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <path to analyze>", args[0]);
        ::std::process::exit(1);
    }

    let mut dir_sizes = Vec::<(String, u64)>::new();

    let path = path::Path::new(&args[1]);
    let size = cumulative_size(&path, &mut dir_sizes);

    match size {
        Ok(_) => {
            dir_sizes.sort_by(|a, b| {
                b.1.cmp(&a.1)
            });

            for pair in dir_sizes.iter().take(100) {
                let size = format!("{}K", pair.1);
                eprintln!("{:20} \t {}", size, pair.0);
            }
        }
        Err(e) => {
            eprintln!("e = {:#?}", e);
        }
    }
}
