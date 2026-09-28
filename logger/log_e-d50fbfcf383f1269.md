# LATM AAC configuration could not be installed

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-d50fbfcf383f1269

## Log pattern

```text
mp4a-latm: unable to set codec params
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
mp4a-latm: unable to set codec params
```

## Short description

LATM AAC configuration could not be installed.

## Trigger condition

The live-stream AAC codec operation rejects the AAC configuration extracted from LATM.

## Explanation

LATM initialization returns failure before selecting the RTP audio stream type.

## Possible causes

- The live-stream AAC codec operation rejects the AAC configuration extracted from LATM.

## What to check

1. Check the extracted AAC configuration and accompanying live-stream codec errors.

## Related log messages

- [AAC configuration could not be extracted from LATM](log_e-3c8156e1b50dc620.md)
