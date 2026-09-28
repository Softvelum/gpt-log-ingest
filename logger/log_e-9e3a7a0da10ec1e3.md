# WHEP control send count inconsistent

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-9e3a7a0da10ec1e3

## Log pattern

```text
whep control socket sent more that requested. impossible
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
whep control socket sent more that requested. impossible
```

## Short description

WHEP control send count inconsistent.

## Trigger condition

Sending buffered control data reports more bytes written than were available in the buffer.

## Explanation

The helper control socket and pending output are cleared, pending starts are marked finished, and unclassified accepted sockets are removed. Playback sockets already assigned to media workers are not directly closed by this cleanup.

## Possible causes

- The socket send result violates its length contract.

## What to check

1. Capture the surrounding helper/socket diagnostics for the inconsistent send count.

## Recommended action

Investigate a repeated occurrence as an internal transport defect.
