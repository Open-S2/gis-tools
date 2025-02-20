<h1 style="text-align: center;">
  <div align="center">Protobuf Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/protobuf-file.svg" alt="protobuf-file-ts">
  <img src="../../assets/badges/protobuf-gzip.svg" alt="protobuf-gzip-ts">
  <img src="../../assets/badges/protobuf-brotli.svg" alt="protobuf-brotli-ts">
</p>

## Description

This module implements the [Protocol Buffer Format](https://protobuf.dev/) in a light weight, minimalistic, and efficient way.

This code is exposed from the [pbf repo](https://github.com/Open-S2/pbf).

## Usage

```ts
import { readFileSync } from 'fs';
import { Pbf } from 'gis-tools-ts';

// Reading:
const pbf = new Pbf(readFileSync(path));

// Writing:
const pbf = new Pbf();
pbf.writeVarintField(1, 1);
// ...
const result = pbf.commit();
```

If you want to reduce build size and know you're only reading data, not writing to it, use the PbfReader class:

```ts
import { readFileSync } from 'fs';
import { PbfReader } from 'gis-tools-ts';

const pbf = new PbfReader(readFileSync(path));
// ...
```

More complex example:

```ts
import { Pbf as Protobuf } from 'gis-tools-ts';

/** Building a class to test with. */
class Test {
    a = 0;
    b = 0;
    c = 0;
    /**
     * @param pbf - the Protobuf object to read from
     * @param end - the position to stop at
     */
    constructor(pbf: Protobuf, end = 0) {
        pbf.readFields(Test.read, this, end);
    }
    /**
     * @param t - the test object to write.
     * @param pbf - the Protobuf object to write to.
     */
    static writeMessage(t: Test, pbf: Protobuf): void {
        pbf.writeVarintField(1, t.a);
        pbf.writeFloatField(2, t.b);
        pbf.writeSVarintField(3, t.c);
    }

    /**
     * @param tag - the tag to read.
     * @param test - the test to modify
     * @param pbf - the Protobuf object to read from
     */
    static read(tag: number, test: Test, pbf: Protobuf): void {
        if (tag === 1) test.a = pbf.readVarint();
        else if (tag === 2) test.b = pbf.readFloat();
        else if (tag === 3) test.c = pbf.readSVarint();
        else throw new Error(`Unexpected tag: ${tag}`);
    }

    /**
     * @returns - a new test object
     */
    static newTest(): Test {
        return { a: 1, b: 2.2, c: -3 } as Test;
    }

    /**
     * @returns - a new default test object
     */
    static newTestDefault(): Test {
        return { a: 0, b: 0, c: 0 } as Test;
    }
}

// Writing the message
const pbf = new Protobuf();
const t = Test.newTest();
pbf.writeMessage(5, Test.writeMessage, t);
const data = pbf.commit();
expect(data).toEqual(new Uint8Array([42, 9, 8, 1, 21, 205, 204, 12, 64, 24, 5]));

// Reading the message
const pbf2 = new Protobuf(data);
expect(pbf2.readTag()).toEqual({ tag: 5, type: Protobuf.Bytes });
const t2 = new Test(pbf2, pbf2.readVarint() + pbf2.pos);
expect(t2).toEqual({ a: 1, b: 2.200000047683716, c: -3 } as Test);
```

## Useful links

- <https://protobuf.dev/>
