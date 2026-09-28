# Unsupported transcription URL protocol

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-d7849aba7245abf4

## Log pattern

```text
unsupported proto in url=%s
```

## Parameters

- Argument 1 — URL: Configured service URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported proto in url=https://speech.example.net/
```

## Short description

Unsupported transcription URL protocol.

## Trigger condition

The service URL protocol is neither ws nor wss.

## Explanation

Connection setup fails before a websocket connection is created.

## Possible causes

- The service URL protocol is neither ws nor wss.

## What to check

1. Check the configured transcription service URL scheme.

## Recommended action

Use a ws or wss endpoint supported by the selected service.
