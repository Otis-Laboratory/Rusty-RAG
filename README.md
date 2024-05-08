WIP RAG (Retrieval Augmented Generation) pipeline re-re-re-rewritten, this time in Rust. Originally, this pipeline was written in Python. Due to the less than satisfactory performance, I rewrote it in TypeScript. The performance improved, however it was still not up to par. Again, I rewrote it but in C++. Rewriting and converting tensors took ages and the performance was only slightly sped up. Finally, I've rewritten it AGAIN, but in Rust. The speed is finally up to par (0.03s/embedding) and I can use it on scale for large context inference.

Thanks to the Rust-BERT, Tch, and libtorch libraries for making this project possible.

## Features
- Extremely fast RAG
- Multiple inference endpoints
- Much more soon?

## Requirements
- Operating System: Any (Tested on Windows 10 w/ CUDA 12.3)
- Python 3.X
- Rust

## Setup
The setup for a project like this is super convoluted, but I will try to explain it as clear as possible.

1. Clone this repo. Clone with submodules if you want to convert your own PyTorch model weights into Rust model weights:
```shell
# With submodules
git clone --recurse-submodules https://github.com/guillaume-be/rust-bert.git

# Without submodules
git clone https://github.com/guillaume-be/rust-bert.git
```

2. Navigate to the directory.
```shell
cd Rusty-RAG

ls
```

3. Included in the repo are ``models/`` and ``lib/`` folders. Install [libtorch](https://pytorch.org/get-started/locally/) v2.1.0 and put the ``libtorch/`` folder in the ``lib/`` directory.

4. Put any models into the ``models/`` folder.

5. You're done... I guess. Not really much to do in this repo yet.

## Converting PyTorch weights to Rust weights
This was explained in the rust-bert repo, but it's kinda complicated so here's how I did it. Firstly, the main rust-bert library uses libtorch v2.1.0, whereas the rust-bert submodule (repo) uses v2.2.0 for converting weights. Not super efficient, I know, but it is what it is.

1. Install [libtorch](https://pytorch.org/get-started/locally/) v2.2.0 and put the ``libtorch/`` folder in the ``lib/`` directory under a different name. I use ``libtorch-rustbert``.

2. Set your environment variables for the libtorch location and the libtorch libraries:
```shell
# Windows 
$Env:LIBTORCH = "path\to\Rusty-RAG\lib\libtorch"
$Env:Path += "path\to\Rusty-RAG\lib\libtorch\lib"

# Linux 
export LIBTORCH=path/to/Rusty-RAG/lib/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

# MacOS + Homebrew
brew install pytorch jq
export LIBTORCH=$(brew --cellar pytorch)/$(brew info --json pytorch | jq -r '.[0].installed[0].version')
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```

3. Clone the repo of the model you want to convert to Rust (we'll use MiniLM for this example):
```shell
git -C models clone https://huggingface.co/sentence-transformers/all-MiniLM-L12-v2
```

4. Run the Python script to convert the weights from PyTorch to Rust:
```shell
python lib/rust-bert/utils/convert_model.py "models/all-MiniLM-L12-v2/pytorch_model.bin"
```

5. There should be a ``rust_model.ot`` file in the model directory upon completion. If not/there was an error, try to install a 2.2.0 version of PyTorch and try again, or view the error message.