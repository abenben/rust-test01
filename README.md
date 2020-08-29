# PythonからRustの呼出し

## ダウンロード

```bash
$ git clone https://github.com/abenben/rust-test01.git
$ cd rust-test01
```

## コンパイル

```bash
$ cargo build --release
$ cp target/release/libtest01.dylib test01.so
```

または

```bash
$ pip install setuptools_rust
$ python setup.py install
```

## 実行

```bash
$ python main.py
Hello, world!
[0.0, 3.141592653589793, 6.283185307179586, 9.42477796076938, 12.566370614359172, 15.707963267948966, 18.84955592153876, 21.991148575128552, 25.132741228718345, 28.274333882308138]
10
[10, 20, 30, 40, 50, 60, 70, 80]
(4, 8)
```

# ライブラリを新規から作る方法

```bash
$ cargo new rust-test01 --lib
Created library `rust-test01` package
```
