# Pay-per-view report request prepared

Product: Nimble Streamer
Component: Pay-per-view API synchronization
Logging function: log_v
Log level: verbose
Documentation ID: log_v-4a6d663deadd6894

## Log pattern

```text
prepared PPV Api request:
%s
```

## Parameters

- Argument 1 — request: Prepared HTTP headers and report body.

## Log examples

An example is omitted to avoid publishing an authentication value.

## Short description

Pay-per-view report request prepared.

## Trigger condition

A connection is available and active-session information has been encoded into an API request.

## Explanation

The complete HTTP request is logged before transmission. JSON or XML body format follows the API setting; its challenge is later used for optional response authentication.
