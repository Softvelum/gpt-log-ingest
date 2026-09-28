# Scenario input buffer removed

Product: Nimble Streamer
Component: Scenario encoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-5f60bdaa9f26ab00

## Log pattern

```text
unregister input stream [%s/%s]
```

## Parameters

- Argument 1 — Application: Live-stream application name.
- Argument 2 — Stream: Live-stream name within its application.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unregister input stream [live/camera]
```

## Short description

Scenario input buffer removed.

## Trigger condition

An unregister request names an application/stream pair whose input buffer currently exists.

## Explanation

The buffer manager destroys that input stream buffer. Entries without a slash or without an existing buffer do not produce this message.

## Possible causes

- A valid unregister request removes an existing scenario input.

## Related log messages

- [Scenario input buffer creation requested](log_d-337c242a7c7d0c53.md)
