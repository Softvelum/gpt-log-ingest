# Downloader socket event received

Product: Nimble Streamer
Component: Background origin download
Logging function: log_d
Log level: debug
Documentation ID: log_d-0861c8035fdfffaa

## Log pattern

```text
[DwnS::PE] p=%p (s=%d ev=0x%x) ssl=%d input=%d
```

## Parameters

- Argument 1 — address: Downloader session memory address.
- Argument 2 — socket: Origin socket descriptor.
- Argument 3 — events: Event flags in hexadecimal.
- Argument 4 — TLS: 1 when the connection uses TLS, otherwise 0.
- Argument 5 — input: 1 for logical input, otherwise 0.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[DwnS::PE] p=0x1000 (s=12 ev=0x1) ssl=1 input=1
```

## Short description

Downloader socket event received.

## Trigger condition

A socket event is dispatched to the background downloader.

## Explanation

The activity time is refreshed. TLS events follow the logical input/output direction; plain socket events dispatch by their flags.
