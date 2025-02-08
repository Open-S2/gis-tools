/** A special buffer that stores a 32-bit number and can be converted to different types respecting bit positions */
export class U32I32F32 {
  private buffer: ArrayBuffer;
  private view: DataView;

  /**
   * @param value - a 32-bit number
   * @param type - the type of the number
   */
  constructor(value: number, type: 'u32' | 'i32' | 'f32' = 'u32') {
    this.buffer = new ArrayBuffer(4); // 4 bytes
    this.view = new DataView(this.buffer);
    this.set(value, type);
  }

  /**
   * @param value - a 32-bit number
   * @param type - the type of the number
   */
  set(value: number, type: 'u32' | 'i32' | 'f32'): void {
    if (type === 'u32') this.view.setUint32(0, value, true);
    else if (type === 'i32') this.view.setInt32(0, value, true);
    else if (type === 'f32') this.view.setFloat32(0, value, true);
  }

  /** @returns - the value as an unsigned 32-bit number */
  get u32(): number {
    return this.view.getUint32(0, true);
  }
  /** sets an unsigned 32-bit number */
  set u32(value: number) {
    this.view.setUint32(0, value, true);
  }

  /** @returns - the value as a signed 32-bit number */
  get i32(): number {
    return this.view.getInt32(0, true);
  }
  /** sets a signed 32-bit number */
  set i32(value: number) {
    this.view.setInt32(0, value, true);
  }

  /** @returns - the value as a 32-bit float */
  get f32(): number {
    return this.view.getFloat32(0, true);
  }
  /** sets a 32-bit float */
  set f32(value: number) {
    this.view.setFloat32(0, value, true);
  }
}

/** A special buffer that stores a 64-bit number and can be converted to different types respecting bit positions */
export class U64I64F64 {
  private buffer: ArrayBuffer;
  private view: DataView;

  /**
   * @param value - the value
   * @param type - the input type
   */
  constructor(value: number | bigint, type: 'u64' | 'i64' | 'f64' = 'u64') {
    this.buffer = new ArrayBuffer(8); // 8 bytes
    this.view = new DataView(this.buffer);
    this.set(value, type);
  }

  /**
   * @param value - the value
   * @param type - the input type
   */
  set(value: number | bigint, type: 'u64' | 'i64' | 'f64'): void {
    if (type === 'u64') this.view.setBigUint64(0, BigInt(value), true);
    else if (type === 'i64') this.view.setBigInt64(0, BigInt(value), true);
    else if (type === 'f64') this.view.setFloat64(0, Number(value), true);
  }

  /** @returns - the value as an unsigned 64-bit number */
  get u64(): bigint {
    return this.view.getBigUint64(0, true);
  }
  /** sets an unsigned 64-bit number */
  set u64(value: bigint | number) {
    this.view.setBigUint64(0, BigInt(value), true);
  }

  /** @returns - the value as a signed 64-bit number */
  get i64(): bigint {
    return this.view.getBigInt64(0, true);
  }
  /** sets a signed 64-bit number */
  set i64(value: number | bigint) {
    this.view.setBigInt64(0, BigInt(value), true);
  }

  /** @returns - the value as a 64-bit float */
  get f64(): number {
    return this.view.getFloat64(0, true);
  }
  /** sets a 64-bit float */
  set f64(value: number | bigint) {
    this.view.setFloat64(0, Number(value), true);
  }
}
