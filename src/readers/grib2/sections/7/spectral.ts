import type {
  ComplexSpectralPackingTemplate,
  Grib2DataRepresentationSection,
  Reader,
} from '../../..';

/**
 * Unpack a data field that was packed using a simple packing algorithm, using info from the GRIB2
 * Data Representation Template 5.0.
 * @param reader - The raw data to convert
 * @param drs - The data representation section
 * @returns - The converted data
 */
export function spectralSimpleUnpacking(
  reader: Reader,
  drs: Grib2DataRepresentationSection,
): number[] {
  const { dataRepresentation, numberOfDataPoints: ndpts } = drs;
  const {
    numberOfBits: nbits,
    decimalScaleFactor,
    referenceValue: ref,
    binaryScaleFactor,
  } = dataRepresentation;
  const dscale = Math.pow(10, decimalScaleFactor);
  const bscale = Math.pow(2, binaryScaleFactor);

  const res: number[] = new Array(ndpts);

  // If nbits equals 0, we have a constant field where the reference value is the data value at
  // each gridpoint.
  if (nbits !== 0) {
    const ifld = gbits(reader, 0, nbits, 0, ndpts);
    for (let j = 0; j < ndpts; j++) res[j] = (ifld[j] * bscale + ref) * dscale;
  } else {
    for (let j = 0; j < ndpts; j++) res[j] = ref;
  }

  return res;
}

/**
 * @param reader - The raw data to convert
 * @param drs - The data representation section
 * @returns - The unpacked values
 */
export function spectralComplexUnpacking(
  reader: Reader,
  drs: Grib2DataRepresentationSection,
): number[] {
  // igdstmpl[0], igdstmpl[2], igdstmpl[2]
  const { dataRepresentation, numberOfDataPoints: ndpts } = drs;
  const {
    numberOfBits: nbits,
    decimalScaleFactor,
    referenceValue: ref,
    binaryScaleFactor,
    precision,
    Js,
    Ks,
    Ms,
    Ts,
    P,
  } = dataRepresentation as ComplexSpectralPackingTemplate;
  const dscale = Math.pow(10, decimalScaleFactor);
  const bscale = Math.pow(2, binaryScaleFactor);
  const JJ = ref;
  const KK = decimalScaleFactor;
  const MM = decimalScaleFactor;

  const res: number[] = new Array(ndpts);

  if (precision.code === 1) {
    /* unpacked floats are 32-bit IEEE */

    let ifld = gbits(reader, 0, 32, 0, Ts);
    const iofst = 32 * Ts;
    const unpk = rdieee(ifld, Ts); /* read IEEE unpacked floats */
    ifld = gbits(reader, iofst, nbits, 0, ndpts - Ts); /* unpack scaled data */

    // Calculate Laplacian scaling factors for each possible wave number.
    const pscale = new Array(JJ + MM + 1);
    const tscale = P * 1e-6;
    for (let n = Js; n <= JJ + MM; n++) pscale[n] = Math.pow(n * (n + 1), -tscale);

    /* Assemble spectral coeffs back to original order. */
    let inc = 0;
    let incu = 0;
    let incp = 0;
    for (let m = 0; m <= MM; m++) {
      let Nm = JJ; /* triangular or trapezoidal */
      if (KK === JJ + MM) Nm = JJ + m; /* rhombodial */
      let Ns = Js; /* triangular or trapezoidal */
      if (Ks === Js + Ms) Ns = Js + m; /* rhombodial */
      for (let n = m; n <= Nm; n++) {
        if (n <= Ns && m <= Ms) {
          /* grab unpacked value */
          res[inc++] = unpk[incu++]; /* real part */
          res[inc++] = unpk[incu++]; /* imaginary part */
        } else {
          /* Calc coeff from packed value */
          res[inc++] = (ifld[incp++] * bscale + ref) * dscale * pscale[n]; /* real part */
          res[inc++] = (ifld[incp++] * bscale + ref) * dscale * pscale[n]; /* imaginary part */
        }
      }
    }
  }

  return res;
}

/**
 * Unpack arbitrary size values from a packed bit string, right
 * justifying each value in the unpacked iout array.
 * @author NOAA Programmer
 * @param reader - Pointer to character array input.
 * @param iskip - Initial number of bits to skip.
 * @param nbits - Number of bits to take.
 * @param nskip - Additional number of bits to skip on each iteration.
 * @param n - Number of iterations.
 * @returns - The unpacked values
 */
function gbits(reader: Reader, iskip: number, nbits: number, nskip: number, n: number): number[] {
  const res: number[] = [];
  let i: number, tbit: number, bitcnt: number, ibit: number, itmp: number;
  let nbit: number, index: number;
  const ones: number[] = [1, 3, 7, 15, 31, 63, 127, 255];

  /* nbit is the start position of the field in bits */
  nbit = iskip;
  for (i = 0; i < n; i++) {
    bitcnt = nbits;
    index = nbit / 8;
    ibit = nbit % 8;
    nbit = nbit + nbits + nskip;

    /* first byte */
    tbit = bitcnt < 8 - ibit ? bitcnt : 8 - ibit; // find min
    itmp = reader.getInt8(index) & ones[7 - ibit];
    if (tbit !== 8 - ibit) itmp >>= 8 - ibit - tbit;
    index++;
    bitcnt = bitcnt - tbit;

    /* now transfer whole bytes */
    while (bitcnt >= 8) {
      itmp = (itmp << 8) | reader.getInt8(index);
      bitcnt = bitcnt - 8;
      index++;
    }

    /* get data from last byte */
    if (bitcnt > 0) {
      itmp = (itmp << bitcnt) | ((reader.getInt8(index) >> (8 - bitcnt)) & ones[bitcnt - 1]);
    }

    res[i] = itmp;
  }

  return res;
}

/**
 * Read a list of real values in 32-bit IEEE floating point format.
 * @author Stephen Gilbert @date 2002-10-25
 * @param rieee - g2int array of floating point values in 32-bit IEEE
 * format.
 * @param num Number of floating point values to convert.
 * @returns - The unpacked values
 */
function rdieee(rieee: number[], num: number): number[] {
  const res: number[] = [];

  let isign: number, iexp: number, imant: number, sign: number, temp: number;
  const two23 = 1.1920928955078125e-7; // Math.pow(2.0, -23);
  const two126 = 1.1754943508222875e-38; // Math.pow(2.0, -126);
  const msk1 = 0x80000000; /* 10000000000000000000000000000000 binary */
  const msk2 = 0x7f800000; /* 01111111100000000000000000000000 binary */
  const msk3 = 0x007fffff; /* 00000000011111111111111111111111 binary */

  for (let j = 0; j < num; j++) {
    /*  Extract sign bit, exponent, and mantissa */
    isign = (rieee[j] & msk1) >> 31;
    iexp = (rieee[j] & msk2) >> 23;
    imant = rieee[j] & msk3;
    /*printf("SAGieee= %ld %ld %ld\n",isign,iexp,imant); */

    sign = 1.0;
    if (isign === 1) sign = -1.0;

    if (iexp > 0 && iexp < 255) {
      temp = Math.pow(2.0, iexp - 127);
      res[j] = sign * temp * (1.0 + two23 * imant);
    } else if (iexp === 0) {
      if (imant !== 0) res[j] = sign * two126 * two23 * imant;
      else res[j] = sign * 0.0;
    } else if (iexp === 255) res[j] = sign * 1e37;
  }

  return res;
}
