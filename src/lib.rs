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

pub fn unit_cube() -> String {
    String::from("# from rust
v 0.000000 0.000000 1.000000
v 0.000000 1.000000 0.000000
v 1.000000 0.000000 1.000000
v 1.000000 1.000000 0.000000
v 1.000000 1.000000 1.000000
v 1.000000 0.000000 0.000000
v 0.000000 1.000000 1.000000
v 0.000000 0.000000 0.000000
f 6 4 5 3
f 8 6 3 1
f 1 3 5 7
f 2 8 1 7
f 4 2 7 5
f 8 2 4 6
")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
