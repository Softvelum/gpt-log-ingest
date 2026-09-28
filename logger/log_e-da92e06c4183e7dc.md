# Advertising validation reported a send failure

Product: Nimble Streamer
Component: Advertising license validation
Logging function: log_e
Log level: error
Documentation ID: log_e-da92e06c4183e7dc

## Log pattern

```text
Advertising validation procedure failed to send request
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Advertising validation procedure failed to send request
```

## Short description

Advertising validation reported a send failure.

## Trigger condition

The license-validation request reports a send failure. The currently used TLS request helper does not produce this particular status.

## Explanation

This license-validation attempt did not complete. Check the license-service connection and the associated transport diagnostics.

## Possible causes

- The license-validation request reports a send failure. The currently used TLS request helper does not produce this particular status.

## What to check

1. No ordinary current TLS trigger is established; check surrounding validation diagnostics if encountered.

## Recommended action

No ordinary current TLS trigger is established; check surrounding validation diagnostics if encountered.
