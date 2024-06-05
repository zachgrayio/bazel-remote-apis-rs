use tonic_build;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("reapi_descriptor.bin"))
        .compile(
        &[
            "proto/bazel-remote-apis/build/bazel/remote/execution/v2/remote_execution.proto",
            "proto/googleapis/google/bytestream/bytestream.proto",
        ],
        &["proto/bazel-remote-apis", "proto/googleapis",],
    )?;
    Ok(())
}
