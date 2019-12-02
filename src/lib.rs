use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Vert(f64, f64, f64);

struct Face(Vert, Vert, Vert, Vert);

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
    let mut out = String::new();

    out.push_str("v 0.000000 0.000000 1.000000\n");
    out.push_str("v 0.000000 1.000000 0.000000\n");
    out.push_str("v 1.000000 0.000000 1.000000\n");
    out.push_str("v 1.000000 1.000000 0.000000\n");
    out.push_str("v 1.000000 1.000000 1.000000\n");
    out.push_str("v 1.000000 0.000000 0.000000\n");
    out.push_str("v 0.000000 1.000000 1.000000\n");
    out.push_str("v 0.000000 0.000000 0.000000\n");
    out.push_str("f 6 4 5 3\n");
    out.push_str("f 8 6 3 1\n");
    out.push_str("f 1 3 5 7\n");
    out.push_str("f 2 8 1 7\n");
    out.push_str("f 4 2 7 5\n");
    out.push_str("f 8 2 4 6\n");

    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
