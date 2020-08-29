# PythonからRustの呼出し

## ダウンロード

```bash
$ git clone https://github.com/abenben/rust-test01.git
$ cd rust-test01
```

## コンパイル・実行

```bash
$ cargo build --release
$ cp target/release/libtest01.dylib test01.so
$ python main.py 
```

または

```bash
$ pip install setuptools_rust
$ python setup.py install
$ python main.py 
```

# ライブラリを新規から作る方法

```bash
$ cargo new rust-test01 --lib
Created library `rust-test01` package
```
