# Child shutdown raised an exception

Product: Nimble Streamer
Component: Operating system utilities
Logging function: log_e
Log level: error
Documentation ID: log_e-86d22ac79ec9df57

## Log pattern

```text
unknown exception caught during encoder termination
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception caught during encoder termination
```

## Short description

Child shutdown raised an exception.

## Trigger condition

The child termination or waiting sequence raises an unrecognized exception.

## Explanation

The exception is caught and the helper returns without another termination attempt.

## Possible causes

- The child-process termination or wait operation raised an exception.

## What to check

1. Check the child state and exception description where present.

## Recommended action

Investigate whether the child stopped successfully.
