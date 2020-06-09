# ApiGateway Wss Experiment

Creating a websocket ApiGateway API.

## Project Structure

* `src/${PROJECT}/**` - a single Lambda function.
* `template.yml` - CloudFormation template. Links it all together

## Build Process

This ones a bit weird, due to how lambdas work with CloudFormation, but it _sorta_ makes sense.

0. `npm run build` - packages each project in its own `out/${PROJECT}.zip`.
0. `npm run stack:package` - runs `aws cloudformation package`, to upload the zips to S3 and fix the references in `template.yml`, creates `package.yml`.
0. `npm run stack:deploy` - runs `aws cloudformation deploy`, to deploy the updated `package.yml` template.
