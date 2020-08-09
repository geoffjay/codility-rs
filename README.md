![Rust](https://github.com/geoffjay/codility-rs/workflows/Rust/badge.svg)
![Security audit](https://github.com/geoffjay/codility-rs/workflows/Security%20Audit/badge.svg)
![clippy](https://github.com/geoffjay/codility-rs/workflows/Clippy/badge.svg)
[![codecov](https://codecov.io/gh/geoffjay/codility-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/geoffjay/codility-rs)

---

# Codility Lessons in Rust

Requires `nightly` toolchain for `![features(test)]`.

```shell
cargo build
```

## Testing

```shell
cargo test
```

## Benchmarks

The benchmark tests have been ignored because the flag `-Zpanic_abort_tests` that's needed for `grcov` causes them to
fail. To run these add `--ignored` to the test.

```shell
cargo test -- --ignored
```