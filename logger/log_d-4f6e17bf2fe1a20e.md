# VAAPI target bitrate selected

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-4f6e17bf2fe1a20e

## Log pattern

```text
bitrate=%d

```

## Parameters

- Argument 1 — Bitrate: Selected target bitrate, integer bits per second; zero means no positive bitrate was supplied.

## Log examples

Illustrative examples; these are not captured server logs.

```text
bitrate=4000000

```

## Short description

VAAPI target bitrate selected.

## Trigger condition

Encoder setup reads the bitrate option, or its b alias, and converts it from kilobits per second to bits per second.

## Explanation

The selected bitrate is reported before rate-control configuration.
