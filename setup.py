from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='test01',
        version='0.1',
        rust_extensions=[
            RustExtension('test01', 'Cargo.toml',
                binding=Binding.PyO3)],
            zip_safe=False)
