# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Documentation sync rule

**`README.md` and in-code `cargo doc` comments must stay in sync.** Any change to a public API (new method, renamed parameter, changed signature, new variant) requires updating both:
- The relevant doc comment in `src/` (used by `cargo doc`)
- The corresponding section in `README.md` under **API Reference**

They must match exactly — same parameter names, same option strings (e.g. `"songs" | "videos"`), same `Result<Value>` return type.

## Build

```sh
uv sync          # create .venv and install ytmusicapi (only needed once)
cargo build
cargo test
cargo doc --open
```

`build.rs` runs `uv sync` automatically on every `cargo build`, so a manual `uv sync` is only needed before the first build. It also bakes the absolute `.venv` site-packages path into the binary via `YTMUSICAPI_SITE_PACKAGES` so the binary works from any directory.

Run a single test:
```sh
cargo test constructs_unauthenticated
```

Run an example (browser auth required):
```sh
python3 curl2headers.py curl.txt | cargo run --example login_and_playlists
```

## Architecture

This crate is a thin PyO3 bridge — no reimplementation of ytmusicapi logic.

**Data flow:** every method acquires the GIL → calls the Python `YTMusic` instance → serializes the result via `json.dumps` → deserializes via `serde_json`. Input `Value` arguments go the other way through `json.loads`. This round-trip is the only conversion layer; no manual struct mapping exists.

**Key files:**
- `src/lib.rs` — `YTMusic` struct, constructors, `ensure_venv()` (injects site-packages into `sys.path` once via `OnceLock`), `py_to_json`/`json_to_py` helpers, `patch_refreshing_token` monkey-patch
- `src/error.rs` — `YtMusicError` with `Python(String)` and `Json(serde_json::Error)` variants
- `src/browse.rs`, `search.rs`, `library.rs`, `playlists.rs`, `explore.rs`, `podcasts.rs`, `uploads.rs` — method groups implemented as `impl YTMusic` blocks, each a direct call to the Python counterpart
- `build.rs` — runs `uv sync`, emits `YTMUSICAPI_SITE_PACKAGES` env var
- `.cargo/config.toml` — sets `PYO3_PYTHON` to `.venv/bin/python`
- `curl2headers.py` — converts a cURL command (from Chrome DevTools "Copy as cURL") into the header format `YTMusic::setup()` expects on stdin

**`YTMusic` is `Send + Sync`** (`unsafe impl`) because every GIL acquisition is explicit and `Py<PyAny>` is safe to move across threads under that contract.

**`patch_refreshing_token`** is a runtime monkey-patch applied before any OAuth flow. It makes `RefreshingToken.__init__` silently drop unknown kwargs (e.g. `refresh_token_expires_in` added by Google in late 2024 that ytmusicapi 1.x doesn't declare). Uses `PyModule::from_code` with `CString` — required by PyO3 0.23 (`&CStr`, not `&str`).

## PyO3 quirk

`PyModule::from_code` requires `&CStr` arguments, not `&str`. Always construct with:
```rust
let code = CString::new("...").unwrap();
PyModule::from_code(py, code.as_c_str(), c"filename.py", c"module_name")?;
```

## Authentication

- **Browser auth** (working): `curl2headers.py` → `YTMusic::setup()` → `browser.json`
- **OAuth** (broken upstream [#813](https://github.com/sigma67/ytmusicapi/issues/813)): flow completes but YouTube rejects Bearer tokens from user-created clients. Do not add workarounds until upstream resolves this.
