import { buildXmlElement } from '../../src/index.js';
import { describe, expect, it } from 'bun:test';

describe('buildXmlElement', () => {
  it('simple no key-values', () => {
    const converted = buildXmlElement('test');
    expect(converted).toEqual('<test />');
  });

  it('simple', () => {
    const converted = buildXmlElement('test', { foo: 'bar' });
    expect(converted).toEqual('<test foo="bar" />');
  });

  it('add inner', () => {
    const converted = buildXmlElement('test', { foo: 'bar', num: 3, inner: true }, ['<inner />']);
    expect(converted).toEqual('<test foo="bar" num="3" inner="true"><inner /></test>');
  });

  it('add multiple inner', () => {
    const converted = buildXmlElement('test', { foo: 'bar', num: 3, inner: true }, [
      '<inner />',
      '<inner />',
    ]);
    expect(converted).toEqual(
      '<test foo="bar" num="3" inner="true">\n\t<inner />\n\t<inner />\n</test>',
    );
  });

  it('add 3 layers deep with the correct indentation', () => {
    // Level 3 (Deepest)
    const leaf = buildXmlElement('leaf', { status: 'green' }, []);
    // Level 2
    const branch = buildXmlElement('branch', {}, [leaf]);
    // Level 1 (Root - defaults to 1, no need to pass it)
    const tree = buildXmlElement('tree', { id: 'forest-1' }, [branch]);
    expect(tree).toEqual(`<tree id="forest-1"><branch><leaf status="green" /></branch></tree>`);
  });
});
