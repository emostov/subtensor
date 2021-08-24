use sp_core::{Pair, Public, sr25519};
use node_subtensor_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use sc_service::config::MultiaddrWithPeerId;
use std::str::FromStr;
use sp_core::crypto::Ss58Codec;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

pub fn authority_keys_from_ss58(s_aura :&str, s_grandpa : &str) -> (AuraId, GrandpaId) {
	(
		get_aura_from_ss58_addr(s_aura),
		get_grandpa_from_ss58_addr(s_grandpa),
	)
}

pub fn get_aura_from_ss58_addr(s: &str) -> AuraId {
	AuraId::from_ss58check(s).unwrap()
}

pub fn get_grandpa_from_ss58_addr(s: &str) -> GrandpaId {
	GrandpaId::from_ss58check(s).unwrap()
}


type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Sudo account
			AccountId::from_ss58check("5FsVmCKVDvkUvXR42ckdi9GxmgU8C2zQvKm7Mi62199qfCDt").unwrap(), // Sudo
			// Pre-funded accounts
			vec![
				AccountId::from_ss58check("5FsVmCKVDvkUvXR42ckdi9GxmgU8C2zQvKm7Mi62199qfCDt").unwrap(), // Sudo
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn akatsuki_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Akatsuki bittensor main net",
		"akatsuki_mainnet",
		ChainType::Live,
		move || network_genesis(
			wasm_binary,
			vec![
				authority_keys_from_ss58("5CCfqZuwygPhjQN2SYobCjyKDJtN2HhNsUESFkqfF1FPFK2q", "5HaTasFAThLhHQjYbxaSLbCcFpMr5U48j57cdrKBnPKDzo7y"), // Paul
				authority_keys_from_ss58("5ELRcx6SiGr3c3T6qAAUQz2EeUDJUq2QdgCBG1k8A2mYDm39", "5DkDqEhXGmPwYktt5E3PKfXujAZw1T4xF6T9t1YRaBonJ9e4"), // Philip
				authority_keys_from_ss58("5GzwfhtLyKQ1KoiAdmFw5kFytxESU2K3uo27ZAkBswxHPWuC", "5GLzcfVersKv1TotgXvd5AiNfU6XdEXhPdhjL7dMcsovnQmk"), // Andrew
				authority_keys_from_ss58("5DPuSeJQpEbmS4uz27upUAmUTuwVfJob6pJVn6ahcjf8suqe", "5CcCpjr33kq5NUDeYxWr4df8v4X6MkSAqUSUonzzLAYm4VxE"), // James
				authority_keys_from_ss58("5HmPj6eAFR9VXYst9X9KE9FZYJVhULTgcbJXaWNp611sVKDS", "5CrG4gHMxDyFVW2Sn1W2VSAJBUvsB1wuLGdupexYTTD6N4SW"), // John
				authority_keys_from_ss58("5HMXmC3tTJVsqz1LftwZAfRTch1sdsimHbmsw4kGUfzVvJXA", "5EZM4K9pLRCFXCfj1Mr9SiQ4xiM15hnECEofQUqSRwjLiseb"), // Judas
			],
			AccountId::from_ss58check("5FsVmCKVDvkUvXR42ckdi9GxmgU8C2zQvKm7Mi62199qfCDt").unwrap(), // Sudo
			vec![
				AccountId::from_ss58check("5FsVmCKVDvkUvXR42ckdi9GxmgU8C2zQvKm7Mi62199qfCDt").unwrap(), // Sudo
			],
			true,
		),
		vec![
			MultiaddrWithPeerId::from_str("/dns4/Peter.akatsuki.bittensor.com/tcp/30333/ws/p2p/12D3KooWRxmVnU2EMar4Bsg3sYfVJVEhpLqy9Us82AgzRHpnjBES").unwrap(),
			MultiaddrWithPeerId::from_str("/dns4/Thaddeus.akatsuki.bittensor.com/tcp/30333/ws/p2p/12D3KooWG5XbAU9gdGCHJyAy2BwK95gqjYz48AUzwY5ruDK6FCB6").unwrap()
	    ],
		None,
		None,
		None,
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
	}
}

/// Configure initial storage state for FRAME modules.
fn network_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1000 tokens for sudo.
			balances: endowed_accounts.iter().cloned().map(|k|(k, u64::pow(10,9))).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
	}
}
