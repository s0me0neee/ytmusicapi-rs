#!/usr/bin/env python3
"""Convert a 'Copy as cURL' command to plain request headers for ytmusicapi setup.

Usage:
    python3 curl2headers.py curl.txt | cargo run --example login_and_playlists
    python3 curl2headers.py < curl.txt  | cargo run --example login_and_playlists
"""

import re
import sys


def convert(text: str) -> str:
    headers: dict[str, str] = {}

    # -H 'Name: value' lines
    for m in re.finditer(r"-H '([^']+)'", text):
        line = m.group(1)
        if ": " in line:
            k, v = line.split(": ", 1)
            headers[k.lower()] = v

    # -b 'cookie string' → cookie header
    m = re.search(r"-b '([^']+)'", text, re.DOTALL)
    if m:
        headers["cookie"] = m.group(1)

    if not headers:
        sys.exit("error: no headers found — make sure the file is a 'Copy as cURL' export")

    missing = {"cookie", "x-goog-authuser"} - headers.keys()
    if missing:
        sys.exit(f"error: required headers missing: {', '.join(sorted(missing))}\n"
                 "Copy a request from music.youtube.com while logged in.")

    return "\n".join(f"{k}: {v}" for k, v in headers.items())


if __name__ == "__main__":
    if len(sys.argv) > 1:
        text = open(sys.argv[1]).read()
    else:
        text = sys.stdin.read()
    print(convert(text))
