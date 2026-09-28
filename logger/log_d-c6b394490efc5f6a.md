# Audio filter worker stopped

Product: Nimble Streamer
Component: Audio filtering worker
Logging function: log_d
Log level: debug
Documentation ID: log_d-c6b394490efc5f6a

## Log pattern

```text
AudioFilterThread(%s) stopped
```

## Parameters

- Argument 1 — Filter identifier: Configured filter identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
AudioFilterThread(audio-main) stopped
```

## Short description

Audio filter worker stopped.

## Trigger condition

Filter creation fails or the processing loop ends because of a stop request or a frame-processing failure.

## Explanation

The audio filter worker returns; preceding diagnostics distinguish normal stopping from filtering failure.

## Related log messages

- [Audio filter worker started](log_d-4f8a527afbb3a7c8.md)
- [Audio filtering fell behind its input](log_e-0dc1ac5a05f53fe4.md)
- [Audio filter input write failed](log_e-1345d03f0e6be3c5.md)
