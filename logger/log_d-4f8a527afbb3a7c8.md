# Audio filter worker started

Product: Nimble Streamer
Component: Audio filtering worker
Logging function: log_d
Log level: debug
Documentation ID: log_d-4f8a527afbb3a7c8

## Log pattern

```text
AudioFilterThread(%s) started
```

## Parameters

- Argument 1 — Filter identifier: Configured filter identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
AudioFilterThread(audio-main) started
```

## Short description

Audio filter worker started.

## Trigger condition

The audio filter worker begins execution.

## Explanation

Filter creation follows, then it reads inputs and refreshes output connections periodically.

## Related log messages

- [Audio filter worker stopped](log_d-c6b394490efc5f6a.md)
