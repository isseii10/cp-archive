type Fn = (n: number, i: number) => any;

function filter(arr: number[], fn: Fn): number[] {
  const res: number[] = [];
  arr.forEach((n, i) => {
    if (fn(n, i)) {
      res.push(n);
    }
  });
  return res;
}
