# Transcoder manager stopped

Product: Nimble Streamer
Component: Transcoder orchestration
Logging function: log_d
Log level: debug
Documentation ID: log_d-a3cb4e0677ed7a86

## Log pattern

```text
TranscoderMainThread stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
TranscoderMainThread stopped
```

## Short description

Transcoder manager stopped.

## Trigger condition

The exit or stop request has ended the main loop, worker shutdown has run, and enabled device contexts have been released.

## Explanation

The main transcoder worker returns after cleanup.
