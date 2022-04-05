from mpi4py import MPI
import numpy as np

from rusty_tree import ffi
from rusty_tree.distributed import DistributedTree

comm = MPI.COMM_WORLD
ptr = MPI._addressof(comm)
raw = ffi.cast('uintptr_t*', ptr)

points = np.random.rand(10, 3)
tree = DistributedTree.from_global_points(points, True, raw)


print(tree.keys[:1])

print(tree.points[0:1])