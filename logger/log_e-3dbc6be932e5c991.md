# License server response signature does not match

Product: Nimble Streamer
Component: Transcoder and Addenda licensing
Logging function: log_e
Log level: error
Documentation ID: log_e-3dbc6be932e5c991

## Log pattern

```text
validation signature invalid
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
validation signature invalid
```

## Short description

License server response signature does not match.

## Trigger condition

The license service response fails signature validation.

## Explanation

Validation returns failure; the worker stops transcoding or Addenda processing and waits for a changed license.

## Possible causes

- The license service response was not accepted for this registration.

## What to check

1. Check the server registration identity and license used for this response.

## Recommended action

Restore matching registration and license data.

## Related log messages

- [License validation response lacks required fields](log_e-44229e10585536d6.md)
- [License validation credentials are incomplete](log_e-941e53609713ade3.md)
