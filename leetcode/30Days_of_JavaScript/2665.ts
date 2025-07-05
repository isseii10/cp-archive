type Counter = {
  increment: () => number;
  decrement: () => number;
  reset: () => number;
};

function createCounter(init: number): Counter {
  let n = init;
  return {
    increment: () => {
      return ++n;
    },
    decrement: () => {
      return --n;
    },
    reset: () => {
      n = init;
      return n;
    },
  };
}
