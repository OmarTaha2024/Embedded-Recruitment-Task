fn main() {
    // Path to the directory containing your .proto files
    let proto_dir = "proto";  // Update this to match the location of your .proto file

    // Use tonic-build to compile the .proto files
    tonic_build::configure()
        .compile(&["proto/messages.proto"], &[proto_dir]) // Specify the path to your proto files
        .expect("Failed to compile Protobuf files");
}
