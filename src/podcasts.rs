use crate::{py_err, py_to_json, with_gil, Result, YTMusic};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

impl YTMusic {
    /// Fetch a podcast by its playlist ID (starts with `MPSP`).
    pub fn get_podcast(&self, playlist_id: &str, limit: Option<u32>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("playlistId", playlist_id)?;
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            let result = self.inner.bind(py).call_method("get_podcast", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch a single podcast episode by its video ID.
    pub fn get_episode(&self, video_id: &str) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_episode", (video_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the "New Episodes" playlist. `playlist_id` defaults to `"RDPN"`.
    pub fn get_episodes_playlist(&self, playlist_id: Option<&str>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = playlist_id { kw.set_item("playlist_id", v)?; }
            let result = self.inner.bind(py).call_method("get_episodes_playlist", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch a podcast channel (show) page by its channel ID.
    pub fn get_channel(&self, channel_id: &str) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_channel", (channel_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch episodes for a channel. `params` comes from `get_channel()` response.
    pub fn get_channel_episodes(&self, channel_id: &str, params: &str) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_channel_episodes", (channel_id, params))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the library of saved episodes (requires auth).
    pub fn get_saved_episodes(&self, limit: Option<u32>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            let result = self.inner.bind(py).call_method("get_saved_episodes", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch podcasts saved in the library (requires auth).
    pub fn get_library_podcasts(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_podcasts", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch podcast channels (shows) saved in the library (requires auth).
    pub fn get_library_channels(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_channels", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }
}
