![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust 1.90+](https://img.shields.io/badge/rustc-1.90+-blue.svg)
[![Tests](https://github.com/contextgeneric/cgp-examples/actions/workflows/tests.yml/badge.svg)](https://github.com/contextgeneric/cgp-examples/actions/workflows/tests.yml)

# cgp-examples: example crates for CGP

**Five independent example crates, each a small program that shows one area of
[Context-Generic Programming (CGP)](https://contextgeneric.dev/) in use.**

The crates share a workspace and nothing else. Each one is a demonstration rather than a library, and
each is documented on the website, where the code is explained rather than only listed.

**[Website](https://contextgeneric.dev/) · [CGP quickstart](https://contextgeneric.dev/docs/quickstart) · [Project pages](https://contextgeneric.dev/docs/projects/cgp-examples/) · [Reference](https://contextgeneric.dev/docs/reference/)**

## The crates

| Crate | What it shows | Run it |
| --- | --- | --- |
| [`transfer`](transfer) | A money-transfer HTTP service: an environmental context with one handler dispatched per endpoint, and a namespace-organized wiring table | `cargo run -p cgp-example-transfer --bin server` |
| [`expression`](expression) | An arithmetic interpreter open to new variants and operations: a parameter-targeted component with per-type dispatch | `cargo test -p cgp-example-expression` |
| [`builder`](builder) | An application context assembled from per-subsystem builders | `cargo build -p cgp-example-builder` |
| [`web-app`](web-app) | One social-media backend wired four ways, from one trait per domain up to namespace defaults | `cargo build -p cgp-example-web-app` |
| [`greet`](greet) | A greeting written three ways: as a function, as a component, and over an abstract type | `cargo run -p cgp-example-greet --bin greet-function` |

`transfer` is the only crate with a README of its own: it is a
[walkthrough](transfer/README.md) that teaches the CGP constructs the service uses, and
[`example.sh`](transfer/example.sh) shows a session against a running server.

Nothing in `web-app` or `builder` runs. Their value is the wiring they compile, so a successful build
is most of their verification, and `check_components!` blocks check the rest.

Two crates have a page of their own on the website, which explains the code rather than listing it:
[`expression`](https://contextgeneric.dev/docs/projects/cgp-examples/expression) and
[`web-app`](https://contextgeneric.dev/docs/projects/cgp-examples/web-app).

## Building

The workspace builds against unreleased `cgp` code: its `[patch.crates-io]` section overrides `cgp`
and `cgp-error-anyhow` with the `cgp` repository's `main` branch by git URL, and the lockfile pins the
commit. To test against a local change to `cgp`, switch the same entries to the commented-out paths
into `../cgp`.

It needs stable Rust 1.90 or later and uses the 2024 edition. The only tests in the workspace are
three unit tests in `expression`; the other crates check their wiring at compile time with
`check_components!` blocks.

```sh
cargo build --workspace
cargo test --workspace
```

## Learn CGP

The website is the main documentation: the [quickstart](https://contextgeneric.dev/docs/quickstart),
the [tutorials](https://contextgeneric.dev/docs/tutorials/hello), the
[concepts](https://contextgeneric.dev/docs/concepts/), and the
[construct reference](https://contextgeneric.dev/docs/reference/). The
[project pages](https://contextgeneric.dev/docs/projects/cgp-examples/) explain these crates, and the
[CGP knowledge base](https://github.com/contextgeneric/cgp-knowledge-base) records each crate in depth
for contributors and agents.

[sibling-projects.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/sibling-projects.md)
lists the related repositories.

## License

Licensed under the MIT license.
