# Invalid Icecast metadata mount

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-fdef4f13524fb1e0

## Log pattern

```text
invalid icecast metadata mount url='%s' client=%d
```

## Parameters

- Argument 1 — URL: Received metadata-update URL.
- Argument 2 — Socket: Numeric connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid icecast metadata mount url='/admin/metadata?mount=radio&mode=updinfo' client=42
```

## Short description

Invalid Icecast metadata mount.

## Trigger condition

The decoded mount parameter cannot provide both a nonempty application and stream name.

## Explanation

The connection is shut down without applying the metadata update.

## Possible causes

- The decoded mount parameter cannot provide both a nonempty application and stream name.

## What to check

1. Check the mount parameter after URL decoding.

## Recommended action

Supply the target application/stream mount.
