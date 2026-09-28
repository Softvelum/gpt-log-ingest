# No default SCTE-35 packet is available

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-c79ef1f30a38bb3a

## Log pattern

```text
updateAvailable default scte35 stream doesn't exists
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable default scte35 stream doesn't exists
```

## Short description

No default SCTE-35 packet is available.

## Trigger condition

SCTE-35 processing is enabled and both the active and default SCTE-35 identifiers are zero.

## Explanation

The active identifier remains zero. The availability update continues without selecting this auxiliary stream.
