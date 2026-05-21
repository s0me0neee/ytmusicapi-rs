//! Methods that require authentication.
use crate::{json_to_py, py_err, py_to_json, with_gil, Result, YTMusic};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

impl YTMusic {
    /// Fetch the user's saved songs (requires auth).
    pub fn get_library_songs(
        &self,
        limit: Option<u32>,
        validate_responses: Option<bool>,
        order: Option<&str>,
    ) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = validate_responses { kw.set_item("validate_responses", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_songs", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the user's saved albums (requires auth).
    pub fn get_library_albums(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_albums", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the user's saved artists (requires auth).
    pub fn get_library_artists(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_artists", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the user's subscribed artists (requires auth).
    pub fn get_library_subscriptions(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_subscriptions", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the user's saved playlists (requires auth).
    pub fn get_library_playlists(&self, limit: Option<u32>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            let result = self.inner.bind(py).call_method("get_library_playlists", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the user's liked songs playlist (requires auth).
    pub fn get_liked_songs(&self, limit: Option<u32>) -> Result<Value> {
        with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            let result = self.inner.bind(py).call_method("get_liked_songs", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch the user's playback history (requires auth).
    pub fn get_history(&self) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method0("get_history")?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// `song` should be a song object previously returned by `get_history` or similar.
    pub fn add_history_item(&self, song: &Value) -> Result<Value> {
        with_gil(|py| {
            let py_song = json_to_py(py, song)?;
            let result = self.inner.bind(py).call_method1("add_history_item", (py_song,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Remove history entries by feedback token (requires auth).
    pub fn remove_history_items(&self, feedback_tokens: &[&str]) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("remove_history_items", (feedback_tokens.to_vec(),))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// `rating` must be `"LIKE"`, `"DISLIKE"`, or `"INDIFFERENT"`.
    pub fn rate_song(&self, video_id: &str, rating: &str) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("rate_song", (video_id, rating))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Rate a playlist (requires auth). `rating` must be `"LIKE"`, `"DISLIKE"`, or `"INDIFFERENT"`.
    pub fn rate_playlist(&self, playlist_id: &str, rating: &str) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("rate_playlist", (playlist_id, rating))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Toggle a song's library status via feedback token (requires auth).
    pub fn edit_song_library_status(&self, feedback_tokens: &[&str]) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("edit_song_library_status", (feedback_tokens.to_vec(),))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Subscribe to artists by channel ID (requires auth).
    pub fn subscribe_artists(&self, channel_ids: &[&str]) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("subscribe_artists", (channel_ids.to_vec(),))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Unsubscribe from artists by channel ID (requires auth).
    pub fn unsubscribe_artists(&self, channel_ids: &[&str]) -> Result<Value> {
        with_gil(|py| {
            let result = self.inner.bind(py).call_method1("unsubscribe_artists", (channel_ids.to_vec(),))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }
}
