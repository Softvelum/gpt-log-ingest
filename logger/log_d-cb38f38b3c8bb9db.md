# subtitle packet automatically selected

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-cb38f38b3c8bb9db

## Log pattern

```text
updateAvailable subtitle type implicitly set found pid=%d
```

## Parameters

- Argument 1 — packet\_id: MPEG-TS packet identifier printed as a decimal integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable subtitle type implicitly set found pid=300
```

## Short description

subtitle packet automatically selected.

## Trigger condition

DVB subtitle processing is enabled and the active subtitle identifier is zero while the input default is nonzero.

## Explanation

The default identifier becomes active and is added to the set of packets requested from the input.
