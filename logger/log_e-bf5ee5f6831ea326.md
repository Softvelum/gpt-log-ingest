# Requested log file could not be opened

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-bf5ee5f6831ea326

## Log pattern

```text
prepareLogPost failed to open log file='%s'
```

## Parameters

- Argument 1 — log\_file: Configured runtime log-file path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
prepareLogPost failed to open log file='/var/log/nimble/nimble.log'
```

## Short description

Requested log file could not be opened.

## Trigger condition

Preparing a panel-requested log upload fails to open the configured log file for reading.

## Explanation

Preparation continues with a multipart response explaining that the log file could not be opened or is empty; no log attachment is included when no bytes were read.

## Possible causes

- Opening the configured log file failed.

## What to check

1. Check that the runtime log file exists and is readable by the server process.

## Recommended action

Correct the log path or file access if log upload is required.
