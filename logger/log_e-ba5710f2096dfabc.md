# LATM payload length is incomplete

Product: Nimble Streamer
Component: AAC LATM parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-ba5710f2096dfabc

## Log pattern

```text
failed to process PayloadLengthInfo
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process PayloadLengthInfo
```

## Short description

LATM payload length is incomplete.

## Trigger condition

Reading the variable-length payload-size bytes exhausts the available mux-element data.

## Explanation

Audio mux processing fails before extracting this subframe.

## Possible causes

- Reading the variable-length payload-size bytes exhausts the available mux-element data.

## What to check

1. Check whether the LATM element is truncated.
