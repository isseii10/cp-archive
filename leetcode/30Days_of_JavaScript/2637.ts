type Fn2637 = (...params: any[]) => Promise<any>;

function timeLimit(fn: Fn2637, t: number): Fn2637 {
  return async function(...args) {
    return new Promise((resolve, reject) => {
      const timerId = setTimeout(() => {
        reject("Time Limit Exceeded")
      }, t)

      fn(...args)
        .then((res) => {
          clearTimeout(timerId)
          resolve(res)
        })
        .catch((err) => {
          clearTimeout(timerId)
          reject(err)
        })
    })
  }
};
