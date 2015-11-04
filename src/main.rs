// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sem
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate sem;

#[allow(unused_assignments)]
fn main () {
    if let Some(key) = ftok!() {
        if let Some(id) = semget_id!(key, 1) {
            println!("work: {}", semctl!(id, sem::ffi::Ipc::SET));
        }
    }
}
