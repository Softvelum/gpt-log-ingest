# AAC LATM stream-multiplex configuration was rejected

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-f1c074fd0e3f3a4c

## Log pattern

```text
mp4a-latm: failed to parse latm config
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
mp4a-latm: failed to parse latm config
```

## Short description

AAC LATM stream-multiplex configuration was rejected.

## Trigger condition

Decoding the SDP LATM stream-multiplex configuration returns failure.

## Explanation

LATM initialization stops before extracting an AAC audio configuration or registering the codec.

## Possible causes

- Decoding the SDP LATM stream-multiplex configuration returns failure.

## What to check

1. Check the SDP LATM configuration and any preceding LATM parser messages.

## Related log messages

- [AAC configuration could not be extracted from LATM](log_e-3c8156e1b50dc620.md)
