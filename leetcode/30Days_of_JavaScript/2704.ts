type ToBeOrNotToBe = {
  toBe: (val: any) => boolean;
  notToBe: (val: any) => boolean;
};

function expect(val: any): ToBeOrNotToBe {
  const throwError = (s: string) => {
    throw new Error(s);
  };
  return {
    toBe: (v) => v === val || throwError("Not Equal"),
    notToBe: (v) => v !== val || throwError("Equal"),
  };
}
