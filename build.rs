fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        // .build_server(false)
        .compile_protos(&["proto/hello_world.proto", "proto/echo.proto"], &["proto"])?;
    Ok(())
}
