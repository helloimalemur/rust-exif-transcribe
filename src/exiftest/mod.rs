use exif::{Error, Exif, Reader};
use std::path::Path;
use std::*;

pub fn test_grab_exif_and_print() {
    let p = Path::new("./");

    for path in &["./image.NEF"] {
        let file = fs::File::open(path).unwrap();
        let mut bufreader = io::BufReader::new(&file);
        let exifreader = Reader::new();
        let mut exif = exifreader.read_from_container(&mut bufreader).unwrap();
        for f in exif.fields() {
            println!(
                "{} {} {}",
                f.tag,
                f.ifd_num,
                f.display_value().with_unit(&exif)
            );
        }
    }
}
