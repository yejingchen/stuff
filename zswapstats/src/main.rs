use std::{ffi::OsStr, fs};
use std::env;

const MEM_SIZE_UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
const MEM_SIZE_STEP: u64 = 1024;

unsafe extern "C" {
    safe fn getpagesize() -> i32;
}

/// Returns human size and division count in a tuple.
fn human_size(mut num: u64, div: u64) -> (f32, usize) {
    let mut prev = num;
    let mut count = 0;
    while num > div {
        prev = num;
        num /= div;
        count += 1;
    }

    (prev as f32 / div as f32, count)
}

fn main() {
    let default_zswap_debugfs_path = OsStr::new("/sys/kernel/debug/zswap");
    let stat_in_pages = [OsStr::new("stored_pages"), OsStr::new("stored_incompressible_pages")];
    let stat_in_bytes = [OsStr::new("pool_total_size")];

    let mut args = env::args_os().enumerate();
    args.next();
    let arg1 = args.next();
    let zswap_debugfs_path = if let Some((1, path)) = arg1.as_ref() {
        path
    } else {
        default_zswap_debugfs_path
    };

    let readdir = match fs::read_dir(zswap_debugfs_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error opening {}: {}", zswap_debugfs_path.display(), e);
            return;
        }
    };

    for entry in readdir {
        let direntry = match entry {
            Ok(direntry) => direntry,
            Err(e) => {
                eprint!("error reading directory entry: {}", e);
                continue;
            }
        };

        let path = direntry.path();
        let value = match fs::read_to_string(&path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error reading {}: {}", path.display(), e);
                continue;
            }
        };
        let value = &value[..value.len()-1];

        let keyname = direntry.file_name();
        let key = keyname.display();
        if stat_in_pages.contains(&direntry.file_name().as_ref()) {
            let nbytes = value.parse::<u64>().unwrap() * getpagesize() as u64;
            let (human_num, count) = human_size(nbytes, MEM_SIZE_STEP);
            println!("{}:\t{:.1} {}", key, human_num, MEM_SIZE_UNITS[count]);
        } else if stat_in_bytes.contains(&direntry.file_name().as_ref()) {
            let nbytes = value.parse().unwrap();
            let (human_num, count) = human_size(nbytes, MEM_SIZE_STEP);
            println!("{}:\t{:.1} {}", key, human_num, MEM_SIZE_UNITS[count]);
        } else {
            println!("{}:\t{}", key, value);
        }
    }
}
