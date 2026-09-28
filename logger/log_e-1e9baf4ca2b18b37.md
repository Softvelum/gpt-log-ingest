# Remote item pending queue is full

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_e
Log level: error
Documentation ID: log_e-1e9baf4ca2b18b37

## Log pattern

```text
too many pending requests for url='%s'
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
too many pending requests for url='https://media.example/movie.mp4'
```

## Short description

Remote item pending queue is full.

## Trigger condition

Another request is active for the item and its pending queue already contains at least 2,000 requests.

## Explanation

The newly allocated request object is deleted and the new fragment request reports failure.

## Possible causes

- The item has reached the per-URL pending-request limit.

## What to check

1. Check demand for distinct fragments and whether active downloads complete.

## Recommended action

Reduce pending fragment demand or resolve stalled origin downloads.
