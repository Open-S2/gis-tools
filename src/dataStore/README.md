# DataBases

The value of the `kv` and `multimap` structures is that almost all GIS computations are done in two stages: write all then read. So once we get to the reading stage, the DB is essentially immutable. This allows a ton of optimization.
