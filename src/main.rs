use huml_lsp::rpc::decode_from_buffer_rpc;
use serde_json::Value;
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Write},
};

fn build_logger() -> impl FnMut(&str) -> () {
    let log_file_path_result = env::var("HUML_LOG_PATH");
    let log_path = log_file_path_result.as_deref().unwrap_or("/tmp/huml.log");
    let mut log_file = File::create(log_path).unwrap();
    let logger = move |message: &str| {
        let _ = writeln!(log_file, "{message}");
    };
    logger
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin_reader = io::stdin().lock();
    let mut read_buf = Vec::new();
    let mut log = build_logger();

    loop {
        log("waiting for request");
        let value: Value = match decode_from_buffer_rpc(&mut stdin_reader, &mut read_buf) {
            Ok(val) => val,
            Err(e) => panic!("Invalid data format {e}"),
        };

        log("Recieved request");
        let json = serde_json::to_string_pretty(&value).expect("Serialization failed");
        log(&json);
    }
}
