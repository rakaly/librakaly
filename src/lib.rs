use libc::{c_char, c_uchar, size_t};

/// Melts binary encoded ironman data into normal plaintext data that can be understood by EU4
/// natively.
///
/// Parameters:
///
///  - data: Pointer to immutable data that represents the ironman data. The data can be a ironman
///  savefile in a zip format, in which case rakaly will take care of unzipping, melting, and
///  concatenating the data into a single plaintext output. The pointer can point to ironman data
///  that has already been unzipped.
///  - data_len: Length of the data indicated by the data pointer. It is undefined behavior if the
///  given length does not match the actual length of the data
///  - out: Mutable pointer to data which will be filled with the plaintext savefile. Will end
///  in a null terminator (but the null terminator is not counted as part of the length).
///  The encoding of the plaintext is not strictly defined, rakaly will dump
///  whatever data is found as strings in the binary data such that they are bit for bit
///  compatible. While this could mean that string data could be have a different encoding from the
///  rest of the file, in practice only windows-1252 encoding has been seen for EU4 ironman saves.
///  - out_len: Number of elements now contained in the out pointer.
///
/// This function will return non-zero to indicate an error. A non-zero status code can occur from
/// the following:
///
///  - An early EOF
///  - Invalid format encountered
///  - Too many close delimiters
///
/// If an unknown token is encountered and rakaly doesn't know how to convert it to plaintext there
/// are two possible outcomes:
///
///  - If the token is part of an object's key then key and value will not appear in the plaintext
///  output
///  - Else the object value (or array value) will be string of "__unknown_x0$z" where z is the
///  hexadecimal representation of the unknown token.
///
/// A future improvement should allow a client to expose the exact error message or expose the
/// option to custom behavior on unknown tokens.
#[no_mangle]
pub extern "C" fn rakaly_eu4_melt(
    data_ptr: *const c_char,
    data_len: size_t,
    out: *mut *mut c_char,
    out_len: *mut size_t,
) -> c_char {
    std::panic::catch_unwind(|| _rakaly_eu4_melt(data_ptr, data_len, out, out_len)).unwrap_or(1)
}

fn _rakaly_eu4_melt(
    data_ptr: *const c_char,
    data_len: size_t,
    out: *mut *mut c_char,
    out_len: *mut size_t,
) -> c_char {
    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    match eu4save::melt(&data, eu4save::FailedResolveStrategy::Ignore) {
        Ok(mut d) => {
            unsafe { *out_len = d.len() };

            // Push the null terminating character after getting the length so that the
            // length doesn't include the terminator
            d.push(b'\0');
            let out_ptr = d.as_mut_ptr() as *mut c_char;
            unsafe { *out = out_ptr };
            std::mem::forget(d);
            0
        }
        Err(_e) => 1,
    }
}
