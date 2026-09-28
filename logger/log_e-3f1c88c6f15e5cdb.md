# RTMP advertising cannot parse SCTE-35 section

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-3f1c88c6f15e5cdb

## Log pattern

```text
[SCTE35] failed to read splice info section, client: %d
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SCTE35] failed to read splice info section, client: 42
```

## Short description

RTMP advertising cannot parse SCTE-35 section.

## Trigger condition

An SCTE-35 buffer item does not contain a readable splice-info section header.

## Explanation

Advertising event processing for this item stops. The playback sender continues without scheduling an advertisement from this failed section.

## Possible causes

- An SCTE-35 buffer item does not contain a readable splice-info section header.

## What to check

1. Inspect the SCTE-35 section bytes and earlier metadata-input errors.

## Recommended action

Correct malformed or incomplete SCTE-35 input.
