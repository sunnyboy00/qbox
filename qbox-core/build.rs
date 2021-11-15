use std::env;
use std::path::PathBuf;

fn main() {
    // tonic_build::configure()
    //     // .out_dir("src/comm/proto/qbox.proto")
    //     .build_server(true)
    //     .build_client(true)
    //     .compile(&["proto/qbox.proto"], &["proto"])
    //     .unwrap();
    tonic_build::compile_protos("proto/qbox.proto").unwrap();
}
