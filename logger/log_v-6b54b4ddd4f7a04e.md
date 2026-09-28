# Remote fragment request is already queued

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_v
Log level: verbose
Documentation ID: log_v-6b54b4ddd4f7a04e

## Log pattern

```text
request for item key='%s' fi=%u is already pending
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.
- Argument 2 — fragment: Zero-based fragment index.

The fragment index is a platform-sized value passed to a 32-bit unsigned conversion.

## Log examples

No portable example is provided for the mismatched fragment-index conversion.

## Short description

Remote fragment request is already queued.

## Trigger condition

An active request exists and an identical byte range is found in its pending queue.

## Explanation

No second pending request is added. The caller reference is released when ownership is transferred.
