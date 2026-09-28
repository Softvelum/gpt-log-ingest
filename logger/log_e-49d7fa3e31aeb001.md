# Cannot open license file for update

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-49d7fa3e31aeb001

## Log pattern

```text
cannot update '%s' license file! Fix this ASAP
```

## Parameters

- Argument 1 — license\_file: Configured Transcoder or Addenda license-file path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot update '/var/lib/nimble/transcoder.license' license file! Fix this ASAP
```

## Short description

Cannot open license file for update.

## Trigger condition

A received Transcoder or Addenda license passes signature validation, but its configured destination cannot be opened for writing.

## Explanation

The license update returns before writing the exported license. This message concerns opening the file; it does not report a later stream-write failure.

## Possible causes

- Opening the license destination for output failed.

## What to check

1. Check the destination directory, permissions, and storage availability.

## Recommended action

Restore write access to the configured license destination.
