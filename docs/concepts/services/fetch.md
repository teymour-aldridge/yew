---
id: fetch
title: The fetch service
---
## Introduction
The fetch service can be used to make HTTP requests to a server. This enables applications to
communicate with external services.

:::note
You might find it helpful to read the [documentation for the format module](format.md) before 
reading this page.
:::

## Making requests
### Building requests
Yew re-exports the `Request` struct from the `http` crate that is used to 'build' requests 
before they can be dispatched to a server. The value of the request body must implement 
`Into<Text>` or `Into<Binary>`. 
```rust
use yew::format::Nothing;
use yew::services::fetch::Request;
let get_request = Request::get("https://example.com/api/v1/get/something")
    .body(Nothing)
    .expect("Could not build that request");
```
```rust
use serde_json::json;
use yew::format::Json;
use yew::services::fetch::Request;
let post_request = Request::post("https://example.com/api/v1/post/something")
    .header("Content-Type", "application/json")
    .body(Json(&json!({"key": "value"})))
    .expect("Could not build that request.");
```

:::note
Note that the structs in the format module take references to values instead of values 
(i.e. `&T` not `T`).
:::

### Dispatching requests
The `FetchService` provides a binding to the browser's `fetch` API. Requests can be sent using 
either `FetchService::fetch` or `FetchService::fetch_with_options` (`fetch_with_options` should be 
used where cookies need to be sent in a request).

`FetchService::fetch` accepts two parameters: a `Request` object and a `Callback`. The `Callback` is
called once the request has completed allowing you to handle the data returned from the request.
The callback you pass needs to take a single parameter of type `Response<T>` where `T` is the body
of the response. Yew needs to be able to parse the response body to create an instance of the data
type `T` so it needs to implement `From<Text>` or `From<Binary>`. To fetch data in a binary format
you should use `FetchService::fetch_binary` rather than `FetchService::fetch`.

:::note
Because something could go wrong trying to deserialize data `From<Text>` and `From<Binary>` are only 
implemented for `FormatDataType<Result<T, ::anyhow::Error>>` (not `FormatDataType<T>`) where 
`FormatDataType` is used as a placeholder for any type in the format module (e.g. `Json`).

This means that your callbacks should look like
```rust
self.link.callback(|response: Json<anyhow::Result<ResponseType>>|)
```
rather than
```rust
self.link.callback(|response: Json<ResponseType>|)
```
:::

:::danger
It's important that the `FetchTask` isn't dropped until the request has completed. If the 
`FetchTask` is dropped before the request has finished then the request will be cancelled.
:::

:::important info
If you keep getting an error saying that "the operation was aborted" or "Error 408" this might be 
because the [CORS headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) of the website 
you are trying to access are not set correctly. Please see the linked article from Mozilla about
how to resolve these errors.
:::

An illustrated example of how to fetch data from an API giving information about the ISS's 
(International Space Station) location is given below.

```rust
// requires the serde and anyhow crates

use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Deserialize, Debug, Clone)]
pub struct ISSPosition {
    latitude: String,
    longitude: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ISS {
    message: String,
    timestamp: i32,
    iss_position: ISSPosition,
}

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<ISS, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    ft: Option<FetchTask>,
    iss: Option<ISS>,
    link: ComponentLink<Self>,
    error: Option<String>,
}
/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl FetchServiceExample {
    fn view_iss_location(&self) -> Html {
        match self.iss {
            Some(ref space_station) => {
                html! {
                    <>
                        <p>{ "The ISS is at:" }</p>
                        <p>{ format!("Latitude: {}", space_station.iss_position.latitude) }</p>
                        <p>{ format!("Longitude: {}", space_station.iss_position.longitude) }</p>
                    </>
                }
            }
            None => {
                html! {
                     <button onclick=self.link.callback(|_| Msg::GetLocation)>
                         { "Where is the ISS?" }
                     </button>
                }
            }
        }
    }
    fn view_fetching(&self) -> Html {
        if self.ft.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}
impl Component for FetchServiceExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ft: None,
            iss: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            GetLocation => {
                // 1. build the request
                let request = Request::get("http://api.open-notify.org/iss-now.json")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<ISS, anyhow::Error>>>| {
                            // split up the response into data about the request's status and the returned
                            // data from the request
                            let (_, Json(data)) = response.into_parts();
                            // the condition `data.message == "success" is specific to this API which
                            // returns a JSON object with a "message" key – not all APIs will do this!
                            Msg::ReceiveResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't cancelled immediately
                self.ft = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        self.iss = Some(location);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.ft = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_iss_location() }
                { self.view_error() }
            </>
        }
    }
}
```

## Debugging the `FetchService`

Most browsers' developer tools have a "network" panel which can be used to inspect HTTP requests. 
This can be used to gain insight into what requests are being made, the data being sent to the 
server, as well as the response.

The Rust Wasm Book also contains [useful debugging tips](https://rustwasm.github.io/book/reference/debugging.html)
for Wasm applications.

## Further reading
* [The API documentation](https://docs.rs/yew/0.14.3/yew/services/fetch/index.html)
* The [dashboard](https://github.com/yewstack/yew/tree/master/examples/dashboard) and 
[npm_and_rest](https://github.com/yewstack/yew/tree/master/examples/web_sys/npm_and_rest) examples.
* [The Rust Wasm Book on debugging Wasm applications](https://rustwasm.github.io/book/reference/debugging.html)
