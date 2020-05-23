# EnclaveVerifier

## Build Enclave Apps

1. Install Docker
	* Windows and Mac can install [Docker Desktop https://www.docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop)
	* Ubuntu may follow instruction at [https://docs.docker.com/engine/install/ubuntu/](https://docs.docker.com/engine/install/ubuntu/)

2. Install SGX-Rust Docker Image
```shell
docker pull baiduxlab/sgx-rust
```

3. Run Docker Image

Under the **root** of the **repo folder** run:

```shell
docker run -v $(pwd):/root/sgx -ti baiduxlab/sgx-rust
```

Or you can replace the `$(pwd)` with the *absolute* path to the repo folder.

4. Build Apps (within the docker instance)

	* interpreter

```shell
export SGX_MODE=SW

cd /root/sgx/enclave-bin/interpreter

make
```

	* type_checker

```shell
export SGX_MODE=SW

cd /root/sgx/enclave-bin/type_checker

make
```

## Run Enclave Apps (inside the docker instance)

* interpreter

```shell
cd /root/sgx/enclave-bin/interpreter/bin

./app
```

* type_checker

```shell
cd /root/sgx/enclave-bin/type_checker/bin

./app
```

## Edit Enclave Apps' Code

An enclave app consist of two binaries - *app* and *enclave*

### App

App is an executable binary, where the *main* function is defined, so the entire program starts from here.

It's implemented in */root/sgx/enclave-bin/interpreter/app/src/main.rs*. You can make system calls in here; for example, it can read in a file content, and pass that content into the enclave, since enclaves caonnot make system calls at their own.

### Enclave

Enclave is an shared (dynamic) library binary, which contains all the functions defined in enclave environment.

It's implemented in */root/sgx/enclave-bin/interpreter/enclave/src/lib.rs*. *app* side can make function calls to here to start the computation in enclave. So, for example, there could be a function that accepts the file content read by the *app*.

However, to declare a function in enclave that is callable to the *app* side, in addition to define the function in *... /enclave/src/lib.rs*, you also need to declare the prototype of the function in two places - one in *... /enclave/Enclave.edl*, another one in *... /app/src/main.rs*. The function prototypes declared in these two files and the one declared in *... /enclave/src/lib.rs* are slightly different. You can refer to the *interpret_byte_code* enclave function defined in *interpreter* app as an example.
