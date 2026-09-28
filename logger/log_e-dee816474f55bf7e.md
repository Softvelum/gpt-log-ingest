# Icecast Basic credentials rejected

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-dee816474f55bf7e

## Log pattern

```text
basic icecast auth failed, client=%d
```

## Parameters

- Argument 1 — Socket: Numeric connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
basic icecast auth failed, client=42
```

## Short description

Icecast Basic credentials rejected.

## Trigger condition

Decoded Basic credentials differ from the configured application login and password.

## Explanation

A 401 response is sent. Publishing can return to request reception after that response; a metadata-update connection closes after its response.

## Possible causes

- Decoded Basic credentials differ from the configured application login and password.

## What to check

1. Check the credentials configured for the target application.

## Recommended action

Correct the publishing or metadata-update credentials.
