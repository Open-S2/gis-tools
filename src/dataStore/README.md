# DataBases

The value/purpose of the `kv`, `vector` and `multimap` structures is that almost all GIS computations are done in two stages: write all you data to the DB and then read requests after. So once we get to the reading stage, the DB is essentially immutable. This allows for a ton of optimization.
