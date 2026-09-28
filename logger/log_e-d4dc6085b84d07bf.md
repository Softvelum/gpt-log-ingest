# Icecast Basic credentials missing

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-d4dc6085b84d07bf

## Log pattern

```text
failed to get icecast auth scheme=%s, client=%d
```

## Parameters

- Argument 1 — Scheme: Parsed authorization scheme, Basic on this branch.
- Argument 2 — Socket: Numeric connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get icecast auth scheme=Basic, client=42
```

## Short description

Icecast Basic credentials missing.

## Trigger condition

A Basic authorization header has no credentials token after the scheme.

## Explanation

The metadata-update or publishing connection is shut down. Despite the message wording, the missing part is the credentials token.

## Possible causes

- A Basic authorization header has no credentials token after the scheme.

## What to check

1. Check that the authorization header includes encoded credentials.

## Recommended action

Send a complete Basic authorization value.
