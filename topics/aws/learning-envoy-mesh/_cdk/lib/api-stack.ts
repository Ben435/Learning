import { Duration, Stack, StackProps } from "aws-cdk-lib";
import { AppMeshProxyConfiguration, AwsLogDriver, Compatibility, ContainerImage, Ec2Service, NetworkMode, TaskDefinition } from "aws-cdk-lib/aws-ecs";
import { Construct } from "constructs";
import { InfraStack } from "./infra-stack";
import { Repository } from "aws-cdk-lib/aws-ecr";
import { HealthCheck, ServiceDiscovery, VirtualNode, VirtualNodeListener, VirtualService, VirtualServiceProvider } from "aws-cdk-lib/aws-appmesh";

const envoyVersion = '1.26.4.0'
const envoyUID = 1337
const namespace = 'envoy.learning'

// Based on https://github.com/nathanpeck/greeter-app-mesh-cdk/blob/master/index.js
export class APIStack extends Stack {
    constructor(scope: Construct, id: string, props: StackProps & { infraStack: InfraStack }) {
        super(scope, id, props)

        const mesh = props.infraStack.mesh

        const serviceName = 'api'

        const appECR = Repository.fromRepositoryName(this, 'APIECR', 'api')
        const appPort = 3000

        const taskDefinition = new TaskDefinition(this, 'APITaskDef', {
            compatibility: Compatibility.EC2,
            networkMode: NetworkMode.AWS_VPC,
            proxyConfiguration: new AppMeshProxyConfiguration({
                containerName: 'envoy',
                properties: {
                appPorts: [appPort],
                proxyEgressPort: 15001,
                proxyIngressPort: 15000,
                ignoredUID: envoyUID,
                egressIgnoredIPs: [
                    '169.254.170.2',
                    '169.254.169.254'
                ]
                }
            })
        })

        const apiContainer = taskDefinition.addContainer('APIContainer', {
            containerName: 'api',
            image: ContainerImage.fromEcrRepository(appECR, process.env.IMAGE_TAG),
            portMappings: [{
                containerPort: appPort,
                hostPort: appPort
            }],
            essential: true,
            memoryLimitMiB: 256,
            environment: {
                PORT: `${appPort}`,
            }
        })
        const envoyContainer = taskDefinition.addContainer('EnvoyContainer', {
            containerName: 'envoy',
            image: ContainerImage.fromRegistry(`public.ecr.aws/appmesh/aws-appmesh-envoy:arm64-v${envoyVersion}-prod`),
            essential: true,
            environment: {
                APPMESH_VIRTUAL_NODE_NAME: `mesh/${mesh.meshName}/virtualNode/${serviceName}`,
                AWS_REGION: Stack.of(this).region
            },
            healthCheck: {
                command: [
                  'CMD-SHELL',
                  'curl -s http://localhost:9901/server_info | grep state | grep -q LIVE'
                ],
                startPeriod: Duration.seconds(10),
                interval: Duration.seconds(5),
                timeout: Duration.seconds(2),
                retries: 3
              },
              memoryLimitMiB: 128,
              user: `${envoyUID}`,
              logging: new AwsLogDriver({
                streamPrefix: `${serviceName}-envoy`
              })
        })
        apiContainer.addContainerDependencies({ container: envoyContainer })

        const service = new Ec2Service(this, 'APIService', {
            cluster: props.infraStack.cluster,
            vpcSubnets: props.infraStack.vpc.selectSubnets(),
            taskDefinition,
            serviceName,
        })
        service.autoScaleTaskCount({
            minCapacity: 2,
            maxCapacity: 3
        })

        const virtualNode = new VirtualNode(this, `VirtualNode`, {
            mesh: mesh,
            virtualNodeName: serviceName,
            serviceDiscovery: ServiceDiscovery.dns('hostname'),
            listeners: [VirtualNodeListener.http({
                port: appPort,
                healthCheck: HealthCheck.http({
                    healthyThreshold: 2,
                    path: '/',
                    unhealthyThreshold: 2
                })
            })],
        });
    
        new VirtualService(this, `VirtualService`, {
            virtualServiceName: `${serviceName}.${namespace}`,
            virtualServiceProvider: VirtualServiceProvider.virtualNode(virtualNode)
        });
    }
}
