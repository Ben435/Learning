#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib/core';
import { WidgetStack } from '../lib/widget-stack';

const app = new cdk.App();
new WidgetStack(app, 'WidgetStack');
