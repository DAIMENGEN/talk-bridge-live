use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let proto_files = vec!["chat.proto"];
    let out_dir = current_dir().unwrap().join("src").join("protos_gen");
    let out_dir_str = out_dir
        .to_str()
        .expect("Output path contains invalid UTF-8 characters")
        .to_string();
    // Ensure output directory exists
    if let Err(e) = create_dir_all(&out_dir) {
        panic!("Failed to create output directory {:?}: {}", out_dir, e);
    }
    println!("Preparing to compile proto files: {:?}", proto_files);
    println!("Output directory: {}", out_dir_str);
    tonic_build::configure()
        .build_server(true)
        .out_dir(&out_dir_str)
        .compile_protos(&proto_files, &["../grpc/protos"])
        .unwrap_or_else(|e| {
            eprintln!("Protobuf compilation failed!");
            eprintln!("Proto files: {:?}", proto_files);
            eprintln!("Include paths: {:?}", ["../grpc/protos_gen"]);
            eprintln!("Output directory: {}", out_dir_str);
            panic!("Detailed error: {}", e);
        });
    tauri_build::build()
}
