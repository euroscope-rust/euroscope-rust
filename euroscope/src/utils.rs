//! Small shared helpers for converting between C strings and Rust strings at
//! the FFI boundary.

// `unreachable_pub` (stable, enabled) wants `pub(crate)` on these free functions
// in this private module, but `redundant_pub_crate` (nursery) then flags that as
// redundant. The two lints are in direct conflict here; defer to the stable one.
#![allow(clippy::redundant_pub_crate)]

use std::{
    ffi::{CStr, CString, OsString, c_char},
    os::windows::ffi::OsStringExt as _,
    path::PathBuf,
    ptr,
};

use winapi::{
    shared::{
        minwindef::{DWORD, HMODULE, MAX_PATH},
        winerror::ERROR_INSUFFICIENT_BUFFER,
    },
    um::{
        errhandlingapi::GetLastError,
        libloaderapi::{
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            GetModuleFileNameW, GetModuleHandleExW,
        },
    },
};

/// Borrow a C string as `&str`. ATC data is ASCII, so we fall back to `""` on
/// the rare non-UTF-8 byte or null. The result borrows `'a`.
///
/// # Safety
/// `ptr` must be null or a valid NUL-terminated string that outlives `'a`.
pub(crate) unsafe fn cstr<'a>(ptr: *const c_char) -> &'a str {
    if ptr.is_null() {
        return "";
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().unwrap_or("")
}

/// Build a `CString` from `&str`, dropping any interior NUL bytes rather than
/// failing.
///
/// `CString::new` errors on an interior NUL (it would truncate the C string).
/// EuroScope's data is NUL-free in practice, so instead of surfacing a `Result`
/// through every setter we strip the offending bytes and retry — the second
/// construction cannot fail because the input is now guaranteed NUL-free.
pub(crate) fn cstr_lossy(s: &str) -> CString {
    CString::new(s)
        .unwrap_or_else(|_| CString::new(s.replace('\0', "")).expect("no interior NUL after strip"))
}

/// Returns the handle of the module (DLL) this code is compiled into, rather
/// than the host process. Without this, [`GetModuleFileNameW`] with a null
/// handle would return EuroScope's executable path instead of the plugin's.
fn current_module() -> Option<HMODULE> {
    // Any address that lives inside this module; its own data section will do.
    // Typed `u16` so its address is already an `LPCWSTR` with no pointer cast.
    static ANCHOR: u16 = 0;
    let mut handle: HMODULE = ptr::null_mut();
    // SAFETY: `handle` is a valid out-pointer, `ANCHOR` lies within this module,
    // and `UNCHANGED_REFCOUNT` means we don't take a reference to release.
    let ok = unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            &raw const ANCHOR,
            &raw mut handle,
        )
    };
    if ok == 0 { None } else { Some(handle) }
}

/// Return a path where the plugin DLL has been loaded from.
///
/// [`None`] means we weren't able to figure that out.
pub fn get_plugin_path() -> Option<PathBuf> {
    fn get_plugin_path(module: HMODULE, len: usize) -> Option<PathBuf> {
        let mut buf = Vec::with_capacity(len);
        #[expect(clippy::as_conversions, reason = "We should never get that far")]
        #[expect(
            clippy::cast_possible_truncation,
            reason = "We should never get that far"
        )]
        // SAFETY: `buf` has capacity for `len`.
        let ret = unsafe { GetModuleFileNameW(module, buf.as_mut_ptr(), len as DWORD) } as usize;
        if ret == 0 {
            None
        } else if ret < len {
            // Success, we need to trim trailing null bytes from the vec.
            // SAFETY: the call initialized `ret` code units and `ret < len`, so
            // the new length stays within the initialized region.
            unsafe {
                buf.set_len(ret);
            }
            let s = OsString::from_wide(&buf);
            Some(s.into())
        } else {
            // The buffer might not be big enough so we need to check errno.
            // SAFETY: [`GetLastError`] has no preconditions.
            let errno = unsafe { GetLastError() };
            if errno == ERROR_INSUFFICIENT_BUFFER {
                get_plugin_path(module, len * 2)
            } else {
                None
            }
        }
    }

    let module = current_module()?;
    get_plugin_path(module, MAX_PATH)
}
