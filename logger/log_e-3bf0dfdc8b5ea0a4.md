# VAAPI hardware display unavailable

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-3bf0dfdc8b5ea0a4

## Log pattern

```text
no hw found
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
no hw found
```

## Short description

VAAPI hardware display unavailable.

## Trigger condition

Opening and initializing a VA display fails during native encoder initialization.

## Explanation

Encoder initialization returns failure before selecting a profile or allocating surfaces.

## Possible causes

- No suitable accessible render device or usable VA driver can be initialized.

## What to check

1. Check preceding VA device diagnostics and render-device access.

## Recommended action

Restore a compatible accessible VA device and restart the encoder.
