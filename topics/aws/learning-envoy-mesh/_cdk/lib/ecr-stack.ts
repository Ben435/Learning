import { RemovalPolicy, Stack, StackProps, Tags } from "aws-cdk-lib";
import { Repository } from "aws-cdk-lib/aws-ecr";
import { Construct } from "constructs";

export class ECRStack extends Stack {
    public readonly appECR: Repository

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id, props)

        this.appECR = new Repository(this, 'AppECR', {
            removalPolicy: RemovalPolicy.DESTROY,
        })

        Tags.of(this.appECR).add("service", "app")
    }
}
