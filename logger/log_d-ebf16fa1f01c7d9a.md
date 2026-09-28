# Country routing cannot identify the client country

Product: Nimble Streamer
Component: Request routing
Logging function: log_d
Log level: debug
Documentation ID: log_d-ebf16fa1f01c7d9a

## Log pattern

```text
unable to detect country code for IP '%s'
```

## Parameters

- Argument 1 — Client address: Text IP address submitted to country lookup.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unable to detect country code for IP '192.0.2.10'
```

## Short description

Country routing cannot identify the client country.

## Trigger condition

A matching host/path rule uses country routing but country lookup returns empty text.

## Explanation

This country rule is skipped and later rules are examined. No country-based redirect is selected by this rule.

## Possible causes

- The country lookup returned no country code.

## What to check

1. Check whether the geolocation database is open and has a country result for this address.

## Related log messages

- [Client country was found for a routing rule](log_d-b364fa89de98f590.md)
