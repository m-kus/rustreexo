#!/usr/bin/env python

from setuptools import setup

LONG_DESCRIPTION = """# python-rustreexo
The Python language bindings for the [Rustreexo](https://github.com/mit-dci/rustreexo) library

## Install the package
```shell
pip install python-rustreexo
```

## Simple example
```python
import python-rustreexo as rustreexo
"""

setup(
    name="python-rustreexo",
    version="0.1.0",
    description="The Python language bindings for the rustreexo library",
    long_description=LONG_DESCRIPTION,
    long_description_content_type="text/markdown",
    include_package_data = True,
    zip_safe=False,
    packages=["rustreexo-library"],
    package_dir={"rustreexo-library": "./src/"},
    url="https://github.com/mit-dci/rustreexo",
    license="MIT",
    # This is required to ensure the library name includes the python version, abi, and platform tags
    # See issue #350 for more information
    has_ext_modules=lambda: True,
)
