# AV1 transport header cannot allocate its initial delimiter

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_e
Log level: error
Documentation ID: log_e-cbee6be517ff2266

## Log pattern

```text
[VM] cannot allocate memory for mpeg2ts_av1_header [%s/%s]
```

## Parameters

- Argument 1 — Application: Application containing the affected live stream.
- Argument 2 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] cannot allocate memory for mpeg2ts_av1_header [live/camera]
```

## Short description

AV1 transport header cannot allocate its initial delimiter.

## Trigger condition

Writing the initial Annex B delimiter into the AV1 transport-header buffer fails.

## Explanation

AV1 setup returns failure before parsing decoder configuration. The initial write requires only four bytes of the 16 KiB buffer.

## Possible causes

- Writing the initial Annex B delimiter into the AV1 transport-header buffer fails.

## What to check

1. Check local memory allocation failures.

## Recommended action

Resolve the header-buffer allocation failure before retrying codec setup.
