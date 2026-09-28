# RTMP authentication challenge nonce cannot be decoded

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-823fb7b3cd3bce83

## Log pattern

```text
failed to read nounce for setChallenge
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read nounce for setChallenge
```

## Short description

RTMP authentication challenge nonce cannot be decoded.

## Trigger condition

The setChallenge command cannot supply its nonce as an AMF string.

## Explanation

The command response is marked failed and connection processing stops before sending the calculated client login.

## Possible causes

- The setChallenge command cannot supply its nonce as an AMF string.

## What to check

1. Inspect the nonce in the destination challenge and its AMF type.

## Recommended action

Correct the malformed challenge at the authentication endpoint.
