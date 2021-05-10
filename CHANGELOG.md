# Changes for [`fibonacci_codec`](https://crates.io/crates/fibonacci_codec)

<!-- next-header -->

## [Unreleased] - ReleaseDate

## Changed

* The encoding function now does less unnecessary work, speeding it up
  by about 10% in local benchmarks.
* The benchmarks now measure performance for a more even (and larger)
  distribution of integers.


## [[0.2.0](https://docs.rs/fibonacci_codec/0.2.0/fibonacci_codec/)] - 2021-05-01

## Changed

* Update rust edition to `2018`
* Use a `build.rs` script to generate the fibonacci table.
* Use github workflows to run CI, use dependabot, and use `cargo deny`
  to ensure reasonable dependencies are in place.

## v0.1.1 - 2018-04-06

(This release predates the existence of the Changelog file)

### Fixed
* Clippy no longer complains
* Some grammar fixes to docs

## v0.1.0 - 2018-04-04

* Initial release
