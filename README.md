# actix-web-hello

This is a minimal example of using Docker to package a Rust application that uses the Actix-Web library.

## Dependencies 
[Docker](https://www.docker.com) 

## Instructions 
We're going to begin by cloning this repository. Then we'll build the Docker container on our local machine. After that, we'll call `docker run` to start the container. Finally, we can use `curl` (or a web browser) to confirm the app is running.


### Clone This Repo
```
git clone https://github.com/brown-ccv/actix-web-hello.git 
```


### Build the Container
Build the container and tag it as `actix-hello`
```
cd actix-web-hello
docker build -t actix-hello .
```

### Run the container
Note that we are mapping port 8080 on localhost to 8080 on the running container
```
docker run -p 8080:8080 actix-hello
```

### Confirm the Container is Running 
Finally, we confirm the container is running. Note, we can replace `paul` with any string, and it will be echoed back by the application. Also note you can check this in the browser if you prefer. 
```
curl http://127.0.0.1:8080/paul
```
