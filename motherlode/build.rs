fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile all proto files into Rust code
    // Output goes to OUT_DIR, included via tonic::include_proto!

    tonic_build::configure()
        .build_server(true)
        .build_client(false) // motherlode is server-only
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
            &["../"], // include root so imports resolve
        )?;

    Ok(())
}
