# Remote log settings response is not valid JSON

Product: Nimble Streamer
Component: Remote log reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-e9a6f73bc81bc32c

## Log pattern

```text
failed to parse response: '%s'
```

## Parameters

- Argument 1 — Response body: Text returned by the remote log synchronization endpoint.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse response: '<html>unavailable</html>'
```

## Short description

Remote log settings response is not valid JSON.

## Trigger condition

The synchronization request succeeds but the response body cannot be parsed as JSON.

## Explanation

This response is ignored and the reporting loop continues without applying new settings.

## Possible causes

- The response is not parseable JSON.

## What to check

1. Inspect the displayed response body.

## Recommended action

Ensure the remote endpoint supplies a JSON object with log reporting settings.

## Related log messages

- [Remote log settings response has the wrong JSON type](log_e-9b6cd21425ef0d44.md)
- [Remote log response received](log_v-12298e1205567192.md)
