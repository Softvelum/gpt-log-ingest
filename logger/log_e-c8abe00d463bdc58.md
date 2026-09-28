# DVB subtitle palette is truncated

Product: Nimble Streamer
Component: DVB subtitle text recognition
Logging function: log_e
Log level: error
Documentation ID: log_e-c8abe00d463bdc58

## Log pattern

```text
wrong color plane size
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong color plane size
```

## Short description

DVB subtitle palette is truncated.

## Trigger condition

The declared color count times four bytes exceeds the remaining bytes after the bitmap plane.

## Explanation

The subtitle message is rejected before text recognition.

## Possible causes

- The declared color count times four bytes exceeds the remaining bytes after the bitmap plane.

## What to check

1. Check the declared palette color count and transmitted palette bytes.
