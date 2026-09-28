# WHEP session request exceeds control buffer

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-b0427cdd702ea1fb

## Log pattern

```text
too big sdp for whep session=%s/%s
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
too big sdp for whep session=live/camera
```

## Short description

WHEP session request exceeds control buffer.

## Trigger condition

Writing the length-prefixed WHEP session-start JSON into the 65,536-byte control buffer fails. The request contains base64 SDP, selected codecs, and helper parameters.

## Explanation

The current session is marked finished without a successful helper start request.

## Possible causes

- The complete encoded start request exceeds the available buffer.

## What to check

1. Check the SDP and encoded helper-request size.

## Recommended action

Reduce oversized negotiation data or correct unexpectedly large session parameters.
