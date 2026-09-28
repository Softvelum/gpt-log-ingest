# License TLS context could not be created

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-46cdccf6b1af3b78

## Log pattern

```text
Transcoder validation procedure failed. Cannot create SSL CTX
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Transcoder validation procedure failed. Cannot create SSL CTX
```

## Short description

License TLS context could not be created.

## Trigger condition

The TLS client context could not be created for the license-validation request.

## Explanation

This license-validation attempt did not complete. The server can retry the license service; this message alone does not establish that the license was rejected.

## Possible causes

- The TLS context allocation or initialization failed.

## What to check

1. Check TLS runtime availability and allocation diagnostics.

## Recommended action

Resolve the specific synchronization failure so license validation can succeed.
