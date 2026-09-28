# RIST receiver connection-status reporting could not be enabled

Product: Nimble Streamer
Component: RIST media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-76005b7cbcfe19be

## Log pattern

```text
failed to set rist player connection status callback, id=%s
```

## Parameters

- Argument 1 — Connection ID: Configured identifier for the RIST child-process connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set rist player connection status callback, id=rist-receiver
```

## Short description

RIST receiver connection-status reporting could not be enabled.

## Trigger condition

A nonzero statistics interval enables callback setup, but registration of the connection-status callback returns a nonzero result.

## Explanation

The failure is logged and transport initialization continues. This message alone does not mean media transport failed; the requested reporting callback was not successfully installed.

## Possible causes

- A nonzero statistics interval enables callback setup, but registration of the connection-status callback returns a nonzero result.

## What to check

1. Check the configured statistics interval and whether connection-status reporting is available from the loaded RIST runtime.

## Related log messages

- [RIST child connection entered transport initialization](log_i-66b0f0cc7975ea01.md)
