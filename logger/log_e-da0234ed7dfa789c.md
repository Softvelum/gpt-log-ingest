# RTMP authentication method is unsupported

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-da0234ed7dfa789c

## Log pattern

```text
not supported auth method [%s]
```

## Parameters

- Argument 1 — Authentication method: authmod query value supplied by the publishing peer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
not supported auth method [custom]
```

## Short description

RTMP authentication method is unsupported.

## Trigger condition

An application requiring publishing credentials receives a nonempty authentication method other than adobe.

## Explanation

Connection authorization rejects this method and fails. An empty method follows a separate credential or challenge path.

## Possible causes

- An application requiring publishing credentials receives a nonempty authentication method other than adobe.

## What to check

1. Check the authmod value sent by the encoder and the application publishing credentials.

## Recommended action

Use a supported authentication exchange for the application.
