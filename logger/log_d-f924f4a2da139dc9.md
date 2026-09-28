# Reading recovered-statistics response

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_d
Log level: debug
Documentation ID: log_d-f924f4a2da139dc9

## Log pattern

```text
parsing json[recovered]='%s'
```

## Parameters

- Argument 1 — response\_body: Full recovered-statistics response body.

## Log examples

Illustrative examples; these are not captured server logs.

```text
parsing json[recovered]='{"status":"success"}'
```

## Short description

Reading recovered-statistics response.

## Trigger condition

The recovered-statistics response handler is entered.

## Explanation

The full response is printed before the empty-body check and JSON parsing. An empty body returns permission to continue synchronization without parsing.

## Related log messages

- [Panel does not recognize recovered-statistics server](log_e-77065bbaaa78732a.md)
- [Recovered-statistics response has invalid JSON](log_e-f3e8b1394a3be775.md)
