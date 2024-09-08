const numArray: number[] = new Array(1_000_000);
const bigintArray: bigint[] = new Array(1_000_000);
const uint64Array: BigUint64Array = new BigUint64Array(1_000_000);
const float64Array: Float64Array = new Float64Array(1_000_000);

// Initialize the array with random numbers
for (let i = 0; i < numArray.length; i++) {
  numArray[i] = Math.random() * 1_000_000;
}

// Initialize the array with random BigInts
for (let i = 0; i < bigintArray.length; i++) {
   bigintArray[i] = BigInt(Math.floor(Math.random() * 1_000_000));
}

// Initialize the array with random BigUint64s
for (let i = 0; i < uint64Array.length; i++) {
  uint64Array[i] = BigInt(Math.floor(Math.random() * 1_000_000));
}

const firstElement = numArray[0];
let greaterCount = 0;
let lessCount = 0;

let start = Bun.nanoseconds();
// Compare each element with the first element
for (let i = 1; i < numArray.length; i++) {
  if (numArray[i] > firstElement) {
    greaterCount++;
  } else if (numArray[i] < firstElement) {
    lessCount++;
  }
}
let end = Bun.nanoseconds();
console.log(`Number: ${(end - start) / 1e6} ms`);

const firstBigint = bigintArray[0];
let greaterCountBigint = 0;
let lessCountBigint = 0;

start = Bun.nanoseconds();
// Compare each element with the first element
for (let i = 1; i < bigintArray.length; i++) {
  if (bigintArray[i] > firstBigint) {
    greaterCountBigint++;
  } else if (bigintArray[i] < firstBigint) {
    lessCountBigint++;
  }
}
end = Bun.nanoseconds();
console.log(`BigInt: ${(end - start) / 1e6} ms`);

const firstUint64 = uint64Array[0];
let greaterCountUint64 = 0;
let lessCountUint64 = 0;

start = Bun.nanoseconds();
// Compare each element with the first element
for (let i = 1; i < uint64Array.length; i++) {
  if (uint64Array[i] > firstUint64) {
    greaterCountUint64++;
  } else if (uint64Array[i] < firstUint64) {
    lessCountUint64++;
  }
}
end = Bun.nanoseconds();
console.log(`Uint64: ${(end - start) / 1e6} ms`);

const firstFloat64 = float64Array[0];
let greaterCountFloat64 = 0;
let lessCountFloat64 = 0;

start = Bun.nanoseconds();
// Compare each element with the first element
for (let i = 1; i < float64Array.length; i++) {
  if (float64Array[i] > firstFloat64) {
    greaterCountFloat64++;
  } else if (float64Array[i] < firstFloat64) {
    lessCountFloat64++;
  }
}
end = Bun.nanoseconds();
console.log(`Float64: ${(end - start) / 1e6} ms`);
