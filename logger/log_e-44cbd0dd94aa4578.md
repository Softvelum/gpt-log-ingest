# GeoIP update has no temporary directory

Product: Nimble Streamer
Component: GeoIP database updates
Logging function: log_e
Log level: error
Documentation ID: log_e-44cbd0dd94aa4578

## Log pattern

```text
failed to create temp directory to save GeoIP database archive
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create temp directory to save GeoIP database archive
```

## Short description

GeoIP update has no temporary directory.

## Trigger condition

A nonempty archive response is available but temporary-directory creation returns an empty path.

## Explanation

The update attempt returns before saving or extracting the archive.

## Possible causes

- The temporary directory could not be created.

## What to check

1. Check temporary storage availability and directory access.

## Recommended action

Restore usable temporary storage before retrying the update.
