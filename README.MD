# A simple project to test Rocket, a web framework for rust <img src="crab.svg" width="50px" height="50px" />

## Project structure
```
├── Cargo.lock
├── Cargo.toml
├── docker
│   └── docker-compose.yml
├── README.MD
├── Rocket.toml
└── src
    └── main.rs
```

* docker/docker-compose.yml: includes a postgresql service to test the integration of the rust project
* src/main.rs: source code with a simple example of how to use the rocket framework to connect to a database, retrieve data and use http GET method to interact with the web service.
* Rocket.toml: db configuration 