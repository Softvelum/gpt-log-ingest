# File ownership could not be assigned to Nimble

Product: Nimble Streamer
Component: Operating system utilities
Logging function: log_e
Log level: error
Documentation ID: log_e-6aba61131f3dbcb9

## Log pattern

```text
Failed to set 'nimble' as owner of '%s' file errno=%d
```

## Parameters

- Argument 1 — File: Affected file path, text.
- Argument 2 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Failed to set 'nimble' as owner of '/var/lib/nimble/example.dat' file errno=5
```

## Short description

File ownership could not be assigned to Nimble.

## Trigger condition

On non-Windows systems, changing a file owner to the Nimble account fails while logging is enabled.

## Explanation

The helper logs the failure without retrying; the group is not changed by this operation.

## Possible causes

- The operating system rejected the ownership change.

## What to check

1. Check the file path, current privileges, and operating system error.

## Recommended action

Correct the ownership or access issue.
