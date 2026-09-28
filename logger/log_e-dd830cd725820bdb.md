# Addenda license signature does not match

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-dd830cd725820bdb

## Log pattern

```text
Addenda license signature is invalid. Register Addenda license again
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Addenda license signature is invalid. Register Addenda license again
```

## Short description

Addenda license signature does not match.

## Trigger condition

Local license validation rejected the stored signature.

## Explanation

Local license validation fails.

## Possible causes

- The stored license data did not pass validation.

## What to check

1. Check that the installed license belongs to this server and registration is current.

## Recommended action

Register the Addenda license again.
