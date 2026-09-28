# TLS session could not retain its peer association

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_e
Log level: error
Documentation ID: log_e-a1d682c00ccc20dc

## Log pattern

```text
SSL_set_ex_data(kNimbleSSLPeerSocketIndex) failed ssl_=%p s=%d
```

## Parameters

- Argument 1 — TLS session: TLS session address used for diagnostic correlation.
- Argument 2 — Socket: Integer socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
SSL_set_ex_data(kNimbleSSLPeerSocketIndex) failed ssl_=0x1000 s=12
```

## Short description

TLS session could not retain its peer association.

## Trigger condition

During accepted-session setup or a later association update, attaching the peer connection to TLS session metadata fails.

## Explanation

The failure is reported without aborting the surrounding operation. This path exists when TLS server-name-extension support is available.

## Possible causes

- Attaching connection metadata to the TLS session failed.

## What to check

1. Compare the TLS session and socket with initialization diagnostics.
