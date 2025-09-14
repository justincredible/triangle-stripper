# Triangle Stripper

Determine if a triangle list can be stripped

Usage: triangle_stripper [OPTIONS] <INDEX_LIST>...

Arguments:
  <INDEX_LIST>...  Excess arguments (mod 3) will be ignored

Options:
  -e, --echo-input  Print the parsed input
  -h, --help        Print help
  -V, --version     Print version

## Limitations

- Assumes distinct indices represent distinct vertex positions
- Assumes face culling will be enabled, so reverse winding is not considered
- Superexponential in the worst case (O(n!)), however, this is mitigated by the next point
- Uses recursion to handle backtracking, so stack overflows can occur on large input sets
- False negatives are possible depending on how the vertices are tessellated (e.g., consider two pentagons sharing an edge)

