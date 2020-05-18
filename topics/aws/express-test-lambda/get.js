const 
const AWS = require('aws-sdk');
const dynamodb = new AWS.DynamoDB();

const carsDbName = process.env.CarsDb;

exports.handler = async event => {
    console.log(`Ran get handler with: ${JSON.stringify(event)}`)

    if (!event.key) {
        throw Error("No key!")
    }

    return dynamodb
    .getItem({
        TableName: carsDbName,
        Key: {
            MakeId: {
                S: event.key
            },
            ModelId: {
                S: "test model"
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
