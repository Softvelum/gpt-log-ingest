# External request connection is in progress

Product: Nimble Streamer
Component: External HTTP requests
Logging function: log_v
Log level: verbose
Documentation ID: log_v-2644c1dfcf70eaeb

## Log pattern

```text
connecting to peer s=%d '%s:%d'
```

## Parameters

- Argument 1 — socket: Peer socket descriptor.
- Argument 2 — host: Resolved target hostname text.
- Argument 3 — port: Target TCP port.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connecting to peer s=12 'media.example:443'
```

## Short description

External request connection is in progress.

## Trigger condition

The nonblocking peer connect reports an in-progress operation.

## Explanation

TLS initialization, when requested, and event registration follow. This does not confirm the connection succeeded.
