# Sem

[![docs-badge][]][docs] [![license-badge][]][license] [![travis-badge][]][travis] [![circle-badge][]][circle]

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
*sem*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license][license].

[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://adjivas.github.io/sem/sem
[license-badge]: http://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square
[license]: https://github.com/adjivas/sem/blob/master/LICENSE
[travis-badge]: https://travis-ci.org/adjivas/sem.svg?style=flat-square
[travis]: https://travis-ci.org/adjivas/sem
[circle-badge]: https://circleci.com/gh/adjivas/sem/tree/master.svg?style=svg
[circle]: https://circleci.com/gh/adjivas/sem/tree/master
