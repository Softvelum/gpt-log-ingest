# Windows networking initialization failed

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-730d3eeb3e61b6a5

## Log pattern

```text
WSAStartup failed error=%d
```

## Parameters

- Argument 1 — Networking error: Integer result returned by Windows socket initialization.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WSAStartup failed error=10091
```

## Short description

Windows networking initialization failed.

## Trigger condition

Windows socket-library initialization returns a nonzero error in server startup or the registration utility.

## Explanation

The server or utility exits with a failure status before continuing network operations.

## Possible causes

- Windows socket initialization returned an error.

## What to check

1. Inspect the returned Windows networking error.
