# Pytreexo

## Introduction

Pytreexo is a collection of bindings over the rustreexo library. It provides all the functionality of rustreexo in python, with the speed of Rust.

## Installation

### Requirements

- Python 3.6+
- Rust 1.41+ (if you build from source)
- librustreexo (if you don't build from source)

### From PyPI

```bash
pip install pytreexo
```

### From source

Clone the repository and build librustreexo
```bash
git clone https://github.com/mit-dci/utreexo
cd utreexo/c-rustreexo
make install
```

Install the python lib
```
pip install ./pytreexo/
```

## Usage

```python
from pytreexo import Proof, Stump

# Create a new stump
s = Stump()

# Create a new proof
p = Proof()

# Add some elements
i = [bytes.fromhex("37cb3336f40b13170f5eb17b056adbee825c5287e9f1440ab63482aea51e85da")] * 100
update_data = s.modify(i, [], p)

# Update the proof to reflect the changes
proof.update(update_data, [], i, [], [0, 1, 2, 3])

# See if serialization rtt
assert Proof.parse(i) == proof
```

## Documentation

Documentation is available at [https://pytreexo.readthedocs.io/en/latest/](https://pytreexo.readthedocs.io/en/latest/)

## License

Pytreexo is licensed under the MIT license. See [LICENSE](LICENSE) for more information.
