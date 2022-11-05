# Quadtrees and Octrees

## 1.1 Definition

Binary tree -> Quadtree -> Octree.

## 1.2 Complexity and Construction

```admonish quote
The depth of a quadtree for a set \\(P\\) of points in the plane is at most \\(\log(s/c)+\frac{3}{2}\\), where \\(c\\) is the smallest distance between any two points in \\(P\\) and \\(s\\) is the side length of the initial square.
```

Neighbor finding: \\(\mathcal{O}(d+1)\\) time

Balancing a quadtree: \\(\mathcal{O}((d + 1)n)\\) time and \\(\mathcal{O}(n)\\) space, see [de Berg et al., 2008](https://link.springer.com/book/10.1007/978-3-662-04245-8).

## 1.3 Height Field Visualization

Example: Digital Elevation Model (DEM).

Height fields can be stored as *Triangular Irregular Networks* (TINs). TINs are efficient but not visualization-friendly.

Use quadtrees.

Problem: neighboring fine and coarse cells. This is called *T-vertices*.

Solution: triangulate each node. When subdividing one triangle, also subdivide the one on the otherside. This leads to a `4-8` division.

## 1.4 Isosurface Generation

Curvilinear grids (physical space) -> regular grids (computational space).

Octrees offer a simple way to computer isosurfaces efficiently. It helps to discard large parts of the volume where the isosurface is guaranteed to *not* be.

Each leaf holds the data range information for the eight nodes of the cell.

If the isosurface crosses an edge of a cell, that edge will be visited exactly four times. Use a hash table to memoize. And delete the entry after the fourth visit to keep the hash table small.

## 1.5 Ray Shooting

Partition the universe into a regular grid to avoid checking the ray against all objects.

## 1.6 3D Octree

Ray shooting with octrees:

- Bottom-up: starts at the leaf that contains the origin of the ray, and tries to find the next neighbor cell.
- Top-down: starts at the root and tries to find all the nodes that are stabbed by the ray.

A top-down method:

- Parameterize the ray as: \\(\vec{r}(t) = \vec{p} + t\vec{d}\\)
- Determine on which side the ray enters the current cell
- Determine the first subcell to visit

Negative directions can be handled efficiently by an XOR operation.

## 1.7 5D Octree

Rays are essentially **static** objects, just like the geometry of the scene!

Discretize the rays based on the direction cube. There is a one-to-one mapping between direction vectors and points on all six sides of the cube.

We need six 5D octrees, one for each side. For a given side, the \\(xyz\\)-intervals define a box, and the remaining \\(uv\\) intervals define a cone.
