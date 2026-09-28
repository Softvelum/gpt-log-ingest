# DVR archive batch reload started

Product: Nimble Streamer
Component: DVR
Logging function: log_i
Log level: info
Documentation ID: log_i-de7e0c1489dda95f

## Log pattern

```text
Reloading DVR archives
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Reloading DVR archives
```

## Short description

DVR archive batch reload started.

## Trigger condition

A nonempty archive loading list is submitted.

## Explanation

Archive loaders run concurrently using the requested worker count or an automatically selected count.

## Possible causes

- A nonempty archive loading list is submitted.
