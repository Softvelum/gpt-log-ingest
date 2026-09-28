# Common-encryption initialization failed

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-dc6ef6e153601da9

## Log pattern

```text
encryptCencChunk: failed to add cenc init segment for [%s/%s], response cenc_required: %d
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Encryption required: Integer boolean showing whether CENC was required: 0 false, 1 true.

## Log examples

Illustrative examples; these are not captured server logs.

```text
encryptCencChunk: failed to add cenc init segment for [live/camera], response cenc_required: 1
```

## Short description

Common-encryption initialization failed.

## Trigger condition

Encryption initialization reports failure or does not mark CENC required for an initialization segment.

## Explanation

The segment encryption operation returns failure.

## Possible causes

- Encryption initialization reports failure or does not mark CENC required for an initialization segment.

## What to check

1. Check preceding key-availability, license, and initialization errors.

## Recommended action

Check preceding key-availability, license, and initialization errors.
