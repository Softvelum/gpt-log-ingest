# GeoIP database member found

Product: Nimble Streamer
Component: GeoIP database updates
Logging function: log_v
Log level: verbose
Documentation ID: log_v-b58e9d31c916acb9

## Log pattern

```text
found '%s' of size=%llu in '%s'
```

## Parameters

- Argument 1 — member: Expected database filename inside the archive.
- Argument 2 — bytes: Uncompressed archive-member size in bytes.
- Argument 3 — archive: Downloaded GeoIP archive path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
found 'GeoLite2-Country.mmdb' of size=1048576 in '/var/cache/geo/update.jar'
```

## Short description

GeoIP database member found.

## Trigger condition

The archive lookup locates the expected database file.

## Explanation

The reported uncompressed size is used for allocation and reading. Member opening, reading, and saving still follow.
