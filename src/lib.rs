//! Rust bindings for the Python [`ytmusicapi`](https://github.com/sigma67/ytmusicapi) library,
//! implemented via [PyO3](https://pyo3.rs/).
//!
//! All methods return [`Result<serde_json::Value>`]. The conversion goes through Python's
//! `json.dumps` / `json.loads`, so every returned value is guaranteed to be valid JSON.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use ytmusicapi_rs::YTMusic;
//!
//! // Unauthenticated — public endpoints only
//! let yt = YTMusic::new().unwrap();
//! let results = yt.search("Radiohead", Some("artists"), None, Some(5), None).unwrap();
//!
//! // Browser auth — personal library and write operations
//! let yt = YTMusic::authenticated("browser.json").unwrap();
//! let songs = yt.get_liked_songs(Some(25)).unwrap();
//! ```
//!
//! # Thread safety
//!
//! [`YTMusic`] implements `Send + Sync`. The GIL is acquired explicitly on every call,
//! so instances can be shared across threads via `Arc<YTMusic>`.

pub mod error;
mod browse;
mod explore;
mod library;
mod playlists;
mod podcasts;
mod search;
mod uploads;

pub use error::YtMusicError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

/// Shorthand `Result` used by all `YTMusic` methods.
pub type Result<T> = std::result::Result<T, YtMusicError>;

// Embedded at build time by build.rs so we can inject the venv into sys.path
// even when the binary is run from an arbitrary working directory.
const VENV_SITE_PACKAGES: &str = env!("YTMUSICAPI_SITE_PACKAGES");

static PYTHON_SETUP: std::sync::OnceLock<()> = std::sync::OnceLock::new();

fn ensure_venv() {
    PYTHON_SETUP.get_or_init(|| {
        let _ = with_gil(|py| -> PyResult<()> {
            let sys = PyModule::import(py, "sys")?;
            let path = sys.getattr("path")?;
            let already: bool = path
                .call_method1("__contains__", (VENV_SITE_PACKAGES,))?
                .extract()?;
            if !already {
                path.call_method1("insert", (0i32, VENV_SITE_PACKAGES))?;
            }
            Ok(())
        });
    });
}

/// Rust wrapper around the Python `ytmusicapi.YTMusic` class.
///
/// All methods return `serde_json::Value` since the underlying library returns
/// arbitrary nested dicts/lists. The conversion goes through `json.dumps` /
/// `json.loads` so every value is guaranteed to be valid JSON.
pub struct YTMusic {
    pub(crate) inner: Py<PyAny>,
}

// Py<PyAny> is Send + Sync because GIL access is always acquired explicitly.
unsafe impl Send for YTMusic {}
unsafe impl Sync for YTMusic {}

impl YTMusic {
    /// Unauthenticated instance — only public endpoints work.
    pub fn new() -> Result<Self> {
        Self::with_options(None, "en", "")
    }

    /// Authenticated instance. `auth` is a path to a `browser.json` file
    /// (created by `YTMusic::setup()`) or a raw JSON string.
    pub fn authenticated(auth: &str) -> Result<Self> {
        Self::with_options(Some(auth), "en", "")
    }

    /// Full constructor — `auth` is optional, `language` defaults to `"en"`.
    pub fn with_options(auth: Option<&str>, language: &str, location: &str) -> Result<Self> {
        ensure_venv();
        with_gil(|py| {
            let module = PyModule::import(py, "ytmusicapi")?;
            let cls = module.getattr("YTMusic")?;
            let kw = PyDict::new(py);
            if let Some(a) = auth {
                kw.set_item("auth", a)?;
            }
            kw.set_item("language", language)?;
            if !location.is_empty() {
                kw.set_item("location", location)?;
            }
            let instance = cls.call((), Some(&kw))?;
            Ok(YTMusic {
                inner: instance.unbind(),
            })
        })
        .map_err(|e: PyErr| YtMusicError::Python(format!("{e}")))
    }

    /// Run the interactive setup wizard to create a `browser.json` auth file.
    /// Returns the path written, or a JSON token string.
    pub fn setup(filepath: Option<&str>) -> Result<String> {
        ensure_venv();
        with_gil(|py| {
            let module = PyModule::import(py, "ytmusicapi")?;
            let kw = PyDict::new(py);
            if let Some(p) = filepath {
                kw.set_item("filepath", p)?;
            }
            let result = module.call_method("setup", (), Some(&kw))?;
            result.extract::<String>()
        })
        .map_err(|e: PyErr| YtMusicError::Python(format!("{e}")))
    }

    /// Run the OAuth authorization flow (one-time setup).
    ///
    /// When `open_browser` is `true` a browser window opens automatically;
    /// otherwise a URL is printed for manual copy-paste.
    /// The resulting token is saved to `filepath` (defaults to `"oauth.json"`).
    /// After this call, use [`YTMusic::with_oauth`] for every subsequent session.
    pub fn setup_oauth(
        client_id: &str,
        client_secret: &str,
        filepath: Option<&str>,
        open_browser: bool,
    ) -> Result<()> {
        ensure_venv();
        with_gil(|py| {
            patch_refreshing_token(py)?;
            let module = PyModule::import(py, "ytmusicapi")?;
            let kw = PyDict::new(py);
            kw.set_item("client_id", client_id)?;
            kw.set_item("client_secret", client_secret)?;
            kw.set_item("filepath", filepath.unwrap_or("oauth.json"))?;
            kw.set_item("open_browser", open_browser)?;
            module.call_method("setup_oauth", (), Some(&kw))?;
            Ok(())
        })
        .map_err(|e: PyErr| YtMusicError::Python(format!("{e}")))
    }

    /// Create an authenticated instance from an OAuth token file.
    ///
    /// `oauth_file` is the path written by [`YTMusic::setup_oauth`].
    /// `client_id` and `client_secret` are passed as `OAuthCredentials` so the
    /// library can silently refresh the access token when it expires.
    pub fn with_oauth(oauth_file: &str, client_id: &str, client_secret: &str) -> Result<Self> {
        ensure_venv();
        with_gil(|py| {
            let module = PyModule::import(py, "ytmusicapi")?;
            let cls = module.getattr("YTMusic")?;
            let creds_cls = module.getattr("OAuthCredentials")?;
            let creds_kw = PyDict::new(py);
            creds_kw.set_item("client_id", client_id)?;
            creds_kw.set_item("client_secret", client_secret)?;
            let credentials = creds_cls.call((), Some(&creds_kw))?;
            let kw = PyDict::new(py);
            kw.set_item("auth", oauth_file)?;
            kw.set_item("oauth_credentials", credentials)?;
            let instance = cls.call((), Some(&kw))?;
            Ok(YTMusic {
                inner: instance.unbind(),
            })
        })
        .map_err(|e: PyErr| YtMusicError::Python(format!("{e}")))
    }
}

// ── internal helpers ──────────────────────────────────────────────────────────

/// Convert a Python object → `serde_json::Value` via `json.dumps`.
pub(crate) fn py_to_json(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    let json_mod = PyModule::import(py, "json")?;
    let s: String = json_mod.call_method1("dumps", (obj,))?.extract()?;
    serde_json::from_str(&s)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

/// Convert a `serde_json::Value` → Python object via `json.loads`.
pub(crate) fn json_to_py<'py>(
    py: Python<'py>,
    value: &Value,
) -> PyResult<Bound<'py, PyAny>> {
    let s = serde_json::to_string(value)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    PyModule::import(py, "json")?.call_method1("loads", (s,))
}

/// Map a `PyErr` to `YtMusicError`.
pub(crate) fn py_err(e: PyErr) -> YtMusicError {
    YtMusicError::Python(format!("{e}"))
}

/// Acquire the GIL and run `f`. With the `auto-initialize` feature enabled this
/// always succeeds; the `expect` is a safety net that should never trigger.
pub(crate) fn with_gil<F, R>(f: F) -> R
where
    F: for<'py> FnOnce(Python<'py>) -> R,
{
    Python::attach(f)
}

/// Patch `RefreshingToken.__init__` to silently drop any keyword arguments that
/// the dataclass doesn't declare (e.g. `refresh_token_expires_in` added by
/// Google's token endpoint in late 2024, which ytmusicapi 1.12 doesn't handle).
/// Idempotent — guarded by a `_patched` flag on the class.
fn patch_refreshing_token(py: Python<'_>) -> PyResult<()> {
    use std::ffi::CString;
    let code = CString::new(
        "import dataclasses\n\
         from ytmusicapi.auth.oauth.token import RefreshingToken\n\
         if not getattr(RefreshingToken, '_patched', False):\n\
         \x20   _orig = RefreshingToken.__init__\n\
         \x20   _known = {f.name for f in dataclasses.fields(RefreshingToken)}\n\
         \x20   def _init(self, **kw): _orig(self, **{k: v for k, v in kw.items() if k in _known})\n\
         \x20   RefreshingToken.__init__ = _init\n\
         \x20   RefreshingToken._patched = True\n",
    )
    .unwrap();
    PyModule::from_code(py, code.as_c_str(), c"_rt_patch.py", c"_rt_patch")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_unauthenticated() {
        let yt = YTMusic::new().expect("should construct without auth");
        drop(yt);
    }

    #[test]
    fn search_returns_list() {
        let yt = YTMusic::new().unwrap();
        let results = yt.search("Radiohead", Some("artists"), None, Some(5), None).unwrap();
        assert!(results.is_array());
    }
}
