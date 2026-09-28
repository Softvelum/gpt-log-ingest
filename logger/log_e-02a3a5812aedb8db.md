# Rules or playlist HTTP request could not be queued

Product: Nimble Streamer
Component: Advertising and server playlist synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-02a3a5812aedb8db

## Log pattern

```text
failed to POST session rules request to url='%s'
```

## Parameters

- Argument 1 — Request URL: Text giving the configured request destination.

The original message says POST for both uses; server playlist synchronization actually submits GET.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to POST session rules request to url='https://api.example/rules'
```

## Short description

Rules or playlist HTTP request could not be queued.

## Trigger condition

Submitting an advertising session rules POST request or a server playlist synchronization GET request fails.

## Explanation

Advertising requests receive failed-rule handling and the in-progress flag clears; server playlist synchronization returns failure.

## Possible causes

- Submitting an advertising session rules POST request or a server playlist synchronization GET request fails.

## What to check

1. Check the configured destination and request-processing service.

## Recommended action

Check the configured destination and request-processing service.
