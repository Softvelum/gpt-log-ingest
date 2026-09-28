# Advertising download worker stopped

Product: Nimble Streamer
Component: Advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-ddf9dcd6ba51dfc9

## Log pattern

```text
AdvertisingDownloadThread stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
AdvertisingDownloadThread stopped
```

## Short description

Advertising download worker stopped.

## Trigger condition

An exit request ends the advertising download loop.

## Explanation

The worker returns after it stops processing download events.

## Possible causes

- An exit request ends the advertising download loop.

## Related log messages

- [Advertising download worker started](log_d-112ddbf5246bdc2e.md)
