# rust-test01

```bash
$ git clone https://github.com/abenben/rust-test01.git
$ cd rust-test01
```

```bash
$ cargo build --release
$ cp target/release/libtest01.dylib test01.so
$ python main.py 
```

または

```bash
$ cargo build --release
$ pip install setuptools_rust
$ python setup.py install
$ python main.py 
```
