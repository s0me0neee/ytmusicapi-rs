use crate::{py_err, py_to_json, Result, YTMusic};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;

impl YTMusic {
    pub fn get_library_upload_songs(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_upload_songs", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    pub fn get_library_upload_artists(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_upload_artists", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    pub fn get_library_upload_albums(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            if let Some(v) = order { kw.set_item("order", v)?; }
            let result = self.inner.bind(py).call_method("get_library_upload_albums", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    pub fn get_library_upload_artist(&self, browse_id: &str, limit: Option<u32>) -> Result<Value> {
        Python::with_gil(|py| {
            let kw = PyDict::new(py);
            kw.set_item("browseId", browse_id)?;
            if let Some(v) = limit { kw.set_item("limit", v)?; }
            let result = self.inner.bind(py).call_method("get_library_upload_artist", (), Some(&kw))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    pub fn get_library_upload_album(&self, browse_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("get_library_upload_album", (browse_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    /// Upload a local audio file. `filepath` must be an absolute path.
    pub fn upload_song(&self, filepath: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("upload_song", (filepath,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }

    pub fn delete_upload_entity(&self, entity_id: &str) -> Result<Value> {
        Python::with_gil(|py| {
            let result = self.inner.bind(py).call_method1("delete_upload_entity", (entity_id,))?;
            py_to_json(py, &result)
        })
        .map_err(py_err)
    }
}
