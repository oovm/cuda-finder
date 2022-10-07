# Cuda Finder

This is a build plugin that helps you find the CUDA installation path.

## Installation

```bash
wee install cuda-finder
```

Reading configuration files has a higher priority than environment variables.

If a path is already set in the configuration, then the environment variable will have no effect

## CUDA

- Manual Setting

```sh
CUDA_PATH
```

- Auto Search

| Operating System | Path                                                  |
|------------------|-------------------------------------------------------|
| Windows          | `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\` |
| Linux            | `/usr/local/cuda`                                     |
| MacOS            | `/usr/local/cuda`                                     |

## CUDNN

- Manual Setting

```sh
CUDNN_PATH
```

- Auto Search

| Operating System | Path                                                  |
|------------------|-------------------------------------------------------|
| Windows          | `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\` |
| Linux            | `/usr/local/cuda`                                     |
| MacOS            | `/usr/local/cuda`                                     |

## CUBLAS

- Manual Setting

```sh
CUBLAS_PATH
```

## TensorRT

- Manual Setting

```sh
TENSORRT_PATH
```

## NCCL

- Manual Setting

```sh
NCCL_PATH
```
