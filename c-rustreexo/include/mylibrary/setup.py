from distutils.core import setup
from Cython.Build import cythonize
import Cython
from distutils.extension import Extension
setup(
    ext_modules=cythonize([
    Extension("mylibrary", ["mylibrary.pyx"],
              libraries=[":libc_rustreexo.so"])
    ])
)
