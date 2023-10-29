# OpenModel

OpenModel is a generic, local AI API.  Essentially, it is the equivalent of OpenGL or Direct3D, but for AI models rather than 3D.

Just as OpenGL or DirectX support various frontend features and multiple backend graphics cards, OpenModel supports various (initially) generative AI and (later) other use AI model types and use cases, and supports multiple backends for actually running various local models, or connecting to remote models.

## Service and REST API

The main daemon (service) which provides the API, is called `openmodeld`.

See [openmodeld/README.md](openmodeld/README.md) for details on both running this service, and using its REST API.


## Commandline tools

`openmodelctl` is the main commandline tool used for controlling `openmodeld` when it's running. It provides user-friendly, high-level tools for listing models, installing new models, and so on.

See [openmodelctl/README.md](openmodelctl/README.md) for details.


## Library

`libopenmodel_shared` is not for public use at this time.  Rather, it provides low-level internal shared functionality between `openmodelctl` and `openmodeld`.  Think of it as the common language that the commandline tool `openmodelctl` shares with the `openmodeld` service/daemon.
