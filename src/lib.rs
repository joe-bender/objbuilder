use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Vert {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Face(pub Vert, pub Vert, pub Vert, pub Vert);

pub struct Writer;

impl Writer {
    pub fn write_faces(faces: Vec<Face>) {
        unimplemented!();
    }

    pub fn face_output(f: Face) -> String {
        let mut out = String::new();

        out.push_str(&format!("v {} {} {}\n", (f.0).x, (f.0).y, (f.0).z));
        out.push_str(&format!("v {} {} {}\n", (f.1).x, (f.1).y, (f.1).z));
        out.push_str(&format!("v {} {} {}\n", (f.2).x, (f.2).y, (f.2).z));
        out.push_str(&format!("v {} {} {}\n", (f.3).x, (f.3).y, (f.3).z));
        out.push_str(&format!("f {} {} {} {}\n", 1, 2, 3, 4));

        out
    }

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
}

pub fn simple_plane() -> String {
    let mut out = String::new();

    let v1 = Vert {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let v2 = Vert {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let v3 = Vert {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    };
    let v4 = Vert {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let f1 = Face(v1, v2, v3, v4);

    out.push_str(&format!("v {} {} {}\n", (f1.0).x, (f1.0).y, (f1.0).z));
    out.push_str(&format!("v {} {} {}\n", (f1.1).x, (f1.1).y, (f1.1).z));
    out.push_str(&format!("v {} {} {}\n", (f1.2).x, (f1.2).y, (f1.2).z));
    out.push_str(&format!("v {} {} {}\n", (f1.3).x, (f1.3).y, (f1.3).z));
    out.push_str(&format!("f {} {} {} {}\n", 1, 2, 3, 4));

    out
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
