# Live protocol listener settings changed

Product: Nimble Streamer
Component: Live input and publishing workers
Logging function: log_i
Log level: info
Documentation ID: log_i-a6cdb61a3b88328c

## Log pattern

```text
new interface information comes from panel
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
new interface information comes from panel
```

## Short description

Live protocol listener settings changed.

## Trigger condition

RTMP, RTSP, or Icecast interface information differs from the current applied marker.

## Explanation

The corresponding thread adds newly requested listeners and removes obsolete listeners and their clients. Individual bind failures can follow this announcement.
