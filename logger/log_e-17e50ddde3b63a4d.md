# Cannot create SRT bonding group

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-17e50ddde3b63a4d

## Log pattern

```text
srt group creation failed (errno=%d)
```

## Parameters

- Argument 1 — Error: Operating-system socket error number at the failed group creation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
srt group creation failed (errno=12)
```

## Short description

Cannot create SRT bonding group.

## Trigger condition

SRT group creation fails for a configured broadcast or backup group.

## Explanation

No usable group socket is returned. The printed error comes from the operating-system socket error accessor, rather than the SRT error-description accessor.

## Possible causes

- SRT group creation fails for a configured broadcast or backup group.

## What to check

1. Check bonding availability and nearby library diagnostics.
