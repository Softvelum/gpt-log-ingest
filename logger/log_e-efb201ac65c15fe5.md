# RTMP handshake read fails after a PROXY header

Product: Nimble Streamer
Component: RTMP connection transport
Logging function: log_e
Log level: error
Documentation ID: log_e-efb201ac65c15fe5

## Log pattern

```text
failed reading remained data after proxy protocol header
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed reading remained data after proxy protocol header
```

## Short description

RTMP handshake read fails after a PROXY header.

## Trigger condition

After accepting and removing a PROXY protocol v1 header, reading the missing client-handshake bytes reports an error or orderly closure.

## Explanation

The incoming connection is marked shut down before the RTMP handshake is accepted.

## Possible causes

- After accepting and removing a PROXY protocol v1 header, reading the missing client-handshake bytes reports an error or orderly closure.

## What to check

1. Check whether the proxy forwards the complete client handshake after its header and keeps the connection open.

## Recommended action

Correct the proxy forwarding or upstream connection failure.
