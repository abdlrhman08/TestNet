fn main() {
    let result = tonic_build::compile_protos("proto/service.proto");
    if let Err(e) = result {
        panic!("Error while building: {}", e);
    }
}
