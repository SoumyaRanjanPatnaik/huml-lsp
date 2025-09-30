use serde::{Deserialize, Serialize};

use crate::rpc::DecodeError;

pub const RPC_HEADER_PREFIX: &str = "Content-Length: ";
pub const RPC_HEADER_LEN: usize = RPC_HEADER_PREFIX.len();

/// Encode a json serializable object as per the BASE_PROTOCOL specified
/// in the LSP specification
///
/// SEE [BASE_PROTOCOL](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#baseProtocol)
pub fn jsonrpc_encode<DType: Serialize>(
    data: &DType,
) -> Result<String, Box<dyn std::error::Error>> {
    let json = serde_json::to_string(data)?;
    let content_length = json.len();

    Ok(format!("Content-Length: {content_length}\r\n\r\n{json}"))
}

/// in the LSP specification
///
/// SEE [BASE_PROTOCOL](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#baseProtocol)
pub fn jsonrpc_decode<'de, DType>(data: &'de str) -> Result<DType, DecodeError>
where
    DType: Deserialize<'de>,
{
    // Split header and body
    let mut split_data_iter = data.split("\r\n\r\n");

    // Extract header and body
    let header = split_data_iter
        .next()
        .ok_or(DecodeError::MissingOrInvalidHeader)?;
    let body = split_data_iter.next().ok_or(DecodeError::IncompleteData)?;

    // Prase Content-Length from header
    if !header.starts_with(RPC_HEADER_PREFIX) {
        return Err(DecodeError::MissingOrInvalidHeader);
    }
    let content_length_str = &header[RPC_HEADER_LEN..];
    let content_length: usize = content_length_str
        .trim()
        .parse()
        .map_err(|e| DecodeError::ContentLengthNotNumber(e))?;

    // Validate body length
    if body.len() != content_length {
        return Err(DecodeError::IncompleteData);
    }

    // Deserialize JSON body
    let deserialized_data: DType = serde_json::from_str(body)?;

    Ok(deserialized_data)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::rpc::jsonrpc_decode;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct TestStruct {
        jsonrpc: String,
    }

    #[test]
    fn test_encode() {
        let test_data = TestStruct {
            jsonrpc: "2.0".to_string(),
        };
        let encoded = super::jsonrpc_encode(&test_data).expect("Encoding failed");
        assert_eq!(encoded, "Content-Length: 17\r\n\r\n{\"jsonrpc\":\"2.0\"}");
    }

    #[test]
    fn test_decode() {
        let jsonrpc_data = "Content-Length: 17\r\n\r\n{\"jsonrpc\":\"2.0\"}";
        let decoded_value: TestStruct = jsonrpc_decode(jsonrpc_data).expect("Decoding failed");
        let expected_value = TestStruct {
            jsonrpc: "2.0".to_string(),
        };
        assert_eq!(decoded_value, expected_value);
    }
}
