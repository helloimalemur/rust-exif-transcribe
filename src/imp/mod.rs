// use std::env;
// use std::fs::File;
// use std::io::BufReader;
// use std::path::{Path, PathBuf};
//
// pub fn ar() {
//     for path in env::args_os().skip(1).map(PathBuf::from) {
//         if let Err(e) = dump_file(&path) {
//             println!("{}: {}", path.display(), e);
//         }
//     }
// }
//
// fn dump_file(path: &Path) -> Result<(), exif::Error> {
//     let file = File::open(path)?;
//     let exif = exif::Reader::new().read_from_container(
//         &mut BufReader::new(&file))?;
//
//     println!("{}", path.display());
//     for f in exif.fields() {
//         println!("  {}/{}: {}",
//                  f.ifd_num.index(), f.tag,
//                  f.display_value().with_unit(&exif));
//         println!("      {:?}", f.value);
//     }
//     Ok(())
// }