use async_nats::{Client, ConnectError, Message, ServerAddr};
use futures::StreamExt;

pub struct Server {
    outpost_name: String,
    nats_client: Client
}

impl Server {
    pub async fn setup(outpost_name: String, nats_url: ServerAddr) -> Result<Self, ConnectError> {
        let client = async_nats::connect(nats_url).await?;

        Ok(Server { outpost_name, nats_client: client })
    }

    pub async fn run(&self) -> Result<(), async_nats::SubscribeError>{

        let mut subscriber = self.nats_client.subscribe(format!("yawn.outposts.{}", self.outpost_name)).await?;

        while let Some(msg) = subscriber.next().await {
            println!("{:?}", msg);
            self.process_req(msg);
        }

        Ok(())
    }

    fn process_req(&self, msg: Message)  {
        todo!()
    }
}
