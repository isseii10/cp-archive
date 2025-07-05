class TimeLimitedCache {
  cache: { [key: string]: { value: number, expire: number } }
  constructor() {
    this.cache = {}
  }

  set(key: number, value: number, duration: number): boolean {
    const now = performance.now()
    const exist = !!this.cache[key]
    const isExpired = exist && this.cache[key].expire < now
    this.cache[key] = { value, expire: now + duration }
    return exist && !isExpired
  }

  get(key: number): number {
    const now = performance.now()
    if (!!this.cache[key] && this.cache[key].expire >= now) {
      return this.cache[key].value
    }
    return -1
  }

  count(): number {
    const now = performance.now()
    const vals = Object.values(this.cache)
    return vals.reduce((acc: number, cur: { value: number; expire: number }): number => {
      return cur.expire >= now ? acc + 1 : acc;
    }, 0);
  }
}
