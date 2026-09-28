# Client request buffer is full

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-05d14bd1d5e437ee

## Log pattern

```text
client buffer is too small to fit the request (buf_size=%d)
```

## Parameters

- Argument 1 — Buffer capacity: Integer request-buffer capacity in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
client buffer is too small to fit the request (buf_size=16384)
```

## Short description

Client request buffer is full.

## Trigger condition

The request buffer is full when another client read is attempted.

## Explanation

An HTTP 500 response is prepared; the request is not extended further.

## Possible causes

- The request buffer is full when another client read is attempted.

## What to check

1. Check whether the client sends excessive request headers or a body larger than the request buffer.

## Recommended action

Check whether the client sends excessive request headers or a body larger than the request buffer.
