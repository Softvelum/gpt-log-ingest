# PallyCon KMS token is missing

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-dca895bc615db7dd

## Log pattern

```text
KMS token must be provided for PallyCon
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
KMS token must be provided for PallyCon
```

## Short description

PallyCon KMS token is missing.

## Trigger condition

The PallyCon or DoveRunner KMS token is empty.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- The PallyCon or DoveRunner KMS token is empty.

## What to check

1. Provide the KMS token for the selected provider.

## Recommended action

Provide the KMS token for the selected provider.
