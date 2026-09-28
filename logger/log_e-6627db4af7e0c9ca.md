# Remote fragment response has no positive length

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_e
Log level: error
Documentation ID: log_e-6627db4af7e0c9ca

## Log pattern

```text
key='%s' f=%llu-%llu request failed content_length=%lld
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.
- Argument 2 — from: Inclusive starting byte offset.
- Argument 3 — to: Exclusive ending byte offset.
- Argument 4 — length: Selected response content length in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
key='https://media.example/movie.mp4' f=0-1048576 request failed content_length=0
```

## Short description

Remote fragment response has no positive length.

## Trigger condition

A fragmented item receives a successful HTTP status but the selected content length is zero or negative.

## Explanation

The fragment is marked failed and the active request is released. Chunked responses use the received body length; other responses use the declared length.

## Possible causes

- The response does not provide a positive content length.

## What to check

1. Check response framing, declared length, and body data.

## Recommended action

Make the origin return valid nonempty media data.
