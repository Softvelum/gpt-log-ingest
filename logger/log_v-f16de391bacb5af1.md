# Disk cache version check started

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-f16de391bacb5af1

## Log pattern

```text
Check cache version
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Check cache version
```

## Short description

Disk cache version check started.

## Trigger condition

The cache begins reading or creating its stored compatibility marker.

## Explanation

If the existing marker matches the expected version, stored cache files are retained. A missing or differing marker is rewritten before cache cleaning is attempted.
