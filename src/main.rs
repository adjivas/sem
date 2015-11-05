// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sem
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate sem;

#[allow(unused_unsafe)]
fn main () {
    let id: i32 = semget_id! (
        ftok!().expect("ftok! fail")
    ).expect("semget! fail");

    //semctl!(id, sem::ffi::SEM_NUM, sem::ffi::Ipc::SET);
    //semop_lock!(id);
    semop_unlock!(id);
    println!("work: {}", semctl_clear!(id));
}
