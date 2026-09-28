# DVR direct read mode is unavailable on Windows

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-efbee802249bb11a

## Log pattern

```text
dvr_read_mode_enabled is not supported on Windows
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
dvr_read_mode_enabled is not supported on Windows
```

## Short description

DVR direct read mode is unavailable on Windows.

## Trigger condition

The DVR direct read mode setting is encountered in a Windows build, regardless of its text value.

## Explanation

Configuration processing fails at this line.

## Possible causes

- This setting is unsupported by the Windows branch.

## What to check

1. Check whether the unsupported setting is present.

## Recommended action

Remove this setting from Windows configuration.
