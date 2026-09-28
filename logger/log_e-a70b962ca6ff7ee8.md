# Temporary file path creation raised an exception

Product: Nimble Streamer
Component: Operating system utilities
Logging function: log_e
Log level: error
Documentation ID: log_e-a70b962ca6ff7ee8

## Log pattern

```text
unknown exception caught in Utils::GenTempFile
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception caught in Utils::GenTempFile
```

## Short description

Temporary file path creation raised an exception.

## Trigger condition

Constructing the temporary file path encounters an unrecognized exception.

## Explanation

No temporary filename is returned.

## Possible causes

- The path-generation or filesystem operation raised the reported exception.

## What to check

1. Check the exception text where available and the temporary or cache directory access.

## Recommended action

Restore usable temporary storage.
