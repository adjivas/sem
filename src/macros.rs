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
            id => /*match semctl_init!(id, $nsems) {
                false => None,
                true =>*/ Some(id)/*,
            }*/,
        }
    });
}

/// The `semget_id` macro obtains a semaphore id
/// and determinates the required flag.

#[macro_export]
macro_rules! semget_id {
    ($key: expr) => ({
        semget_id!($key, sem::ffi::NSEMS)
    });
    ($key: expr, $nsems: expr) => ({
        semget! (
            $key,
            $nsems,
            0o0666 | sem::ffi::Ipc::CREAT as i32
                   | sem::ffi::Ipc::EXCL as i32
        )
    });
}

/// The `semop` macro sets the semaphore
/// index according to a SemBuffer.

#[macro_export]
macro_rules! semop {
    ($id: expr, $lock: expr) => ({
        semop!($id, sem::ffi::SEM_NUM, $lock)
    });
    ($id: expr, $index: expr, $lock: expr) => ({
        semop!($id, $index, $lock, sem::ffi::NSOPS)
    });
    ($id: expr, $index: expr, $lock: expr, $nbop: expr) => ({
        semop!($id, $index, $lock, $nbop, 0)
    });
    ($id: expr, $index: expr, $lock: expr, $nbop: expr, $flag: expr) => ({
        extern crate sem;
        let mut op = sem::ffi::SemBuf {
            sem_num: $index as u16,
            sem_op: if $lock {
                -1
            } else {
                1
            },
            sem_flg: $flag,
        };

        match unsafe {
            sem::ffi::semop (
                $id,
                &mut op,
                $nbop
            )
        } {
            -1 => false,
            _ => true,
        }
    });
}

/// The `semop_lock` macro locks the semaphore.

#[macro_export]
macro_rules! semop_lock {
    ($id: expr) => ({
        semop!($id, true)
    });
    ($id: expr, $index) => ({
        semop!($id, $index, true)
    });
    ($id: expr, $index: expr, $nbop: expr) => ({
        semop!($id, $index, true, $nbop)
    });
}

/// The `semop_lock` macro unlocks the semaphore.

#[macro_export]
macro_rules! semop_unlock {
    ($id: expr) => ({
        semop!($id, false)
    });
    ($id: expr, $index) => ({
        semop!($id, $index, false)
    });
    ($id: expr, $index: expr, $nbop: expr) => ({
        semop!($id, $index, false, $nbop)
    });
}

/// The `semctl` macro...

#[macro_export]
macro_rules! semctl {
    ($id: expr, $cmd: expr) => ({
        semctl!($id, sem::ffi::SEM_NUM, $cmd)
    });
    ($id: expr, $semnum: expr, $cmd: expr) => ({
        /*let mut op = sem::ffi::SemUn {
            sem_num: $index as u16,
            sem_op: if $lock {
                -1
            } else {
                1
            },
            sem_flg: $flag,
        };*/
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

/// The `semctl_init` macro sets/inits the semaphore at 0.

#[macro_export]
macro_rules! semctl_init {
    ($id: expr) => ({
        semctl!($id, 0, sem::ffi::Ipc::SET)
    });
    ($id: expr, $semnum: expr) => ({
        semctl!($id, $semnum, sem::ffi::Ipc::SET)
    });
}

/// The `semctl_clear` macro removes the goup of semaphores.

#[macro_export]
macro_rules! semctl_clear {
    ($id: expr) => ({
        semctl!($id, sem::ffi::Ipc::RMID, 0)
    });
}
