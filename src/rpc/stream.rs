use crate::rpc::{DecodeError, RPC_HEADER_LEN, RPC_HEADER_PREFIX};
use std::io::Read;

/// A stream of messages parsed from a reader
pub struct RPCMessageStream<R>
where
    R: Read,
{
    reader: R,
    read_buffer: Vec<u8>,
}

impl<R> RPCMessageStream<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            read_buffer: Vec::with_capacity(1024),
        }
    }

    pub fn get_message_from_reader(&mut self) -> Result<&str, DecodeError>
    where
        R: Read,
    {
        let message_end_index: usize;
        loop {
            let mut read_buf = [0; 400];
            let Ok(bytes_read) = self.reader.read(&mut read_buf) else {
                continue;
            };
            self.read_buffer.extend_from_slice(&read_buf[..bytes_read]);

            // Ensure we have enough bytes to test for header
            if self.read_buffer.len() <= RPC_HEADER_LEN {
                continue;
            }

            // Check for header presence a the beginning of the message
            // RPC_HEADER_PREFIX - Content-Length: <number>
            if !self.read_buffer.starts_with(RPC_HEADER_PREFIX.as_bytes()) {
                return Err(DecodeError::MissingOrInvalidHeader);
            }

            // Find index of crlf, i.e. (\r\n\r\n) to find the header boundary
            let Some(content_length_digits) = self.read_buffer[RPC_HEADER_LEN..]
                .iter()
                .position(|&byte| byte == b'\r')
            else {
                // Have not recieved enough bytes yet.
                continue;
            };

            // Calculate the length of the body
            let double_crlf_loc = RPC_HEADER_LEN + content_length_digits;
            let content_length_str =
                str::from_utf8(&self.read_buffer[RPC_HEADER_LEN..double_crlf_loc])
                    .map_err(|e| DecodeError::InvalidContentLengthEncoding(e))?;

            let content_length: usize = content_length_str
                .trim()
                .parse()
                .map_err(|e| DecodeError::ContentLengthNotNumber(e))?;

            // Check the presence of body, i.e. the content after the double crlf
            let body_start_pos = double_crlf_loc + "\r\n\r\n".len();
            let body_end_pos = body_start_pos + content_length;

            // Enough of the body is not recieved yet
            if body_end_pos > self.read_buffer.len() {
                continue;
            }

            message_end_index = body_end_pos;
            break;
        }

        let message = str::from_utf8(&self.read_buffer[..message_end_index].as_ref())
            .expect("Invalid Message Format - Conversion to utf8 failed");

        Ok(message)
    }
}

impl<R> Iterator for RPCMessageStream<R>
where
    R: Read,
{
    type Item = Result<String, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        let message = self
            .get_message_from_reader()
            .map(|message| message.to_string())
            .inspect(|message| {
                self.read_buffer.drain(..message.len());
            });

        Some(message)
    }
}

#[cfg(test)]
mod tests {
    use crate::rpc::{DecodeError, RPCMessageStream};
    use std::{
        io::{self, Cursor, Write},
        thread,
        time::Duration,
    };

    #[test]
    fn should_deserialize_from_buf_with_payload() {
        let json_str =
            format!("Content-Length: 35\r\n\r\n{{\"jsonrpc\":\"2.0\",\"message\":\"Hello\"}}");

        let json_buf = Cursor::new(json_str.clone());
        let mut rpc_stream = RPCMessageStream::new(json_buf);

        let message = rpc_stream.next().unwrap().expect("Decode Failed");

        assert_eq!(message, json_str);
    }

    #[test]
    fn should_decode_multiple_messages() {
        let json_msg1 = "Content-Length: 35\r\n\r\n{\"jsonrpc\":\"2.0\",\"message\":\"Hello\"}";
        let json_msg2 = "Content-Length: 17\r\n\r\n{\"jsonrpc\":\"2.0\"}";
        let json_buf = Cursor::new(format!("{json_msg1}{json_msg2}"));
        let mut rpc_stream = RPCMessageStream::new(json_buf);

        assert_eq!(rpc_stream.next().unwrap().unwrap(), json_msg1);

        assert_eq!(rpc_stream.next().unwrap().unwrap(), json_msg2);
    }

    #[test]
    fn should_wait_till_payload_ready() {
        let json_str =
            format!("Content-Length: 35\r\n\r\n{{\"jsonrpc\":\"2.0\",\"message\":\"Hello\"}}");

        let (reader, mut writer) = io::pipe().unwrap();
        thread::spawn({
            let json_str = json_str.clone();
            move || {
                for string_chunk in json_str.as_bytes().chunks(5) {
                    writer.write(string_chunk).unwrap();

                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        let mut rpc_stream = RPCMessageStream::new(reader);

        assert_eq!(rpc_stream.next().unwrap().unwrap(), json_str);
    }

    #[test]
    fn should_err_for_invalid_header() {
        let json_str = format!("{{\"jsonrpc\":\"2.0\",\"message\":\"Hello\"}}");
        let json_buf = Cursor::new(json_str);
        let mut rpc_stream = RPCMessageStream::new(json_buf);

        assert!(matches!(
            rpc_stream.next().unwrap(),
            Err(DecodeError::MissingOrInvalidHeader)
        ));
    }
}
