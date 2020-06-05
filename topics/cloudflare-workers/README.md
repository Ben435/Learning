# Cloudflare Workers

Investigation into CloudFlare Workers, and the CloudFlare platform in general.

## Background

I know of CloudFlare as a security company, providing DDOS protection and various other high-bandwidth defense mechanisms.
In amongst that, they also do bot protection (ScrapeShield), content delivery (Stream Delivery for media, cache and hosting 
for general web traffic), web-app firewalls, among a few other things. They've also done some virus defense stuff in the past, 
providing DNS black holes and stepping in during large scale DDOS ransom attempts.

## Pricing

### Common to both plans:

* Deploy up to 30 workers
* Global deployments
* No "cold starts"
* Auto-scaled to required
* 128mb memory per worker
* Free usage of the `<username>.workers.dev` domain, to test deploy your workers too.

### Free plan

* 100'000 reqests per day (across _all_ your workers)
* Up to 10ms CPU time per request (reality ~20ms)
* First request _may_ be slow, subsequent will be lowest latency

### Paid plan

* 10'000'000 requests per month
* Up to 50ms CPU time per request (reality ~60ms)
* Every request will be lowest latency
* Access to worker KV storage (100 namespaces, 1gb storage 10mil read, 1mil write, 1mil delete, 1mil list, pay for more)

## Dev Experience

### `wrangler`

The CloudFlare version of `gcloud` or `aws` CLI's. Only used for CloudFlare workers and the worker KV database.
Can also generate template projects (`wrangler generate` for a full folder, `wrangler init` for just the `wrangler.toml` config file)

#### `wrangler preview`

Basic "npm start" equivilant. Builds and uploads the local code to a push-button-esque environment, exposing an "IDE" style environment.
Shows the response from the worker on the first page, a rudimentary request crafter + sender on the second, and an iframe to the CloudFormation docs on the third.

Need to re-deploy to see changes. On the free plan, can sometimes take a few requests to actually start responding 
(its got a some sort of warmup time).

![Wrangler-Preview-Example](./docs/wrangler-preview.png)

#### `wrangler publish`

Publishes to the domain + route specified in the `wrangler.toml`. If no domain or route provided, will deploy to `<project_name>.<account_username>.workers.dev`, a free domain provided under either plan.


