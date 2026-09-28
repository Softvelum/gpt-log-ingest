# Accepted connection TLS session creation failed

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_e
Log level: error
Documentation ID: log_e-f1ebccdc5b470bf7

## Log pattern

```text
ssl_ == NULL
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
ssl_ == NULL
```

## Short description

Accepted connection TLS session creation failed.

## Trigger condition

Creating a TLS session from the default server TLS context returns no session.

## Explanation

TLS resources are released and accepted-connection TLS initialization returns failure.

## Possible causes

- The TLS library did not create a session.

## What to check

1. Check the TLS context and preceding TLS initialization diagnostics.
