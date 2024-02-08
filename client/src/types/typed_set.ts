import { ReactNode } from "react";

class TypedSet<V> {
  private innerSet: Set<string>;

  constructor(elements?: V[]) {
    // const neSet: Set<string> = new Set(elements?.map(this.getValueString) || []);
    this.innerSet = new Set(elements?.map(this.getValueString) || []);
  }

  private getValueString(value: V): string {
    return JSON.stringify(value);
  }
  private getValue(s: string): V {
    return JSON.parse(s);
  }

  add(value: V): this {
    const elementString = this.getValueString(value);
    this.innerSet.add(elementString);
    return this;
  }

  addImmutable(value: V): TypedSet<V> {
    const newSet: TypedSet<V> = { ...this };
    return newSet.add(value);
    // const valueString = this.getValueString(value);
    // return new TypedSet<V>(Array.from(this.innerSet).concat([valueString]).map(this.getValue));
  }

  has(value: V): boolean {
    const valueString = this.getValueString(value);
    return this.innerSet.has(valueString);
  }

  delete(value: V): boolean {
    const valueString = this.getValueString(value);
    return this.innerSet.delete(valueString);
  }

  get size(): number {
    return this.innerSet.size;
  }

  clear(): void {
    this.innerSet.clear();
  }

  map(f: (v: V) => ReactNode) {
    return Array.from(this.innerSet).map((v) => {
      return f(JSON.parse(v));
    });
  }
}

export default TypedSet;
