use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(err) => panic!("Couldn't create {}: {}", display, err.description()),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(err) => panic!("Couldn't write to {}: {}", display, err.description()),
        Ok(_) => println!("Wrote model to {}", display),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
