# Transport input searches for a new packet boundary

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_d
Log level: debug
Documentation ID: log_d-8aecf2fd85f6d96e

## Log pattern

```text
camera stream reset s=%d
```

## Parameters

- Argument 1 — Socket: Numeric socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
camera stream reset s=42
```

## Short description

Transport input searches for a new packet boundary.

## Trigger condition

A complete 188-byte input block is rejected by transport parsing.

## Explanation

The receiver searches bytes 1 through 187 for the next 0x47 sync byte, retains that suffix if found, or clears the partial block otherwise. Input processing continues.

## Possible causes

- A complete 188-byte input block is rejected by transport parsing.

## What to check

1. Check packet alignment and preceding transport parser errors.

## Recommended action

Correct malformed input framing or repeated packet corruption.
