use std::{env::current_dir, ffi::OsString, path::PathBuf};

use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use walkdir::WalkDir;

#[derive(Debug)]
struct PathData {
    size: u64,
    name: OsString,
    icon: String,
}

impl PathData {
    fn get_human_readable_size(self) -> String {
        let mut out = self.size as f64;
        let mut suffix = "YiB";
        for unit in vec!["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"] {
            if out < 1024.0 {
                suffix = unit;
                break;
            }
            out /= 1024.0;
        }
        return format!("{:.3} {}", out, suffix);
    }
}

fn main() -> Result<(), std::io::Error> {
    let cwd = current_dir()?;

    let mut data: Vec<PathData> = vec![];
    let mut total: u64 = 0;
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    for file in cwd.read_dir()?.filter_map(|f| f.ok()) {
        if file.metadata()?.is_dir() {
            let size = get_size_of_directory(file.path());
            total += size;
            println!("{}", size);
            data.push(PathData {
                size,
                name: file.file_name(),
                icon: " ".to_owned(),
            })
        } else if file.metadata()?.is_file() {
            let size = file.metadata()?.len();
            total += size;
            data.push(PathData {
                size,
                name: file.file_name(),
                icon: " ".to_owned(),
            })
        }
    }

    data.push(PathData {
        size: total,
        name: OsString::from("Total"),
        icon: "".to_string(),
    });

    data.sort_by_key(|k| k.size);
    for d in data {
        grid.add(Cell::from(d.icon.clone()));
        grid.add(Cell::from(d.name.to_str().unwrap_or("???")));
        grid.add(Cell::from(d.get_human_readable_size()))
    }

    println!("{}", grid.fit_into_columns(3));

    Ok(())
}

fn get_size_of_directory(root: PathBuf) -> u64 {
    return WalkDir::new(root)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter_map(|f| f.metadata().ok())
        // Folders technically take up 4kb of space, but we only care about file sizes
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum();
}

#[test]
fn low_file_sizes_should_have_byte_prefix() {}