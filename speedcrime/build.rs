fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)  // speedcrime is client-only
        .build_client(true)
        .compile_protos(
            &[
                "../proto/common.proto",
                "../proto/auth.proto",
                "../proto/stokvels.proto",
                "../proto/contributions.proto",
                "../proto/wallet.proto",
                "../proto/market.proto",
                "../proto/fair_score.proto",
                "../proto/messages.proto",
                "../proto/fraud.proto",
            ],
            &["../"],
        )?;

    Ok(())
}
