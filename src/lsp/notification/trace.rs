use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SetTraceParams {
    value: TraceValue,
}

impl SetTraceParams {
    pub fn value(&self) -> TraceValue {
        self.value
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum TraceValue {
    Off,
    Message,
    Verbose,
}

impl TraceValue {
    /// Returns `true` if the trace value is [`Off`].
    ///
    /// [`Off`]: TraceValue::Off
    #[must_use]
    pub fn is_off(&self) -> bool {
        matches!(self, Self::Off)
    }

    /// Returns `true` if the trace value is [`Message`].
    ///
    /// [`Message`]: TraceValue::Message
    #[must_use]
    pub fn is_message(&self) -> bool {
        matches!(self, Self::Message)
    }

    /// Returns `true` if the trace value is [`Verbose`].
    ///
    /// [`Verbose`]: TraceValue::Verbose
    #[must_use]
    pub fn is_verbose(&self) -> bool {
        matches!(self, Self::Verbose)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogTraceParams {
    /// The message to be logged.
    message: String,
    /// Additional information that can be computed if the `trace` configuration
    /// is set to `'verbose'`
    verbose: Option<String>,
}

impl LogTraceParams {
    pub fn new(message: String, verbose: Option<String>) -> Self {
        Self { message, verbose }
    }

    pub fn with_trace_level(self, trace: TraceValue) -> Option<Self> {
        match trace {
            TraceValue::Off => None,
            TraceValue::Message => Some(Self::new(self.message, None)),
            TraceValue::Verbose => Some(Self::new(self.message, self.verbose)),
        }
    }
}
