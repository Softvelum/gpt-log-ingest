# License server response is not parseable JSON

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-3da32f6e198e09cc

## Log pattern

```text
failed to parse license server response
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse license server response
```

## Short description

License server response is not parseable JSON.

## Trigger condition

The license service response cannot be parsed as JSON.

## Explanation

The license service response cannot be used, so this validation attempt does not complete.

## Possible causes

- The service returned a body with an unsupported JSON structure.

## What to check

1. Check the license-server body and any proxy modification.

## Recommended action

Restore the expected license-server response.
