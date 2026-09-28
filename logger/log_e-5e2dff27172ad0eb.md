# License request send failure reported

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-5e2dff27172ad0eb

## Log pattern

```text
Transcoder validation procedure failed to send request
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Transcoder validation procedure failed to send request
```

## Short description

License request send failure reported.

## Trigger condition

The license-validation request reports a send failure. The currently used TLS request helper does not produce this particular status.

## Explanation

This license-validation attempt did not complete. The server can retry the license service; this message alone does not establish that the license was rejected.

## Possible causes

- This is a retained status-handling branch, not a failure currently emitted by the called TLS helper.

## What to check

1. Check which synchronization status is actually being returned.

## Recommended action

Resolve the specific synchronization failure so license validation can succeed.
