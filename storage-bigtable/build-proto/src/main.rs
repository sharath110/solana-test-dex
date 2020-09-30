fn main() -> Result<(), std::io::Error> {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir = manifest_dir.join("../proto");
    let googleapis = manifest_dir.join("googleapis");

    println!("Google API directory: {}", googleapis.display());
    println!("output directory: {}", out_dir.display());

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(&out_dir)
        .compile(
            &[googleapis.join("google/bigtable/v2/bigtable.proto")],
            &[googleapis],
        )?;

    let out_dir = manifest_dir.join("../proto");
    let proto_files = manifest_dir.join("../src");

    println!("Protobuf directory: {}", proto_files.display());
    println!("output directory: {}", out_dir.display());

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(&out_dir)
        .compile(
            &[proto_files.join("confirmed_block.proto")],
            &[proto_files],
        )
}
