# https://gis.stackexchange.com/questions/403432/how-do-i-dump-proj-epsg-database-to-proj4js-compatible-proj-strings/
# https://pyproj4.github.io/pyproj/stable/api/crs/coordinate_operation.html
import json
import pyproj

# Query CRS info
crs_list = pyproj.database.query_crs_info()

# Initialize TypeScript file content
ts_content = """// Auto-generated file from `tools/buildProjCrs.py`, do not edit

/**
 * WGS 84
 * - **GEOGRAPHIC_2D_CRS**: 4326
 * - **Projection** None
 * - **Area**: World
 * - **Unit**: degree
 * - **Ellipsoid**: WGS 84
 * - **Accuracy**: `null` (in metre)
 * - **Bounds**: `[-180.0, -90.0, 180.0, 90.0]`
 */
export const WGS84 =
  '+title=WGS 84 (long/lat) +proj=longlat +ellps=WGS84 +datum=WGS84 +units=degrees';
"""

crs_dict = {}
dest_crs = pyproj.CRS.from_epsg(4326)  # WGS84 (lat/lon)

# Loop through CRS info
for info in crs_list:
    try:
        # setup CRS
        crs = pyproj.CRS.from_authority(info.auth_name, info.code)
        proj4 = crs.to_proj4()
        name = f'{info.auth_name}_{info.code.replace('.', '_')}'

        # avoid duplicates
        if (name in crs_dict):
            continue
        crs_dict[name] = True

        # Accuracy
        transformer_accuracy = "`null`"  # Default to null if no valid accuracy
        # Only try to create transformer if transformation exists
        try:
            transformer = pyproj.Transformer.from_crs(crs, dest_crs)
            transformer_accuracy = transformer.accuracy if transformer.accuracy != -1 else "`null`"
            proj4 = transformer.to_proj4()
        except pyproj.exceptions.ProjError as e:
            # Handle transformation errors: set accuracy to null
            print(f"Skipping transformation for CRS {info.code}: {e}")

        # Description (if available)
        description = f"""
/**
 * {info.name if info.name else 'No description'}
 * - **{info.type.name}**: {info.code}
 * - **Projection** {info.projection_method_name}
 * - **Area**: {info.area_of_use.name if info.area_of_use else 'Unknown area'}
 * - **Unit**: {crs.axis_info[0].unit_name if crs.axis_info[0].unit_name else '`null`'}
 * - **Ellipsoid**: {crs.ellipsoid.name if crs.ellipsoid else '`null`'}
 * - **Accuracy**: {transformer_accuracy} (in metre)
 * - **Bounds**: `[{info.area_of_use.west}, {info.area_of_use.south}, {info.area_of_use.east}, {info.area_of_use.north}]`
 */
export const {name} = '{proj4}';
"""

        ts_content += description
    except pyproj.exceptions.CRSError as e:
        # Handle CRS that can't be converted to a PROJ string
        print(f"Skipping CRS {info.code}: {e}")


# Output to a .ts file
with open("definitions.ts", "w") as f:
    f.write(ts_content)

print("crs_definitions.ts has been generated.")
