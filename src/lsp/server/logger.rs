use std::{
    io::{self, Write, stdout},
    sync::mpsc,
    thread,
};

use crate::{
    lsp::notification::{
        ServerClientNotification,
        trace::{LogTraceParams, TraceValue},
    },
    rpc::jsonrpc_encode,
};

pub enum LogEvent {
    SetTrace(TraceValue),
    LogMessage(LogTraceParams),
    Shutdown,
}

impl From<LogTraceParams> for LogEvent {
    fn from(v: LogTraceParams) -> Self {
        Self::LogMessage(v)
    }
}

impl From<TraceValue> for LogEvent {
    fn from(v: TraceValue) -> Self {
        Self::SetTrace(v)
    }
}

impl LogEvent {
    /// Returns `true` if the log event is [`Shutdown`].
    ///
    /// [`Shutdown`]: LogEvent::Shutdown
    #[must_use]
    pub fn is_shutdown(&self) -> bool {
        matches!(self, Self::Shutdown)
    }
}

/// Initialize logger to send log notifications to the client. Spawns a thread
/// that monitors a channel for [`LogEvent`].
/// Returns a sender to transmit said [`LogEvent`]
fn initialize_logger<F>(initial_trace: TraceValue, mut write_log: F) -> mpsc::Sender<LogEvent>
where
    F: (FnMut(&str) -> io::Result<()>) + Send + 'static,
{
    let (log_sender, log_reciever) = mpsc::channel::<LogEvent>();

    // thread to handle logging requests
    thread::spawn(move || {
        let mut trace = initial_trace;
        for event in log_reciever {
            match event {
                LogEvent::Shutdown => break,
                LogEvent::SetTrace(TraceValue::Off) => break,
                LogEvent::SetTrace(t) => {
                    trace = t;
                    continue;
                }
                LogEvent::LogMessage(log_info) => {
                    // Adjust the trace according to the log level set by the client
                    let trace_adjusted_log = log_info
                        .with_trace_level(trace)
                        .expect("Trace level to not be off");

                    // Get encoded notification value
                    let notification = ServerClientNotification::from(trace_adjusted_log);
                    let Ok(encoded_notification) = jsonrpc_encode(&notification) else {
                        continue;
                    };

                    // Lock stdin and write output
                    let _ = write_log(&encoded_notification);
                }
            }
        }
    });

    log_sender
}

/// Initialize logger to send log notifications to the client via stdout.
/// Spawns a thread that monitors a channel for [`LogEvent`].
/// Returns a sender to transmit said [`LogEvent`]

pub fn initialize_stdout_logger(initial_trace: TraceValue) -> mpsc::Sender<LogEvent> {
    initialize_logger(initial_trace, |log| {
        let mut stdout = stdout().lock();
        write!(stdout, "{log}")
    })
}

#[cfg(test)]
mod test {

    use std::io::Read;

    use super::*;

    use crate::lsp::server::logger::initialize_logger;

    #[test]
    fn should_log() {
        // The log we want to write
        let log = LogTraceParams::new("Hello World".to_string(), None);

        // Reader and writer
        let (mut reader, mut writer) = std::io::pipe().unwrap();

        // Scope for handling send events. Ensures `log_event_sender`
        // gets dropped before we start reading
        {
            // Logger
            let log_event_sender =
                initialize_logger(TraceValue::Message, move |log| write!(writer, "{log}"));

            log_event_sender
                .send(LogEvent::LogMessage(log.clone()))
                .expect("Logging should be successful");
        }

        let log_nofitication = ServerClientNotification::from(log.clone());
        let expected_message =
            jsonrpc_encode(&log_nofitication).expect("Should be able to encode log_event");

        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Should be able to read data back");

        assert_eq!(String::from_utf8_lossy(&buffer), expected_message);
    }
}
