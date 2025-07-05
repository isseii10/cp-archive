function createCounter(n: number): () => number {
  return function () {
    return n++;
  };
}

// 後置インクリメントは先に評価してから、インクリメントする
// つまり、return n++は、インクリメント前のnを返し、その後n = n + 1をする
