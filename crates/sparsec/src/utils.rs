use super::modname::SparsecError;

pub fn utf8_string(utf8: &[u8]) -> Result<String, modname::SparsecError> {
    String::from_utf8(utf8.to_vec()).map_err(|e| SparsecError::InternalParserError {
        details: e.to_string(),
    })
}
