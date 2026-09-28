# Cannot connect to panel endpoint

Product: Nimble Streamer
Component: Stream checking and panel uploads
Logging function: log_e
Log level: error
Documentation ID: log_e-dd2559aadf0b60de

## Log pattern

```text
failed to connect to %s:%d
```

## Parameters

- Argument 1 — host: Selected panel hostname or address.
- Argument 2 — port: Destination TCP port.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect to panel.example:443
```

## Short description

Cannot connect to panel endpoint.

## Trigger condition

The panel socket helper returns a negative descriptor when a stream checker contacts the panel, or when requested log or DVR timeline upload attempts a connection.

## Explanation

Stream-checker loops retain their normal interval-based scheduling after the failed connection. The log and timeline helpers return without sending their prepared request; they do not retry inside these helpers.

## Possible causes

- The connection helper did not provide a usable socket.

## What to check

1. Check earlier connection diagnostics and the selected panel host, port, and applicable proxy settings.

## Recommended action

Restore access to the selected panel endpoint.
