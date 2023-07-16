use crate::error::SparsecError;

pub fn utf8_string(utf8: &[u8]) -> Result<String, crate::error::SparsecError> {
    String::from_utf8(utf8.to_vec()).map_err(|e| SparsecError::InternalParserError {
        details: e.to_string(),
    })
}
