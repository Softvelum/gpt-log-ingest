# DVR cleanup time argument cannot be converted

Product: Nimble Streamer
Component: DVR
Logging function: log_d
Log level: debug
Documentation ID: log_d-811db21d5ca9c4cb

## Log pattern

```text
e.what()='%s'
```

## Parameters

- Argument 1 — Conversion error: Text describing the numeric conversion exception.

## Log examples

Illustrative examples; these are not captured server logs.

```text
e.what()='bad lexical cast'
```

## Short description

DVR cleanup time argument cannot be converted.

## Trigger condition

A cleanup depth or UTC boundary cannot be converted to an unsigned 64-bit number.

## Explanation

An HTTP 200 response reports a failed operation with an invalid-argument message.

## Possible causes

- A cleanup depth or UTC boundary cannot be converted to an unsigned 64-bit number.

## What to check

1. Supply nonnegative integer cleanup depth and UTC boundary values.

## Recommended action

Supply nonnegative integer cleanup depth and UTC boundary values.
