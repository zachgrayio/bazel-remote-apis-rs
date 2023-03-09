use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &[
            "proto/bazel-remote-apis/build/bazel/remote/execution/v2/remote_execution.proto",
            "proto/googleapis/google/bytestream/bytestream.proto",
            "proto/kv_storage/kv_storage.proto"
        ],
        &["proto/bazel-remote-apis", "proto/googleapis", "proto/kv_storage"],
    )?;
    Ok(())
}
