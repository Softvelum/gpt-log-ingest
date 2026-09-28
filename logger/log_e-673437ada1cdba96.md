# Teletext text header is incomplete

Product: Nimble Streamer
Component: Caption and subtitle handling
Logging function: log_e
Log level: error
Documentation ID: log_e-673437ada1cdba96

## Log pattern

```text
failed to parse (8 comma) teletext='%s'
```

## Parameters

- Argument 1 — Text: Remaining subtitle text at the point a required comma cannot be found.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse (8 comma) teletext='Hello'
```

## Short description

Teletext text header is incomplete.

## Trigger condition

The decoded subtitle text has fewer than the eight comma-separated header fields expected for stripping its header.

## Explanation

Header stripping stops, but remaining text is still processed for line breaks and returned. The log does not itself discard the subtitle.

## Possible causes

- The decoded subtitle text has fewer than the eight comma-separated header fields expected for stripping its header.

## What to check

1. Check the displayed partially stripped teletext text.
