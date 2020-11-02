import * as cdk from '@aws-cdk/core';
import * as widget_service from './widget-service';

export class WidgetStack extends cdk.Stack {
    constructor(scope: cdk.App, id: string, props?: cdk.StackProps) {
        super(scope, id, props);
  
        new widget_service.WidgetService(this, 'Widgets');
    }
}
