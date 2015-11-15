# Sem

[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)
[![Build Status](https://travis-ci.org/adjivas/sem.svg)](https://travis-ci.org/adjivas/sem)
[![Circle CI](https://circleci.com/gh/adjivas/sem/tree/master.svg?style=svg)](https://circleci.com/gh/adjivas/sem/tree/master)

#### How to build:
```shell
git clone https://github.com/adjivas/sem.git sem && cd sem
cargo build
```

#### How to use:
```shell
cargo run --example lock_and_unlock
```

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
|__ examples
|   \__ lock_and_unlock.rs
|__ tests
|   \__ lib.rs
\__ src
    |__ ffi.rs
    |__ lib.rs
    \__ macros.rs
```

#### License:
*sem*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/sem/blob/master/LICENSE).
