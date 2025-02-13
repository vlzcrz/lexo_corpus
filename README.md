install rust (by rustup)
install gcc compiler (varies on OS)
install python (> 3.8)

also needs to install python-dev libraries to set up python binding with rust

for Ubuntu
sudo apt install python3-dev

create venv (python environment) (pref: "lexo_corpus_env")
once installed activate env
inside the created env install listed depedencies

1.- pip install maturin
2.- pip install --upgrade pymupdf

---

rename library to tetlib and paste inside ./python/ folder's project. Rust auto insert library path so the venv allows to use it.

to run the program, you must be inside the env and do

cargo run

because to compile and generate the binary file rust must know where the python interpreter is located and that's where maturin do the task for us.

TODO: REWRITE AND FORMAT README.md
