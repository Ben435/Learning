import * as cdk from "@aws-cdk/core";
import * as apigateway from "@aws-cdk/aws-apigateway";
import * as lambda from "@aws-cdk/aws-lambda";
import * as s3 from "@aws-cdk/aws-s3";
import { join } from 'path';

export class WidgetService extends cdk.Construct {
    constructor(scope: cdk.Construct, id: string) {
        super(scope, id);

        const bucket = new s3.Bucket(this, 'WidgetStore', {
            removalPolicy: cdk.RemovalPolicy.DESTROY
        });

        const handler = new lambda.Function(this, 'WidgetHandler', {
            runtime: lambda.Runtime.NODEJS_12_X,
            code: lambda.Code.fromAsset(join('resources')),
            handler: 'widgets.main',
            environment: {
                BUCKET: bucket.bucketName
            }
        });

        bucket.grantReadWrite(handler);

        const api = new apigateway.RestApi(this, 'widget-api', {
            restApiName: 'Widget Service',
        })

        const getWidgetsIntegration = new apigateway.LambdaIntegration(handler, {
            requestTemplates: { 'application/json': JSON.stringify({ statusCode: '200' })}
        })

        api.root.addMethod('GET', getWidgetsIntegration);
    }
}
