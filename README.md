# write-only

[![Checks](https://img.shields.io/github/checks-status/regexident/write-only/main?style=flat-square)](https://github.com/regexident/write-only/)
[![Downloads](https://img.shields.io/crates/d/write-only.svg?style=flat-square)](https://crates.io/crates/write-only/)
[![Version](https://img.shields.io/crates/v/write-only.svg?style=flat-square)](https://crates.io/crates/write-only/)
[![License](https://img.shields.io/crates/l/write-only.svg?style=flat-square)](https://crates.io/crates/write-only/)

## Synopsis

Rust references/slices that provide write-access, but no read-access.

## Motivation

Sometimes is is desirable to only provide write-access to a value or slice of values without also providing read-access.

This is where `write-only` comes in handy!

## Usage

Write-only reference:

```rust
use write_only::{prelude::*, Put};

fn write<T: Put<u8>>(write_only: &mut T) {
    write_only.put(42u8);
}

let mut value: u8 = 0;

let mut write_only = WriteOnlyRef::from(&mut value);
write(&mut write_only);

assert_eq!(value, 42);
```

Write-only slice:

```rust
use write_only::{prelude::*, PutAt};

fn write<T: PutAt<u8>>(write_only: &mut T) {
    write_only.put_at(2, 42u8);
}

let mut values: Vec<u8> = (0..10).collect();

let mut write_only = WriteOnlySlice::from(&mut values[..]);
write(&mut write_only);

assert_eq!(values[2], 42u8);
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),  
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/write-only/tags).

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
