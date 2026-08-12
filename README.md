# CS336 Spring 2025 Assignment 1: Basics

For a full description of the assignment, see the assignment handout at
[cs336_assignment1_basics.pdf](./cs336_assignment1_basics.pdf)

If you see any issues with the assignment handout or code, please feel free to
raise a GitHub issue or open a pull request with a fix.

## Setup

### Environment
We manage our environments with `uv` to ensure reproducibility, portability, and ease of use.
Install `uv` [here](https://github.com/astral-sh/uv#installation) (recommended), or run `pip install uv`/`brew install uv`.
We recommend reading a bit about managing projects in `uv` [here](https://docs.astral.sh/uv/guides/projects/#managing-dependencies) (you will not regret it!).

You can now run any code in the repo using
```sh
uv run <python_file_path>
```
and the environment will be automatically solved and activated when necessary.

### Run unit tests


```sh
uv run pytest
```

Initially, all tests should fail with `NotImplementedError`s.
To connect your implementation to the tests, complete the
functions in [./tests/adapters.py](./tests/adapters.py).

### Download data
Download the TinyStories data and a subsample of OpenWebText

``` sh
mkdir -p data
cd data

wget https://huggingface.co/datasets/roneneldan/TinyStories/resolve/main/TinyStoriesV2-GPT4-train.txt
wget https://huggingface.co/datasets/roneneldan/TinyStories/resolve/main/TinyStoriesV2-GPT4-valid.txt

wget https://huggingface.co/datasets/stanford-cs336/owt-sample/resolve/main/owt_train.txt.gz
gunzip owt_train.txt.gz
wget https://huggingface.co/datasets/stanford-cs336/owt-sample/resolve/main/owt_valid.txt.gz
gunzip owt_valid.txt.gz

cd ..
```

## Rust BPE tokenizer (boltffi integration)

The tokenizer is implemented in Rust at `rust/bpe/` and exposed to Python as the `bpe` module (not `cs336_basics.bpe`). The wheel is built with the `boltffi` CLI and installed as a path dependency, so `import bpe` resolves from the virtualenv.

### Prerequisites
- `boltffi` CLI (currently pinned to 0.29.3): `cargo install boltffi`
- `cargo` (Rust toolchain, edition 2024)
- A Python interpreter **with pip** for the wheel step (the project's `.venv` has none). The repo uses the miniconda `python3` at `/home/abdullah/miniconda3/bin/python3`; it must match the venv's CPython version/ABI.

### Rebuilding after a Rust change
```sh
cd rust/bpe
boltffi pack python --release --python /home/abdullah/miniconda3/bin/python3
cd ../..
uv lock --refresh && uv sync
```
`uv lock --refresh` is required because uv pins the wheel's content hash; without it, `uv sync` fails with a hash mismatch.

### Gotchas
- `boltffi pack python` compiles the crate with an "IR expansion" env set, so the built `.so` exports the full-ABI symbols the generated bindings expect (e.g. `boltffi_function_bpe_add`). A plain `cargo build --release` emits only legacy symbols (`boltffi_add`) and produces a wheel that fails at import. If you ever see `ImportError: failed to resolve native symbol boltffi_function_bpe_add`, run `cargo clean -p bpe --release` once and re-pack.
- Cargo's incremental cache does not track the expansion env vars: the crate is only recompiled when `rust/bpe/src/lib.rs` actually changes.
- The generated `_native.cpython-*.so` is compiled against the `--python` interpreter, so keep it matched to the venv's Python (3.12).

### Files
- `rust/bpe/boltffi.toml` — target config; Python output goes to `rust/bpe/target/python/` (wheel in `target/python/wheelhouse/`).
- `rust/bpe/src/lib.rs` — tokenizer logic and `#[boltffi::export]` API.
- `pyproject.toml` — `bpe` path dependency pointing at the wheel.
- `tests/adapters.py` — wire `import bpe` here (`get_tokenizer`, `run_train_bpe`).

