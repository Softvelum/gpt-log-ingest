# Server process-ID file could not be opened

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-d5c9bc50aa41bc1a

## Log pattern

```text
failed to create pidfile='%s'
```

## Parameters

- Argument 1 — Process-ID file: Configured path of the process-ID file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create pidfile='/var/run/nimble.pid'
```

## Short description

Server process-ID file could not be opened.

## Trigger condition

A process-ID file is configured and opening it for truncating output fails.

## Explanation

Initialization terminates the process with a failure status before socket and service setup.

## Possible causes

- The configured process-ID file could not be opened for writing.

## What to check

1. Check the indicated path and whether the process can create or write the file.

## Recommended action

Make the configured process-ID file writable before startup.
