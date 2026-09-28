# Stream-check connection is in progress

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_v
Log level: verbose
Documentation ID: log_v-55f52afc7f0effbd

## Log pattern

```text
connecting to s=%d '%s:%d' task='%s'
```

## Parameters

- Argument 1 — socket: Socket descriptor for the stream-check connection.
- Argument 2 — host: Origin host name from the task URL.
- Argument 3 — port: Origin TCP port.
- Argument 4 — task: Panel-supplied stream-check task identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connecting to s=12 'media.example:443' task='check-a'
```

## Short description

Stream-check connection is in progress.

## Trigger condition

The nonblocking TCP connection attempt reports that connection establishment is in progress.

## Explanation

The task prepares its TLS state and waits for socket events. The same message covers the initial and additional-address attempts.
