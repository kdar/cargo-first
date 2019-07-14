# cargo-first

A cargo subcommand to stop execution after the first error. This is a stop-gap until there is proper support in rustc: https://github.com/rust-lang/rust/issues/27189.

This is a very simple tool and may not catch all edge cases. Please submit an issue if you find a problem.

## Usage

cargo first <cargo command> [command args...]

## Example

```
$ cargo first build

Compiling adbridge v0.1.0 (/home/kdar/dev/adbridge)
error[E0432]: unresolved import `types::Device`
  --> src/lib.rs:12:13
   |
12 | use types::{Device, DeviceDescriptor, DeviceState, Status};
   |             ^^^^^^ no `Device` in `types`

```

## License

Distributed under the MIT license. See `LICENSE` for more information.

## Contributing

1.  Fork it (<https://github.com/kdar/cargo-first/fork>)
2.  Create your feature branch (`git checkout -b feature/fooBar`)
3.  Commit your changes (`git commit -am 'Add some fooBar'`)
4.  Push to the branch (`git push origin feature/fooBar`)
5.  Create a new Pull Request