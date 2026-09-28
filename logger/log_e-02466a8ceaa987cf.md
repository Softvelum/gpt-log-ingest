# TLS license response could not be read

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-02466a8ceaa987cf

## Log pattern

```text
Transcoder validation procedure failed. Cannot read SSL response
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Transcoder validation procedure failed. Cannot read SSL response
```

## Short description

TLS license response could not be read.

## Trigger condition

The license-validation request could not read its TLS response.

## Explanation

This license-validation attempt did not complete. The server can retry the license service; this message alone does not establish that the license was rejected.

## Possible causes

- The TLS library could not read the response.

## What to check

1. Check TLS response and connection errors.

## Recommended action

Resolve the specific synchronization failure so license validation can succeed.
