extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["proto/msg.proto"], &["proto"]).unwrap();
    println!("cargo:rerun-if-changed=proto");
}
