# Timings worker

To figure out the real timing limits for cloudflare workers.

## Findings:

On paid plan ($5/month):

* 60ms max CPU time (fib=31 gets to ~57ms, fib=32 times out)
* Unlimited non-CPU time (browser timed out before the worker did)