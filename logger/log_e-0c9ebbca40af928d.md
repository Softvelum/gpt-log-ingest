# External request connection completed with an error

Product: Nimble Streamer
Component: External HTTP requests
Logging function: log_e
Log level: error
Documentation ID: log_e-0c9ebbca40af928d

## Log pattern

```text
failed to connect to %s s=%d errno=%d url='%s'
```

## Parameters

- Argument 1 — peer: Peer role label.
- Argument 2 — socket: Peer socket descriptor.
- Argument 3 — error: Operating system socket error code.
- Argument 4 — URL: Current peer URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect to peer s=12 errno=111 url='https://media.example/movie.mp4'
```

## Short description

External request connection completed with an error.

## Trigger condition

The socket-error query after connection readiness reports a nonzero error.

## Explanation

The request session is marked finished before sending the HTTP request.

## Possible causes

- Asynchronous connection establishment failed.

## What to check

1. Interpret the error and verify the target service is reachable.

## Recommended action

Correct the target connection problem.
