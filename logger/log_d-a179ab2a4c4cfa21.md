# Media module attempted to open a file

Product: Nimble Streamer
Component: Non-Windows progressive download modules
Logging function: log_d
Log level: debug
Documentation ID: log_d-a179ab2a4c4cfa21

## Log pattern

```text
ngx_open_cached_file %.*s fd %d
```

## Parameters

- Argument 1 — length: Number of filename characters printed.
- Argument 2 — file: Media file path.
- Argument 3 — descriptor: File open result; -1 indicates failure.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ngx_open_cached_file /media/movie.mp4 fd 12
```

## Short description

Media module attempted to open a file.

## Trigger condition

The file passes metadata and non-directory checks and a read-only open is attempted.

## Explanation

The resulting descriptor is logged before failure checking. A negative descriptor means the open failed; a nonnegative result is followed by copying file metadata into the module response.
