# Adobe authentication challenge has no salt

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-5ca61a3c8c224b0f

## Log pattern

```text
server returned empty salt for '%s'
```

## Parameters

- Argument 1 — Stream URL: RTMP or RTMPS URL identifying the outbound stream connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
server returned empty salt for 'rtmp://destination.example:1935/live/camera'
```

## Short description

Adobe authentication challenge has no salt.

## Trigger condition

A destination Adobe authentication challenge for the expected user contains an empty salt value.

## Explanation

The incomplete challenge is discarded and final authentication is not prepared. The enclosing error-response path ends the current connection.

## Possible causes

- A destination Adobe authentication challenge for the expected user contains an empty salt value.

## What to check

1. Inspect the salt query value in the destination challenge description.

## Recommended action

Correct the incomplete Adobe authentication challenge at the destination.
