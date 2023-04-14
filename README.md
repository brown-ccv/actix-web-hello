# actix-web-hello

This is a minimal example of using Docker to package a Rust application that uses the Actix-Web library.

## Dependencies 
[Docker](https://www.docker.com) 

## Instructions 

```
git clone git@github.com:brown-ccv/actix-web-hello.git
```

Build the Docker container:
```
cd actix-web-hello
docker build -t actix-hello .
```

Run the container, mapping port 8080 on localhost to 8080 on the running container
```
docker run -p 8080:8080 actix-hello
```

Confirm the container is running: 
```
curl http://127.0.0.1:8080/paul
```
