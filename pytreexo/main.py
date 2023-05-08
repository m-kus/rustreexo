from mylibrary import NodeHash, Proof
import hashlib
import numpy as np
import array

i = hashlib.sha256()
i.update(b"Satoshi Nakamoto")
i1 = i.digest()

i.update(b"Bitcoin")
i2 = i.digest()

p = Proof().new([i1, i2], [2, 6, 7, 8, 9, 10])
