addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const fib = target => {
  if (target === 1) {
    return 1
  } else if (target === 0) {
    return 0
  } else {
    return fib(target-1) + fib(target-2)
  }
}

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { run } = wasm_bindgen;
  await wasm_bindgen(wasm)

  const queryStringLoc = request.url.lastIndexOf("?")

  let result = null

  if (queryStringLoc > 0) {
    const queryString = request.url.slice(queryStringLoc+1)
    const params = queryString.split("&").reduce((agg, keyVal) => {
      const [key, val] = keyVal.split("=")
      agg[key] = val

      return agg
    }, {})

    const param = Number.parseInt(params["fib"], 10)

    if (params["env"] == "js") {
      result = fib(param)
    } else {
      result = run(param)
    }
  }
  return new Response(JSON.stringify({result}), {status: 200})
}
