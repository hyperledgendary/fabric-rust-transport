fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .build_server(false)
    .compile(
        &["protos/common/common.proto",
            "protos/common/policies.proto","protos/ledger/queryresult/kv_query_result.proto","protos/msp/identities.proto",
            "protos/msp/msp_principal.proto",
            "protos/peer/chaincode.proto",
            "protos/peer/chaincode_event.proto",
            "protos/peer/chaincode_shim.proto",
            "protos/peer/proposal.proto",
            "protos/peer/proposal_response.proto",
            "protos/peer/transaction.proto",
           ],
            &["protos"],
    )?;
    Ok(())
}