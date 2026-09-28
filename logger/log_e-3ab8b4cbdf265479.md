# AES-128 VOD video chunk protection failed

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-3ab8b4cbdf265479

## Log pattern

```text
failed to encrypt aes128 VOD video fmp4 segment for [%s/%s]
```

## Parameters

- Argument 1 — Application: Text naming the VOD application.
- Argument 2 — Stream: Text naming the protected media stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to encrypt aes128 VOD video fmp4 segment for [vod/movie.mp4]
```

## Short description

AES-128 VOD video chunk protection failed.

## Trigger condition

The non-CENC protection path returns failure while preparing a generated fragmented-MP4 video chunk for HLS delivery.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- The non-CENC protection path returns failure while preparing a generated fragmented-MP4 video chunk for HLS delivery.

## What to check

1. Check preceding AES key, license, and encryption errors and available encryption-buffer capacity. This path can fail before encryption begins when the required key is unavailable.

## Recommended action

Check preceding AES key, license, and encryption errors and available encryption-buffer capacity. This path can fail before encryption begins when the required key is unavailable.
