import { expect as expectCDK, matchTemplate, MatchStyle } from '@aws-cdk/assert';
import * as cdk from 'aws-cdk-lib/core';
import { WidgetService } from '../lib/widget-service';

test('Empty Stack', () => {
    const app = new cdk.App();
    const stack = new WidgetService(app, 'MyTestStack');
    expectCDK(stack).to(matchTemplate({
      "Resources": {}
    }, MatchStyle.EXACT))
});
