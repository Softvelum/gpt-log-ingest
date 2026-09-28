# VAAPI rate control selected

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-7e3beb4bc92c09ec

## Log pattern

```text
rc_method=%d

```

## Parameters

- Argument 1 — Rate control: VAAPI rate-control value, integer bit flag value; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rc_method=2

```

## Short description

VAAPI rate control selected.

## Trigger condition

Encoder setup selects the rate_control option or defaults to constant bitrate for a positive bitrate and constant quantization otherwise.

## Explanation

The VA rate-control value is reported before configuration creation.
