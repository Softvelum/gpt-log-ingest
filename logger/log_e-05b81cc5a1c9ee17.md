# TLS license request could not be sent

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-05b81cc5a1c9ee17

## Log pattern

```text
Transcoder validation procedure failed. Cannot send SSL request
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Transcoder validation procedure failed. Cannot send SSL request
```

## Short description

TLS license request could not be sent.

## Trigger condition

The license-validation request could not be sent over TLS.

## Explanation

This license-validation attempt did not complete. The server can retry the license service; this message alone does not establish that the license was rejected.

## Possible causes

- The TLS handshake or request write failed.

## What to check

1. Check the preceding TLS write diagnostic.

## Recommended action

Resolve the specific synchronization failure so license validation can succeed.
