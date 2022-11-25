use exif::*;
use std::path::*;
use std::*;
use std::borrow::Borrow;
// use hex::*;

pub fn test_grab_exif_and_print() {
    let p = Path::new("./");

    for path in &["./image.NEF"] {
        let file = fs::File::open(path).unwrap();
        let mut bufreader = io::BufReader::new(&file);
        let exifreader = Reader::new();
        let mut exif = exifreader.read_from_container(&mut bufreader).unwrap();
        for f in exif.fields() {

            if f.tag.to_string().eq("UserComment") {
                println!("{} {}", f.tag.to_string(), f.display_value().with_unit(&exif));

                // let decoded_string = hex::decode("48656c6c6f20776f726c6421");
                // let decoded_string = hex::decode();
                //
                // unsafe { println!("{}", String::from_utf8_unchecked(decoded_string.unwrap())); }
            }
            // println!(
            //     "{} {} {}",
            //     f.tag,
            //     f.ifd_num,
            //     f.display_value().with_unit(&exif)
            // );
        }
    }
}
