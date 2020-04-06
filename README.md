# simplet2s

[![Rust](https://github.com/bosondata/simplet2s-rs/workflows/Rust/badge.svg)](https://github.com/bosondata/simplet2s-rs/actions?query=workflow%3ARust)
[![Python](https://github.com/bosondata/simplet2s-rs/workflows/Python/badge.svg)](https://github.com/bosondata/simplet2s-rs/actions?query=workflow%3APython)
[![codecov](https://codecov.io/gh/bosondata/simplet2s-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/bosondata/simplet2s-rs)
[![Crates.io](https://img.shields.io/crates/v/simplet2s.svg)](https://crates.io/crates/simplet2s)
[![docs.rs](https://docs.rs/simplet2s/badge.svg)](https://docs.rs/simplet2s/)
[![PyPI](https://img.shields.io/pypi/v/simplet2s.svg)](https://pypi.python.org/pypi/simplet2s)

A simple traditional Chinese to simplified Chinese converter

## Installation

Add it to your ``Cargo.toml``:

```toml
[dependencies]
simplet2s = "0.2"
```

Add ``extern crate simplet2s`` to your crate root and your're good to go!

## Example

```rust
extern crate simplet2s;

fn main() {
    let simplified = simplet2s::convert("憂鬱的台灣烏龜");
    println!("{}", simplified);
}
```

## Python package

There is also a Python package named `simplet2s`, you can install it via `pip`:

```bash
pip install -U simplet2s
```

Usage example:

```python
import simplet2s

if __name__ == '__main__':
    print(simplet2s.convert("憂鬱的台灣烏龜"))
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
