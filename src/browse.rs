use crate::{py_err, py_to_json, Result, YTMusic};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

impl YTMusic {
    /// Fetch the home feed. `limit` caps the number of sections returned.
    pub fn get_home(&self, limit: Option<u32>) -> Result<Value> {
        Python::with_gil(|py| {
            let inner = self.inner.bind(py);
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            let result = inner.call_method("get_home", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch an artist page by channel ID.
    pub fn get_artist(&self, channel_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_artist", (channel_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch albums for an artist. `params` comes from the `get_artist()` response.
    /// `order` is `"Recency"`, `"Popularity"`, or `"Alphabetical order"`.
    pub fn get_artist_albums(
        &self,
        channel_id: &str,
        params: &str,
        limit: Option<u32>,
        order: Option<&str>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("channelId", channel_id)?;
            kw.set_item("params", params)?;
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_artist_albums", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch an album by its browse ID.
    pub fn get_album(&self, browse_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_album", (browse_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Convert an `audioPlaylistId` (starts with `OLAK5uy_`) to a `browseId`.
    pub fn get_album_browse_id(&self, audio_playlist_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_album_browse_id", (audio_playlist_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch song metadata. `signature_timestamp` is needed for streaming URL decryption.
    pub fn get_song(&self, video_id: &str, signature_timestamp: Option<u64>) -> Result<Value> {
        Python::with_gil(|py| {
            let inner = self.inner.bind(py);
            let kw = PyDict::new(py);
            kw.set_item("videoId", video_id)?;
            if let Some(v) = signature_timestamp { kw.set_item("signatureTimestamp", v)?; }
            let result = inner.call_method("get_song", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch content related to a song. `browse_id` comes from the `get_song()` response.
    pub fn get_song_related(&self, browse_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_song_related", (browse_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// `browse_id` is returned by `get_song()` under `lyrics.browseId`.
    /// Set `timestamps` to include word-level timing data.
    pub fn get_lyrics(&self, browse_id: &str, timestamps: Option<bool>) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("browseId", browse_id)?;
            if let Some(v) = timestamps { kw.set_item("timestamps", v)?; }
            let result = self.inner.bind(py).call_method("get_lyrics", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch a user's public profile by channel ID.
    pub fn get_user(&self, channel_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_user", (channel_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch a user's public playlists. `params` comes from the `get_user()` response.
    pub fn get_user_playlists(&self, channel_id: &str, params: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_user_playlists", (channel_id, params))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch a user's uploaded videos. `params` comes from the `get_user()` response.
    pub fn get_user_videos(&self, channel_id: &str, params: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_user_videos", (channel_id, params))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Fetch credits (writers, producers, etc.) for a song. `browse_id` comes from `get_song()`.
    pub fn get_song_credits(&self, browse_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_song_credits", (browse_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }
}
