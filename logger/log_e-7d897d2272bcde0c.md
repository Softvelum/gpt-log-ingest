# RTMP value has the wrong type

Product: Nimble Streamer
Component: RTMP data parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-7d897d2272bcde0c

## Log pattern

```text
rtmp_read_string buffer not string[%d]
```

## Parameters

- Argument 1 — Received type: Received one-byte RTMP data-type marker, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rtmp_read_string buffer not string[5]
```

## Short description

RTMP value has the wrong type.

## Trigger condition

The next RTMP data-type marker is not the expected string type.

## Explanation

The value parser returns failure without consuming this value.

## Possible causes

- The payload has another data type where a string is expected.

## What to check

1. Check the data-type marker and RTMP message structure.

## Recommended action

Send the expected string value.
