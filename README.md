# rbase64

This is just a pet project implementing a base64 encoder/decoder.

Build it with `cargo build`.

The program encodes/decodes stdin and prints the result to stdout.

Usage examples:
```bash
printf 'Man' |rbase64 # yields TWFu
printf 'TWFu' |rbase64 -D # yields Man
```
