# Remote completion has an unsupported item state

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_e
Log level: error
Documentation ID: log_e-3e193cd7c826b5ee

## Log pattern

```text
invalid cache item key='%s' state=%d after request type=%d url='%s'
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.
- Argument 2 — state: Cache state: 0 empty, 1 downloading, 2 fragmented, 3 ready, or 4 marked for removal.
- Argument 3 — request type: Numeric external request method.
- Argument 4 — URL: URL supplied by the completion callback.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid cache item key='https://media.example/movie.mp4' state=4 after request type=0 url='https://media.example/movie.mp4'
```

## Short description

Remote completion has an unsupported item state.

## Trigger condition

A request completion finds the cache item in a state other than empty or fragmented.

## Explanation

The callback leaves through its default branch without removing the active request or releasing its item reference here.

## Possible causes

- The item state changed outside the states accepted by this completion path.

## What to check

1. Compare request lifetime and cache state transitions for the key.

## Recommended action

Investigate the unexpected completion state and retained request.
