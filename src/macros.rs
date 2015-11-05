// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sem
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `ftok` macro returns the System-V'IPC
/// key from pathname.

#[macro_export]
macro_rules! ftok {
    () => ({
        extern crate sem;
        match unsafe {
          sem::ffi::ftok (
            sem::ffi::TOK_PATHNAME.as_ptr() as *mut i8,
            sem::ffi::TOK_PROJ_ID as i32,
          )
        } {
            -1 => None,
            key => Some(key as u64),
        }
    });
    ($pathname: expr) => ({
        extern crate sem;
        match unsafe {
            sem::ffi::ftok (
                $pathname.as_ptr() as *mut i8,
                sem::ffi::TOK_PROJ_ID as i32
            )
        } {
            -1 => None,
            key => Some(key as u64),
        }
    });
}

/// The `semget` macro obtains a semaphore id.

#[macro_export]
macro_rules! semget {
    ($key: expr, $nsems: expr, $semflg: expr) => ({
        extern crate sem;
        match unsafe {
          sem::ffi::semget (
            $key as u32,
            $nsems as i32,
            $semflg as i32,
          )
        } {
            -1 => None,
            id => Some(id as i32),
        }
    });
}

/// The `semget` macro obtains a semaphore id
/// and determinates the required flag.

#[macro_export]
macro_rules! semget_id {
    ($key: expr, $nsems: expr) => ({
        match semget! (
            $key,
            0,
            0
        ) {
            Some(id) => Some(id),
            None => semget! (
                $key,
                $nsems,
                0o0666 | sem::ffi::Ipc::CREAT as i32
                       | sem::ffi::Ipc::EXCL as i32
            ),
        }
    });
}

#[macro_export]
macro_rules! semctl {
    ($id: expr) => ({
        semctl!($id, sem::ffi::Ipc::SET)
    });
    ($id: expr, $cmd: expr) => ({
        semctl!($id, 0, $cmd)
    });

    ($id: expr, $semnum: expr, $cmd: expr) => ({
        extern crate std;
        semctl!($id, $semnum, $cmd, std::ptr::null_mut())
    });
    ($id: expr, $semnum: expr, $cmd: expr, $arg: expr) => ({
        extern crate sem;
        match unsafe {
            sem::ffi::semctl (
                $id as i32,
                $semnum as i32,
                $cmd as i32,
                $arg
            )
        } {
            -1 => false,
            _ => true,
        }
    });
}
