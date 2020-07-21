# complex-worker

Eg: `https://example.com/?fib=29&env=js`

`env` -> js = uses javascript func, else uses wasm (rust) func
`fib` -> fibonacci number to calculate. Uses quadratic algorithm, so keep it low (meant to be slow).
