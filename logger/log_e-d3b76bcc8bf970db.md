# External request startup raised an exception

Product: Nimble Streamer
Component: External HTTP requests
Logging function: log_e
Log level: error
Documentation ID: log_e-d3b76bcc8bf970db

## Log pattern

```text
ConnectToPeer exception url='%s' request failed
```

## Parameters

- Argument 1 — URL: Original external request URL, retained even when following a redirect.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ConnectToPeer exception url='https://media.example/movie.mp4' request failed
```

## Short description

External request startup raised an exception.

## Trigger condition

URL parsing, connection setup, or event registration raises an exception before request startup completes.

## Explanation

The response buffer is freed, the requester receives a failure callback, the socket is removed and closed when present, and the exception is rethrown. The message name does not limit the failure to connecting.

## Possible causes

- An exception interrupted request startup.

## What to check

1. Check the preceding URL, connection, TLS, or event-registration error.

## Recommended action

Correct the earlier startup failure.
