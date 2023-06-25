fn main() {
    let path = "./proto/api.proto";

    tonic_build::configure()
        .build_client(true)
        .out_dir("./src/auto_generate")
        .compile(&[path], &["."])
        .unwrap();
}
