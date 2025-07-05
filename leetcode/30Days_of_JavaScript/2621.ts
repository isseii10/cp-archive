async function sleep(millis: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, millis)
  })
}

// setTimeoutは渡した関数を指定ミリ秒待ってから実行する
// このとき、タスクキューに入れる

/** 
 * let t = Date.now()
 * sleep(100).then(() => console.log(Date.now() - t)) // 100
 */
