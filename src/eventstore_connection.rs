use eventstore::{ Client, EventData };
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    is_event_store_a_good_db: bool,
}

#[tokio::main]
pub async fn connect_event_store() -> Result<(), Box<dyn std::error::Error>>{
    let settings = "esdb://127.0.0.1:2113?tls=false&keepAliveTimeout=10000&keepAliveInterval=10000".parse()?;

    let client = Client::new(settings)?;

    let payload = Foo {
        is_event_store_a_good_db: true,
    };

    let evt = EventData::json("language-poll", &payload)?;

    client
        .append_to_stream("language-stream", &Default::default(), evt)
        .await?;

    let mut stream = client
        .read_stream("language-stream", &Default::default())
        .await?;

    while let Some(event) = stream.next().await? {
        let event = event.get_original_event()
            .as_json::<Foo>()?;

        println!("{:?}", event);
    }

    Ok(())

}  