# SLDP sender cannot enter a supported wait state

Product: Nimble Streamer
Component: HTTP/2 and HTTP/3 request handling
Logging function: log_e
Log level: error
Documentation ID: log_e-fb85ea266143ce9a

## Log pattern

```text
unknown sldp state for http2/http3 %d
```

## Parameters

- Argument 1 — Sender state: Internal live-sender state code, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown sldp state for http2/http3 0
```

## Short description

SLDP sender cannot enter a supported wait state.

## Trigger condition

After filling an HTTP/2 or HTTP/3 SLDP output buffer, the sender is active but has no output enabled and its state is not one of the supported waits for play, media, advertising content, or advertising rules.

## Explanation

The reply is marked for an internal stream reset instead of being placed on a waiting list.

## Possible causes

- The live sender reached a state that the HTTP/2 and HTTP/3 reply code does not handle at this point.

## What to check

1. Correlate the numeric sender state with preceding playback or advertising transitions.

## Recommended action

Investigate repeated unsupported sender transitions; reconnect affected playback.
