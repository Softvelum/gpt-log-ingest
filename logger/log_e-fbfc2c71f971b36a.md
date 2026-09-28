# Disk cache version marker cannot be read

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-fbfc2c71f971b36a

## Log pattern

```text
failed to open cache version file '%s'. Most probably this is permission issue
```

## Parameters

- Argument 1 — file: Filesystem path of the cache data or metadata file being handled.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open cache version file '/var/cache/nimble/version'. Most probably this is permission issue
```

## Short description

Disk cache version marker cannot be read.

## Trigger condition

An existing regular version-marker file cannot be opened for reading.

## Explanation

The compatibility check returns without cleaning the cache. The suggested permission cause is a guess in the log text, not a confirmed diagnosis.

## Possible causes

- Opening the existing version file for reading failed; permissions are one possible cause.

## What to check

1. Check readability and filesystem access for the named marker.

## Recommended action

Restore access so cache compatibility can be checked.
