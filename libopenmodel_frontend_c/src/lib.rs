use std::ffi::CStr;
use std::os::raw::c_char;

use openmodel_frontend;
use openmodel_frontend::errors;

#[repr(C)]
pub enum Status {
    Ok,
    ConnectionError,
    ServerError,
    ClientError,
}

extern "C" fn generate_text(prompt: *const c_char, output_buf: *mut c_char, output_buf_len: usize) -> Status
{
    let prompt_str = unsafe {
        assert!(!prompt.is_null());

        CStr::from_ptr(prompt)
    }.to_str().unwrap().to_string();

    let mut output_str = String::with_capacity(output_buf_len);

    unsafe {
        let last_byte_ptr = output_buf.add(output_buf_len - 1);
        *last_byte_ptr = 0 as c_char;
    }

    match openmodel_frontend::generate_text(&prompt_str, &mut output_str) {
        Ok(()) => Status::Ok,
        Err(errors::GenerateText::ConnectionError) => Status::ConnectionError,
        Err(errors::GenerateText::ServerError) => Status::ServerError,
        Err(errors::GenerateText::ClientError) => Status::ClientError,
    }
}
