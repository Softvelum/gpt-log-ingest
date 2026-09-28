# Transcoder license is incomplete

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-ab58552b47aab7ac

## Log pattern

```text
Transcoder license is invalid. license, expire or signature fields empty
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Transcoder license is invalid. license, expire or signature fields empty
```

## Short description

Transcoder license is incomplete.

## Trigger condition

The stored Transcoder license has an empty license key, expiration text, or signature.

## Explanation

Local signature validation returns failure, so the license is not accepted as valid.

## Possible causes

- At least one required license field is missing.

## What to check

1. Check that registration produced a complete license for this server.

## Recommended action

Register the Transcoder license again to restore complete license data.
