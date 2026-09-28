# SRT connection acceptance failed

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-ddaced0f51428648

## Log pattern

```text
srt_accept failed, s=%d: %d %s
```

## Parameters

- Argument 1 — Socket: Stored sender socket or listener descriptor; the multipoint output accept path can still print its pre-accept value.
- Argument 2 — Error number: SRT error code.
- Argument 3 — Error: SRT library error description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
srt_accept failed, s=100: 6002 Non-blocking call failure
```

## Short description

SRT connection acceptance failed.

## Trigger condition

An SRT accept operation returns an invalid socket.

## Explanation

No new connection is accepted. The ingest listener suppresses ordinary asynchronous no-data errors; sender acceptance paths can report them. Dedicated sender paths may enter shutdown while multipoint listeners remain available.

## Possible causes

- An SRT accept operation returns an invalid socket.

## What to check

1. Inspect the SRT error and whether the message comes from ingest or output acceptance.
