# H.265 video parameter sets could not be separated

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-e367df20c234aa7f

## Log pattern

```text
H265: vps processing failed
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
H265: vps processing failed
```

## Short description

H.265 video parameter sets could not be separated.

## Trigger condition

The H.265 video parameter sets input is empty or contains more than 16 separated parameter sets.

## Explanation

H.265 setup stops before the codec configuration is installed. Unseparated nonempty data is accepted as one parameter set; this message does not establish a particular bitstream syntax error.

## Possible causes

- The H.265 video parameter sets input is empty or contains more than 16 separated parameter sets.

## What to check

1. Check that the source supplies nonempty H.265 video parameter sets within the supported count of 16.

## Recommended action

Correct the parameter-set data supplied by the RTSP source.
