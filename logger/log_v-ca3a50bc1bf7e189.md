# Smooth Streaming manifest parsing failed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-ca3a50bc1bf7e189

## Log pattern

```text
failed to parse Smooth playlist:
%s
```

## Parameters

- Argument 1 — Playlist text: Text of the playlist that could not be parsed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse Smooth playlist:
<html>Not found</html>
```

## Short description

Smooth Streaming manifest parsing failed.

## Trigger condition

The downloaded Smooth Streaming manifest cannot be parsed for edge delivery.

## Explanation

An HTTP 406 response is prepared instead of the manifest.

## Possible causes

- The downloaded Smooth Streaming manifest cannot be parsed for edge delivery.

## What to check

1. Check the origin manifest for valid Smooth Streaming XML and ensure the URL returns media metadata rather than an HTML error.

## Recommended action

Check the origin manifest for valid Smooth Streaming XML and ensure the URL returns media metadata rather than an HTML error.
