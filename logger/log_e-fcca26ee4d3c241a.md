# RTMP value has the wrong type

Product: Nimble Streamer
Component: RTMP data parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-fcca26ee4d3c241a

## Log pattern

```text
rtmp_read_number buffer not number[%d]
```

## Parameters

- Argument 1 — Received type: Received one-byte RTMP data-type marker, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rtmp_read_number buffer not number[5]
```

## Short description

RTMP value has the wrong type.

## Trigger condition

The next RTMP data-type marker is not the expected number type.

## Explanation

The value parser returns failure without consuming this value.

## Possible causes

- The payload has another data type where a number is expected.

## What to check

1. Check the data-type marker and RTMP message structure.

## Recommended action

Send the expected number value.
