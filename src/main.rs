use huml_lsp::{
    lsp::{recieved_message::RecievedMessage, server::Server},
    rpc::{RPCMessageStream, jsonrpc_decode, jsonrpc_encode},
};
use serde_json::Value;
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Write},
    panic,
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
    let mut log = build_logger();
    let mut server = Server::new();

    let stdin_reader = io::stdin().lock();
    let rpc_reader = RPCMessageStream::new(stdin_reader);

    log("Started Server. Waiting for Messages...");
    for message_result in rpc_reader {
        let message_string = match message_result {
            Ok(s) => s,
            Err(e) => {
                log(&format!("Error reading from stream: {}", e));
                continue; // Skip to the next message on read error
            }
        };

        // Debug logging to inspect requests
        #[cfg(debug_assertions)]
        {
            if let Ok(json_value) = jsonrpc_decode::<Value>(&message_string) {
                if let Ok(pretty_json) = serde_json::to_string_pretty(&json_value) {
                    log(&format!("Message: {}", pretty_json));
                }
            }
        }

        // Parse / recieve the message
        let parsed_message: RecievedMessage =
            match jsonrpc_decode::<RecievedMessage>(&message_string) {
                Ok(msg) => msg,
                Err(decode_err) => {
                    log(&format!("Error parsing message: {decode_err}"));
                    panic!("Failed to parse message");
                }
            };

        let response = match parsed_message {
            RecievedMessage::Request(req) => server.handle_request(&req),
            RecievedMessage::Notification(notification) => {
                server.handle_notification(notification).unwrap();
                continue;
            }
        };

        let encoded_response = match response.map(|msg| jsonrpc_encode(&msg)) {
            Ok(Ok(res)) => res,
            Err(e) => {
                log(&format!("Failed to handle request: {e}"));
                panic!("Request Handlingg Error: {e}")
            }
            Ok(Err(e)) => {
                log(&format!("Failed to encode response: {e}"));
                panic!("Response encoding faileed: {e}")
            }
        };

        log(encoded_response.as_ref());

        let mut stdout_writer = io::stdout().lock();
        stdout_writer.write_all(encoded_response.as_bytes())?;
        stdout_writer.flush()?;
    }
    Ok(())
}
