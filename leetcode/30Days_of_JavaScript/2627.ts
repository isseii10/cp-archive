type F = (...args: number[]) => void

function debounce(fn: F, t: number): F {
  let prevTimerId: ReturnType<typeof setTimeout>;
  return function(...args) {
    clearTimeout(prevTimerId)
    // At first, clearTimeout(undefined) does nothing.
    // At second or more, clearTimeout(prevTimerId) does nothing if the previous timer did fn() 
    // and clearTimeout(prevTimerId) clear the previous timer if it did not yet.
    prevTimerId = setTimeout(fn, t, ...args)
  }
};

/**
 * const log = debounce(console.log, 100);
 * log('Hello'); // cancelled
 * log('Hello'); // cancelled
 * log('Hello'); // Logged at t=100ms
 */
