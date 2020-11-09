#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { WidgetStack } from '../lib/widget-stack';

const app = new cdk.App();
new WidgetStack(app, 'WidgetStack');
