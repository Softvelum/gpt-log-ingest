# VCAS key has an unexpected length

Product: Nimble Streamer
Component: DRM key management
Logging function: log_e
Log level: error
Documentation ID: log_e-354d72ffdf305aa1

## Log pattern

```text
unexpected VCAS key format key='%s' reponse:
%.*s
```

## Parameters

- Argument 1 — Key text: Text containing the returned hexadecimal key.
- Argument 2 — Response length: Integer limit in bytes for the response text that follows.
- Argument 3 — Response body: Text from the key server, limited by the preceding length.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unexpected VCAS key format key='abc' reponse:
{}
```

## Short description

VCAS key has an unexpected length.

## Trigger condition

A parsed VCAS key response contains a key string whose length is not 32 characters.

## Explanation

The key is marked failed and its encryption bytes are not updated.

## Possible causes

- A parsed VCAS key response contains a key string whose length is not 32 characters.

## What to check

1. Return a 32-character hexadecimal AES-128 key from VCAS.

## Recommended action

Return a 32-character hexadecimal AES-128 key from VCAS.
