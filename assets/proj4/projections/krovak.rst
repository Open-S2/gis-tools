.. _krovak:

********************************************************************************
Krovak
********************************************************************************

+---------------------+----------------------------------------------------------+
| **Classification**  | Conformal Conical                                        |
+---------------------+----------------------------------------------------------+
| **Available forms** | Forward and inverse, spherical and ellipsoidal           |
+---------------------+----------------------------------------------------------+
| **Defined area**    | Global, but more accurate around Czech Republic and      |
|                     | Slovakia                                                 |
+---------------------+----------------------------------------------------------+
| **Alias**           | krovak                                                   |
+---------------------+----------------------------------------------------------+
| **Domain**          | 2D                                                       |
+---------------------+----------------------------------------------------------+
| **Input type**      | Geodetic coordinates                                     |
+---------------------+----------------------------------------------------------+
| **Output type**     | Projected coordinates                                    |
+---------------------+----------------------------------------------------------+


.. figure:: ./images/krovak.png
   :width: 500 px
   :align: center
   :alt:   Krovak

   proj-string: ``+proj=krovak``

By default, coordinates in the forward direction are output in easting, northing,
and negative in the Czech Republic and Slovakia, with absolute value of
easting/westing being smaller than absolute value of northing/southing.

See also :ref:`mod_krovak` for a variation of Krovak used with the S-JTSK/05 datum
in the Czech Republic.

.. note:: Before PROJ 9.4, using other values for x_0 or y_0 than the default 0
          would lead to incorrect results when not using the ``+czech`` switch.

Parameters
################################################################################

.. note:: All parameters are optional for the Krovak projection.

          The latitude of pseudo standard parallel is hardcoded to 78.5° and
          the ellipsoid to Bessel.

.. option:: +czech

    Reverse the sign of the output coordinates, as is tradition in the
    Czech Republic, to be westing, southing (positive values in Czech Republic
    and Slovakia).

.. option:: +lon_0=<value>

    Longitude of projection center.

    *Defaults to 24°50' (24.8333333333333)*

.. option:: +lat_0=<value>

    Latitude of projection center.

    *Defaults to 49.5*

.. option:: +k_0=<value>

    Scale factor. Determines scale factor used in the projection.

    *Defaults to 0.9999*

.. include:: ../options/x_0.rst

.. include:: ../options/y_0.rst
