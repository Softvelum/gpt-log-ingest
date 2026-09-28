# No automatic audio stream is available

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-e8bebb91a7161bc1

## Log pattern

```text
updateAvailable default audio stream doesn't exists
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable default audio stream doesn't exists
```

## Short description

No automatic audio stream is available.

## Trigger condition

No explicit or active audio packet is selected and the effective default identifier is zero.

## Explanation

Audio codec initialization, timestamps, AAC header state, and the audio frame buffer are reset. The selected audio identifier and type remain cleared. This can also follow an out-of-range configured media index.
