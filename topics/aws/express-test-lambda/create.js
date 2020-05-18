const AWS = require('aws-sdk');
const dynamodb = new AWS.DynamoDB();

const carsDbName = process.env.CarsDb;

exports.handler = async event => {
    console.log(`Ran create handler with: ${JSON.stringify(event)}`)

    if (!event.key) {
        throw Error("No key!")
    }

    return dynamodb
    .putItem({
        TableName: carsDbName,
        Item: {
            MakeId: {
                S: event.key
            },
            ModelId: {
                S: "test model"
            },
            Color: {
                S: "red"
            }
        }
    })
    .promise()
    .then(car => {
        console.log("Got car:", car);
        return {
            car, 
        }
    })
    .catch(e => {
        console.error("Error fetching car", e);
        return Promise.resolve(e);
    });
}
