# HTTP client setup ran out of memory

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_e
Log level: error
Documentation ID: log_e-d615aaa5ea5c24b9

## Log pattern

```text
bad_alloc while creating client s=%d what='%s'
```

## Parameters

- Argument 1 — Socket: Integer socket descriptor.
- Argument 2 — Exception detail: Allocation-exception diagnostic text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
bad_alloc while creating client s=12 what='std::bad_alloc'
```

## Short description

HTTP client setup ran out of memory.

## Trigger condition

An allocation exception occurs during accepted-client session setup.

## Explanation

The partially created session is deleted, or its socket is closed directly if no session exists. The descriptor is removed from the active map and other clients can still be processed.

## Possible causes

- Memory allocation failed during session setup.

## What to check

1. Inspect allocation diagnostics and the affected connection.
