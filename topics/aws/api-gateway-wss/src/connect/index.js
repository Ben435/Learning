const DDB = require('aws-sdk/clients/dynamodb');

const { TABLE_NAME, AWS_REGION } = process.env;

const ddb = new DDB.DocumentClient({ 
    apiVersion: '2012-08-10', 
    region: AWS_REGION 
});

exports.handler = async event => {
    const { connectionId } = event.requestContext

    console.log('New connection:', connectionId)

    return { statusCode: 200, body: 'Connected' }
}
