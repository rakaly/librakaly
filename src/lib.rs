use libc::{c_char, c_int, c_uchar, size_t};

/// A MeltedBuffer holds the result of a melting operation (binary to plaintext translation).
/// Either the melting operation succeeded, and the buffer is filled with plaintext or it contains
/// an error.
pub struct MeltedBuffer {
    buffer: Vec<u8>,
    error: Option<Box<dyn std::error::Error>>,
}

/// A non-zero return value indicates an error with the melted buffer
///
/// A non-zero status code can occur from the following:
///
///  - An early EOF
///  - Invalid format encountered
///  - Too many close delimiters
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_error_code(res: *const MeltedBuffer) -> c_int {
    if res.is_null() || (*res).error.is_some() {
        -1
    } else {
        0
    }
}

/// Destroys a `MeltedBuffer` once you are done with it.
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_free_melt(res: *mut MeltedBuffer) {
    if !res.is_null() {
        drop(Box::from_raw(res));
    }
}

/// Returns the length of the melted data in bytes. Length excludes null terminator if present,
/// so make sure you add 1 to this result to ensure a buffer big enough to hold the data is
/// allocated
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_data_length(res: *const MeltedBuffer) -> size_t {
    if res.is_null() {
        return 0;
    }

    let melted = &*res;

    // Since `strlen` does not include null terminator in length calculation, neither will we.
    // And if there isn't an error, we know that the melted data will end with a null terminator
    // (as we're the ones that added it)
    if melted.error.is_some() {
        0 as size_t
    } else {
        // but as a sanity check, we'll make sure we can't underflow
        std::cmp::max(melted.buffer.len(), 1) - 1 as size_t
    }
}

/// Writes the melted data into a provided buffer that is a given length.
///
/// Returns the number of bytes copied from the melted data to the provided buffer.
///
/// If the buffer is not long enough for the melted data, then -1 is returned.
///
/// If the melted data or provided buffer are null, then -1 is returned.
///
/// # Safety
///
/// - Must pass in a valid pointer to a `MeltedBuffer`
/// - Given buffer must be at least the given length in size
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_write_data(
    res: *const MeltedBuffer,
    buffer: *mut c_char,
    length: size_t,
) -> size_t {
    if res.is_null() || buffer.is_null() {
        let result = -1;
        return result as size_t;
    }

    let res = &*res;
    let buffer: &mut [u8] = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if buffer.len() < res.buffer.len() {
        let result = -1;
        return result as size_t;
    }

    std::ptr::copy_nonoverlapping(res.buffer.as_ptr(), buffer.as_mut_ptr(), res.buffer.len());
    res.buffer.len() as size_t
}

/// Melts binary encoded ironman data into normal plaintext data that can be understood by EU4
/// natively. The melted buffer, when written out will contain windows-1252 encoded plaintext.
///
/// Parameters:
///
///  - data: Pointer to immutable data that represents the ironman data. The data can be a ironman
///  savefile in a zip format, in which case rakaly will take care of unzipping, melting, and
///  concatenating the data into a single plaintext output. The pointer can point to ironman data
///  that has already been unzipped.
///  - data_len: Length of the data indicated by the data pointer. It is undefined behavior if the
///  given length does not match the actual length of the data
///
/// If an unknown token is encountered and rakaly doesn't know how to convert it to plaintext there
/// are two possible outcomes:
///
///  - If the token is part of an object's key then key and value will not appear in the plaintext
///  output
///  - Else the object value (or array value) will be string of "__unknown_x0$z" where z is the
///  hexadecimal representation of the unknown token.
#[no_mangle]
pub extern "C" fn rakaly_eu4_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let buffer = _rakaly_eu4_melt(data_ptr, data_len);
        Box::into_raw(Box::new(buffer))
    })
    .unwrap_or_else(|_| std::ptr::null_mut())
}

fn _rakaly_eu4_melt(data_ptr: *const c_char, data_len: size_t) -> MeltedBuffer {
    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    match eu4save::melt(&data, eu4save::FailedResolveStrategy::Ignore) {
        Ok(mut d) => {
            // Push the null terminating for our C friends
            d.push(b'\0');
            MeltedBuffer {
                buffer: d,
                error: None,
            }
        }
        Err(e) => MeltedBuffer {
            buffer: Vec::new(),
            error: Some(Box::new(e)),
        },
    }
}

/// Melts binary encoded CK3 data into normal plaintext data. The melted buffer, when written utf-8 encoded
/// plaintext.
///
/// Parameters:
///
///  - data: Pointer to immutable data that represents the binary data. The data can be:
///    - autosave save
///    - ironman save
///    - binary data
///  - data_len: Length of the data indicated by the data pointer. It is undefined behavior if the
///  given length does not match the actual length of the data
///
/// If an unknown token is encountered and rakaly doesn't know how to convert it to plaintext there
/// are two possible outcomes:
///
///  - If the token is part of an object's key then key and value will not appear in the plaintext
///  output
///  - Else the object value (or array value) will be string of "__unknown_x0$z" where z is the
///  hexadecimal representation of the unknown token.
#[no_mangle]
pub extern "C" fn rakaly_ck3_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let buffer = _rakaly_ck3_melt(data_ptr, data_len);
        Box::into_raw(Box::new(buffer))
    })
    .unwrap_or_else(|_| std::ptr::null_mut())
}

fn _rakaly_ck3_melt(data_ptr: *const c_char, data_len: size_t) -> MeltedBuffer {
    use ck3save::{FailedResolveStrategy, Melter};
    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    let melter = Melter::new().with_on_failed_resolve(FailedResolveStrategy::Ignore);
    match melter.melt(data) {
        Ok(mut d) => {
            // Push the null terminating for our C friends
            d.push(b'\0');
            MeltedBuffer {
                buffer: d,
                error: None,
            }
        }
        Err(e) => MeltedBuffer {
            buffer: Vec::new(),
            error: Some(Box::new(e)),
        },
    }
}
