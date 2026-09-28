# Subscription alert ends

Product: Nimble Streamer
Component: Server registration
Logging function: log_always
Log level: info (always)
Documentation ID: log_always-9dc3499a3dbba210

## Log pattern

```text
!!!!!!! END OF IMPORTANT SUBSCRIPTION ALERT !!!!!!
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
!!!!!!! END OF IMPORTANT SUBSCRIPTION ALERT !!!!!!
```

## Short description

Subscription alert ends.

## Trigger condition

During startup, at least one of the panel domain, server identifier, or panel password is empty and visible registration warnings are enabled.

## Explanation

Closes the four-message registration warning. Startup then exits with status 4 before running the server. These messages carry an informational tag and bypass the normal level filter; they still require a log destination. A later periodic registration check can request shutdown but suppresses this warning block.

## Possible causes

- The panel domain, server identifier, or password is empty.

## What to check

1. Check that all three registration values are present.

## Recommended action

Complete the missing WMSPanel registration values before starting the server.

## Related log messages

- [WMSPanel registration is required](log_always-3a647abf8d9f75ef.md)
- [Subscription alert begins](log_always-4a2131fb4345eed8.md)
- [Subscription information link](log_always-a6e3355e835ca707.md)
