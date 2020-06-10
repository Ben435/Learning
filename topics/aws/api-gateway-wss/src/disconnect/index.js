const DDB = require('aws-sdk/clients/dynamodb');

const { TABLE_NAME, AWS_REGION } = process.env;

const ddb = new DDB.DocumentClient({
    apiVersion: '2012-08-10', 
    region: AWS_REGION 
});

exports.handler = async event => {
    const { connectionId } = event.requestContext

    console.log('Disconnect connection:', connectionId)

    const deleteParams = {
        TableName: TABLE_NAME,
        Key: {
            connectionId
        }
    };

    try {
        await ddb.delete(deleteParams).promise();
    } catch (err) {
        console.error(err)
        return { statusCode: 500, body: 'Failed to connect to db: ' + JSON.stringify(err) };
    }

    return { statusCode: 200, body: 'Connected' }
}
