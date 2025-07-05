/**
 * @param {number[]} arr
 * @param {Function} fn
 * @return {number[]}
 */
function map(arr: number[], fn: (n: number, i: number) => number): number[] {
  const ret: number[] = [];
  for (let i = 0; i < arr.length; i++) {
    ret.push(fn(arr[i], i));
  }
  return ret;
}

// foreach
function map2(arr: number[], fn: (n: number, i: number) => number): number[] {
  const ret: number[] = [];
  arr.forEach((v, i) => {
    ret.push(fn(v, i));
  });
  return ret;
}
