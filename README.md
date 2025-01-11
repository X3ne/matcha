# Matcha

## Architecture

```
src
├── domain                  # Domain layer
│   ├── entities            # Domain Entities
│   ├── errors              # Domain Errors
│   ├── repositories        # Repository traits
│   ├── services            # Domain Services
│   └── value_objects       # Value Objects
│
├── infrastructure          # Infrastructure layer
│   ├── config.rs           # Configuration
│   ├── databases           # Database related stuff
│   ├── error.rs            # Error handling
│   ├── models              # Database models
│   ├── repositories        # Repository implementations
│   ├── server              # Web server
│   ├── services            # Infrastructure services
│   ├── tracing             # Logging and OpenTelemetry
│   └── web                 # Web server utils   
│
├── presentation            # Presentation layer
│   ├── controllers         # HTTP Controllers
│   ├── dto                 # Data Transfer Objects
│   ├── extractors          # Request extractors
│   ├── middlewares         # Middlewares
│   └── routes              # Routes
│
├── services                # Application services
│
└── shared                  # Shared stuff
    ├── types               # Shared types
    └── utils               # Utils
```
