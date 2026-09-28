# Writable socket error state inspected

Product: Nimble Streamer
Component: Socket event monitoring
Logging function: log_d
Log level: debug
Documentation ID: log_d-9dbf8a3b84ab54f5

## Log pattern

```text
getsockopt() fd=%d retval=%d error=%d FD_ISSET(&except_fds)=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Query result: Return value of the socket error query, signed integer.
- Argument 3 — Pending socket error: Queried socket error code; initialized to zero before the query.
- Argument 4 — Exception readiness: Nonzero if the socket also belongs to the returned exception set, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
getsockopt() fd=12 retval=0 error=0 FD_ISSET(&except_fds)=0
```

## Short description

Writable socket error state inspected.

## Trigger condition

On Windows with debug logging, a socket is reported writable and its pending error state is queried.

## Explanation

The query details are logged; the writable event is delivered afterward regardless of this diagnostic result.
