from distutils.core import setup
from Cython.Build import cythonize
import Cython
from distutils.extension import Extension
setup(
    ext_modules=cythonize([
    Extension("pytreexo", ["pytreexo.pyx"],
              libraries=[":libc_rustreexo.so"],
              library_dirs=["../"])
    ])
)
