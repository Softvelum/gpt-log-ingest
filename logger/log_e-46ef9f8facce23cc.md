# Nimble service account was not found

Product: Nimble Streamer
Component: Operating system utilities
Logging function: log_e
Log level: error
Documentation ID: log_e-46ef9f8facce23cc

## Log pattern

```text
getpwnam failed for user 'nimble'
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
getpwnam failed for user 'nimble'
```

## Short description

Nimble service account was not found.

## Trigger condition

The system account lookup for the Nimble user returns no account, either before changing file ownership or before a privileged scheduled task is launched.

## Explanation

File ownership is not changed. A scheduled task launched while running as root fails before creating its pipes or child.

## Possible causes

- The system account lookup could not obtain the Nimble user record.

## What to check

1. Check whether the Nimble service account is available to the system account service.

## Recommended action

Restore the Nimble service account required by the operation.
