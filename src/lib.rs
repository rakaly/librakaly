mod errors;
mod file;
mod melter;
mod tokens;

use crate::errors::LibError;
use errors::PdsError;
use file::{PdsFile, PdsFileResult, PdsMeta};
use libc::{c_char, c_int, c_uchar, size_t};
use melter::{MeltedBuffer, MeltedBufferResult};
use std::hint::unreachable_unchecked;

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
    let buffer: &mut [u8] = std::slice::from_raw_parts_mut(buffer as *mut u8, length);

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
    }

    res.len()
}

/// Consume a result and return the underlying error. If the result does not
/// encompass an error, the result is not consumed.
///
/// # Safety
///
/// - Must pass in a valid pointer to a `PdsFileResult`
#[no_mangle]
pub unsafe extern "C" fn rakaly_file_error(ptr: *mut PdsFileResult<'static>) -> *mut PdsError {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    match &*ptr {
        PdsFileResult::Ok(_) => std::ptr::null_mut(),
        PdsFileResult::Err(e) => {
            let res = Box::from_raw(ptr);
            let error = Box::into_raw(Box::new(PdsError::from(e)));
            drop(res);
            error
        }
    }
}

/// Calculate the number of bytes in the for the melted output's error message.
/// The length excludes null termination
///
/// # Safety
///
/// Must pass in a valid pointer to a `PdsError`
#[no_mangle]
pub unsafe extern "C" fn rakaly_error_length(res: *const PdsError) -> c_int {
    if res.is_null() {
        0
    } else {
        (*res).msg().len() as c_int
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
/// `-1` is returned if there are any errors, for example when passed a
/// null pointer or a buffer of insufficient size.
///
/// The buffer will not be null terminated.
///
/// # Safety
///
/// - Must pass in a valid pointer to a `PdsError`
/// - Given buffer must be at least the given length in size
#[no_mangle]
pub unsafe extern "C" fn rakaly_error_write_data(
    res: *const PdsError,
    buffer: *mut c_char,
    length: c_int,
) -> c_int {
    if res.is_null() || buffer.is_null() {
        return -1;
    }

    let err = &*res;
    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if err.msg().len() > buffer.len() {
        return -1;
    }

    std::ptr::copy_nonoverlapping(err.msg().as_ptr(), buffer.as_mut_ptr(), err.msg().len());

    err.msg().len() as c_int
}

/// Destroys a `PdsError`
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_free_error(res: *mut PdsError) {
    if !res.is_null() {
        drop(Box::from_raw(res));
    }
}

/// Destroys a `PdsFile`
///
/// # Safety
///
/// Must pass in a valid pointer to a `PdsFile`
#[no_mangle]
pub unsafe extern "C" fn rakaly_free_file(res: *mut PdsFile) {
    if !res.is_null() {
        drop(Box::from_raw(res));
    }
}

/// Consume a result and return the underlying value. If the result does not
/// encompass a value, the result is not consumed.
///
/// # Safety
///
/// - Must pass in a valid pointer to a `PdsFileResult`
#[no_mangle]
pub unsafe extern "C" fn rakaly_file_value(
    ptr: *mut PdsFileResult<'static>,
) -> *mut PdsFile<'static> {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    match &*ptr {
        PdsFileResult::Ok(_) => {
            let res = Box::from_raw(ptr);
            match *res {
                PdsFileResult::Ok(file) => Box::into_raw(Box::new(file)),
                PdsFileResult::Err(_) => unreachable_unchecked(),
            }
        }
        PdsFileResult::Err(_) => std::ptr::null_mut(),
    }
}

/// Returns a pointer to data that can decode a save's metadata. If a save does
/// not have easily extractable metadata, then a null pointer is returned.
///
/// # Safety
///
/// - Must pass in a valid pointer to a `PdsFile`
#[no_mangle]
pub unsafe extern "C" fn rakaly_file_meta(ptr: *const PdsFile<'static>) -> *mut PdsMeta<'static> {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    (*ptr)
        .meta()
        .map(|x| Box::into_raw(Box::new(x)))
        .unwrap_or(std::ptr::null_mut())
}

/// Return the result of converting the metadata of a save to plaintext
///
/// # Safety
///
/// - Must pass in a valid pointer to a `PdsMeta`
#[no_mangle]
pub unsafe extern "C" fn rakaly_file_meta_melt(ptr: *const PdsMeta) -> *mut MeltedBufferResult {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    let res = std::panic::catch_unwind(|| {
        let result = match (*ptr).melt() {
            Ok(x) => MeltedBufferResult::Ok(x),
            Err(err) => MeltedBufferResult::Err(err),
        };
        Box::into_raw(Box::new(result))
    });

    match res {
        Ok(x) => x,
        Err(_) => Box::into_raw(Box::new(MeltedBufferResult::Err(LibError::Panic))),
    }
}

/// Return the result of converting the save to plaintext
///
/// # Safety
///
/// - Must pass in a valid pointer to a `PdsFile`
#[no_mangle]
pub unsafe extern "C" fn rakaly_file_melt(ptr: *const PdsFile) -> *mut MeltedBufferResult {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    let res = std::panic::catch_unwind(|| {
        let result = match (*ptr).melt_file() {
            Ok(x) => MeltedBufferResult::Ok(x),
            Err(err) => MeltedBufferResult::Err(err),
        };
        Box::into_raw(Box::new(result))
    });

    match res {
        Ok(x) => x,
        Err(_) => Box::into_raw(Box::new(MeltedBufferResult::Err(LibError::Panic))),
    }
}

/// Returns true if the melter needed to convert the binary input
///
/// # Safety
///
/// Must pass in a valid pointer to a `MeltedBuffer`
#[no_mangle]
pub unsafe extern "C" fn rakaly_file_is_binary(res: *const PdsFile) -> bool {
    if res.is_null() {
        return false;
    }

    (*res).is_binary()
}

/// Consume a result and return the underlying error. If the result does not
/// encompass an error, the result is not consumed.
///
/// # Safety
///
/// - Must pass in a valid pointer to a `MeltedBufferResult`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_error(ptr: *mut MeltedBufferResult) -> *mut PdsError {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    match &*ptr {
        MeltedBufferResult::Ok(_) => std::ptr::null_mut(),
        MeltedBufferResult::Err(e) => {
            let res = Box::from_raw(ptr);
            let error = Box::into_raw(Box::new(PdsError::from(e)));
            drop(res);
            error
        }
    }
}

/// Consume a result and return the underlying value. If the result does not
/// encompass a value, the result is not consumed.
///
/// # Safety
///
/// - Must pass in a valid pointer to a `MeltedBufferResult`
#[no_mangle]
pub unsafe extern "C" fn rakaly_melt_value(ptr: *mut MeltedBufferResult) -> *mut MeltedBuffer {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    match &*ptr {
        MeltedBufferResult::Ok(_) => {
            let res = Box::from_raw(ptr);
            match *res {
                MeltedBufferResult::Ok(buf) => Box::into_raw(Box::new(buf)),
                MeltedBufferResult::Err(_) => unreachable_unchecked(),
            }
        }
        MeltedBufferResult::Err(_) => std::ptr::null_mut(),
    }
}

/// Initializes an EU4 save from a pointer the save data bytes and a number of
/// those bytes.
///
/// # Safety
///
/// The data is assumed to exist for the duration while the result of this
/// function lives.
#[no_mangle]
pub unsafe extern "C" fn rakaly_eu4_file(
    data_ptr: *const c_char,
    data_len: size_t,
) -> *mut PdsFileResult<'static> {
    let res = std::panic::catch_unwind(|| {
        let dp = data_ptr as *const c_uchar;
        let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
        let result = match eu4save::Eu4File::from_slice(data) {
            Ok(x) => PdsFileResult::Ok(PdsFile::Eu4(x)),
            Err(err) => PdsFileResult::Err(err.into()),
        };
        Box::into_raw(Box::new(result))
    });

    match res {
        Ok(x) => x,
        Err(_) => Box::into_raw(Box::new(PdsFileResult::Err(LibError::Panic))),
    }
}

/// Initializes an CK3 save from a pointer the save data bytes and a number of
/// those bytes.
///
/// # Safety
///
/// The data is assumed to exist for the duration while the result of this
/// function lives.
#[no_mangle]
pub unsafe extern "C" fn rakaly_ck3_file(
    data_ptr: *const c_char,
    data_len: size_t,
) -> *mut PdsFileResult<'static> {
    let res = std::panic::catch_unwind(|| {
        let dp = data_ptr as *const c_uchar;
        let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
        let result = match ck3save::Ck3File::from_slice(data) {
            Ok(x) => PdsFileResult::Ok(PdsFile::Ck3(x)),
            Err(err) => PdsFileResult::Err(err.into()),
        };
        Box::into_raw(Box::new(result))
    });

    match res {
        Ok(x) => x,
        Err(_) => Box::into_raw(Box::new(PdsFileResult::Err(LibError::Panic))),
    }
}

/// Initializes an Imperator save from a pointer the save data bytes and a number of
/// those bytes.
///
/// # Safety
///
/// The data is assumed to exist for the duration while the result of this
/// function lives.
#[no_mangle]
pub unsafe extern "C" fn rakaly_imperator_file(
    data_ptr: *const c_char,
    data_len: size_t,
) -> *mut PdsFileResult<'static> {
    let res = std::panic::catch_unwind(|| {
        let dp = data_ptr as *const c_uchar;
        let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
        let result = match imperator_save::ImperatorFile::from_slice(data) {
            Ok(x) => PdsFileResult::Ok(PdsFile::Imperator(x)),
            Err(err) => PdsFileResult::Err(err.into()),
        };
        Box::into_raw(Box::new(result))
    });

    match res {
        Ok(x) => x,
        Err(_) => Box::into_raw(Box::new(PdsFileResult::Err(LibError::Panic))),
    }
}

/// Initializes an HOI4 save from a pointer the save data bytes and a number of
/// those bytes.
///
/// # Safety
///
/// The data is assumed to exist for the duration while the result of this
/// function lives.
#[no_mangle]
pub unsafe extern "C" fn rakaly_hoi4_file(
    data_ptr: *const c_char,
    data_len: size_t,
) -> *mut PdsFileResult<'static> {
    let res = std::panic::catch_unwind(|| {
        let dp = data_ptr as *const c_uchar;
        let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
        let result = match hoi4save::Hoi4File::from_slice(data) {
            Ok(x) => PdsFileResult::Ok(PdsFile::Hoi4(x)),
            Err(err) => PdsFileResult::Err(err.into()),
        };
        Box::into_raw(Box::new(result))
    });

    match res {
        Ok(x) => x,
        Err(_) => Box::into_raw(Box::new(PdsFileResult::Err(LibError::Panic))),
    }
}

/// Initializes a Vic3 save from a pointer the save data bytes and a number of
/// those bytes.
///
/// # Safety
///
/// The data is assumed to exist for the duration while the result of this
/// function lives.
#[no_mangle]
pub unsafe extern "C" fn rakaly_vic3_file(
    data_ptr: *const c_char,
    data_len: size_t,
) -> *mut PdsFileResult<'static> {
    let res = std::panic::catch_unwind(|| {
        let dp = data_ptr as *const c_uchar;
        let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
        let result = match vic3save::Vic3File::from_slice(data) {
            Ok(x) => PdsFileResult::Ok(PdsFile::Vic3(x)),
            Err(err) => PdsFileResult::Err(err.into()),
        };
        Box::into_raw(Box::new(result))
    });

    match res {
        Ok(x) => x,
        Err(_) => Box::into_raw(Box::new(PdsFileResult::Err(LibError::Panic))),
    }
}

/// Initializes an EU5 save from a pointer the save data bytes and a number of
/// those bytes.
///
/// # Safety
///
/// The data is assumed to exist for the duration while the result of this
/// function lives.
#[no_mangle]
pub unsafe extern "C" fn rakaly_eu5_file(
    data_ptr: *const c_char,
    data_len: size_t,
) -> *mut PdsFileResult<'static> {
    let res = std::panic::catch_unwind(|| {
        let dp = data_ptr as *const c_uchar;
        let data = unsafe { std::slice::from_raw_parts(dp, data_len) };
        let result = match eu5save::Eu5File::from_slice(data) {
            Ok(x) => PdsFileResult::Ok(PdsFile::Eu5(x)),
            Err(err) => PdsFileResult::Err(err.into()),
        };
        Box::into_raw(Box::new(result))
    });

    match res {
        Ok(x) => x,
        Err(_) => Box::into_raw(Box::new(PdsFileResult::Err(LibError::Panic))),
    }
}
