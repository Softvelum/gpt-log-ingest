# Scenario encoder reached an unsupported sleep operation

Product: Nimble Streamer
Component: Scenario encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-54dc9c5cb82c33bd

## Log pattern

```text
Server::ThreadSleep is currently unimplemented for scenario encoder
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Server::ThreadSleep is currently unimplemented for scenario encoder
```

## Short description

Scenario encoder reached an unsupported sleep operation.

## Trigger condition

The scenario server is asked to use its unimplemented thread-sleep operation.

## Explanation

The server requests exit immediately and returns the value that tells the caller to exit, instead of sleeping.

## Possible causes

- The scenario server is asked to use its unimplemented thread-sleep operation.

## What to check

1. Check which scenario activity requested the unsupported server sleep operation.
