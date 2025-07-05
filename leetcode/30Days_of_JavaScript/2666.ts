type JSONValue =
  | null
  | boolean
  | number
  | string
  | JSONValue[]
  | { [key: string]: JSONValue };

type OnceFn = (...args: JSONValue[]) => JSONValue | undefined;

function once(fn: Function): OnceFn {
  let counter: number = 0;
  return function (...args) {
    if (counter === 0) {
      counter++;
      return fn(...args);
    }
    return undefined;
  };
}
