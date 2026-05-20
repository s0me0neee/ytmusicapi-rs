// Authentication note: ytmusicapi's OAuth with user-created clients is currently
// broken upstream (issue #813 — YouTube rejects tokens from non-whitelisted client
// IDs). The working method is browser auth: copy request headers from DevTools.

use std::path::Path;
use ytmusicapi::YTMusic;

const BROWSER_FILE: &str = "browser.json";

fn main() {
    if !Path::new(BROWSER_FILE).exists() {
        run_setup();
    }

    let yt = YTMusic::authenticated(BROWSER_FILE).unwrap_or_else(|e| {
        eprintln!("error: failed to create session: {e}");
        std::process::exit(1);
    });

    println!("Fetching library playlists…");

    let playlists = yt.get_library_playlists(None).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    let list = playlists
        .as_array()
        .expect("get_library_playlists should return an array");

    if list.is_empty() {
        println!("No playlists found.");
        return;
    }

    println!("\n{} playlist(s):\n", list.len());
    println!("{:<4} {:<50} {:<34} {}", "#", "Title", "Playlist ID", "Songs");
    println!("{}", "─".repeat(100));

    for (i, pl) in list.iter().enumerate() {
        let title = pl["title"].as_str().unwrap_or("<untitled>");
        let id = pl["playlistId"].as_str().unwrap_or("—");
        let count = pl.get("count").and_then(|v| v.as_str()).unwrap_or("?");
        println!("{:<4} {:<50} {:<34} {}", i + 1, truncate(title, 49), id, count);
    }
}

fn run_setup() {
    println!("=== YouTube Music browser auth setup ===\n");
    println!("No {BROWSER_FILE} found. You need to copy your browser's request headers once.\n");
    println!("Steps:");
    println!("  1. Open https://music.youtube.com and make sure you're logged in.");
    println!("  2. Open DevTools (F12) → Network tab.");
    println!("  3. Reload the page, then click any request to music.youtube.com.");
    println!("  4. Right-click → Copy → Copy request headers (or 'Copy as cURL' then paste the -H lines).");
    println!("  5. Paste everything at the prompt below and press Enter twice.\n");
    println!("Full guide: https://ytmusicapi.readthedocs.io/en/stable/setup/browser.html\n");

    YTMusic::setup(Some(BROWSER_FILE)).unwrap_or_else(|e| {
        eprintln!("error: setup failed: {e}");
        std::process::exit(1);
    });

    println!("\nSaved to {BROWSER_FILE}. You won't need to repeat this unless you log out.\n");
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut cut = max.saturating_sub(1);
    while !s.is_char_boundary(cut) {
        cut -= 1;
    }
    &s[..cut]
}
