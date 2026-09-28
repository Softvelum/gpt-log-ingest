# EIT section buffer cannot accept more payload

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-d97db95d8bd5d78a

## Log pattern

```text
EITProcessor: cannot allocate appropriate buffer
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
EITProcessor: cannot allocate appropriate buffer
```

## Short description

EIT section buffer cannot accept more payload.

## Trigger condition

Appending transport payload to the 4096-byte EIT buffer fails.

## Explanation

The packet is rejected and transport input searches for another packet boundary. The EIT reset operation does not clear its partial section, so this line alone does not establish successful recovery.

## Possible causes

- Appending transport payload to the 4096-byte EIT buffer fails.

## What to check

1. Check EIT section length, transport framing, and repeated allocation failures.

## Recommended action

Correct malformed or oversized EIT input and investigate persistent failed section assembly.
