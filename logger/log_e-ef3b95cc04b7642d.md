# License server response is not a JSON object

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-ef3b95cc04b7642d

## Log pattern

```text
failed to process license server response
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process license server response
```

## Short description

License server response is not a JSON object.

## Trigger condition

The license service response is parseable JSON but has an unsupported top-level structure.

## Explanation

The license service response cannot be used, so this validation attempt does not complete.

## Possible causes

- The service returned a body with an unsupported JSON structure.

## What to check

1. Check the license-server body and any proxy modification.

## Recommended action

Restore the expected license-server response.
