use std::{io, sync::mpsc, thread};

use crate::{lsp::notification::ServerClientNotification, rpc::jsonrpc_encode};

pub fn initialize_notification_loop<WriteOutput>(
    mut write_output: WriteOutput,
) -> mpsc::Sender<ServerClientNotification>
where
    WriteOutput: FnMut(&str) -> io::Result<()> + Send + 'static,
{
    let (msg_sender, msg_reciever) = mpsc::channel::<ServerClientNotification>();
    thread::spawn(move || {
        for msg in msg_reciever {
            let payload = jsonrpc_encode(&msg).unwrap();
            let _ = write_output(&payload);
        }
    });
    msg_sender
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::lsp::notification::trace::LogTraceParams;

    use super::*;
    use io::Write;

    #[test]
    fn should_write_notification() {
        let (mut reader, mut writer) = io::pipe().unwrap();
        let notification =
            ServerClientNotification::from(LogTraceParams::new("Hello World".to_string(), None));

        // Send message and drop sender to close channel
        {
            let sender = initialize_notification_loop(move |msg| write!(writer, "{msg}"));
            sender
                .send(notification.clone())
                .expect("Sender shouldn't fail");
        }

        let mut actual_content_written = String::new();
        reader.read_to_string(&mut actual_content_written).unwrap();

        let expected_jsonrpc_payload =
            jsonrpc_encode::<ServerClientNotification>(&notification).unwrap();
        assert_eq!(actual_content_written, expected_jsonrpc_payload);
    }
}
