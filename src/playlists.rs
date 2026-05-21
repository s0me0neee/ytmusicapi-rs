use crate::{py_err, py_to_json, Result, YTMusic};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

impl YTMusic {
    /// Fetch a playlist's metadata and tracks.
    /// `related` includes related playlists; `suggestions_limit` caps suggested tracks.
    pub fn get_playlist(
        &self,
        playlist_id: &str,
        limit: Option<u32>,
        related: Option<bool>,
        suggestions_limit: Option<u32>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("playlistId", playlist_id)?;
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = related { kw.set_item("related", v)?; }
            if let Some(v) = suggestions_limit { kw.set_item("suggestions_limit", v)?; }
            let result = self.inner.bind(py).call_method("get_playlist", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// `privacy_status` is `"PUBLIC"`, `"PRIVATE"` (default), or `"UNLISTED"`.
    pub fn create_playlist(
        &self,
        title: &str,
        description: &str,
        privacy_status: Option<&str>,
        video_ids: Option<&[&str]>,
        source_playlist: Option<&str>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("title", title)?;
            kw.set_item("description", description)?;
            if let Some(v) = privacy_status { kw.set_item("privacy_status", v)?; }
            if let Some(v) = video_ids { kw.set_item("video_ids", v.to_vec())?; }
            if let Some(v) = source_playlist { kw.set_item("source_playlist", v)?; }
            let result = self.inner.bind(py).call_method("create_playlist", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Edit playlist metadata or reorder tracks (requires auth).
    /// `move_item` is `(set_video_id, successor_video_id)` — moves `set_video_id` to after `successor_video_id`.
    pub fn edit_playlist(
        &self,
        playlist_id: &str,
        title: Option<&str>,
        description: Option<&str>,
        privacy_status: Option<&str>,
        collaboration: Option<bool>,
        move_item: Option<(&str, &str)>,
        add_playlist_id: Option<&str>,
        sort_order: Option<&str>,
        add_to_top: Option<bool>,
        vote_option: Option<&str>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("playlistId", playlist_id)?;
            if let Some(v) = title { kw.set_item("title", v)?; }
            if let Some(v) = description { kw.set_item("description", v)?; }
            if let Some(v) = privacy_status { kw.set_item("privacyStatus", v)?; }
            if let Some(v) = collaboration { kw.set_item("collaboration", v)?; }
            if let Some((set_video_id, successor_video_id)) = move_item {
                kw.set_item("moveItem", (set_video_id, successor_video_id))?;
            }
            if let Some(v) = add_playlist_id { kw.set_item("addPlaylistId", v)?; }
            if let Some(v) = sort_order { kw.set_item("sortOrder", v)?; }
            if let Some(v) = add_to_top { kw.set_item("addToTop", v)?; }
            if let Some(v) = vote_option { kw.set_item("voteOption", v)?; }
            let result = self.inner.bind(py).call_method("edit_playlist", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Delete a playlist by its ID (requires auth).
    pub fn delete_playlist(&self, playlist_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("delete_playlist", (playlist_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Add videos to a playlist by video ID or by copying from another playlist (requires auth).
    pub fn add_playlist_items(
        &self,
        playlist_id: &str,
        video_ids: Option<&[&str]>,
        source_playlist: Option<&str>,
        duplicates: Option<bool>,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("playlistId", playlist_id)?;
            if let Some(v) = video_ids { kw.set_item("videoIds", v.to_vec())?; }
            if let Some(v) = source_playlist { kw.set_item("source_playlist", v)?; }
            if let Some(v) = duplicates { kw.set_item("duplicates", v)?; }
            let result = self.inner.bind(py).call_method("add_playlist_items", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// `videos` is a list of track objects (as returned by `get_playlist`).
    pub fn remove_playlist_items(&self, playlist_id: &str, videos: &Value) -> Result<Value> {
        Python::with_gil(|py| {
            let py_videos = crate::json_to_py(py, videos)?;
            let result = self.inner.bind(py).call_method1("remove_playlist_items", (playlist_id, py_videos))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Join a collaborative playlist. `join_collaboration_token` is found in
    /// the playlist's share link (the `music.youtube.com/playlist?list=…` URL).
    pub fn join_collaborative_playlist(
        &self,
        playlist_id: &str,
        join_collaboration_token: &str,
    ) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1(
                "join_collaborative_playlist",
                (playlist_id, join_collaboration_token),
            )?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }
}
