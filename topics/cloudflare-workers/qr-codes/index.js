const qr = require('qr-image')

const generate = async request => {
  const body = await request.json()
  const text = body.text
  const qr_png = qr.imageSync(text || "http://www.google.com")

  return new Response(qr_png, { headers: { 'Content-Type': 'image/png' }})
}

const landing = `
<h1>QR Generator</h1>
<p>Click the below button to generate a new QR code. This will make a request to your serverless function.</p>
<input type="text" id="text" value="https://workers.dev"></input>
<button onclick='generate()'>Generate QR Code</button>
<p>Check the "Network" tab in your browser's developer tools to see the generated QR code.</p>
<img id="myImage"></img>
<script>
  function generate() {
    fetch(window.location.pathname, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text: document.querySelector("#text").value })
    })
    .then(resp => resp.blob())
    .then(blob => {
      const urlCreator = window.URL || window.webkitURL;
      document.getElementById('myImage').src = urlCreator.createObjectURL(blob)
    })
  }
</script>`

/**
 * Respond with hello worker text
 * @param {Request} request
 */
async function handleRequest(request) {
  if (request.method === 'POST') {
    return await generate(request)
  } else {
    return new Response(landing, { headers: { 'Content-Type': 'text/html' } })
  }
}

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})
