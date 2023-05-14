# A simple web server written in Rust
This is a personal exercise project.

## Features:
- Serving files from the `/pub` folder 
- Server side persistance using a json file in the `/data` folder
- Logging last execution messages upon graceful shutdown
- Basic multithreaded requests handling


## TODO:
- improve documentation
- improve testing
- provide mini JS library to write and read pieces of data
- manage concurrency in write operations
- improve included sample web application

---
## Codebase structure:
```
- src/ 
    Server code
- tests/
    Server code integration tests
- pub/
    public stuff for the client to get
- data/
    data storage for persistance
```