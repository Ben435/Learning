import { RemovalPolicy, Stack, StackProps } from "aws-cdk-lib";
import { Repository } from "aws-cdk-lib/aws-ecr";
import { Construct } from "constructs";

export class ECRStack extends Stack {
    public readonly apiECR: Repository

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id, props)

        this.apiECR = new Repository(this, 'ApiECR', {
            repositoryName: 'api',
            removalPolicy: RemovalPolicy.DESTROY,
        })
    }
}
