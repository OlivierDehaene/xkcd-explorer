use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir("src/pb").unwrap_or(());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .out_dir("src/pb")
        .include_file("mod.rs")
        .compile(&["../proto/xkcd_explorer.proto"], &["../proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    Ok(())
}
