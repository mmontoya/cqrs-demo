use eventstore::Client;

pub fn get_client() -> Result<Client, Box<dyn std::error::Error>> {

    let settings = "esdb://127.0.0.1:2113?tls=false&keepAliveTimeout=10000&keepAliveInterval=10000".parse()?;

    let client = Client::new(settings)?;
    
    Ok(client)

}
