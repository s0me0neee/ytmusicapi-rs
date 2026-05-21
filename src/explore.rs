use crate::{json_to_py, py_err, py_to_json, Result, YTMusic};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

impl YTMusic {
    /// Fetch the list of mood and genre categories.
    pub fn get_mood_categories(&self) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method0("get_mood_categories")?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch playlists for a mood category. `params` comes from the `get_mood_categories()` response.
    pub fn get_mood_playlists(&self, params: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_mood_playlists", (params,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// `country` is a 2-letter ISO country code, or `"ZZ"` for global charts.
    pub fn get_charts(&self, country: Option<&str>) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = country { kw.set_item("country", v)?; }
            let result = self.inner.bind(py).call_method("get_charts", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the user's taste profile (pass to `set_tasteprofile` to update it).
    pub fn get_tasteprofile(&self) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method0("get_tasteprofile")?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// `artists` is a list of artist names. `taste_profile` can be the result of `get_tasteprofile`.
    pub fn set_tasteprofile(
        &self,
        artists: &[&str],
        taste_profile: Option<&Value>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("artists", artists.to_vec())?;
            if let Some(v) = taste_profile {
                kw.set_item("taste_profile", json_to_py(py, v)?)?;
            }
            let result = self.inner.bind(py).call_method("set_tasteprofile", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the Explore page (new releases, charts, moods).
    pub fn get_explore(&self) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method0("get_explore")?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Returns info about the currently authenticated account (name, channel ID, etc.).
    pub fn get_account_info(&self) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method0("get_account_info")?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the auto-play queue for a track or playlist.
    /// At least one of `video_id` or `playlist_id` must be provided.
    pub fn get_watch_playlist(
        &self,
        video_id: Option<&str>,
        playlist_id: Option<&str>,
        limit: Option<u32>,
        radio: Option<bool>,
        shuffle: Option<bool>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = video_id { kw.set_item("videoId", v)?; }
            if let Some(v) = playlist_id { kw.set_item("playlistId", v)?; }
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = radio { kw.set_item("radio", v)?; }
            if let Some(v) = shuffle { kw.set_item("shuffle", v)?; }
            let result = self.inner.bind(py).call_method("get_watch_playlist", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }
}
