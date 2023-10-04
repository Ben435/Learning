import { Construct } from "constructs";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as s3 from "aws-cdk-lib/aws-s3";
import { RemovalPolicy } from "aws-cdk-lib";

export class WidgetService extends Construct {
    constructor(scope: Construct, id: string) {
        super(scope, id);

        const bucket = new s3.Bucket(this, 'WidgetStore', {
            removalPolicy: RemovalPolicy.DESTROY
        });

        const handler = new lambda.Function(this, 'WidgetHandler', {
            runtime: lambda.Runtime.NODEJS_18_X,
            code: lambda.Code.fromInline('function main(){}'),
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
