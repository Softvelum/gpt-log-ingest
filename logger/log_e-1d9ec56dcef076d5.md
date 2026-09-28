# File modification time could not be read

Product: Nimble Streamer
Component: Local file metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-1d9ec56dcef076d5

## Log pattern

```text
failed to get last write time for '%s'
```

## Parameters

- Argument 1 — File: Filesystem pathname whose modification time could not be obtained, text.

One occurrence is the non-forced cross-domain policy refresh. Four occurrences cover local DASH manifests and HLS playlists in HTTP/1 and HTTP/2 or HTTP/3 request handling.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get last write time for '/srv/media/playlist.m3u8'
```

## Short description

File modification time could not be read.

## Trigger condition

The filesystem modification-time query returns an error for the named file.

## Explanation

Periodic cross-domain policy reload keeps the existing cached policy. Local prepackaged HLS or DASH delivery instead aborts the request with HTTP 500.

## Possible causes

- The file was removed or replaced, access is denied, or the filesystem cannot provide metadata.

## What to check

1. Check existence, permissions, and filesystem health for the exact file. For media requests, the content had already loaded before this metadata lookup failed.

## Recommended action

Restore metadata access to the file and retry the request or policy reload.
