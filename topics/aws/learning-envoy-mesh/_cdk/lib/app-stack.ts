import { Stack, StackProps } from "aws-cdk-lib";
import { Compatibility, ContainerImage, Ec2Service, TaskDefinition } from "aws-cdk-lib/aws-ecs";
import { Construct } from "constructs";
import { InfraStack } from "./infra-stack";
import { Repository } from "aws-cdk-lib/aws-ecr";

export class APIStack extends Stack {
    constructor(scope: Construct, id: string, props: StackProps & { infraStack: InfraStack, appECRARN: string }) {
        super(scope, id, props)

        const taskDefinition = new TaskDefinition(this, 'APITaskDef', {
            compatibility: Compatibility.EC2
        })

        const appECR = Repository.fromRepositoryArn(this, 'APIECR', props.appECRARN)
        
        taskDefinition.addContainer('APIContainer', {
            image: ContainerImage.fromEcrRepository(appECR, process.env.IMAGE_TAG)
        })

        new Ec2Service(this, 'APIService', {
            cluster: props.infraStack.cluster,
            vpcSubnets: props.infraStack.vpc.selectSubnets(),
            taskDefinition,
        })
    }
}
