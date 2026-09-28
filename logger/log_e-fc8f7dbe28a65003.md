# File write failed

Product: Nimble Streamer
Component: Operating system utilities
Logging function: log_e
Log level: error
Documentation ID: log_e-fc8f7dbe28a65003

## Log pattern

```text
failed to write data to file='%s' size=%u
```

## Parameters

- Argument 1 — File: Affected file path, text.
- Argument 2 — Bytes: Requested write length in bytes, unsigned integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to write data to file='/var/lib/nimble/example.dat' size=4096
```

## Short description

File write failed.

## Trigger condition

The file stream reports failure after attempting to write the requested bytes.

## Explanation

The file is closed and removal of the partly written destination is attempted. The operation returns failure.

## Possible causes

- The file stream could not complete the requested data write.

## What to check

1. Check the destination storage and requested byte count.

## Recommended action

Restore writable storage and retry according to the caller requirements.
