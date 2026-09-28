# Transport restreaming buffer is created

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-13ba1bb26e6c6ec6

## Log pattern

```text
restreaming buffer added for cam=%s
```

## Parameters

- Argument 1 — Camera ID: Transport input camera identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
restreaming buffer added for cam=camera1
```

## Short description

Transport restreaming buffer is created.

## Trigger condition

A restreaming buffer is constructed for the named camera.

## Explanation

The buffer object exists, but packet retention starts only when restreaming is activated.

## Possible causes

- A restreaming buffer is constructed for the named camera.

## What to check

1. Correlate with a later restreaming-start message.

## Recommended action

No action is needed for expected buffer creation.
