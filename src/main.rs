use huml_lsp::rpc::decode_from_buffer_rpc;
use serde_json::Value;
use std::{
    error::Error,
    fs::File,
    io::{self, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin_reader = io::stdin().lock();
    let mut read_buf = Vec::new();

    let mut log_file = File::create("/tmp/huml.log")?;
    writeln!(log_file, "Started Server")?;
    loop {
        writeln!(log_file, "waiting for request")?;
        let value: Value = match decode_from_buffer_rpc(&mut stdin_reader, &mut read_buf) {
            Ok(val) => val,
            Err(e) => panic!("Invalid data format {e}"),
        };
        writeln!(log_file, "Recieved request")?;

        let json = serde_json::to_string_pretty(&value).expect("Serialization failed");
        writeln!(log_file, "{json}")?;
    }
}
