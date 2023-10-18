import { Stack, StackProps } from "aws-cdk-lib";
import { Mesh } from "aws-cdk-lib/aws-appmesh";
import { InstanceClass, InstanceSize, InstanceType, Vpc } from "aws-cdk-lib/aws-ec2";
import { Cluster } from "aws-cdk-lib/aws-ecs";
import { Construct } from "constructs";

export class InfraStack extends Stack {
    public readonly vpc: Vpc
    public readonly cluster: Cluster
    public readonly mesh: Mesh

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id, props)

        this.vpc = new Vpc(this, 'Vpc')
        this.cluster = new Cluster(this, 'Cluster', {
            vpc: this.vpc,
            capacity: {
                instanceType: InstanceType.of(InstanceClass.T4G, InstanceSize.NANO),
                spotPrice: "0.002",
            }
        })

        this.mesh = new Mesh(this, 'Mesh')
    }
}
