# RTMP application name becomes empty

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-1eee8c9a67c3aae9

## Log pattern

```text
application name empty
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
application name empty
```

## Short description

RTMP application name becomes empty.

## Trigger condition

Connection authorization finds an empty application name after removing query parameters, a trailing slash, and any default-instance suffix.

## Explanation

Authorization cannot proceed without an application and connection processing fails.

## Possible causes

- Connection authorization finds an empty application name after removing query parameters, a trailing slash, and any default-instance suffix.

## What to check

1. Check the application in the connect request after excluding query parameters and the default-instance suffix.

## Recommended action

Supply a nonempty RTMP application name.
