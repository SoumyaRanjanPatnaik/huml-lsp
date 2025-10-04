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

    let mut stdout_writer = io::stdout().lock();

    log("Started Server. Waiting for Messages...");
    for message_result in rpc_reader {
        // Debug logging to inspect requests
        #[cfg(debug_assertions)]
        {
            let message_json = &message_result
                .as_ref()
                .map(|msg| {
                    let json_value = jsonrpc_decode::<Value>(msg).unwrap();
                    serde_json::to_string_pretty(&json_value).unwrap()
                })
                .unwrap();
            // Log the message for inspection
            log(&format!("Message: {message_json}",));
        }

        // Parse / recieve the message
        let message: RecievedMessage = match message_result.map(|msg| jsonrpc_decode(&msg)) {
            Ok(Ok(msg)) => msg,
            Ok(Err(_)) => continue,
            Err(decode_err) => {
                log(&format!("Error parsing message: {decode_err}"));
                panic!("Failed to parse message");
            }
        };

        let response = match message {
            RecievedMessage::Request(req) => server.handle_request(req),
            RecievedMessage::Notification(notification) => {
                server.handle_notification(notification)?;
                continue;
            }
        };

        // Hanndle the request
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

        stdout_writer.write_all(encoded_response.as_ref())?;
        stdout_writer.flush()?;
    }
    Ok(())
}
