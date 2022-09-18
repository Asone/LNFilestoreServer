
# Build

## Note about compilation

Do note that compilation can take a while, be slow and have heavy resources consumption. 

The initial build will have to download and compile every dependency which may take a while. 

If you intend to build the docker image, ensure your docker provides the sufficient ressources like memory in order to avoid panic errors at compile time. 

## Build locally

1. Clone the project :

```
git clone <project_url>
```

2. go to the folder and build the binary : 
```
cargo install --path .
```

then you can run it : 
````
cargo run

````

## Build with docker 

The repo provides a Dockerfile to build an image with the compiled binary of the server. 

You can build your own image with
````
docker build .
````

If you need to build an image for another target like arm64 ( e.g: for Raspberry ) you can use the following command :

````
docker buildx build --platform linux/arm64 .
````

