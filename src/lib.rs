use libc::{c_char, c_int, c_uchar, size_t};
use std::error::Error;

/// An opaque struct that holds the results of the melting operatation
pub enum MeltedBuffer {
    Verbatim,
    Text { header: Vec<u8>, body: Vec<u8> },
    Binary { body: Vec<u8>, unknown_tokens: bool },
    Error(Box<dyn Error>),
}

impl MeltedBuffer {
    fn len(&self) -> usize {
        match self {
            MeltedBuffer::Verbatim => 0,
            MeltedBuffer::Text { header, body } => header.len() + body.len(),
            MeltedBuffer::Binary { body, .. } => body.len(),
            MeltedBuffer::Error(_) => 0,
        }
    }
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
    if res.is_null() || matches!(&*res, MeltedBuffer::Error(_)) {
        -1
    } else {
        0
    }
}

/// Calculate the number of bytes in the for the melted output's error message.
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_error_length(res: *const MeltedBuffer) -> c_int {
    if res.is_null() {
        0
    } else {
        match &*res {
            MeltedBuffer::Error(x) => x.to_string().len() as c_int,
            _ => 0,
        }
    }
}

/// Write the most recent error message into a caller-provided buffer as a UTF-8
/// string, returning the number of bytes written.
///
/// # Note
///
/// This writes a **UTF-8** string into the buffer. Windows users may need to
/// convert it to a UTF-16 "unicode" afterwards.
///
/// If there are no recent errors then this returns `0` (because we wrote 0
/// bytes). `-1` is returned if there are any errors, for example when passed a
/// null pointer or a buffer of insufficient size.
///
/// # Safety
///
/// - Must pass in a valid pointer to a `MeltedBuffer`
/// - Given buffer must be at least the given length in size
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_error_write_data(
    res: *const MeltedBuffer,
    buffer: *mut c_char,
    length: c_int,
) -> c_int {
    if res.is_null() || buffer.is_null() {
        return -1;
    }

    let last_error = match &*res {
        MeltedBuffer::Error(e) => e,
        _ => return 0,
    };

    let error_message = last_error.to_string();
    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if error_message.len() >= buffer.len() {
        return -1;
    }

    std::ptr::copy_nonoverlapping(
        error_message.as_ptr(),
        buffer.as_mut_ptr(),
        error_message.len(),
    );

    error_message.len() as c_int
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

    (*res).len()
}

/// Returns true if the melter performed no work on the input
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_is_verbatim(res: *const MeltedBuffer) -> bool {
    if res.is_null() {
        return false;
    }

    matches!(&*res, MeltedBuffer::Verbatim)
}

/// Returns true if the melter needed to convert the binary input
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_binary_translated(res: *const MeltedBuffer) -> bool {
    if res.is_null() {
        return false;
    }

    matches!(&*res, MeltedBuffer::Binary { .. })
}

/// Returns true if the melter encountered unknown tokens in the binary input
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_binary_unknown_tokens(res: *const MeltedBuffer) -> bool {
    if res.is_null() {
        return false;
    }

    matches!(
        &*res,
        MeltedBuffer::Binary {
            unknown_tokens: true,
            ..
        }
    )
}

/// Writes plaintext data into a provided buffer that is a given length.
///
/// The encoding of the written data is dependant on the game. For instance, EU4
/// will fill the provided buffer with Windows-1252 encoded data, while CK3 uses
/// UTF-8.
///
/// Returns the number of bytes copied from the melted data to the provided
/// buffer.
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

    if buffer.len() < res.len() {
        return 0;
    }

    match res {
        MeltedBuffer::Verbatim => {}
        MeltedBuffer::Text { header, body } => {
            std::ptr::copy_nonoverlapping(header.as_ptr(), buffer.as_mut_ptr(), header.len());
            let offset = buffer.as_mut_ptr().add(header.len());
            std::ptr::copy_nonoverlapping(body.as_ptr(), offset, body.len());
        }
        MeltedBuffer::Binary { body, .. } => {
            std::ptr::copy_nonoverlapping(body.as_ptr(), buffer.as_mut_ptr(), body.len());
        }
        MeltedBuffer::Error(_) => {}
    }

    res.len()
}

/// Converts a save into uncompressed plaintext data.
///
/// Parameters:
///
///  - data: Pointer to the save data to convert. It is valid for this data to
///    be uncompressed plaintext data, compressed plaintext data, or binary data
///  - data_len: Length of the data indicated by the data pointer. It is
///    undefined behavior if the given length does not match the actual length
///    of the data
///
/// If an unknown binary token is encountered there are two possible outcomes:
///
///  - If the token is part of an object's key, then key and value will not
///    appear in the plaintext output
///  - Else the object value (or array value) will be string of "__unknown_x0$z"
///    where z is the hexadecimal representation of the unknown token.
#[no_mangle]
pub extern "C" fn rakaly_eu4_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let result =
            _rakaly_eu4_melt(data_ptr, data_len).unwrap_or_else(|e| MeltedBuffer::Error(e));
        Box::into_raw(Box::new(result))
    })
    .unwrap_or(std::ptr::null_mut())
}

fn _rakaly_eu4_melt(
    data_ptr: *const c_char,
    data_len: size_t,
) -> Result<MeltedBuffer, Box<dyn Error>> {
    use eu4save::{file::Eu4ParsedFileKind, Encoding};

    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    let file = eu4save::Eu4File::from_slice(data)?;

    if matches!(file.encoding(), Encoding::Text) {
        return Ok(MeltedBuffer::Verbatim);
    }

    let mut zip_sink = Vec::new();
    let parsed = file.parse(&mut zip_sink)?;
    match parsed.kind() {
        Eu4ParsedFileKind::Text(_) => Ok(MeltedBuffer::Text {
            header: b"EU4txt".to_vec(),
            body: zip_sink,
        }),
        Eu4ParsedFileKind::Binary(bin) => {
            let melted = bin
                .melter()
                .on_failed_resolve(eu4save::FailedResolveStrategy::Stringify)
                .verbatim(true)
                .melt(&eu4save::EnvTokens)?;

            Ok(MeltedBuffer::Binary {
                unknown_tokens: !melted.unknown_tokens().is_empty(),
                body: melted.into_data(),
            })
        }
    }
}

/// See `rakaly_eu4_melt` for more information
#[no_mangle]
pub extern "C" fn rakaly_ck3_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let result =
            _rakaly_ck3_melt(data_ptr, data_len).unwrap_or_else(|e| MeltedBuffer::Error(e));
        Box::into_raw(Box::new(result))
    })
    .unwrap_or(std::ptr::null_mut())
}

fn _rakaly_ck3_melt(
    data_ptr: *const c_char,
    data_len: size_t,
) -> Result<MeltedBuffer, Box<dyn Error>> {
    use ck3save::{file::Ck3ParsedFileKind, Encoding, SaveHeaderKind};

    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    let file = ck3save::Ck3File::from_slice(data)?;

    if matches!(file.encoding(), Encoding::Text) {
        return Ok(MeltedBuffer::Verbatim);
    }

    let mut zip_sink = Vec::new();
    let parsed = file.parse(&mut zip_sink)?;
    match parsed.kind() {
        Ck3ParsedFileKind::Text(_) => {
            let mut new_header = file.header().clone();
            new_header.set_kind(SaveHeaderKind::Text);
            let mut out_header = Vec::new();
            new_header.write(&mut out_header).unwrap();
            Ok(MeltedBuffer::Text {
                header: out_header,
                body: zip_sink,
            })
        }
        Ck3ParsedFileKind::Binary(bin) => {
            let melted = bin
                .melter()
                .on_failed_resolve(ck3save::FailedResolveStrategy::Stringify)
                .verbatim(true)
                .melt(&ck3save::EnvTokens)?;

            Ok(MeltedBuffer::Binary {
                unknown_tokens: !melted.unknown_tokens().is_empty(),
                body: melted.into_data(),
            })
        }
    }
}

/// See `rakaly_eu4_melt` for more information
#[no_mangle]
pub extern "C" fn rakaly_imperator_melt(
    data_ptr: *const c_char,
    data_len: size_t,
) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let result =
            _rakaly_imperator_melt(data_ptr, data_len).unwrap_or_else(|e| MeltedBuffer::Error(e));
        Box::into_raw(Box::new(result))
    })
    .unwrap_or(std::ptr::null_mut())
}

fn _rakaly_imperator_melt(
    data_ptr: *const c_char,
    data_len: size_t,
) -> Result<MeltedBuffer, Box<dyn Error>> {
    use imperator_save::{file::ImperatorParsedFileKind, Encoding, SaveHeaderKind};

    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    let file = imperator_save::ImperatorFile::from_slice(data)?;

    if matches!(file.encoding(), Encoding::Text) {
        return Ok(MeltedBuffer::Verbatim);
    }

    let mut zip_sink = Vec::new();
    let parsed = file.parse(&mut zip_sink)?;
    match parsed.kind() {
        ImperatorParsedFileKind::Text(_) => {
            let mut new_header = file.header().clone();
            new_header.set_kind(SaveHeaderKind::Text);
            let mut out_header = Vec::new();
            new_header.write(&mut out_header).unwrap();
            Ok(MeltedBuffer::Text {
                header: out_header,
                body: zip_sink,
            })
        }
        ImperatorParsedFileKind::Binary(bin) => {
            let melted = bin
                .melter()
                .on_failed_resolve(imperator_save::FailedResolveStrategy::Stringify)
                .verbatim(true)
                .melt(&imperator_save::EnvTokens)?;

            Ok(MeltedBuffer::Binary {
                unknown_tokens: !melted.unknown_tokens().is_empty(),
                body: melted.into_data(),
            })
        }
    }
}

/// See `rakaly_eu4_melt` for more information
#[no_mangle]
pub extern "C" fn rakaly_hoi4_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let result =
            _rakaly_hoi4_melt(data_ptr, data_len).unwrap_or_else(|e| MeltedBuffer::Error(e));
        Box::into_raw(Box::new(result))
    })
    .unwrap_or(std::ptr::null_mut())
}

fn _rakaly_hoi4_melt(
    data_ptr: *const c_char,
    data_len: size_t,
) -> Result<MeltedBuffer, Box<dyn Error>> {
    use hoi4save::{file::Hoi4ParsedFileKind, Encoding};

    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };

    let file = hoi4save::Hoi4File::from_slice(data)?;
    if matches!(file.encoding(), Encoding::Plaintext) {
        return Ok(MeltedBuffer::Verbatim);
    }

    let parsed = file.parse()?;
    match parsed.kind() {
        Hoi4ParsedFileKind::Text(_) => Ok(MeltedBuffer::Verbatim),
        Hoi4ParsedFileKind::Binary(bin) => {
            let melted = bin
                .melter()
                .on_failed_resolve(hoi4save::FailedResolveStrategy::Stringify)
                .verbatim(true)
                .melt(&hoi4save::EnvTokens)?;

            Ok(MeltedBuffer::Binary {
                unknown_tokens: !melted.unknown_tokens().is_empty(),
                body: melted.into_data(),
            })
        }
    }
}

/// See `rakaly_eu4_melt` for more information
#[no_mangle]
pub extern "C" fn rakaly_vic3_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer {
    std::panic::catch_unwind(|| {
        let result =
            _rakaly_vic3_melt(data_ptr, data_len).unwrap_or_else(|e| MeltedBuffer::Error(e));
        Box::into_raw(Box::new(result))
    })
    .unwrap_or(std::ptr::null_mut())
}

fn _rakaly_vic3_melt(
    data_ptr: *const c_char,
    data_len: size_t,
) -> Result<MeltedBuffer, Box<dyn Error>> {
    use vic3save::{file::Vic3ParsedFileKind, Encoding, SaveHeaderKind};

    let dp = data_ptr as *const c_uchar;
    let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
    let file = vic3save::Vic3File::from_slice(data)?;

    if matches!(file.encoding(), Encoding::Text) {
        return Ok(MeltedBuffer::Verbatim);
    }

    let mut zip_sink = Vec::new();
    let parsed = file.parse(&mut zip_sink)?;
    match parsed.kind() {
        Vic3ParsedFileKind::Text(_) => {
            let mut new_header = file.header().clone();
            new_header.set_kind(SaveHeaderKind::Text);
            let mut out_header = Vec::new();
            new_header.write(&mut out_header).unwrap();
            Ok(MeltedBuffer::Text {
                header: out_header,
                body: zip_sink,
            })
        }
        Vic3ParsedFileKind::Binary(bin) => {
            let melted = bin
                .melter()
                .on_failed_resolve(vic3save::FailedResolveStrategy::Stringify)
                .verbatim(true)
                .melt(&vic3save::EnvTokens)?;

            Ok(MeltedBuffer::Binary {
                unknown_tokens: !melted.unknown_tokens().is_empty(),
                body: melted.into_data(),
            })
        }
    }
}
