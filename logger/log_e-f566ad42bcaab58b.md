# License socket could not be attached to TLS

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-f566ad42bcaab58b

## Log pattern

```text
Transcoder validation procedure failed. Cannot bind socket to SSL session
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Transcoder validation procedure failed. Cannot bind socket to SSL session
```

## Short description

License socket could not be attached to TLS.

## Trigger condition

The license-validation connection could not be attached to its TLS session.

## Explanation

This license-validation attempt did not complete. The server can retry the license service; this message alone does not establish that the license was rejected.

## Possible causes

- The TLS library rejected the socket association.

## What to check

1. Check the connection and TLS socket-attachment failure.

## Recommended action

Resolve the specific synchronization failure so license validation can succeed.
