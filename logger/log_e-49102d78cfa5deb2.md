# RTSP publishing Digest response did not match

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-49102d78cfa5deb2

## Log pattern

```text
digest auth failed, client=%d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
digest auth failed, client=25
```

## Short description

RTSP publishing Digest response did not match.

## Trigger condition

The supplied Digest response differs from the expected value calculated using the application credentials, nonce, method, and URL.

## Explanation

The server generates a new nonce, sends another unauthorized response, and stops processing the current ANNOUNCE.

## Possible causes

- The supplied Digest response differs from the expected value calculated using the application credentials, nonce, method, and URL.

## What to check

1. Check the publisher credentials and Digest response calculation.

## Recommended action

Respond to the current challenge using the correct publishing credentials.

## Related log messages

- [RTSP publishing URL authorization failed](log_e-2dbe0318acf70446.md)
