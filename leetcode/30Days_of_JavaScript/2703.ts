type JSONValue =
  | null
  | boolean
  | number
  | string
  | JSONValue[]
  | { [key: string]: JSONValue };

function argumentsLength(...args: JSONValue[]): number {
  return args.length;
}

// argumentsで引数をまとめて取れる
// function argumentsLength(...args: JSONValue[]): number {
//   return arguments.length;
// }
