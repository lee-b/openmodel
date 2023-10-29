# OpenModel

OpenModel is a generic, abstract AI interface, for all your generative AI needs.  The intent is to abstract away all of the details of which AI you are using, so that you can build applications and games and utilities and mobile apps on that USE AI, instead of re-implementing an AI engine for every use case.

Think of it like Direct3D, OpenGL, or Vulcan, but for AI.


***************** !!!!! THIS IS A WORK IN PROGRESS !!!! *******************

Just as OpenGL or DirectX support various frontend features and multiple backend graphics cards, OpenModel supports various (initially) generative AI and (later) other use AI model types and use cases, and supports multiple backends for actually running various local models, or connecting to remote models.


## Service and REST API

The main daemon (service) which provides the API, is called `openmodeld`.


## Commandline tools

`openmodelctl` is the main commandline tool used for controlling `openmodeld` when it's running. It provides user-friendly, high-level tools for listing models, installing new models, and so on.


## Rust Libraries

- `libopenmodel_frontend` is the Rust client library.  It holds the definitions that the frontend `openmodelctl` and `openmodeld` use to communicate, and that your Rust-based client code can use, too.
- `libopenmodel_backend` is not for public use.  Rather, it provides low-level internal API shared functionality between `openmodeld` and the backend libraries that implement various AI engines and remote AI servers.

## Compatibility Libraries

- `libopenmodel_frontend_c` is the C-style library for use by C and other C ABI compatible languages.  It's a lightweight wrapper around `libopenmodel_frontend`.
