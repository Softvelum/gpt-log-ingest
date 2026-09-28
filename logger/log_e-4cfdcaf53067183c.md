# DVB subtitle message has no metadata terminator

Product: Nimble Streamer
Component: DVB subtitle text recognition
Logging function: log_e
Log level: error
Documentation ID: log_e-4cfdcaf53067183c

## Log pattern

```text
wrong dvb sub message size
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong dvb sub message size
```

## Short description

DVB subtitle message has no metadata terminator.

## Trigger condition

The received message contains no terminating zero byte for its leading JSON metadata within the message length.

## Explanation

The subtitle message is rejected before rectangle data or OCR is processed.

## Possible causes

- The received message contains no terminating zero byte for its leading JSON metadata within the message length.

## What to check

1. Check the producer of the internal DVB subtitle message and its message framing.
