pub mod errors {
    pub enum GenerateText {
        ConnectionError,
        ServerError,
        ClientError,
    }
}

pub fn generate_text(prompt: &String, buf: &mut String) -> Result<(), errors::GenerateText> {
    // TODO
    buf.clear();
    Err(errors::GenerateText::ServerError)
}
