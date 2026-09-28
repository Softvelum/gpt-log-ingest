# VAAPI supported rate controls reported

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-4c1a1dbb68df2b41

## Log pattern

```text
supported rc_methods: %s
```

## Parameters

- Argument 1 — Modes: Space-separated supported mode names, each followed by a space; text. An empty string is possible if no recognized bits are present.

## Log examples

Illustrative examples; these are not captured server logs.

```text
supported rc_methods: cbr vbr cqp 
```

## Short description

VAAPI supported rate controls reported.

## Trigger condition

The rate-control capability query succeeds.

## Explanation

The recognized supported modes are listed; selecting and creating an encoder configuration follows.
