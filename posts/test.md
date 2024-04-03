---
title "How I made the blog"
draft true
---

My website is made in Rust. How did I make it? Let's figure out how it works by recreating it!

# A new project

```ansi
j0@j0s-MBP Projects % cargo new demo-website
(B[0;1m[32m     Created(B[0m binary (application) `demo-website` package
j0@j0s-MBP Projects % cd demo-website
```

Let's add `axum`! Axum is my favorite web framework, and it uses the `tokio` runtime, so let's add that too :)

```ansi
j0@j0s-MBP demo-website %(B[0m[3m[30m # -F is a shorthand for --features
(B[0mj0@j0s-MBP demo-website % cargo add axum tokio -F tokio/full
(B[0;1m[32m    Updating(B[0m crates.io index
(B[0;1m[32m      Adding(B[0m axum v0.7.4 to dependencies.
             Features:
             (B[0;1m[32m+(B[0m form
             (B[0;1m[32m+(B[0m http1
             (B[0;1m[32m+(B[0m json
             (B[0;1m[32m+(B[0m matched-path
             (B[0;1m[32m+(B[0m original-uri
             (B[0;1m[32m+(B[0m query
             (B[0;1m[32m+(B[0m tokio
             (B[0;1m[32m+(B[0m tower-log
             (B[0;1m[32m+(B[0m tracing
             (B[0;1m[31m-(B[0m __private_docs
             (B[0;1m[31m-(B[0m http2
             (B[0;1m[31m-(B[0m macros
             (B[0;1m[31m-(B[0m multipart
             (B[0;1m[31m-(B[0m ws
(B[0;1m[32m      Adding(B[0m tokio v1.36.0 to dependencies.
             Features:
             (B[0;1m[32m+(B[0m bytes
             (B[0;1m[32m+(B[0m fs
             (B[0;1m[32m+(B[0m full
             (B[0;1m[32m+(B[0m io-std
             (B[0;1m[32m+(B[0m io-util
             (B[0;1m[32m+(B[0m libc
             (B[0;1m[32m+(B[0m macros
             (B[0;1m[32m+(B[0m net
             (B[0;1m[32m+(B[0m num_cpus
             (B[0;1m[32m+(B[0m parking_lot
             (B[0;1m[32m+(B[0m process
             (B[0;1m[32m+(B[0m rt
             (B[0;1m[32m+(B[0m rt-multi-thread
             (B[0;1m[32m+(B[0m signal
             (B[0;1m[32m+(B[0m signal-hook-registry
             (B[0;1m[32m+(B[0m socket2
             (B[0;1m[32m+(B[0m sync
             (B[0;1m[32m+(B[0m time
             (B[0;1m[32m+(B[0m tokio-macros
             (B[0;1m[31m-(B[0m mio
             (B[0;1m[31m-(B[0m test-util
             (B[0;1m[31m-(B[0m tracing
             (B[0;1m[31m-(B[0m windows-sys
(B[0;1m[32m    Updating(B[0m crates.io index
j0@j0s-MBP demo-website %
```

## A simple webserver

Let's make a simple webserver with axum!

```rust
use axum::{routing::get, Router};

const URL: &str = "0.0.0.0:8000";
#[tokio::main]
async fn main() {

    let app = Router::new()
        // 👇 Here we serve a path with an anonymous function (closure)
        .route("/", get(|| async { "Hello world!" }));

    println!("Binding to {}", URL);
    let listener = tokio::net::TcpListener::bind(URL).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Let's run it:

```ansi
j0@j0s-MBP demo-website % cargo run
(B[0;1m[32m   Compiling(B[0m scopeguard v1.2.0
(B[0;1m[32m   Compiling(B[0m cfg-if v1.0.0
(B[0;1m[32m   Compiling(B[0m signal-hook-registry v1.4.1
(B[0;1m[32m   Compiling(B[0m num_cpus v1.16.0
(B[0;1m[32m   Compiling(B[0m parking_lot_core v0.9.9
(B[0;1m[32m   Compiling(B[0m lock_api v0.4.11
(B[0;1m[32m   Compiling(B[0m parking_lot v0.12.1
(B[0;1m[32m   Compiling(B[0m tokio v1.36.0
(B[0;1m[32m   Compiling(B[0m tokio-util v0.7.10
(B[0;1m[32m   Compiling(B[0m tower v0.4.13
(B[0;1m[32m   Compiling(B[0m h2 v0.4.3
(B[0;1m[32m   Compiling(B[0m hyper v1.2.0
(B[0;1m[32m   Compiling(B[0m hyper-util v0.1.3
(B[0;1m[32m   Compiling(B[0m axum v0.7.4
(B[0;1m[32m   Compiling(B[0m demo-website v0.1.0 (/Users/j0/Projects/demo-website)
(B[0;1m[32m    Finished(B[0m dev [unoptimized + debuginfo] target(s) in 8.77s
(B[0;1m[32m     Running(B[0m `target/debug/demo-website`
Binding to 0.0.0.0:8000
```

... and in another terminal:

```ansi
j0@j0s-MBP demo-website % curl localhost:8000
Hello world!(B[0;1;7m%(B[0m
```

Our webserver responsed!

## Basic HTML

Let's make our webserver serve webpages! In the root of your project, add an `index.html`:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width" />
    <title>Index</title>
  </head>

  <body>
    <h1>Hello world!</h1>
  </body>
</html>
```

Now, we need to serve this file on a route. To do this, we need a handler, which is a special function that returns a response, denoted by `impl IntoResponse`. If you are unfamiliar with this notation, we are essentially returning a generic that is constrained so that anything returned must implement the trait `IntoResponse`.

```rs
use std::fs::read_to_string;

use axum::{body::Body, http::StatusCode, response::IntoResponse, routing::get, Router};

const URL: &str = "0.0.0.0:8000";
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    println!("Binding to {}", URL);
    let listener = tokio::net::TcpListener::bind(URL).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {

    // 👇 Here we specify what we want to serve with a tuple
    (
        StatusCode::OK,
        // 👇 Here's how we set the body of the response!
        Body::new(read_to_string("index.html").expect("No index file!")),
    )
}
```

You may be wondering, how does a tuple turn into a response? Well,
<details>
<summary>it's complicated:</summary>
<p>If we look at the docs for <a href="https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html">axum::response::IntoResponse</a>, we find that it has a LOT of "Implementations on Foreign Types". The specific one our tuple is using is:
<pre>impl&LT;R&GT; IntoResponse for (StatusCode, R) <i>where R: IntoResponse</i></pre> In human terms, this means that <code>IntoResponse</code> is implemented on a tuple with types: <code>StatusCode</code>, and a type that implements <code>IntoResponse</code>. In our case, <code>R</code> is type <code>Body</code>, which implements <code>IntoResponse</code>! I hope that made sense!
</p>
</details>

Anyway, lets run our website!

```ansi
j0@j0s-MBP demo-website % cargo run
(B[0;1m[32m    Finished(B[0m dev [unoptimized + debuginfo] target(s) in 0.17s
(B[0;1m[32m     Running(B[0m `target/debug/demo-website`
Binding to 0.0.0.0:8000
```

And, go to http://localhost:8000:

<img src="https://f004.backblazeb2.com/file/j0-lol-website/hello-world.png" alt="'Hello world!' Displayed in a browser with no styling">

Awesome! We have a website!

# Templating

It's a bit inflexible though, right? For a blog, it would be nice to have a template we can drop posts into, instead of just writing HTML.

Let's add a templating system:
```ansi
j0@j0s-MBP demo-website % cargo add askama askama_axum -F askama/with-axum
(B[0;1m[32m    Updating(B[0m crates.io index
(B[0;1m[32m      Adding(B[0m askama v0.12.1 to dependencies.
             Features:
             (B[0;1m[32m+(B[0m config
             (B[0;1m[32m+(B[0m dep_humansize
             (B[0;1m[32m+(B[0m dep_num_traits
             (B[0;1m[32m+(B[0m humansize
             (B[0;1m[32m+(B[0m num-traits
             (B[0;1m[32m+(B[0m percent-encoding
             (B[0;1m[32m+(B[0m urlencode
             (B[0;1m[32m+(B[0m with-axum
             (B[0;1m[31m-(B[0m comrak
             (B[0;1m[31m-(B[0m markdown
             (B[0;1m[31m-(B[0m mime
             (B[0;1m[31m-(B[0m mime_guess
             (B[0;1m[31m-(B[0m serde
             (B[0;1m[31m-(B[0m serde-json
             (B[0;1m[31m-(B[0m serde-yaml
             (B[0;1m[31m-(B[0m serde_json
             (B[0;1m[31m-(B[0m serde_yaml
             (B[0;1m[31m-(B[0m with-actix-web
             (B[0;1m[31m-(B[0m with-gotham
             (B[0;1m[31m-(B[0m with-hyper
             (B[0;1m[31m-(B[0m with-mendes
             (B[0;1m[31m-(B[0m with-rocket
             (B[0;1m[31m-(B[0m with-tide
             (B[0;1m[31m-(B[0m with-warp
(B[0;1m[32m      Adding(B[0m askama_axum v0.4.0 to dependencies.
             Features:
             (B[0;1m[31m-(B[0m config
             (B[0;1m[31m-(B[0m humansize
             (B[0;1m[31m-(B[0m markdown
             (B[0;1m[31m-(B[0m num-traits
             (B[0;1m[31m-(B[0m serde-json
             (B[0;1m[31m-(B[0m serde-yaml
             (B[0;1m[31m-(B[0m urlencode
(B[0;1m[32m    Updating(B[0m crates.io index
```
Let's make a base for our pages:

```ansi
j0@j0s-MBP demo-website % mkdir templates
j0@j0s-MBP demo-website % touch templates/base.html
```

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width" />
    {% block title %}
    <title>Base</title>
    {% endblock %}
  </head>
  <body>
    {% block content %}Placeholder text{% endblock %}
  </body>
</html>
```

Now lets add an index:

```html
{% extends "base.html" %}
{% block title %}
<title>Index</title>
{% endblock %}

{% block content %}
<h1>Hello world!</h1>
{% endblock %}
```


---

todo:
- askama

---

## Parsing markdown

`Comrak` makes it very easy to parse markdown:

```rust
// Comrak
use comrak::{markdown_to_html, Options};

let html = markdown_to_html("Hello, **世界**!", &Options::default());
// "<p>Hello, <strong>世界</strong>!</p>\n"
```
