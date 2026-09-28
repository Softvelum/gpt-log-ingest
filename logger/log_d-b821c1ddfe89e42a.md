# No automatic video stream is available

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-b821c1ddfe89e42a

## Log pattern

```text
updateAvailable default video stream doesn't exists
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable default video stream doesn't exists
```

## Short description

No automatic video stream is available.

## Trigger condition

No explicit or active video packet is selected and the effective default identifier is zero.

## Explanation

Video codec initialization, timestamps, parameter sets, and frame assembly buffers are reset. The selected video identifier and type remain cleared. This can also follow an out-of-range configured media index.
