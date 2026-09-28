# NDI output sender creation failed

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-06ee6420c43d6dbe

## Log pattern

```text
failed to create ndi sender, name=%s
```

## Parameters

- Argument 1 — Requested name: NDI output name passed to sender creation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create ndi sender, name=Output
```

## Short description

NDI output sender creation failed.

## Trigger condition

The NDI runtime returns no sender for the requested output name.

## Explanation

Normal, buffered-rate, and SEI-timecode output modes all stop initialization at this point, before their respective media paths are started.

## Possible causes

- The NDI runtime returns no sender for the requested output name.

## What to check

1. Check whether the NDI runtime can create a sender with the requested name.

## Related log messages

- [NDI output sender created](log_i-e6b7fdf69261bbcb.md)
- [Buffered NDI sender is starting its worker threads](log_i-f630d70b2c3afadd.md)
