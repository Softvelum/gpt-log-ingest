# DVR fragmented thumbnail parsing failed

Product: Nimble Streamer
Component: DVR
Logging function: log_e
Log level: error
Documentation ID: log_e-7d3f0ebf4f51bf8f

## Log pattern

```text
failed to create fmp4 thumbnail segment
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create fmp4 thumbnail segment
```

## Short description

DVR fragmented thumbnail parsing failed.

## Trigger condition

An MP4 parsing exception occurs while creating a fragment containing the first recorded video sample.

## Explanation

Fragmented thumbnail creation returns failure.

## Possible causes

- An MP4 parsing exception occurs while creating a fragment containing the first recorded video sample.

## What to check

1. Check the recorded media fragment structure.

## Recommended action

Check the recorded media fragment structure.
