# GRIB2

## Usage

### Parse GRIB File

```typescript
import { GRIB2Reader } from 'gis-tools'

const gribData = new GRIB2Reader(await Bun.file('./path/to/file.grib2').arrayBuffer())
```

## TODO

- [ ] Section 3 templates - the projection data
- [ ] Section 4 templates - product information.
- [ ] Section 5 templates - how to decompress / variables for decompression
- [ ] Section 7 templates - decompression techniques (use section 5 templates for input variables as needed)

- table 3.0 explains the projection data usually grided with datum and projection type included
- table 5.0 (<https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml>) gives information on how to decode using unpack decision from table 7.0
- table 7.0 (<https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table7-0.shtml>) explains how to unpack data
