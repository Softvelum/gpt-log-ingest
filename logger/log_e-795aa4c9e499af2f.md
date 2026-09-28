# Registration rules JSON is invalid

Product: Nimble Streamer
Component: Server registration utility
Logging function: log_e
Log level: error
Documentation ID: log_e-795aa4c9e499af2f

## Log pattern

```text
apply-rules-conf specified but we cannot parse rules.conf json='%s'
```

## Parameters

- Argument 1 — rules: Complete loaded rules JSON text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
apply-rules-conf specified but we cannot parse rules.conf json='{invalid}'
```

## Short description

Registration rules JSON is invalid.

## Trigger condition

The rules document selected for registration cannot be parsed as JSON.

## Explanation

The utility exits with failure before sending the registration request.

## Possible causes

- The loaded rules document is invalid JSON.

## What to check

1. Check the document syntax.

## Recommended action

Correct the rules document before registration.
