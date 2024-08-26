# Radix

Radix is a simple CLI base conversion program that supports converting from base 2 to base 36.

## Example Usage

```sh
radix --from decimal --to hex 17 --simple
# Output: 11
```

Options:

* `--from`: The source base. Either an integer in range [2, 36] or a valid name.
* `--to`: The target base. Either an integer in range [2, 36] or a valid name.
* `--decimal-separator/-d`: The decimal separator in the input, either comma or point.
* `--simple/-s`: Only shows the result instead of printing it in LaTex format.

## Running and Installing

Radix is built on the [Rust Programming Language](https://www.rust-lang.org/)

For running, install `cargo` and run:

```sh
cargo run -- [OPTIONS]
```

For installing, run:

```sh
cargo install
```

After installing, you can call radix directly:

```sh
radix [OPTIONS]
```