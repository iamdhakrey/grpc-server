use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("{:?}", out_dir);
    let descriptor_path = out_dir.join("helloworld_descriptor.bin");
    tonic_prost_build::configure()
        // .build_server(false)
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(&["proto/hello_world.proto", "proto/echo.proto"], &["proto"])?;
    Ok(())
}
