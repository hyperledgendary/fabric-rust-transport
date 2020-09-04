use protos::chaincode_support_client::ChaincodeSupportClient;
use protos::ChaincodeMessage;
use protos::chaincode_message::*;

pub mod protos {
    tonic::include_proto!("protos");
}
pub mod common {
    tonic::include_proto!("common");
}
pub mod msp {
    tonic::include_proto!("msp");
}
pub mod queryresult {
    tonic::include_proto!("queryresult");
}

pub mod api {
    tonic::include_proto!("google.protobuf");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChaincodeSupportClient::connect("http://[::1]:7052").await?;

    let outbound = async_stream::stream! {
        let request = ChaincodeMessage {
            r#type: 1,
            chaincode_event: Option::None,
            timestamp: Option::None,
            txid: "".to_string(),
            proposal: Option::None,
            payload: "chaincodeId".as_bytes().to_vec(),
            channel_id : "chanel".to_string()
       };

       // this is the place to send requests back to the peer
       // eg, return values of transaction functions or requests for
       // ledger api calls eg putState
       println!("{:?}",request);

       yield request;
    };

    let response = client.register(tonic::Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(note) = inbound.message().await? {
        // this is from the peer, and will be either a request for a new transaction or it will be
        // response to a ledger api eg getState

        // match and route answer here
        println!("NOTE = {:?}", note);
    }

    Ok(())
}