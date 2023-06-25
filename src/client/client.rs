use tonic::transport::Channel;

use crate::auto_generate::api::{api_client::ApiClient, Req};

#[derive(Clone)]
pub struct Client {
    client: ApiClient<Channel>,
}

impl Client {
    pub async fn new(host: &'static str) -> Self {
        let channel = tonic::transport::Channel::from_static(host)
            .connect()
            .await
            .unwrap();

        let client = ApiClient::new(channel);
        Self { client: client }
    }

    pub async fn get_keys(
        &mut self,
        req: i32,
    ) -> Result<tonic::Response<crate::auto_generate::api::Resp>, tonic::Status> {
        let req = tonic::Request::new(Req { flag: req as u32 });
        let resp = self.client.get_keys(req).await;
        println!("{:?}", resp);
        resp
    }
}
