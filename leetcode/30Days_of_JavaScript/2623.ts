type Fn = (...params: number[]) => number;

function memoize(fn: Fn): Fn {
  const memo = new Map<string, number>();
  return function (...args): number {
    const s = args.join(",");
    let res = memo.get(s);
    if (res !== undefined) {
      return res;
    }
    res = fn(...args);
    memo.set(s, res);
    return res;
  };
}

/**
 * let callCount = 0;
 * const memoizedFn = memoize(function (a, b) {
 *	 callCount += 1;
 *   return a + b;
 * })
 * memoizedFn(2, 3) // 5
 * memoizedFn(2, 3) // 5
 * console.log(callCount) // 1
 */
