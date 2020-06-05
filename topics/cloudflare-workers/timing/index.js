addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const sleep = async ms => {
  return new Promise(resolve => {
    setTimeout(resolve, ms)
  })
}

const fibonacciGen = end => {
  if (end <= 0) {
    return 0
  } else if (end === 1) {
    return 1
  } else {
    return fibonacciGen(end - 1) + fibonacciGen(end - 2)
  }
}

/**
 * Respond with hello worker text
 * @param {Request} request
 */
async function handleRequest(request) {
  const queryStringLoc = request.url.lastIndexOf("?")

  if (queryStringLoc > 0) {
    const queryString = request.url.slice(queryStringLoc+1)
    const params = queryString.split("&").reduce((agg, keyVal) => {
      const [key, val] = keyVal.split("=")
      agg[key] = val

      return agg
    }, {})

    const timeToWait = parseInt(params["timing"], 10)

    if (timeToWait) {
      await sleep(timeToWait)

      return new Response(JSON.stringify({"waited": timeToWait}), {
        headers: {'content-type': 'application/json'}
      })
    }

    const fibonacci = parseInt(params["fib"], 10)
    if (fibonacci) {
      const res = fibonacciGen(fibonacci)

      return new Response(JSON.stringify({"fibbedFor": fibonacci, "result": res}), {
        headers: {'content-type': 'application/json'}
      })
    }

    return new Response(JSON.stringify({"got": params}), {
      headers: {'content-type': 'application/json'}
    }) 
  }

  return new Response('Hello worker!', {
    headers: { 'content-type': 'text/plain' },
  })
}
