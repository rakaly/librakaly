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

/// Returns the length of the melted data in bytes.
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_data_length(res: *const MeltedBuffer) -> size_t {
    if res.is_null() {
        return 0;
    }

    (*res).buffer.len()
}

/// Writes the melted data into a provided buffer that is a given length.
///
/// Returns the number of bytes copied from the melted data to the provided buffer.
///
/// If the buffer is not long enough for the melted data, then 0 is returned.
///
/// If the melted data or provided buffer are null, then 0 is returned.
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
        return 0;
    }

    let res = &*res;
    let buffer: &mut [u8] = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if buffer.len() < res.buffer.len() {
        return 0;
    }

    std::ptr::copy_nonoverlapping(res.buffer.as_ptr(), buffer.as_mut_ptr(), res.buffer.len());
    res.buffer.len()
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
    let mut zip_sink = Vec::new();
    eu4save::Eu4File::from_slice(data)
        .and_then(|file| file.parse(&mut zip_sink))
        .and_then(|file| {
            file.as_binary()
                .expect("binary file") // todo: replace with error
                .melter()
                .on_failed_resolve(eu4save::FailedResolveStrategy::Ignore)
                .verbatim(true)
                .melt(&eu4save::EnvTokens)
        })
        .map(|melted| MeltedBuffer {
            buffer: melted.into_data(),
            error: None,
        })
        .unwrap_or_else(|e| MeltedBuffer {
            buffer: Vec::new(),
            error: Some(Box::new(e)),
        })
}

/// Melts binary encoded CK3 data into normal plaintext data. The melted buffer will contain utf-8 encoded
/// text.
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
    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    let mut zip_sink = Vec::new();
    ck3save::Ck3File::from_slice(data)
        .and_then(|file| file.parse(&mut zip_sink))
        .and_then(|file| {
            file.as_binary()
                .expect("binary file") // todo: replace with error
                .melter()
                .on_failed_resolve(ck3save::FailedResolveStrategy::Ignore)
                .verbatim(true)
                .melt(&ck3save::EnvTokens)
        })
        .map(|melted| MeltedBuffer {
            buffer: melted.into_data(),
            error: None,
        })
        .unwrap_or_else(|e| MeltedBuffer {
            buffer: Vec::new(),
            error: Some(Box::new(e)),
        })
}

/// Melts binary encoded Imperator data into normal plaintext data. The melted buffer will contain utf-8 encoded
/// text.
///
/// Parameters:
///
///  - data: Pointer to immutable data that represents the binary data. The data can be:
///    - a save file
///    - binary data from already extracted gamestate
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
pub extern "C" fn rakaly_imperator_melt(
    data_ptr: *const c_char,
    data_len: size_t,
) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let buffer = _rakaly_imperator_melt(data_ptr, data_len);
        Box::into_raw(Box::new(buffer))
    })
    .unwrap_or_else(|_| std::ptr::null_mut())
}

fn _rakaly_imperator_melt(data_ptr: *const c_char, data_len: size_t) -> MeltedBuffer {
    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    let mut zip_sink = Vec::new();
    imperator_save::ImperatorFile::from_slice(data)
        .and_then(|file| file.parse(&mut zip_sink))
        .and_then(|file| {
            file.as_binary()
                .expect("binary file") // todo: replace with error
                .melter()
                .on_failed_resolve(imperator_save::FailedResolveStrategy::Ignore)
                .verbatim(true)
                .melt(&imperator_save::EnvTokens)
        })
        .map(|melted| MeltedBuffer {
            buffer: melted.into_data(),
            error: None,
        })
        .unwrap_or_else(|e| MeltedBuffer {
            buffer: Vec::new(),
            error: Some(Box::new(e)),
        })
}

/// Melts binary encoded HOI4 data into normal plaintext data. The melted buffer will contain utf-8 encoded
/// text.
///
/// Parameters:
///
///  - data: Pointer to immutable data that represents the binary data
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
pub extern "C" fn rakaly_hoi4_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let buffer = _rakaly_hoi4_melt(data_ptr, data_len);
        Box::into_raw(Box::new(buffer))
    })
    .unwrap_or_else(|_| std::ptr::null_mut())
}

fn _rakaly_hoi4_melt(data_ptr: *const c_char, data_len: size_t) -> MeltedBuffer {
    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    hoi4save::Hoi4File::from_slice(data)
        .and_then(|file| file.parse())
        .and_then(|file| {
            file.as_binary()
                .expect("binary file") // todo: replace with error
                .melter()
                .on_failed_resolve(hoi4save::FailedResolveStrategy::Ignore)
                .verbatim(true)
                .melt(&hoi4save::EnvTokens)
        })
        .map(|melted| MeltedBuffer {
            buffer: melted.into_data(),
            error: None,
        })
        .unwrap_or_else(|e| MeltedBuffer {
            buffer: Vec::new(),
            error: Some(Box::new(e)),
        })
}

/// Melts binary encoded vic3 data into normal plaintext data. The melted buffer will contain utf-8 encoded
/// text.
///
/// Parameters:
///
///  - data: Pointer to immutable data that represents the binary data
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
pub extern "C" fn rakaly_vic3_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let buffer = _rakaly_vic3_melt(data_ptr, data_len);
        Box::into_raw(Box::new(buffer))
    })
    .unwrap_or_else(|_| std::ptr::null_mut())
}

fn _rakaly_vic3_melt(data_ptr: *const c_char, data_len: size_t) -> MeltedBuffer {
    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    let mut zip_sink = Vec::new();
    vic3save::Vic3File::from_slice(data)
        .and_then(|file| file.parse(&mut zip_sink))
        .and_then(|file| {
            file.as_binary()
                .expect("binary file") // todo: replace with error
                .melter()
                .on_failed_resolve(vic3save::FailedResolveStrategy::Ignore)
                .verbatim(true)
                .melt(&vic3save::EnvTokens)
        })
        .map(|melted| MeltedBuffer {
            buffer: melted.into_data(),
            error: None,
        })
        .unwrap_or_else(|e| MeltedBuffer {
            buffer: Vec::new(),
            error: Some(Box::new(e)),
        })
}
