# Transcoding library loading raised an exception

Product: Nimble Streamer
Component: Transcoding library loading
Logging function: log_d
Log level: debug
Documentation ID: log_d-0b86b02cadea6bdd

## Log pattern

```text
LoadTranscodingLibraries exception what()='%s'
```

## Parameters

- Argument 1 — Exception: Exception diagnostic text describing the loading failure.

The same message covers the different library groups listed above; the text identifies the particular loading exception.

## Log examples

Illustrative examples; these are not captured server logs.

```text
LoadTranscodingLibraries exception what()='cannot load library'
```

## Short description

Transcoding library loading raised an exception.

## Trigger condition

A library load or required symbol lookup throws an exception while loading transcoding support.

## Explanation

The affected library group is closed. Failure of the core FFmpeg group aborts transcoding-library loading; failures of device support, VAAPI, Quick Sync, NVIDIA encoding or decoding, NVIDIA management, or FDK audio leave the corresponding optional feature unavailable. The x264 occurrence fails the separate x264 loading request.

## Possible causes

- A requested shared library or required symbol is unavailable, or NVIDIA encoder API creation fails.

## What to check

1. Use the exception text to identify the library or missing capability; check installed library versions and loader access.

## Recommended action

Restore compatible libraries for the requested transcoding feature.

## Related log messages

- [NVIDIA encoder API creation failed](log_e-2a16f5091eb13594.md)
