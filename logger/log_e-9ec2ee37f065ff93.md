# Authorization response is not a JSON object

Product: Nimble Streamer
Component: Client authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-9ec2ee37f065ff93

## Log pattern

```text
failed to process response from auth handler body='%s'
```

## Parameters

- Argument 1 — Response body: Nonempty text returned by the authorization handler.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process response from auth handler body='[]'
```

## Short description

Authorization response is not a JSON object.

## Trigger condition

A nonempty authorization response either fails JSON parsing or parses to a non-object value.

## Explanation

The waiting client receives a request-failure notification. An empty response is handled separately as successful authorization; it does not produce this message.

## Possible causes

- The handler returned invalid JSON or a value other than a JSON object.

## What to check

1. Inspect the displayed response body and its top-level JSON type.

## Recommended action

Return the authorization response in the expected object form.
