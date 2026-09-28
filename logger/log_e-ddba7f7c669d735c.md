# EzDRM credentials are missing

Product: Nimble Streamer
Component: DRM configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-ddba7f7c669d735c

## Log pattern

```text
user and password must be provided for EzDRM
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
user and password must be provided for EzDRM
```

## Short description

EzDRM credentials are missing.

## Trigger condition

An EzDRM CPIX or CPIX V2 group has an empty user name or password.

## Explanation

The new DRM settings are rejected; an earlier successfully loaded settings set is retained by the file reload path.

## Possible causes

- An EzDRM CPIX or CPIX V2 group has an empty user name or password.

## What to check

1. Provide both EzDRM credentials.

## Recommended action

Provide both EzDRM credentials.
