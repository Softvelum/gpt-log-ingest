# Cannot start constant-bitrate SRT sender

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-45d9df849aec97a4

## Log pattern

```text
failed to start srt cbr sender, s=%d
```

## Parameters

- Argument 1 — Socket: SRT socket identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to start srt cbr sender, s=101
```

## Short description

Cannot start constant-bitrate SRT sender.

## Trigger condition

An accepted sender cannot switch to its blocking worker mode or start its base sending workers.

## Explanation

The sender is removed from the tracked session set and selector dispatch map. This path is available on non-Windows builds.

## Possible causes

- An accepted sender cannot switch to its blocking worker mode or start its base sending workers.

## What to check

1. Inspect preceding SRT blocking-option and sender startup diagnostics.
