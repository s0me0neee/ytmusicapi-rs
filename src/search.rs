use crate::{json_to_py, py_err, py_to_json, Result, YTMusic};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

impl YTMusic {
    /// Search YouTube Music. `filter` can be `"songs"`, `"videos"`, `"albums"`,
    /// `"artists"`, `"playlists"`, `"community_playlists"`, `"featured_playlists"`, `"uploads"`.
    /// `scope` can be `"library"` or `"uploads"`.
    pub fn search(
        &self,
        query: &str,
        filter: Option<&str>,
        scope: Option<&str>,
        limit: Option<u32>,
        ignore_spelling: Option<bool>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let inner = self.inner.bind(py);
            let kw = PyDict::new(py);
            kw.set_item("query", query)?;
            if let Some(v) = filter { kw.set_item("filter", v)?; }
            if let Some(v) = scope { kw.set_item("scope", v)?; }
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = ignore_spelling { kw.set_item("ignore_spelling", v)?; }
            let result = inner.call_method("search", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch autocomplete suggestions for `query`.
    pub fn get_search_suggestions(
        &self,
        query: &str,
        detailed_runs: Option<bool>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let inner = self.inner.bind(py);
            let kw = PyDict::new(py);
            kw.set_item("query", query)?;
            if let Some(v) = detailed_runs { kw.set_item("detailed_runs", v)?; }
            let result = inner.call_method("get_search_suggestions", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Remove search suggestions. `suggestions` is the list returned by
    /// `get_search_suggestions`. Pass `indices` to remove only specific entries;
    /// omit to remove all of them.
    pub fn remove_search_suggestions(
        &self,
        suggestions: &Value,
        indices: Option<&[u32]>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let py_suggestions = json_to_py(py, suggestions)?;
            let kw = PyDict::new(py);
            kw.set_item("suggestions", py_suggestions)?;
            if let Some(v) = indices {
                kw.set_item("indices", v.to_vec())?;
            }
            let result = self.inner.bind(py).call_method("remove_search_suggestions", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }
}
