// This file is Copyright its original authors, visible in version control history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. You may not use this file except in
// accordance with one or both of these licenses.

use crate::payment::SendingParameters;

use lightning::ln::msgs::SocketAddress;
use lightning::routing::gossip::NodeAlias;
use lightning::util::config::UserConfig;
use lightning::util::logger::Level as LogLevel;

use bitcoin::secp256k1::PublicKey;
use bitcoin::Network;

use std::time::Duration;

// Config defaults
const DEFAULT_STORAGE_DIR_PATH: &str = "/tmp/ldk_node/";
const DEFAULT_NETWORK: Network = Network::Bitcoin;
const DEFAULT_BDK_WALLET_SYNC_INTERVAL_SECS: u64 = 80;
const DEFAULT_LDK_WALLET_SYNC_INTERVAL_SECS: u64 = 30;
const DEFAULT_FEE_RATE_CACHE_UPDATE_INTERVAL_SECS: u64 = 60 * 10;
const DEFAULT_PROBING_LIQUIDITY_LIMIT_MULTIPLIER: u64 = 3;
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Debug;
const DEFAULT_ANCHOR_PER_CHANNEL_RESERVE_SATS: u64 = 25_000;

// The 'stop gap' parameter used by BDK's wallet sync. This seems to configure the threshold
// number of derivation indexes after which BDK stops looking for new scripts belonging to the wallet.
pub(crate) const BDK_CLIENT_STOP_GAP: usize = 20;

// The number of concurrent requests made against the API provider.
pub(crate) const BDK_CLIENT_CONCURRENCY: usize = 4;

// The timeout after which we abandon retrying failed payments.
pub(crate) const LDK_PAYMENT_RETRY_TIMEOUT: Duration = Duration::from_secs(10);

// The interval (in block height) after which we retry archiving fully resolved channel monitors.
pub(crate) const RESOLVED_CHANNEL_MONITOR_ARCHIVAL_INTERVAL: u32 = 6;

// The time in-between peer reconnection attempts.
pub(crate) const PEER_RECONNECTION_INTERVAL: Duration = Duration::from_secs(10);

// The time in-between RGS sync attempts.
pub(crate) const RGS_SYNC_INTERVAL: Duration = Duration::from_secs(60 * 60);

// The time in-between node announcement broadcast attempts.
pub(crate) const NODE_ANN_BCAST_INTERVAL: Duration = Duration::from_secs(60 * 60);

// The lower limit which we apply to any configured wallet sync intervals.
pub(crate) const WALLET_SYNC_INTERVAL_MINIMUM_SECS: u64 = 10;

// The timeout after which we abort a wallet syncing operation.
pub(crate) const BDK_WALLET_SYNC_TIMEOUT_SECS: u64 = 90;

// The timeout after which we abort a wallet syncing operation.
pub(crate) const LDK_WALLET_SYNC_TIMEOUT_SECS: u64 = 30;

// The timeout after which we abort a fee rate cache update operation.
pub(crate) const FEE_RATE_CACHE_UPDATE_TIMEOUT_SECS: u64 = 5;

// The timeout after which we abort a transaction broadcast operation.
pub(crate) const TX_BROADCAST_TIMEOUT_SECS: u64 = 5;

// The timeout after which we abort a RGS sync operation.
pub(crate) const RGS_SYNC_TIMEOUT_SECS: u64 = 5;

// The length in bytes of our wallets' keys seed.
pub(crate) const WALLET_KEYS_SEED_LEN: usize = 64;

#[derive(Debug, Clone)]
/// Represents the configuration of an [`Node`] instance.
///
/// ### Defaults
///
/// | Parameter                              | Value              |
/// |----------------------------------------|--------------------|
/// | `storage_dir_path`                     | /tmp/ldk_node/     |
/// | `log_dir_path`                         | None               |
/// | `network`                              | Bitcoin            |
/// | `listening_addresses`                  | None               |
/// | `node_alias`                           | None               |
/// | `default_cltv_expiry_delta`            | 144                |
/// | `onchain_wallet_sync_interval_secs`    | 80                 |
/// | `wallet_sync_interval_secs`            | 30                 |
/// | `fee_rate_cache_update_interval_secs`  | 600                |
/// | `trusted_peers_0conf`                  | []                 |
/// | `probing_liquidity_limit_multiplier`   | 3                  |
/// | `log_level`                            | Debug              |
/// | `anchor_channels_config`               | Some(..)           |
/// | `sending_parameters`                   | None               |
///
/// See [`AnchorChannelsConfig`] and [`SendingParameters`] for more information regarding their
/// respective default values.
///
/// [`Node`]: crate::Node
pub struct Config {
	/// The path where the underlying LDK and BDK persist their data.
	pub storage_dir_path: String,
	/// The path where logs are stored.
	///
	/// If set to `None`, logs can be found in the `logs` subdirectory in [`Config::storage_dir_path`].
	pub log_dir_path: Option<String>,
	/// The used Bitcoin network.
	pub network: Network,
	/// The addresses on which the node will listen for incoming connections.
	///
	/// **Note**: We will only allow opening and accepting public channels if the `node_alias` and the
	/// `listening_addresses` are set.
	pub listening_addresses: Option<Vec<SocketAddress>>,
	/// The node alias that will be used when broadcasting announcements to the gossip network.
	///
	/// The provided alias must be a valid UTF-8 string and no longer than 32 bytes in total.
	///
	/// **Note**: We will only allow opening and accepting public channels if the `node_alias` and the
	/// `listening_addresses` are set.
	pub node_alias: Option<NodeAlias>,
	/// The time in-between background sync attempts of the onchain wallet, in seconds.
	///
	/// **Note:** A minimum of 10 seconds is always enforced.
	pub onchain_wallet_sync_interval_secs: u64,
	/// The time in-between background sync attempts of the LDK wallet, in seconds.
	///
	/// **Note:** A minimum of 10 seconds is always enforced.
	pub wallet_sync_interval_secs: u64,
	/// The time in-between background update attempts to our fee rate cache, in seconds.
	///
	/// **Note:** A minimum of 10 seconds is always enforced.
	pub fee_rate_cache_update_interval_secs: u64,
	/// A list of peers that we allow to establish zero confirmation channels to us.
	///
	/// **Note:** Allowing payments via zero-confirmation channels is potentially insecure if the
	/// funding transaction ends up never being confirmed on-chain. Zero-confirmation channels
	/// should therefore only be accepted from trusted peers.
	pub trusted_peers_0conf: Vec<PublicKey>,
	/// The liquidity factor by which we filter the outgoing channels used for sending probes.
	///
	/// Channels with available liquidity less than the required amount times this value won't be
	/// used to send pre-flight probes.
	pub probing_liquidity_limit_multiplier: u64,
	/// The level at which we log messages.
	///
	/// Any messages below this level will be excluded from the logs.
	pub log_level: LogLevel,
	/// Configuration options pertaining to Anchor channels, i.e., channels for which the
	/// `option_anchors_zero_fee_htlc_tx` channel type is negotiated.
	///
	/// Please refer to [`AnchorChannelsConfig`] for further information on Anchor channels.
	///
	/// If set to `Some`, we'll try to open new channels with Anchors enabled, i.e., new channels
	/// will be negotiated with the `option_anchors_zero_fee_htlc_tx` channel type if supported by
	/// the counterparty. Note that this won't prevent us from opening non-Anchor channels if the
	/// counterparty doesn't support `option_anchors_zero_fee_htlc_tx`. If set to `None`, new
	/// channels will be negotiated with the legacy `option_static_remotekey` channel type only.
	///
	/// **Note:** If set to `None` *after* some Anchor channels have already been
	/// opened, no dedicated emergency on-chain reserve will be maintained for these channels,
	/// which can be dangerous if only insufficient funds are available at the time of channel
	/// closure. We *will* however still try to get the Anchor spending transactions confirmed
	/// on-chain with the funds available.
	pub anchor_channels_config: Option<AnchorChannelsConfig>,
	/// Configuration options for payment routing and pathfinding.
	///
	/// Setting the `SendingParameters` provides flexibility to customize how payments are routed,
	/// including setting limits on routing fees, CLTV expiry, and channel utilization.
	///
	/// **Note:** If unset, default parameters will be used, and you will be able to override the
	/// parameters on a per-payment basis in the corresponding method calls.
	pub sending_parameters: Option<SendingParameters>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			storage_dir_path: DEFAULT_STORAGE_DIR_PATH.to_string(),
			log_dir_path: None,
			network: DEFAULT_NETWORK,
			listening_addresses: None,
			onchain_wallet_sync_interval_secs: DEFAULT_BDK_WALLET_SYNC_INTERVAL_SECS,
			wallet_sync_interval_secs: DEFAULT_LDK_WALLET_SYNC_INTERVAL_SECS,
			fee_rate_cache_update_interval_secs: DEFAULT_FEE_RATE_CACHE_UPDATE_INTERVAL_SECS,
			trusted_peers_0conf: Vec::new(),
			probing_liquidity_limit_multiplier: DEFAULT_PROBING_LIQUIDITY_LIMIT_MULTIPLIER,
			log_level: DEFAULT_LOG_LEVEL,
			anchor_channels_config: Some(AnchorChannelsConfig::default()),
			sending_parameters: None,
			node_alias: None,
		}
	}
}

/// Configuration options pertaining to 'Anchor' channels, i.e., channels for which the
/// `option_anchors_zero_fee_htlc_tx` channel type is negotiated.
///
/// Prior to the introduction of Anchor channels, the on-chain fees paying for the transactions
/// issued on channel closure were pre-determined and locked-in at the time of the channel
/// opening. This required to estimate what fee rate would be sufficient to still have the
/// closing transactions be spendable on-chain (i.e., not be considered dust). This legacy
/// design of pre-anchor channels proved inadequate in the unpredictable, often turbulent, fee
/// markets we experience today.
///
/// In contrast, Anchor channels allow to determine an adequate fee rate *at the time of channel
/// closure*, making them much more robust in the face of fee spikes. In turn, they require to
/// maintain a reserve of on-chain funds to have the channel closure transactions confirmed
/// on-chain, at least if the channel counterparty can't be trusted to do this for us.
///
/// See [BOLT 3] for more technical details on Anchor channels.
///
///
/// ### Defaults
///
/// | Parameter                  | Value  |
/// |----------------------------|--------|
/// | `trusted_peers_no_reserve` | []     |
/// | `per_channel_reserve_sats` | 25000  |
///
///
/// [BOLT 3]: https://github.com/lightning/bolts/blob/master/03-transactions.md#htlc-timeout-and-htlc-success-transactions
#[derive(Debug, Clone)]
pub struct AnchorChannelsConfig {
	/// A list of peers that we trust to get the required channel closing transactions confirmed
	/// on-chain.
	///
	/// Channels with these peers won't count towards the retained on-chain reserve and we won't
	/// take any action to get the required transactions confirmed ourselves.
	///
	/// **Note:** Trusting the channel counterparty to take the necessary actions to get the
	/// required Anchor spending and HTLC transactions confirmed on-chain is potentially insecure
	/// as the channel may not be closed if they refuse to do so, potentially leaving the user
	/// funds stuck *or* even allow the counterparty to steal any in-flight funds after the
	/// corresponding HTLCs time out.
	pub trusted_peers_no_reserve: Vec<PublicKey>,
	/// The amount of satoshis per anchors-negotiated channel with an untrusted peer that we keep
	/// as an emergency reserve in our on-chain wallet.
	///
	/// This allows for having the required Anchor output spending and HTLC transactions confirmed
	/// when the channel is closed.
	///
	/// If the channel peer is not marked as trusted via
	/// [`AnchorChannelsConfig::trusted_peers_no_reserve`], we will always try to spend the Anchor
	/// outputs with *any* on-chain funds available, i.e., the total reserve value as well as any
	/// spendable funds available in the on-chain wallet. Therefore, this per-channel multiplier is
	/// really a emergencey reserve that we maintain at all time to reduce reduce the risk of
	/// insufficient funds at time of a channel closure. To this end, we will refuse to open
	/// outbound or accept inbound channels if we don't have sufficient on-chain funds availble to
	/// cover the additional reserve requirement.
	///
	/// **Note:** Depending on the fee market at the time of closure, this reserve amount might or
	/// might not suffice to successfully spend the Anchor output and have the HTLC transactions
	/// confirmed on-chain, i.e., you may want to adjust this value accordingly.
	pub per_channel_reserve_sats: u64,
}

impl Default for AnchorChannelsConfig {
	fn default() -> Self {
		Self {
			trusted_peers_no_reserve: Vec::new(),
			per_channel_reserve_sats: DEFAULT_ANCHOR_PER_CHANNEL_RESERVE_SATS,
		}
	}
}

/// Returns a [`Config`] object populated with default values.
///
/// See the documentation of [`Config`] for more information on the used defaults.
///
/// This is mostly meant for use in bindings, in Rust this is synonymous with
/// [`Config::default()`].
pub fn default_config() -> Config {
	Config::default()
}

pub(crate) fn may_announce_channel(config: &Config) -> bool {
	config.node_alias.is_some()
		&& config.listening_addresses.as_ref().map_or(false, |addrs| !addrs.is_empty())
}

pub(crate) fn default_user_config(config: &Config) -> UserConfig {
	// Initialize the default config values.
	//
	// Note that methods such as Node::open_channel and Node::open_announced_channel might override
	// some of the values set here, e.g. the ChannelHandshakeConfig, meaning these default values
	// will mostly be relevant for inbound channels.
	let mut user_config = UserConfig::default();
	user_config.channel_handshake_limits.force_announced_channel_preference = false;
	user_config.manually_accept_inbound_channels = true;
	user_config.channel_handshake_config.negotiate_anchors_zero_fee_htlc_tx =
		config.anchor_channels_config.is_some();

	if !may_announce_channel(config) {
		user_config.accept_forwards_to_priv_channels = false;
		user_config.channel_handshake_config.announce_for_forwarding = false;
		user_config.channel_handshake_limits.force_announced_channel_preference = true;
	}

	user_config
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use super::may_announce_channel;
	use super::Config;
	use super::NodeAlias;
	use super::SocketAddress;

	#[test]
	fn node_announce_channel() {
		// Default configuration with node alias and listening addresses unset
		let mut node_config = Config::default();
		assert!(!may_announce_channel(&node_config));

		// Set node alias with listening addresses unset
		let alias_frm_str = |alias: &str| {
			let mut bytes = [0u8; 32];
			bytes[..alias.as_bytes().len()].copy_from_slice(alias.as_bytes());
			NodeAlias(bytes)
		};
		node_config.node_alias = Some(alias_frm_str("LDK_Node"));
		assert!(!may_announce_channel(&node_config));

		// Set node alias with an empty list of listening addresses
		node_config.listening_addresses = Some(vec![]);
		assert!(!may_announce_channel(&node_config));

		// Set node alias with a non-empty list of listening addresses
		let socket_address =
			SocketAddress::from_str("localhost:8000").expect("Socket address conversion failed.");
		if let Some(ref mut addresses) = node_config.listening_addresses {
			addresses.push(socket_address);
		}
		assert!(may_announce_channel(&node_config));
	}
}
