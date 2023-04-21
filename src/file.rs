use chrono::Local;
use std::fs::{DirBuilder, OpenOptions};
use std::path::{Path, PathBuf};

pub fn access_xml() -> std::fs::File {
    let mut dir_path = access_dir();
    let mut date = get_date();
    date.push_str(".xml");
    dir_path.push(date);
    let file_path = dir_path.as_path();

    if file_path.is_file() {
        OpenOptions::new().append(true).open(file_path).unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap()
    }
}

fn access_dir() -> PathBuf {
    let path = Path::new(&"./chat/");
    if !path.is_dir() {
        DirBuilder::new().create(path).unwrap();
    }
    path.to_owned()
}

fn get_date() -> String {
    let now = Local::now().date_naive();
    now.format("%Y%m%d").to_string()
}

#[cfg(test)]
mod tests {
    use crate::file::*;

    #[test]
    fn exist_file() {
        let file = access_xml();
        let file_type = file.metadata().unwrap().file_type();
        assert_eq!(file_type.is_file(), true);
    }
}
