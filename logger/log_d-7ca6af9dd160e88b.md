# DRM keys allow postponed VOD packaging to resume

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_d
Log level: debug
Documentation ID: log_d-7ca6af9dd160e88b

## Log pattern

```text
ready to process postponed request for '%s/%s' chunk='%s' key='%s'
```

## Parameters

- Argument 1 — Application: Text naming the VOD application.
- Argument 2 — Stream: Text naming the protected media stream.
- Argument 3 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.
- Argument 4 — Output cache key: Text cache key reserved for the requested generated output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ready to process postponed request for 'vod/movie.mp4' chunk='n_0_0_0.ts' key='/vod/movie.mp4/n_0_0_0.ts'
```

## Short description

DRM keys allow postponed VOD packaging to resume.

## Trigger condition

A successful DRM-key notification matches a pending VOD output request.

## Explanation

The request is put on the ready queue and removed from the DRM wait group. Packaging runs again and can still encounter later media errors.

## Possible causes

- A successful DRM-key notification matches a pending VOD output request.

## What to check

1. Not applicable; this is normal processing progress. Follow the same source and output cache key if troubleshooting.

## Recommended action

Not applicable; this is normal processing progress. Follow the same source and output cache key if troubleshooting.
