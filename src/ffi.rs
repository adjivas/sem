// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sem
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

/// The `Ipc` enum is a POSIX Standard
/// for System V.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Ipc {
  CREAT  = 0o0001000,
  NOWAIT = 2048,
  EXCL   = 0o0002000,
  RMID   = 0o0000000,
  SET    = 0o0000001,
  STAT   = 0o0000002,
  INFO   = 0o0000003,
}

/// The `TOK_*` const are default values
/// for macros.

pub const TOK_PATHNAME: &'static [u8; 4] = b"/tmp";
pub const TOK_PROJ_ID: u32 = 0;

/// The `C` extern is list of libc functions required
/// by the project.

#[cfg(any(unix))]
extern "C" {
    pub fn ftok(path: *mut i8, id: i32) -> i64;
    pub fn semget(key: u32, nsems: i32, semflg: i32) -> i32;
    pub fn semop(semid: i32, sops: *mut SemBuf, nsops: u64) -> i32;
    pub fn semctl(semid: i32, semnum: i32, cmd: i32, arg: *mut SemUn) -> i32;
}

#[repr(C)]
pub struct SemBuf {
    pub semNum: u16, /* semaphore. # */
    pub semOp: i16, /* semaphore operation. */
    pub semFlg: i16, /* operation flags. */
}

#[repr(C)]
pub struct SemUn {
    pub val: i32, /* value for SETVAL. */
    pub buf: SemIdDs, /* buffer for IPC_STAT & IPC_SET. */
    pub array: i16, /* array for GETALL & SETALL. */
}

#[repr(C)]
pub struct SemIdDs {
    pub semPerm: IpcPerm, /* Ownership and permissions. */
    pub semOtime: i64, /* Last semop time. */
    pub semCtime: i64, /* Last change time. */
    pub semNsems: u16, /* No. of semaphores in set. */
}

#[repr(C)]
pub struct IpcPerm {
    pub key: u32,
    pub uid: u16, /* owner euid and egid */
    pub gid: u16,
    pub cuid: u16, /* creator euid and egid */
    pub cgid: u16,
    pub mode: u16, /* access modes see mode flags below */
    pub seq: u16, /* slot usage sequence number */
}
