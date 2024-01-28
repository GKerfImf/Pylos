import { ReactNode } from "react";

class TypedMap<K, V> {
  private innerMap: Map<string, V>;

  constructor() {
    this.innerMap = new Map();
  }

  private getKeyString(key: K): string {
    return JSON.stringify(key);
  }

  set(key: K, value: V): this {
    const keyString = this.getKeyString(key);
    this.innerMap.set(keyString, value);
    return this;
  }

  get(key: K): V | undefined {
    const keyString = this.getKeyString(key);
    return this.innerMap.get(keyString);
  }

  has(key: K): boolean {
    const keyString = this.getKeyString(key);
    return this.innerMap.has(keyString);
  }

  delete(key: K): boolean {
    const keyString = this.getKeyString(key);
    return this.innerMap.delete(keyString);
  }

  get size(): number {
    return this.innerMap.size;
  }

  clear(): void {
    this.innerMap.clear();
  }

  map(f: (k: K, v: V) => ReactNode) {
    return Array.from(this.innerMap).map(([k, v]) => {
      return f(JSON.parse(k), v);
    });
  }
}

export default TypedMap;
