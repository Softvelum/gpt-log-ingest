# GeoIP archive member read completely

Product: Nimble Streamer
Component: GeoIP database updates
Logging function: log_i
Log level: info
Documentation ID: log_i-fd68c061dc2da312

## Log pattern

```text
read '%s' file from '%s'
```

## Parameters

- Argument 1 — member: Expected database filename inside the archive.
- Argument 2 — archive: Downloaded GeoIP archive path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
read 'GeoLite2-Country.mmdb' file from '/var/cache/geo/update.jar'
```

## Short description

GeoIP archive member read completely.

## Trigger condition

The database member read returns its full expected size.

## Explanation

The archive is then closed and the database is written to its temporary destination. This message precedes saving, and the save result is not checked by this function.
