use crate::rpc::{DecodeError, RPC_HEADER_LEN, RPC_HEADER_PREFIX, jsonrpc_decode};
use serde::Deserialize;
use std::io::Read;

/// Decode a value from a buffer
pub fn decode_from_buffer_rpc<Data, R>(
    reader: &mut R,
    read_buffer: &mut Vec<u8>,
) -> Result<Data, DecodeError>
where
    Data: for<'de> Deserialize<'de>,
    R: Read,
{
    let message_end_index: usize;
    loop {
        let mut read_buf = [0; 400];
        let Ok(bytes_read) = reader.read(&mut read_buf) else {
            continue;
        };
        read_buffer.extend_from_slice(&read_buf[..bytes_read]);

        // Ensure we have enough bytes to test for header
        if read_buffer.len() <= RPC_HEADER_LEN {
            continue;
        }

        // Check for header presence a the beginning of the message
        // RPC_HEADER_PREFIX - Content-Length: <number>
        if !read_buffer.starts_with(RPC_HEADER_PREFIX.as_bytes()) {
            return Err(DecodeError::MissingOrInvalidHeader);
        }

        // Find index of crlf, i.e. (\r\n\r\n) to find the header boundary
        let Some(content_length_digits) = read_buffer[RPC_HEADER_LEN..]
            .iter()
            .position(|&byte| byte == b'\r')
        else {
            // Have not recieved enough bytes yet.
            continue;
        };

        // Calculate the length of the body
        let double_crlf_loc = RPC_HEADER_LEN + content_length_digits;
        let content_length_str = str::from_utf8(&read_buffer[RPC_HEADER_LEN..double_crlf_loc])
            .map_err(|e| DecodeError::InvalidContentLengthEncoding(e))?;

        let content_length: usize = content_length_str
            .trim()
            .parse()
            .map_err(|e| DecodeError::ContentLengthNotNumber(e))?;

        // Check the presence of body, i.e. the content after the double crlf
        let body_start_pos = double_crlf_loc + "\r\n\r\n".len();
        let body_end_pos = body_start_pos + content_length;

        // Enough of the body is not recieved yet
        if body_end_pos > read_buffer.len() {
            continue;
        }

        message_end_index = body_end_pos;
        break;
    }

    let message_buf = read_buffer.drain(..message_end_index);
    let message = str::from_utf8(&message_buf.as_ref())
        .expect("Invalid Message Format - Conversion to utf8 failed");

    println!("{message}");

    jsonrpc_decode(message)
}

#[cfg(test)]
mod tests {
    use std::{
        io::{self, Cursor, Write},
        thread,
        time::Duration,
    };

    use serde::{Deserialize, Serialize};

    use crate::rpc::{DecodeError, decode_from_buffer_rpc};

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct TestData {
        jsonrpc: String,
        message: String,
    }

    #[test]
    fn should_deserialize_from_buf_with_payload() {
        let json_str =
            format!("Content-Length: 35\r\n\r\n{{\"jsonrpc\":\"2.0\",\"message\":\"Hello\"}}");

        let mut read_buf = Vec::new();
        let mut json_buf = Cursor::new(json_str);
        let decoded_data: TestData =
            decode_from_buffer_rpc(&mut json_buf, &mut read_buf).expect("Decode failed");

        assert_eq!(
            decoded_data,
            TestData {
                jsonrpc: "2.0".to_string(),
                message: "Hello".to_string()
            }
        );
    }

    #[test]
    fn should_decode_multiple_messages() {
        let json_str = "Content-Length: 35\r\n\r\n{\"jsonrpc\":\"2.0\",\"message\":\"Hello\"}";

        let mut read_buf = Vec::new();
        let mut json_buf = Cursor::new(format!("{json_str}{json_str}"));

        let decoded_data_first: TestData =
            decode_from_buffer_rpc(&mut json_buf, &mut read_buf).expect("Decode failed");

        assert_eq!(
            decoded_data_first,
            TestData {
                jsonrpc: "2.0".to_string(),
                message: "Hello".to_string()
            }
        );

        let decoded_data_second: TestData =
            decode_from_buffer_rpc(&mut json_buf, &mut read_buf).expect("Decode failed");

        assert_eq!(
            decoded_data_second,
            TestData {
                jsonrpc: "2.0".to_string(),
                message: "Hello".to_string()
            }
        );
    }

    #[test]
    fn should_wait_till_payload_ready() {
        let json_str =
            format!("Content-Length: 35\r\n\r\n{{\"jsonrpc\":\"2.0\",\"message\":\"Hello\"}}");

        let (mut reader, mut writer) = io::pipe().unwrap();
        thread::spawn(move || {
            for string_chunk in json_str.as_bytes().chunks(5) {
                writer.write(string_chunk).unwrap();

                thread::sleep(Duration::from_millis(100));
            }
        });

        let mut read_buf = Vec::new();
        let decoded_data: TestData =
            decode_from_buffer_rpc(&mut reader, &mut read_buf).expect("Decode Failed");

        assert_eq!(
            decoded_data,
            TestData {
                jsonrpc: "2.0".to_string(),
                message: "Hello".to_string()
            }
        );
    }

    #[test]
    fn should_err_for_invalid_header() {
        let json_str = format!("{{\"jsonrpc\":\"2.0\",\"message\":\"Hello\"}}");
        let mut json_buf = Cursor::new(json_str);
        let mut read_buf = Vec::new();
        let decoded_data: DecodeError =
            decode_from_buffer_rpc::<TestData, _>(&mut json_buf, &mut read_buf).unwrap_err();
        assert!(matches!(decoded_data, DecodeError::MissingOrInvalidHeader));
    }
}
