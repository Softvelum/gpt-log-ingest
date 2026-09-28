# RTMP authentication challenge command-object placeholder cannot be decoded

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-ead19e4fff961720

## Log pattern

```text
failed to read null object for setChallenge
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read null object for setChallenge
```

## Short description

RTMP authentication challenge command-object placeholder cannot be decoded.

## Trigger condition

The setChallenge command cannot supply its command-object placeholder as an AMF null value.

## Explanation

The command response is marked failed and connection processing stops before sending the calculated client login.

## Possible causes

- The setChallenge command cannot supply its command-object placeholder as an AMF null value.

## What to check

1. Inspect the command-object placeholder in the destination challenge and its AMF type.

## Recommended action

Correct the malformed challenge at the authentication endpoint.
