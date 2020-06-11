const DDB = require('aws-sdk/clients/dynamodb');
const ApiGatewayManagementApi = require('aws-sdk/clients/apigatewaymanagementapi')

const { TABLE_NAME, AWS_REGION } = process.env;

const ddb = new DDB.DocumentClient({
    apiVersion: '2012-08-10', 
    region: AWS_REGION 
});

exports.handler = async event => {
    const {
        domainName,
        connectionId
    } = event.requestContext;

    const senderConnectionId = connectionId;

    const { body } = event;

    const scanParams = {
        TableName: TABLE_NAME,
        ProjectionExpression: 'connectionId,username'
    };

    let connectionData;

    try {
        connectionData = await ddb.scan(scanParams).promise();
    } catch (err) {
        console.error('Error loading connections:', err)
        return { statusCode: 500, body: 'Failed to connect to db: ' + JSON.stringify(err) };
    }

    const apigwManagementApi = new ApiGatewayManagementApi({
        apiVersion: '2018-11-29',
        endpoint: `${domainName}`,
    });

    const username = connectionData.Items
        .find(({ connectionId }) => connectionId === senderConnectionId)
        .username

    const postData = JSON.parse(body).data;

    const postCalls = connectionData.Items.map(async ({ connectionId }) => {
        try {
            await apigwManagementApi
                .postToConnection({ ConnectionId: connectionId, Data: `${username}: ${postData}`})
                .promise();
        } catch (e) {
            if (e.statusCode === 410) {
                console.log(`Found stale connection, deleting ${connectionId}`);
                await ddb.delete({ TableName: TABLE_NAME, Key: { connectionId } }).promise();
            } else {
                throw e;
            }
        }
    });

    try {
        await Promise.all(postCalls);
    } catch (e) {
        console.error('Error sending wss messages:', e)
        return { statusCode: 500, body: e.stack };
    }

    return { statusCode: 200, body: 'Sent' };
}
