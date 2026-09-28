# HTTP/2 request stream identifier went backwards

Product: Nimble Streamer
Component: HTTP/2 connection
Logging function: log_v
Log level: verbose
Documentation ID: log_v-934ebbacb25a2681

## Log pattern

```text
stream id gap detected for s=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
stream id gap detected for s=12
```

## Short description

HTTP/2 request stream identifier went backwards.

## Trigger condition

A request-header stream identifier is smaller than the next minimum allowed identifier.

## Explanation

The request is rejected and a GOAWAY carrying the excessive-load error code is sent or queued.

## Possible causes

- The client reuses an earlier stream identifier or sends identifiers out of increasing order.

## What to check

1. Check the client request stream sequence. The log wording does not mean that forward gaps alone are rejected.

## Recommended action

Correct the client stream sequencing and reconnect.
