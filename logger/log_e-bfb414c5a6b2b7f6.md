# RTMP connect result second object cannot be opened

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-bfb414c5a6b2b7f6

## Log pattern

```text
cannot read second object start for connect _result
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot read second object start for connect _result
```

## Short description

RTMP connect result second object cannot be opened.

## Trigger condition

The remote connect _result response does not yield the expected second AMF object start.

## Explanation

The parser logs the failure and continues examining response fields. A later parse failure or lack of a success code can then fail the connection.

## Possible causes

- The remote connect _result response does not yield the expected second AMF object start.

## What to check

1. Check the second object and AMF framing in the remote connect response.

## Recommended action

Correct malformed response objects at the remote server.
