# Shared NVIDIA context lock destruction was issued

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-1fd578bb0afd0acb

## Log pattern

```text
destroyed shared lock=%p
```

## Parameters

- Argument 1 — Lock: NVIDIA video-context lock handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
destroyed shared lock=0x12340000
```

## Short description

Shared NVIDIA context lock destruction was issued.

## Trigger condition

Manager release encounters a shared context with a lock and calls lock destruction.

## Explanation

The shared context is destroyed next; the lock-destruction result is not checked.
