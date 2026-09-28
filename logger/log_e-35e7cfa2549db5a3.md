# Private-key password exceeds callback buffer

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-35e7cfa2549db5a3

## Log pattern

```text
ssl key pass length=%llu is bigger than buffer size=%d
```

## Parameters

- Argument 1 — Length: Intended password length in bytes; the format and supplied type differ in the HTTP/3 occurrence.
- Argument 2 — Capacity: Password callback buffer capacity in bytes, including space needed for a terminator.

The HTTP/3 occurrence supplies an int to an unsigned long long conversion. The ordinary TLS occurrence supplies a string size type, whose compatibility is platform dependent.

## Log examples

A portable illustrative rendering is not possible because the password-length argument type does not match the format in all occurrences.

## Short description

Private-key password exceeds callback buffer.

## Trigger condition

The password length is greater than or equal to the supplied password-buffer capacity.

## Explanation

The callback returns zero without providing the password. Certificate key loading can consequently fail. This applies to ordinary TLS and, when enabled, HTTP/3 TLS.

## Possible causes

- The password length is greater than or equal to the supplied password-buffer capacity.

## What to check

1. Check the configured key-password length against the reported callback buffer capacity.
