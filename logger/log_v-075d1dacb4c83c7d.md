# Remote progressive-download delivery is disabled

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-075d1dacb4c83c7d

## Log pattern

```text
PD sessions using remote VOD storage are not enabled
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
PD sessions using remote VOD storage are not enabled
```

## Short description

Remote progressive-download delivery is disabled.

## Trigger condition

A remote VOD request is classified as progressive download while remote PD is disabled and the path is neither MP3 nor AAC.

## Explanation

The request is rejected with HTTP 500.

## Possible causes

- A remote VOD request is classified as progressive download while remote PD is disabled and the path is neither MP3 nor AAC.

## What to check

1. Enable remote VOD progressive download if it is intended, or request an enabled streaming output; this branch explicitly exempts MP3 and AAC paths.

## Recommended action

Enable remote VOD progressive download if it is intended, or request an enabled streaming output; this branch explicitly exempts MP3 and AAC paths.
