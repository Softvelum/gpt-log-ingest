# Cached response has no cache item

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-e8ffe31a2b0417a7

## Log pattern

```text
ClientSession::SendCachedItem assertion failed m_cache_item != NULL
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
ClientSession::SendCachedItem assertion failed m_cache_item != NULL
```

## Short description

Cached response has no cache item.

## Trigger condition

Cached-item sending is requested without a held cache item.

## Explanation

The session is marked finished.

## Possible causes

- Cached-item sending is requested without a held cache item.

## What to check

1. Check the preceding cache reservation and release events; cached response sending requires a retained item.

## Recommended action

Check the preceding cache reservation and release events; cached response sending requires a retained item.
