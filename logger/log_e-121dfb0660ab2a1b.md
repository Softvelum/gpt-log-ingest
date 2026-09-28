# Advertising rules GET request could not be queued

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-121dfb0660ab2a1b

## Log pattern

```text
failed to GET main rules from url='%s'
```

## Parameters

- Argument 1 — Request URL: Text giving the configured request destination.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to GET main rules from url='https://api.example/rules'
```

## Short description

Advertising rules GET request could not be queued.

## Trigger condition

The server is unavailable or submitting the main rules HTTP GET request fails.

## Explanation

The synchronization-in-progress flag is cleared so later synchronization can proceed.

## Possible causes

- The server is unavailable or submitting the main rules HTTP GET request fails.

## What to check

1. Check the rules destination and the request-processing service.

## Recommended action

Check the rules destination and the request-processing service.
