# simplet2s

A simple traditional Chinese to simplified Chinese converter

## Installation

Add it to your ``Cargo.toml``:

```toml
[dependencies]
simplet2s = "0.1"
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

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
