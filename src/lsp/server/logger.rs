use std::{io::Write, sync::mpsc, thread};

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

/// Initialize logger and
pub fn initialize_logger(initial_trace: TraceValue) -> mpsc::Sender<LogEvent> {
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
                    let mut stdout = std::io::stdout().lock();
                    let _ = write!(stdout, "{encoded_notification}");
                }
            }
        }
    });

    log_sender
}
