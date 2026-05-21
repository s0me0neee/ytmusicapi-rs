# ytmusicapi-rs

Rust bindings for the Python [ytmusicapi](https://github.com/sigma67/ytmusicapi) library, via [PyO3](https://pyo3.rs/).

All methods return `Result<serde_json::Value>`. The conversion goes through Python's `json.dumps` / `json.loads`, so every value is valid JSON.

## Requirements

- Rust (stable)
- Python 3.8+
- [uv](https://docs.astral.sh/uv/)

## Setup

```sh
uv sync          # create .venv and install ytmusicapi
cargo build
```

The build script (`build.rs`) runs `uv sync` automatically and embeds the venv path at compile time. The `.cargo/config.toml` sets `VIRTUAL_ENV` to the local `.venv`; `pyo3-build-config` resolves this to the correct Python executable on each platform (Unix: `.venv/bin/python3`, Windows: `.venv/Scripts/python.exe`).

## Authentication

### Unauthenticated (public endpoints only)

```rust
let yt = YTMusic::new()?;
```

### Browser auth (recommended)

Use "Copy as cURL" from Chrome DevTools on any YouTube Music request, pipe it through the included `curl2headers.py` converter, then run the setup wizard:

```sh
python3 curl2headers.py curl.txt | cargo run --example login_and_playlists
```

```rust
let yt = YTMusic::authenticated("browser.json")?;
```

### OAuth (broken upstream — see [#813](https://github.com/sigma67/ytmusicapi/issues/813))

The setup flow works, but YouTube Music rejects Bearer tokens from user-created OAuth clients. Use browser auth instead.

### Full constructor

```rust
let yt = YTMusic::with_options(Some("browser.json"), "en", "")?;
```

### Static setup helpers

```rust
// Interactive browser auth wizard — reads headers from stdin
YTMusic::setup(Some("browser.json"))?;

// OAuth one-time setup — saves token to oauth.json
YTMusic::setup_oauth("CLIENT_ID", "CLIENT_SECRET", Some("oauth.json"), false)?;

// Create instance from OAuth token file
let yt = YTMusic::with_oauth("oauth.json", "CLIENT_ID", "CLIENT_SECRET")?;
```

---

## API Reference

### Search

```rust
// filter: "songs" | "videos" | "albums" | "artists" | "playlists" |
//         "community_playlists" | "featured_playlists" | "uploads"
// scope:  "library" | "uploads"
fn search(
    &self,
    query: &str,
    filter: Option<&str>,
    scope: Option<&str>,
    limit: Option<u32>,
    ignore_spelling: Option<bool>,
) -> Result<Value>

fn get_search_suggestions(
    &self,
    query: &str,
    detailed_runs: Option<bool>,
) -> Result<Value>

// suggestions: list returned by get_search_suggestions
// indices: which entries to remove; omit to remove all
fn remove_search_suggestions(
    &self,
    suggestions: &Value,
    indices: Option<&[u32]>,
) -> Result<Value>
```

### Browse

```rust
fn get_home(&self, limit: Option<u32>) -> Result<Value>

fn get_artist(&self, channel_id: &str) -> Result<Value>

// params comes from get_artist() response
// order: "Recency" | "Popularity" | "Alphabetical order"
fn get_artist_albums(
    &self,
    channel_id: &str,
    params: &str,
    limit: Option<u32>,
    order: Option<&str>,
) -> Result<Value>

fn get_album(&self, browse_id: &str) -> Result<Value>

// Converts an audioPlaylistId (OLAK5uy_…) to a browseId
fn get_album_browse_id(&self, audio_playlist_id: &str) -> Result<Value>

fn get_song(&self, video_id: &str, signature_timestamp: Option<u64>) -> Result<Value>

fn get_song_related(&self, browse_id: &str) -> Result<Value>

// browse_id comes from get_song() → lyrics.browseId
fn get_lyrics(&self, browse_id: &str, timestamps: Option<bool>) -> Result<Value>

fn get_song_credits(&self, browse_id: &str) -> Result<Value>

fn get_user(&self, channel_id: &str) -> Result<Value>

// params comes from get_user() response
fn get_user_playlists(&self, channel_id: &str, params: &str) -> Result<Value>

fn get_user_videos(&self, channel_id: &str, params: &str) -> Result<Value>
```

### Explore

```rust
fn get_explore(&self) -> Result<Value>

fn get_mood_categories(&self) -> Result<Value>

// params comes from get_mood_categories() response
fn get_mood_playlists(&self, params: &str) -> Result<Value>

// country: 2-letter ISO code, or "ZZ" for global charts
fn get_charts(&self, country: Option<&str>) -> Result<Value>

fn get_tasteprofile(&self) -> Result<Value>

fn set_tasteprofile(
    &self,
    artists: &[&str],
    taste_profile: Option<&Value>,
) -> Result<Value>

fn get_watch_playlist(
    &self,
    video_id: Option<&str>,
    playlist_id: Option<&str>,
    limit: Option<u32>,
    radio: Option<bool>,
    shuffle: Option<bool>,
) -> Result<Value>

fn get_account_info(&self) -> Result<Value>
```

### Playlists

```rust
fn get_playlist(
    &self,
    playlist_id: &str,
    limit: Option<u32>,
    related: Option<bool>,
    suggestions_limit: Option<u32>,
) -> Result<Value>

// privacy_status: "PUBLIC" | "PRIVATE" (default) | "UNLISTED"
fn create_playlist(
    &self,
    title: &str,
    description: &str,
    privacy_status: Option<&str>,
    video_ids: Option<&[&str]>,
    source_playlist: Option<&str>,
) -> Result<Value>

// move_item: (set_video_id, successor_video_id) — moves set_video_id after successor_video_id
fn edit_playlist(
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
) -> Result<Value>

fn delete_playlist(&self, playlist_id: &str) -> Result<Value>

fn add_playlist_items(
    &self,
    playlist_id: &str,
    video_ids: Option<&[&str]>,
    source_playlist: Option<&str>,
    duplicates: Option<bool>,
) -> Result<Value>

// videos: track objects as returned by get_playlist
fn remove_playlist_items(&self, playlist_id: &str, videos: &Value) -> Result<Value>

fn join_collaborative_playlist(
    &self,
    playlist_id: &str,
    join_collaboration_token: &str,
) -> Result<Value>
```

### Library (requires auth)

```rust
fn get_library_songs(
    &self,
    limit: Option<u32>,
    validate_responses: Option<bool>,
    order: Option<&str>,
) -> Result<Value>

fn get_library_albums(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value>

fn get_library_artists(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value>

fn get_library_subscriptions(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value>

fn get_library_playlists(&self, limit: Option<u32>) -> Result<Value>

fn get_liked_songs(&self, limit: Option<u32>) -> Result<Value>

fn get_history(&self) -> Result<Value>

// song: a song object previously returned by get_history or similar
fn add_history_item(&self, song: &Value) -> Result<Value>

fn remove_history_items(&self, feedback_tokens: &[&str]) -> Result<Value>

// rating: "LIKE" | "DISLIKE" | "INDIFFERENT"
fn rate_song(&self, video_id: &str, rating: &str) -> Result<Value>

fn rate_playlist(&self, playlist_id: &str, rating: &str) -> Result<Value>

fn edit_song_library_status(&self, feedback_tokens: &[&str]) -> Result<Value>

fn subscribe_artists(&self, channel_ids: &[&str]) -> Result<Value>

fn unsubscribe_artists(&self, channel_ids: &[&str]) -> Result<Value>
```

### Podcasts

```rust
// playlist_id starts with MPSP
fn get_podcast(&self, playlist_id: &str, limit: Option<u32>) -> Result<Value>

fn get_episode(&self, video_id: &str) -> Result<Value>

// playlist_id defaults to "RDPN" (New Episodes feed)
fn get_episodes_playlist(&self, playlist_id: Option<&str>) -> Result<Value>

fn get_channel(&self, channel_id: &str) -> Result<Value>

// params comes from get_channel() response
fn get_channel_episodes(&self, channel_id: &str, params: &str) -> Result<Value>

fn get_saved_episodes(&self, limit: Option<u32>) -> Result<Value>  // requires auth

fn get_library_podcasts(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value>  // requires auth

fn get_library_channels(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value>  // requires auth
```

### Uploads (requires auth)

```rust
fn get_library_upload_songs(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value>

fn get_library_upload_artists(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value>

fn get_library_upload_albums(&self, limit: Option<u32>, order: Option<&str>) -> Result<Value>

fn get_library_upload_artist(&self, browse_id: &str, limit: Option<u32>) -> Result<Value>

fn get_library_upload_album(&self, browse_id: &str) -> Result<Value>

// filepath must be an absolute path
fn upload_song(&self, filepath: &str) -> Result<Value>

fn delete_upload_entity(&self, entity_id: &str) -> Result<Value>
```

---

## Error handling

All methods return `Result<Value, YtMusicError>`. The error type has two variants:

```rust
pub enum YtMusicError {
    Python(String),        // any error raised by the Python library
    Json(serde_json::Error), // JSON serialization failure (should not occur in practice)
}
```

## Thread safety

`YTMusic` implements `Send + Sync`. The GIL is acquired explicitly on every call, so instances can be shared across threads via `Arc<YTMusic>`.
