# VAAPI encoder object created

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-711a4c1ad144cf8f

## Log pattern

```text
VAAPIEncoder::VAAPIEncoder
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
VAAPIEncoder::VAAPIEncoder
```

## Short description

VAAPI encoder object created.

## Trigger condition

A native VAAPI encoder object is being constructed.

## Explanation

Defaults are initialized and surface identifiers are marked invalid. Hardware initialization waits for the first frame.
