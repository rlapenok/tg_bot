use tonic::{transport::Channel, Request, Status, Streaming};

use crate::auto_generate::api::{router_client::RouterClient, GetKeysResp, GlobalRequest};

#[derive(Clone)]
pub struct Client {
    client: RouterClient<Channel>,
}

impl Client {
    pub async fn new(port: String) -> Result<Self, anyhow::Error> {
        let host = format!("http://[::1]:{}", port);
        let endpoint = Channel::from_shared(host)?;
        let channel = endpoint.connect().await?;
        let client = RouterClient::new(channel);
        return Ok(Self { client: client });
    }

    pub async fn get_keys(&mut self) -> Result<Streaming<GetKeysResp>, Status> {
        let request = Request::new(GlobalRequest { req: 0 });
        let resp_stream = self.client.get_keys(request).await?;
        let stream_get_keys = resp_stream.into_inner();

        Ok(stream_get_keys)
    }
}
