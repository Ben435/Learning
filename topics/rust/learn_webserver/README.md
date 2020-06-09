# learn_webserver

Super basic web server. Takes 2 requests then shuts down (to use graceful shutdown behaviour).

Takes in a request, reads the URI, and attempts to resolve that path in the `static` folder.
Doesn't check for `..` or links or any of that. If it doesn't find the file, loads `static/404.html`.

Uses a hand-rolled thread pool. Just "request -> push job down channel to _some_ worker -> hope it gets resolved".

# How to use:

* `cargo run`
* goto http://localhost:8080
