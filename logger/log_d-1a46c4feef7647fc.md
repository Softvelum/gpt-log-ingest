# HTTP/2 live advertising chunk request started

Product: Nimble Streamer
Component: HTTP/2 and HTTP/3 request handling
Logging function: log_d
Log level: debug
Documentation ID: log_d-1a46c4feef7647fc

## Log pattern

```text
[H2C::PLACR] id=%lu a_url='%s'
```

## Parameters

- Argument 1 — Request stream identifier: HTTP request stream identifier, unsigned 64-bit integer displayed in decimal; no unit.
- Argument 2 — Aliased URL: Aliased request URL retained for this request, text; can be empty when no alias applies.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[H2C::PLACR] id=4 a_url='/public/camera/playlist.m3u8'
```

## Short description

HTTP/2 live advertising chunk request started.

## Trigger condition

A routed live advertising chunk request enters processing, after decoding the origin path.

## Explanation

The request path is split, existing advertising tracking information is looked up, and chunk delivery is attempted. This entry message does not establish that the chunk exists or is sent.
