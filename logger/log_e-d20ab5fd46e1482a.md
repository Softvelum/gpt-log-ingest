# DRM initialization box content is invalid

Product: Nimble Streamer
Component: Caption and subtitle handling
Logging function: log_e
Log level: error
Documentation ID: log_e-d20ab5fd46e1482a

## Log pattern

```text
invalid pssh atom content
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid pssh atom content
```

## Short description

DRM initialization box content is invalid.

## Trigger condition

Parsing the supplied protection-system-specific-header data throws a media-box parsing exception.

## Explanation

Validation fails and the supplied bytes are not copied into the output container.

## Possible causes

- Parsing the supplied protection-system-specific-header data throws a media-box parsing exception.

## What to check

1. Check the DRM data length and box structure.
