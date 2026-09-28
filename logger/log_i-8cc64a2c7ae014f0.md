# NVIDIA free-context cache release started

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-8cc64a2c7ae014f0

## Log pattern

```text
release nvenc context cache...
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
release nvenc context cache...
```

## Short description

NVIDIA free-context cache release started.

## Trigger condition

Manager release finds free-context cache entries.

## Explanation

Each cached context will be passed to CUDA destruction and removed from the cache.
