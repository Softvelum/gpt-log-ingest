# Cannot read receiver SRT stream ID

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-48cc6faf28fc9951

## Log pattern

```text
srt_getsockopt failed, s=%d: %d %s
```

## Parameters

- Argument 1 — Socket: SRT socket identifier.
- Argument 2 — Error number: SRT error code.
- Argument 3 — Error: SRT library error description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
srt_getsockopt failed, s=101: 5004 Invalid socket ID
```

## Short description

Cannot read receiver SRT stream ID.

## Trigger condition

The library fails to retrieve the accepted receiver socket stream ID for wildcard output.

## Explanation

Stream selection and output session setup fail.

## Possible causes

- The library fails to retrieve the accepted receiver socket stream ID for wildcard output.

## What to check

1. Inspect the SRT error and receiver handshake stream ID.
