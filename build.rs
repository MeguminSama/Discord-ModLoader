// use asar::AsarWriter;
// use std::env;
// use std::error::Error;

// use std::fs::File;
// use std::path::PathBuf;

// static ASAR_INDEX: &[u8] = include_bytes!("asar/index.js");
// static ASAR_PACKAGE: &[u8] = include_bytes!("asar/package.json");

// fn main() -> std::result::Result<(), Box<dyn Error>> {
//     let mut asar = AsarWriter::new();

//     asar.write_file("index.js", ASAR_INDEX, true)?;
//     asar.write_file("package.json", ASAR_PACKAGE, true)?;

//     let mut out_dir = PathBuf::from("target");

//     out_dir.push(env::var("PROFILE")?);

//     out_dir.push("app.asar");

//     asar.finalize(File::create(out_dir.to_str().unwrap())?)?;

//     Ok(())
// }

fn main() {}