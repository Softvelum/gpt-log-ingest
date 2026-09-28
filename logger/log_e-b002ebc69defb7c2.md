# SRT session reporting requires Addenda

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-b002ebc69defb7c2

## Log pattern

```text
failed to report SRT session for '%s' (requires Addenda package license)
```

## Parameters

- Argument 1 — Stream: Requested reporting application/stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to report SRT session for 'live/news' (requires Addenda package license)
```

## Short description

SRT session reporting requires Addenda.

## Trigger condition

Session reporting is requested with a nonempty stream name while Addenda is unavailable.

## Explanation

Session registration returns failure. Calling output paths reject the receiver setup, so this is not merely a missing statistics update.

## Possible causes

- Session reporting is requested with a nonempty stream name while Addenda is unavailable.

## What to check

1. Check the Addenda license and requested reporting stream name.

## Recommended action

Enable the required package for this reporting configuration.
