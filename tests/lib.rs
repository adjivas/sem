// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sem
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate sem;

#[allow(unused_unsafe)]
#[test]
fn lock_and_unlock () {
    let id: i32 = semget_id! (
        ftok!().expect("ftok! fail")
    ).expect("semget! fail");

    assert_eq!(semctl_init!(id), true);
    assert_eq!(semop_lock!(id), true);
    assert_eq!(semop_unlock!(id), true);
    assert_eq!(semctl_clear!(id), true);
}
