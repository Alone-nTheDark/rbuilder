//! Additional Node command arguments.
//!
//! Copied from OptimismNode to allow easy extension.

//! clap [Args](clap::Args) for optimism rollup configuration

use std::path::PathBuf;

/// Parameters for rollup configuration
#[derive(Debug, Clone, Default, PartialEq, Eq, clap::Args)]
#[command(next_help_heading = "Rollup")]
pub struct OpRbuilderArgs {
    /// HTTP endpoint for the sequencer mempool
    #[arg(long = "rollup.sequencer-http", value_name = "HTTP_URL")]
    pub sequencer_http: Option<String>,

    /// Disable transaction pool gossip
    #[arg(long = "rollup.disable-tx-pool-gossip")]
    pub disable_txpool_gossip: bool,

    /// Enable walkback to genesis on startup. This is useful for re-validating the existing DB
    /// prior to beginning normal syncing.
    #[arg(long = "rollup.enable-genesis-walkback")]
    pub enable_genesis_walkback: bool,

    /// By default the pending block equals the latest block
    /// to save resources and not leak txs from the tx-pool,
    /// this flag enables computing of the pending block
    /// from the tx-pool instead.
    ///
    /// If `compute_pending_block` is not enabled, the payload builder
    /// will use the payload attributes from the latest block. Note
    /// that this flag is not yet functional.
    #[arg(long = "rollup.compute-pending-block")]
    pub compute_pending_block: bool,

    /// enables discovery v4 if provided
    #[arg(long = "rollup.discovery.v4", default_value = "false")]
    pub discovery_v4: bool,

    /// Enable the engine2 experimental features on op-reth binary
    #[arg(long = "engine.experimental", default_value = "false")]
    pub experimental: bool,

    /// Enable the engine2 experimental features on op-reth binary
    #[arg(long = "rbuilder.config")]
    pub rbuilder_config_path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Args, Parser};

    /// A helper type to parse Args more easily
    #[derive(Parser)]
    struct CommandParser<T: Args> {
        #[command(flatten)]
        args: T,
    }

    #[test]
    fn test_parse_optimism_default_args() {
        let default_args = OpRbuilderArgs::default();
        let args = CommandParser::<OpRbuilderArgs>::parse_from(["reth"]).args;
        assert_eq!(args, default_args);
    }

    #[test]
    fn test_parse_optimism_walkback_args() {
        let expected_args = OpRbuilderArgs {
            enable_genesis_walkback: true,
            ..Default::default()
        };
        let args = CommandParser::<OpRbuilderArgs>::parse_from([
            "reth",
            "--rollup.enable-genesis-walkback",
        ])
        .args;
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_parse_optimism_compute_pending_block_args() {
        let expected_args = OpRbuilderArgs {
            compute_pending_block: true,
            ..Default::default()
        };
        let args =
            CommandParser::<OpRbuilderArgs>::parse_from(["reth", "--rollup.compute-pending-block"])
                .args;
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_parse_optimism_discovery_v4_args() {
        let expected_args = OpRbuilderArgs {
            discovery_v4: true,
            ..Default::default()
        };
        let args =
            CommandParser::<OpRbuilderArgs>::parse_from(["reth", "--rollup.discovery.v4"]).args;
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_parse_optimism_sequencer_http_args() {
        let expected_args = OpRbuilderArgs {
            sequencer_http: Some("http://host:port".into()),
            ..Default::default()
        };
        let args = CommandParser::<OpRbuilderArgs>::parse_from([
            "reth",
            "--rollup.sequencer-http",
            "http://host:port",
        ])
        .args;
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_parse_optimism_disable_txpool_args() {
        let expected_args = OpRbuilderArgs {
            disable_txpool_gossip: true,
            ..Default::default()
        };
        let args = CommandParser::<OpRbuilderArgs>::parse_from([
            "reth",
            "--rollup.disable-tx-pool-gossip",
        ])
        .args;
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_parse_optimism_many_args() {
        let expected_args = OpRbuilderArgs {
            disable_txpool_gossip: true,
            compute_pending_block: true,
            enable_genesis_walkback: true,
            sequencer_http: Some("http://host:port".into()),
            ..Default::default()
        };
        let args = CommandParser::<OpRbuilderArgs>::parse_from([
            "reth",
            "--rollup.disable-tx-pool-gossip",
            "--rollup.compute-pending-block",
            "--rollup.enable-genesis-walkback",
            "--rollup.sequencer-http",
            "http://host:port",
        ])
        .args;
        assert_eq!(args, expected_args);
    }
}
