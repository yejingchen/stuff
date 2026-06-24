use std::fs;
use std::ffi::{OsStr, OsString};
use std::env;

unsafe extern "C" {
    safe fn getpagesize() -> i32;
}

/// Returns human size and unit in a tuple.
fn human_size(mut num: u64) -> (f32, &'static str) {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    const STEP: u64 = 1024;

    let mut prev = num;
    let mut count = 0;
    while num > STEP {
        prev = num;
        num /= STEP;
        count += 1;
    }

    (prev as f32 / STEP as f32, UNITS[count])
}

fn main() {
    let default_zswap_debugfs_path = OsStr::new("/sys/kernel/debug/zswap");
    let stat_in_pages: [OsString; 2] = ["stored_pages".into(), "stored_incompressible_pages".into()];
    let stat_in_bytes: [OsString; 1] = ["pool_total_size".into()];

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
        if stat_in_pages.contains(&keyname) {
            let nbytes = value.parse::<u64>().unwrap() * getpagesize() as u64;
            let (human_num, unit) = human_size(nbytes);
            println!("{}:\t{:.1} {}", key, human_num, unit);
        } else if stat_in_bytes.contains(&keyname) {
            let nbytes = value.parse().unwrap();
            let (human_num, unit) = human_size(nbytes);
            println!("{}:\t{:.1} {}", key, human_num, unit);
        } else {
            println!("{}:\t{}", key, value);
        }
    }
}
