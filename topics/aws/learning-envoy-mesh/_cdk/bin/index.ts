import { App, Stage, StageProps } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { APIStack } from '../lib/api-stack'
import { InfraStack } from '../lib/infra-stack'

class DeployStage extends Stage {
    constructor(scope: Construct, id: string, props: StageProps & Required<Pick<StageProps, 'env'>>) {
        super(scope, id, props)

        const infraStack = new InfraStack(this, 'InfraStack', {
            env: props.env
        })

        new APIStack(this, 'APIStack', {
            env: props.env,
            infraStack,
        })
    }
}

const app = new App()

new DeployStage(app, 'dev', {
    env: {
        account: process.env.CDK_DEFAULT_ACCOUNT,
        region: process.env.CDK_DEFAULT_REGION
    },
})
