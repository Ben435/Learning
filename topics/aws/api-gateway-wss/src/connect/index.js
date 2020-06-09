exports.handler = async function(event, context) {
    console.log('Got:', event)
    return { statusCode: 200, body: 'Connected.' }
}
