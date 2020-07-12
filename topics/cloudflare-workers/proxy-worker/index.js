addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

const baseHTML = () => `
<form>
  <input type="text" name="url">
  <input type="submit">
</form>
`

class ScriptHandler {
  element(element) {
    element.remove()
  }
}

class BodyHandler {
  element(element) {
    element.prepend(
      `<div style="position: absolute; background-color: white; width: 100%; height: 50px; z-index: 9999;">${baseHTML()}</div>`, 
      {html: true},
    )
  }
}

async function handleRequest(request) {
  if (request.method === "GET") {
    const queryStringLoc = request.url.lastIndexOf("?")

    if (queryStringLoc > 0) {
      const params = new URLSearchParams(request.url.slice(queryStringLoc+1))

      const url = params.get("url")
      if (url) {
        const content = await fetch(url)

        return new HTMLRewriter()
          .on('script', new ScriptHandler())
          .on('body', new BodyHandler())
          .transform(new Response(content.body, { status: 200, headers: content.headers }))
      }
    }
    
    return new Response(baseHTML(), { headers: { 'content-type': 'text/html' }})
  }
  return new Response('Not found', {
    status: 404,
    headers: { 'content-type': 'text/plain' },
  })
}
