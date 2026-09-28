# GeoIP archive lacks the database member

Product: Nimble Streamer
Component: GeoIP database updates
Logging function: log_e
Log level: error
Documentation ID: log_e-b1202ef04b7b2a3c

## Log pattern

```text
failed to find '%s' in '%s'
```

## Parameters

- Argument 1 — member: Expected database filename inside the archive.
- Argument 2 — archive: Downloaded GeoIP archive path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to find 'GeoLite2-Country.mmdb' in '/var/cache/geo/update.jar'
```

## Short description

GeoIP archive lacks the database member.

## Trigger condition

Looking up the expected database member in the opened archive fails.

## Explanation

The archive is closed and extraction returns failure.

## Possible causes

- The expected database file is absent or its archive metadata cannot be read.

## What to check

1. Check the archive contents and download source.

## Recommended action

Provide an archive containing the expected GeoIP database.
