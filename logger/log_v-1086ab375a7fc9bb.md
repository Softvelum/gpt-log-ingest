# Registering a live stream

Product: Nimble Streamer
Component: Live stream registration and transcription
Logging function: log_v
Log level: verbose
Documentation ID: log_v-1086ab375a7fc9bb

## Log pattern

```text
LiveStreamManager::registerStream name='%s' proto=%d
```

## Parameters

- Argument 1 — stream: Application and stream name joined by a slash.
- Argument 2 — protocol: Input protocol number: 0 RTMP, 1 RTSP, 2 Icecast, 3 MPEG-TS, 4 encoder, 5 NDI, 6 playlist, 7 WebRTC, 8 Dante, 9 OMT, 10 SDI, 11 AJA, 12 none.

## Log examples

Illustrative examples; these are not captured server logs.

```text
LiveStreamManager::registerStream name='live/camera' proto=0
```

## Short description

Registering a live stream.

## Trigger condition

Registration passed the duplicate-name checks. In the main server this form is used when neither publisher address nor source URL is supplied; the scenario encoder also uses it.

## Explanation

The stream object is constructed and entered in name and identity tables after this trace. In the scenario encoder, any pending unregistration for that name is cleared.
