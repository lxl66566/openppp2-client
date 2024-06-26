use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zstd::stream::write::Encoder;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("direct-list.zst");

    let mut encoder = Encoder::new(Vec::new(), 22).unwrap();
    encoder
        .write_all(include_bytes!("direct-list.txt"))
        .unwrap();
    let compressed_bytes = encoder.finish().unwrap();

    let mut f = File::create(dest_path).unwrap();
    f.write_all(&compressed_bytes).unwrap();
}
