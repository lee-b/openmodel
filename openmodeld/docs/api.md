
# OpenModel - API

## Overview

The OpenModel API provides developers with the ability to generate text using various models. The API follows the best practices for OAuth2 and provides endpoints for generating text and listing available backend models.

## Endpoints

### POST /generate

This endpoint generates text based on the input parameters.

#### Parameters:
- `input`: The input text to base the generation on.
- `model`: The model to use for generation. If not provided, the default model is used.
- `temperature`: Controls the randomness of predictions by scaling the logits before applying softmax. Higher values mean more random completions. If not provided, the default value is used.
- `top_p`: Controls diversity via nucleus sampling: only the top p tokens with cumulative probability > top_p are considered for the next token. If not provided, the default value is used.
- `max_tokens`: The maximum length of the generated text. If not provided, the default value is used.

#### Example:

POST /generate
Content-Type: application/json

{
    "input": "Once upon a time",
    "model": "gpt-3",
    "temperature": 0.5,
    "top_p": 0.9,
    "max_tokens": 100
}

### GET /models

This endpoint lists all the available models that can be used for generation.

#### Example:

GET /models

## Authentication

The API uses OAuth2 for authentication. To authenticate your application, you'll need to register it in our developer portal and obtain a client ID and secret. Use these credentials to get an access token, which should be included in the Authorization header in the format `Bearer {access_token}` for all API requests.

For more information on how to authenticate, refer to our OAuth2 guide.

## Errors

The API uses standard HTTP status codes to indicate the success or failure of an API request. In general, codes in the 2xx range signify success, codes in the 4xx range signify an error that resulted from the provided inputs, and codes in the 5xx range signify an error with the service itself.
