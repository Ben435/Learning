# How to run
Example:
```bash
STACKNAME=ecs-exp-v3 npm run build-deploy
```

That will process ecs.config and template.yml file, outputting the processed files to `out/*`.

Then, upload the `out/ecs.config` to `s3://deploy-temp/<STACKNAME>-ecs.config`.
Then, it will deploy the cloud formation `template.yml`. 

STACKNAME -> used for name of stack, and a variety of parameters. eg: used in domain, security groups, anything involving a unique name
