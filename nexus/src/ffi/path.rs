use std::{
    ffi::{CStr, CString, OsString, c_char},
    io,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
};
use windows::{
    Win32::{
        Globalization::{
            CP_ACP, CP_OEMCP, CP_UTF8, GetACP, MB_ERR_INVALID_CHARS, MultiByteToWideChar,
            WC_ERR_INVALID_CHARS, WC_NO_BEST_FIT_CHARS, WideCharToMultiByte,
        },
        Storage::FileSystem::AreFileApisANSI,
    },
    core::{BOOL, PCSTR},
};

/// Resolves the codepage to use for conversions.
pub fn get_codepage() -> u32 {
    if unsafe { GetACP() } == CP_UTF8 {
        CP_UTF8
    } else if !unsafe { AreFileApisANSI() }.as_bool() {
        CP_OEMCP
    } else {
        CP_ACP
    }
}

/// Attempts to convert an ANSI path string pointer to a [`PathBuf`].
#[inline]
pub unsafe fn path_from_ansi(ptr: *const c_char) -> io::Result<PathBuf> {
    let codepage = get_codepage();
    if codepage == CP_UTF8 {
        Ok(unsafe { path_from_utf8(ptr) })
    } else {
        unsafe { path_from_ansi_with(ptr, codepage) }
    }
}

/// Attempts to convert a UTF-8 path string pointer to a [`PathBuf`].
#[inline]
pub unsafe fn path_from_utf8(ptr: *const c_char) -> PathBuf {
    let string = unsafe { CStr::from_ptr(ptr) };
    unsafe { OsString::from_encoded_bytes_unchecked(string.to_bytes().to_vec()) }.into()
}

/// Attempts to convert an ANSI path string pointer to a [`PathBuf`] with the given codepage.
#[inline]
pub unsafe fn path_from_ansi_with(ptr: *const c_char, codepage: u32) -> io::Result<PathBuf> {
    if ptr.is_null() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "path was null"));
    }

    let narrow = unsafe { CStr::from_ptr(ptr) }.to_bytes();
    let flags = MB_ERR_INVALID_CHARS;
    let size = unsafe { MultiByteToWideChar(codepage, flags, narrow, None) };
    if size == 0 {
        return Err(io::Error::last_os_error());
    }

    let mut wide = vec![0; size as usize];
    let written = unsafe { MultiByteToWideChar(codepage, flags, narrow, Some(&mut wide)) };
    if written != size {
        return Err(io::Error::last_os_error());
    }

    Ok(OsString::from_wide(&wide).into())
}

/// Attempts to convert a string to an ANSI [`CString`].
#[inline]
pub fn path_to_ansi(path: impl AsRef<Path>) -> io::Result<CString> {
    let codepage = get_codepage();
    if codepage == CP_UTF8 {
        path_to_utf8(path)
    } else {
        path_to_ansi_with(path, codepage)
    }
}

/// Attempts to convert a string to a UTF-8 [`CString`].
#[inline]
pub fn path_to_utf8(path: impl AsRef<Path>) -> io::Result<CString> {
    let path = path.as_ref();
    if let Some(string) = path.to_str() {
        // already UTF-8, skip conversion
        let bytes = string.bytes().collect();
        Ok(unsafe { CString::from_vec_unchecked(bytes) })
    } else {
        path_to_ansi_with(path, CP_UTF8)
    }
}

/// Attempts to convert a string to an ANSI [`CString`] with the given codepage.
#[inline]
pub fn path_to_ansi_with(path: impl AsRef<Path>, codepage: u32) -> io::Result<CString> {
    let path = path.as_ref();
    let wide = path.as_os_str().encode_wide().collect::<Vec<_>>();

    const CP_GB18030: u32 = 54936;

    let mut used_default = BOOL::from(false);
    let (flags, used_default_char) = if matches!(codepage, CP_UTF8 | CP_GB18030) {
        (WC_ERR_INVALID_CHARS, None)
    } else {
        (WC_NO_BEST_FIT_CHARS, Some(&raw mut used_default))
    };

    let size = unsafe {
        WideCharToMultiByte(
            codepage,
            flags,
            &wide,
            None,
            PCSTR::null(),
            used_default_char,
        )
    };
    if size == 0 {
        return Err(io::Error::last_os_error());
    }

    let mut narrow = vec![0; size as usize];
    let written = unsafe {
        WideCharToMultiByte(
            codepage,
            flags,
            &wide,
            Some(&mut narrow),
            PCSTR::null(),
            used_default_char,
        )
    };

    if written != size {
        return Err(io::Error::last_os_error());
    }
    if used_default.as_bool() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "path is not representable in the current ANSI code page",
        ));
    }

    Ok(unsafe { CString::from_vec_unchecked(narrow) })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CP_OEM_LATIN1: u32 = 850; // OEM Multilingual Latin 1; Western European (DOS)
    const CP_OEM_GREEK: u32 = 869; // OEM Modern Greek; Greek, Modern (DOS)
    const CP_ANSI_LATIN1: u32 = 1252; // ANSI Latin 1; Western European (Windows)
    const CP_ANSI_GREEK: u32 = 1253; // ANSI Greek; Greek (Windows)

    #[test]
    fn roundtrip_delta() {
        let path = Path::new(".\\δ.txt");

        let codepage = CP_OEM_GREEK;
        let narrow = path_to_ansi_with(path, codepage).expect("invalid ansi path");
        let wide =
            unsafe { path_from_ansi_with(narrow.as_ptr(), codepage) }.expect("invalid wtf8 path");
        assert_eq!(wide, path);

        let codepage = CP_ANSI_GREEK;
        let narrow = path_to_ansi_with(path, codepage).expect("invalid ansi path");
        let wide =
            unsafe { path_from_ansi_with(narrow.as_ptr(), codepage) }.expect("invalid wtf8 path");
        assert_eq!(wide, path);
    }

    #[test]
    fn roundtrip_umlaut() {
        let path = Path::new(".\\ä.txt");

        let codepage = CP_OEM_LATIN1;
        let narrow = path_to_ansi_with(path, codepage).expect("invalid ansi path");
        let wide =
            unsafe { path_from_ansi_with(narrow.as_ptr(), codepage) }.expect("invalid wtf8 path");
        assert_eq!(wide, path);

        let codepage = CP_ANSI_LATIN1;
        let narrow = path_to_ansi_with(path, codepage).expect("invalid ansi path");
        let wide =
            unsafe { path_from_ansi_with(narrow.as_ptr(), codepage) }.expect("invalid wtf8 path");
        assert_eq!(wide, path);
    }

    #[test]
    fn roundtrip_utf8() {
        let path = Path::new(".\\♻️.txt");

        let codepage = CP_UTF8;
        let narrow = path_to_ansi_with(path, codepage).expect("invalid ansi path");
        let wide =
            unsafe { path_from_ansi_with(narrow.as_ptr(), codepage) }.expect("invalid wtf8 path");
        assert_eq!(wide, path);
    }

    #[test]
    fn error_utf8() {
        let path = Path::new(".\\♻️.txt");

        let codepage = CP_OEM_LATIN1;
        let result = path_to_ansi_with(path, codepage);
        assert!(result.is_err());
    }
}
