# GeoIP archive could not be opened

Product: Nimble Streamer
Component: GeoIP database updates
Logging function: log_e
Log level: error
Documentation ID: log_e-fc85b2f9d9c7e060

## Log pattern

```text
failed to open GeoIP database archive '%s' error=%d
```

## Parameters

- Argument 1 — archive: Downloaded GeoIP archive path.
- Argument 2 — error: ZIP library archive-open error code.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open GeoIP database archive '/var/cache/geo/update.jar' error=19
```

## Short description

GeoIP archive could not be opened.

## Trigger condition

The ZIP archive library cannot open the downloaded archive.

## Explanation

Extraction returns failure without updating the active GeoIP database.

## Possible causes

- The downloaded archive is unreadable or not a usable ZIP archive.

## What to check

1. Check the archive-open error and download diagnostics.

## Recommended action

Correct archive download or storage problems before retrying the update.
