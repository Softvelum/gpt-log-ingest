# Server startup version banner

Product: Nimble Streamer
Component: Server startup
Logging function: log_always
Log level: info (always)
Documentation ID: log_always-4c382bb3173481f6

## Log pattern

```text
*** Nimble Streamer v%s started ***
```

## Parameters

- Argument 1 — Version: Text containing major, minor and patch version numbers followed by any version-stage suffix.

## Log examples

Illustrative examples; these are not captured server logs.

```text
*** Nimble Streamer v4.3.3 started ***
```

## Short description

Server startup version banner.

## Trigger condition

Logging and configuration initialization have succeeded, immediately before registration is checked.

## Explanation

Reports the version assembled from the major, minor, patch and optional stage suffix. It does not prove that registration or server initialization will succeed. The informational message bypasses the normal level filter and uses an available logging destination.

## Related log messages

- [WMSPanel registration is required](log_always-3a647abf8d9f75ef.md)
