# Cannot initialize TLS Diffie-Hellman numbers

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_e
Log level: error
Documentation ID: log_e-5061d73fcf8add5a

## Log pattern

```text
BN_bin2bn() failed
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
BN_bin2bn() failed
```

## Short description

Cannot initialize TLS Diffie-Hellman numbers.

## Trigger condition

Converting the fixed Diffie-Hellman prime or generator to big integers fails, or a newer TLS library cannot attach them to the parameter object.

## Explanation

The incomplete TLS context and parameter objects are discarded and context creation fails.

## Possible causes

- Converting the fixed Diffie-Hellman prime or generator to big integers fails, or a newer TLS library cannot attach them to the parameter object.

## What to check

1. Check allocation and TLS initialization diagnostics; the message covers both number conversion and parameter attachment.
