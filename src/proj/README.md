# PROJ TODO

- [ ] pipelines
- - [ ] axisswap (+step +proj=axisswap +order=2,1)
- - [ ] Geocentric Latitude (+proj=geoc) or (+step +proj=geoc +inv)
- - [ ] Geographic offsets (+proj=geogoffset +dlon=-13.97 +dlat=7.94 +dh=26.9)
- - [ ] General grid shift (+proj=gridshift +grids=us_noaa_nadcon5_nad83_2007_nad83_2011_conus.tif)
- - [ ] Horizontal grid shift (+proj=hgridshift +grids=us_noaa_nadcon5_nad83_2007_nad83_2011_conus.tif)
- - [ ] unitconvert (+step +proj=unitconvert +xy_in=rad +xy_out=deg)

PJCoordOperation (proj_internal.h)

createFromWKTString steps:

- [ ] WKTParser().attachDatabaseContext(dbContext).createFromWKT(...)
