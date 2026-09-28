# RTMP signed-handshake server challenge cannot be built

Product: Nimble Streamer
Component: RTMP connection transport
Logging function: log_e
Log level: error
Documentation ID: log_e-709cbead8bcbfbfc

## Log pattern

```text
failed to build s0 for client: %d
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to build s0 for client: 42
```

## Short description

RTMP signed-handshake server challenge cannot be built.

## Trigger condition

Building the digest-based server challenge reports failure.

## Explanation

The incoming connection is shut down before the handshake reply is sent. The digest operation can fail when its cryptographic context cannot be allocated.

## Possible causes

- Building the digest-based server challenge reports failure.

## What to check

1. Check associated cryptographic allocation errors and handshake diagnostics.

## Recommended action

Resolve the local cryptographic resource failure before retrying.
