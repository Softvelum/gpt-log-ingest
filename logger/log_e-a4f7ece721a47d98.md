# Advertising fragmented MP4 generation failed

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-a4f7ece721a47d98

## Log pattern

```text
DashTransmuxingException caught while processing content id='%s' msg='%s'
```

## Parameters

- Argument 1 — Content ID: Text identifying the advertising content.
- Argument 2 — Reason: Text describing the reported error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DashTransmuxingException caught while processing content id='ad-01' msg='invalid media data'
```

## Short description

Advertising fragmented MP4 generation failed.

## Trigger condition

Fragmented MP4 advertising media or initialization generation raises a packaging exception.

## Explanation

The reason is logged and the request returns; the pending cache item is canceled during cleanup.

## Possible causes

- Fragmented MP4 advertising media or initialization generation raises a packaging exception.

## What to check

1. Use the reported reason to inspect the advertisement tracks and media structure.

## Recommended action

Use the reported reason to inspect the advertisement tracks and media structure.
