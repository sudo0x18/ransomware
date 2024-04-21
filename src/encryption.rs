#[cfg(windows)]
extern crate winapi;

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileA, ReadFile, WriteFile, OPEN_ALWAYS, OPEN_EXISTING};
use winapi::um::handleapi::CloseHandle;
use winapi::um::wincrypt::{
    CryptAcquireContextA, CryptDecrypt, CryptDestroyKey, CryptEncrypt, CryptExportKey, CryptGenKey,
    CryptImportKey, CryptReleaseContext, CALG_AES_192, CRYPT_EXPORTABLE, CRYPT_VERIFYCONTEXT,
    HCRYPTKEY, HCRYPTPROV, PLAINTEXTKEYBLOB, PROV_RSA_AES,
};
use winapi::um::winnt::{
    DELETE, FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, FILE_WRITE_DATA, HANDLE,
};

static AES_KEY_BLOB: [u8; 36] = [
    8, 2, 0, 0, 15, 102, 0, 0, 24, 0, 0, 0, 8, 68, 217, 142, 222, 209, 85, 216, 44, 88, 2, 170,
    248, 210, 84, 119, 53, 196, 64, 96, 252, 205, 231, 229,
];

pub fn encrypt(source_file_path: CString, dest_file_path: CString) -> bool {
    let mut key_handle: HCRYPTKEY = 0usize;
    let mut crypto_provider_handle: HCRYPTPROV = 0usize;

    unsafe {
        if CryptAcquireContextA(
            &mut crypto_provider_handle,
            null_mut(),
            null_mut(),
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            println!(
                "Error during CryptAcquireContext! Error code: {}",
                GetLastError()
            );
            return false;
        } else {
            println!("A cryptographic provider has been acquired.");
        }

        if CryptImportKey(
            crypto_provider_handle,
            AES_KEY_BLOB.as_ptr(),
            AES_KEY_BLOB.len() as u32,
            0,
            0,
            &mut key_handle,
        ) == 0
        {
            println!("Failed to import key. Error code: {:?}", GetLastError());
            return false;
        } else {
            println!("Import successful. Key handle: 0x{:x}", key_handle);
        }

        let block_size: u32 = 960;
        let buffer_size: u32 = 960;

        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(buffer_size as usize, 0u8);
        println!("Memory allocated for the buffer.");

        let source_handle: HANDLE = CreateFileA(
            source_file_path.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let dest_handle: HANDLE = CreateFileA(
            dest_file_path.as_ptr(),
            FILE_WRITE_DATA | DELETE,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let mut end_of_file = 0;
        let mut bytes_read = 0;

        while end_of_file == 0 {
            if ReadFile(
                source_handle,
                buffer.as_mut_ptr() as *mut _,
                block_size,
                &mut bytes_read,
                null_mut(),
            ) == 0
            {
                println!("Error reading from source file.");
                break;
            }
            if bytes_read < block_size {
                end_of_file = 1;
            }

            if CryptEncrypt(
                key_handle,
                0,
                end_of_file,
                0,
                buffer.as_mut_ptr(),
                &mut bytes_read,
                buffer_size,
            ) == 0
            {
                println!("Failed to encrypt. Error code: {:?}", GetLastError());
                break;
            }

            if WriteFile(
                dest_handle,
                buffer.as_ptr() as *const _,
                bytes_read,
                &mut bytes_read,
                null_mut(),
            ) == 0
            {
                println!("Failed to write to destination file.");
                break;
            }
        }
        CloseHandle(source_handle);
        CloseHandle(dest_handle);
        CryptDestroyKey(key_handle);
        CryptReleaseContext(crypto_provider_handle, 0);
    }
    return true
}

pub fn decrypt(source_file_path: CString, dest_file_path: CString) -> bool {
    let mut key_handle: HCRYPTKEY = 0usize;
    let mut crypto_provider_handle: HCRYPTPROV = 0usize;

    unsafe {
        if CryptAcquireContextA(
            &mut crypto_provider_handle,
            null_mut(),
            null_mut(),
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            println!("Error during CryptAcquireContext!");
            println!("Error code: {}", GetLastError());
            return false;
        } else {
            println!("A cryptographic provider has been acquired.");
        }

        if CryptImportKey(
            crypto_provider_handle,
            AES_KEY_BLOB.as_ptr(),
            AES_KEY_BLOB.len() as u32,
            0,
            0,
            &mut key_handle,
        ) == 0
        {
            println!("Import failed. Error code: {:?}", GetLastError());
            return false;
        } else {
            println!("Import successful. Key handle: {}", key_handle);
        }

        let source_handle: HANDLE = CreateFileA(
            source_file_path.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let dest_handle: HANDLE = CreateFileA(
            dest_file_path.as_ptr(),
            FILE_WRITE_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let block_size: u32 = 960;
        let buffer_size: u32 = 960;

        let mut end_of_file = 0;
        let mut bytes_read = 0;

        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(buffer_size as usize, 0u8);

        while end_of_file == 0 {
            if ReadFile(
                source_handle,
                buffer.as_mut_ptr() as *mut _,
                block_size,
                &mut bytes_read,
                null_mut(),
            ) == 0
            {
                println!("Error reading. Error code: 0x{:x}", GetLastError());
                break;
            }
            println!("Bytes read: {}", bytes_read);
            if bytes_read < block_size {
                end_of_file = 1;
            }

            if CryptDecrypt(
                key_handle,
                0,
                end_of_file,
                0,
                buffer.as_mut_ptr(),
                &mut bytes_read,
            ) == 0
            {
                println!("Failed to decrypt. Error code: 0x{:x}", GetLastError());
                break;
            }

            if WriteFile(
                dest_handle,
                buffer.as_ptr() as *const _,
                bytes_read,
                &mut bytes_read,
                null_mut(),
            ) == 0
            {
                println!("Failed to write to destination file.");
                break;
            }
        }
        CryptDestroyKey(key_handle);
        CryptReleaseContext(crypto_provider_handle, 0);
        CloseHandle(source_handle);
        CloseHandle(dest_handle);
    }
    return true
}

pub fn generate_aes_key_blob() -> [u8; 36] {
    [
        8, 2, 0, 0, 15, 102, 0, 0, 24, 0, 0, 0, 8, 68, 217, 142, 222, 209, 85, 216, 44, 88, 2, 170,
        248, 210, 84, 119, 53, 196, 64, 96, 252, 205, 231, 229,
    ]
}

pub fn get_aes_key_handle(crypto_provider_handle: HCRYPTPROV, aes_key_blob: &[u8]) -> Option<HCRYPTKEY> {
    let mut key_handle: HCRYPTKEY = 0usize;
    unsafe {
        if CryptImportKey(
            crypto_provider_handle,
            aes_key_blob.as_ptr(),
            aes_key_blob.len() as u32,
            0,
            0,
            &mut key_handle,
        ) == 0
        {
            println!("Failed to import key. Error code: {:?}", GetLastError());
            None
        } else {
            println!("Import successful. Key handle: {}", key_handle);
            Some(key_handle)
        }
    }
}

pub fn release_handles_and_close_files(
    key_handle: HCRYPTKEY,
    crypto_provider_handle: HCRYPTPROV,
    source_handle: HANDLE,
    dest_handle: HANDLE,
) {
    unsafe {
        CryptDestroyKey(key_handle);
        CryptReleaseContext(crypto_provider_handle, 0);
        CloseHandle(source_handle);
        CloseHandle(dest_handle);
    }
}

pub fn decrypt_and_write_file(
    key_handle: HCRYPTKEY,
    source_handle: HANDLE,
    dest_handle: HANDLE,
    block_size: u32,
    buffer_size: u32,
) -> bool {
    let mut end_of_file = 0;
    let mut bytes_read = 0;
    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];

    unsafe{
        while end_of_file == 0 {
            if ReadFile(
                source_handle,
                buffer.as_mut_ptr() as *mut _,
                block_size,
                &mut bytes_read,
                null_mut(),
            ) == 0
            {
                println!("Error reading from source file. Error code: 0x{:x}", GetLastError());
                return false;
            }

            println!("Bytes read: {}", bytes_read);

            if bytes_read < block_size {
                end_of_file = 1;
            }

            if CryptDecrypt(
                key_handle,
                0,
                end_of_file,
                0,
                buffer.as_mut_ptr(),
                &mut bytes_read,
            ) == 0
            {
                println!("Failed to decrypt. Error code: 0x{:x}", GetLastError());
                return false;
            }

            if WriteFile(
                dest_handle,
                buffer.as_ptr() as *const _,
                bytes_read,
                &mut bytes_read,
                null_mut(),
            ) == 0
            {
                println!("Failed to write to destination file.");
                return false;
            }
        }
    }

    return true
}
