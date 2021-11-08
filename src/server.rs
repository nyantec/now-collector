use hyper::service::{service_fn, make_service_fn};
use hyper::{Request, Response, Body, Method, StatusCode};
use std::convert::Infallible;
use crate::State;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Server {
    listen_address: String,
    state: State,
}

impl Server {
    pub fn new(listen_address: String, state: State) -> Server {
        Server { listen_address, state }
    }

    pub async fn state_route(&self) -> Result<Response<Body>> {
        let content = {
            let state = self.state.0.read().await;
            serde_json::to_string(&*state)?
        };
        let resp = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(content.into())?;
        Ok(resp)
    }

    pub async fn handle(self, req: Request<Body>) -> Response<Body> {
        let method = req.method();
        let path = req.uri().path();
        let resp = match (method, path) {
            (&Method::GET, "/state.json") => self.state_route().await,
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("route not found".into())
                .unwrap()),
        };

        resp.unwrap_or_else(|e| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Error: {}", e).into())
                .unwrap()
        })
    }

    pub async fn run(self) -> Result<()> {
        let server = self.clone();
        let make_service = make_service_fn(move |_conn| {
            let server = server.clone();
            async {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let server = server.clone();
                    async {
                        Ok::<_, Infallible>(server.handle(req).await)
                    }
                }))
            }
        });

        hyper::Server::bind(&self.listen_address.parse()?).serve(make_service).await?;
        Ok(())
    }
}
