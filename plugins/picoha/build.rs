
use std::io::Result;
use std::path::Path;
use std::env;
use std::fs;

fn main() {
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let source = format!("{}/_.rs", out_dir);

    // Build proto files only if required

    // DIO
    let target = "src/dio/api_dio.rs";
    if !Path::new(target).exists() {
        let dio_proto_path = "api/api_dio.proto";
        prost_build::compile_protos(&[dio_proto_path], &["src/"]).unwrap();
        fs::rename(source, target).unwrap();
    }
    
    
}

