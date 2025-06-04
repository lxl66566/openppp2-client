use std::{env, fs::File, io::Write, path::Path};

use zstd::stream::write::Encoder;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=dns-rules.txt");
    println!("cargo:rerun-if-changed=ip.txt");
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("dns-rules.zst");
    compress_to(include_bytes!("dns-rules.txt"), dest_path);

    let dest_path = Path::new(&out_dir).join("ip.zst");
    compress_to(include_bytes!("ip.txt"), dest_path);
}

fn compress_to(input: &[u8], output: impl AsRef<Path>) {
    let f = File::create(output).unwrap();
    let mut encoder = Encoder::new(f, 22).unwrap();
    encoder.write_all(input).unwrap();
    encoder.finish().unwrap();
}
