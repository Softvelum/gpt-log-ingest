# License TLS session could not be created

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-77fe070ba0404dca

## Log pattern

```text
Transcoder validation procedure failed. Cannot create SSL session
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Transcoder validation procedure failed. Cannot create SSL session
```

## Short description

License TLS session could not be created.

## Trigger condition

A TLS session could not be created for the license-validation request.

## Explanation

This license-validation attempt did not complete. The server can retry the license service; this message alone does not establish that the license was rejected.

## Possible causes

- TLS session allocation failed.

## What to check

1. Check TLS runtime allocation failures.

## Recommended action

Resolve the specific synchronization failure so license validation can succeed.
