#![allow(unused)]

use bridge_proxy::{
    bridge_proxy_contract_proxy,
    config::{self, ProxyTrait as _},
    ProxyTrait as _,
};
use bridged_tokens_wrapper::ProxyTrait as _;
use dcdt_safe::{DcdtSafe, ProxyTrait as _};
use multi_transfer_dcdt::{
    bridged_tokens_wrapper_proxy, dcdt_safe_proxy, multi_transfer_proxy, ProxyTrait as _,
};

use dharitri_sc::{
    api::{HandleConstraints, ManagedTypeApi},
    codec::{
        multi_types::{MultiValueVec, OptionalValue},
        Empty, TopEncode,
    },
    contract_base::ManagedSerializer,
    storage::mappers::SingleValue,
    types::{
        Address, BigUint, CodeMetadata, RewaOrDcdtTokenIdentifier, DcdtLocalRole, ManagedAddress,
        ManagedBuffer, ManagedByteArray, ManagedOption, ManagedVec, MultiValueEncoded,
        ReturnsNewManagedAddress, ReturnsResult, TestAddress, TestSCAddress, TestTokenIdentifier,
        TokenIdentifier,
    },
};
use dharitri_sc_modules::pause::ProxyTrait;
use dharitri_sc_scenario::{
    api::{StaticApi, VMHooksApi, VMHooksApiBackend},
    imports::DrtscPath,
    scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
    scenario_model::*,
    ContractInfo, DebugApi, ExpectError, ExpectValue, ScenarioTxRun, ScenarioWorld,
};

use eth_address::*;
use token_module::ProxyTrait as _;
use transaction::{CallData, EthTransaction};

const UNIVERSAL_TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("UNIV-abc123");
const BRIDGE_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("BRIDGE-123456");
const WRAPPED_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("WRAPPED-123456");
const TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("TOKEN");

const USER_ETHEREUM_ADDRESS: &[u8] = b"0x0102030405060708091011121314151617181920";

const GAS_LIMIT: u64 = 100_000_000;
const ERROR: u64 = 4;

const MULTI_TRANSFER_CODE_PATH: DrtscPath = DrtscPath::new("output/multi-transfer-dcdt.drtsc.json");
const BRIDGE_PROXY_CODE_PATH: DrtscPath =
    DrtscPath::new("../bridge-proxy/output/bridge-proxy.drtsc.json");
const DCDT_SAFE_CODE_PATH: DrtscPath = DrtscPath::new("../dcdt-safe/output/dcdt-safe.drtsc.json");
const BRIDGED_TOKENS_WRAPPER_CODE_PATH: DrtscPath =
    DrtscPath::new("../bridged-tokens-wrapper/output/bridged-tokens-wrapper.drtsc.json");
const PRICE_AGGREGATOR_CODE_PATH: DrtscPath =
    DrtscPath::new("../price-aggregator/price-aggregator.drtsc.json");

const MULTI_TRANSFER_ADDRESS: TestSCAddress = TestSCAddress::new("multi-transfer");
const BRIDGE_PROXY_ADDRESS: TestSCAddress = TestSCAddress::new("bridge-proxy");
const DCDT_SAFE_ADDRESS: TestSCAddress = TestSCAddress::new("dcdt-safe");
const BRIDGED_TOKENS_WRAPPER_ADDRESS: TestSCAddress = TestSCAddress::new("bridged-tokens-wrapper");
const PRICE_AGGREGATOR_ADDRESS: TestSCAddress = TestSCAddress::new("price-aggregator");

const ORACLE_ADDRESS: TestAddress = TestAddress::new("oracle");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const USER1_ADDRESS: TestAddress = TestAddress::new("user1");
const USER2_ADDRESS: TestAddress = TestAddress::new("user2");

const DCDT_SAFE_ETH_TX_GAS_LIMIT: u64 = 150_000;
const MAX_AMOUNT: u64 = 100_000_000_000_000u64;

const BALANCE: &str = "2,000,000";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        MULTI_TRANSFER_CODE_PATH,
        multi_transfer_dcdt::ContractBuilder,
    );
    blockchain.register_contract(BRIDGE_PROXY_CODE_PATH, bridge_proxy::ContractBuilder);

    blockchain.register_contract(DCDT_SAFE_CODE_PATH, dcdt_safe::ContractBuilder);

    blockchain.register_contract(
        BRIDGED_TOKENS_WRAPPER_CODE_PATH,
        bridged_tokens_wrapper::ContractBuilder,
    );

    blockchain
}

type MultiTransferContract = ContractInfo<multi_transfer_dcdt::Proxy<StaticApi>>;
type BridgeProxyContract = ContractInfo<bridge_proxy::Proxy<StaticApi>>;
type DcdtSafeContract = ContractInfo<dcdt_safe::Proxy<StaticApi>>;
type BridgedTokensWrapperContract = ContractInfo<bridged_tokens_wrapper::Proxy<StaticApi>>;

struct MultiTransferTestState {
    world: ScenarioWorld,
}

impl MultiTransferTestState {
    fn new() -> Self {
        let mut world = world();

        world
            .account(OWNER_ADDRESS)
            .nonce(1)
            .dcdt_balance(BRIDGE_TOKEN_ID, 1001u64)
            .dcdt_balance(TOKEN_ID, MAX_AMOUNT)
            .dcdt_balance(WRAPPED_TOKEN_ID, 1001u64)
            .dcdt_balance(UNIVERSAL_TOKEN_IDENTIFIER, 1001u64)
            .account(USER1_ADDRESS)
            .nonce(1)
            .account(USER2_ADDRESS)
            .nonce(1);

        let roles = vec![
            "DCDTRoleLocalMint".to_string(),
            "DCDTRoleLocalBurn".to_string(),
        ];
        world
            .account(DCDT_SAFE_ADDRESS)
            .dcdt_roles(BRIDGE_TOKEN_ID, roles.clone())
            .dcdt_roles(UNIVERSAL_TOKEN_IDENTIFIER, roles.clone())
            .dcdt_roles(WRAPPED_TOKEN_ID, roles.clone())
            .code(DCDT_SAFE_CODE_PATH)
            .owner(OWNER_ADDRESS);

        Self { world }
    }

    fn multi_transfer_deploy(&mut self) -> &mut Self {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
            .init()
            .code(MULTI_TRANSFER_CODE_PATH)
            .new_address(MULTI_TRANSFER_ADDRESS)
            .run();
        self
    }

    fn bridge_proxy_deploy(&mut self) -> &mut Self {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(bridge_proxy_contract_proxy::BridgeProxyContractProxy)
            .init(OptionalValue::Some(MULTI_TRANSFER_ADDRESS.to_address()))
            .code(BRIDGE_PROXY_CODE_PATH)
            .new_address(BRIDGE_PROXY_ADDRESS)
            .run();

        self
    }

    fn safe_deploy(&mut self, price_aggregator_contract_address: Address) -> &mut Self {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .upgrade(
                ManagedAddress::zero(),
                MULTI_TRANSFER_ADDRESS.to_address(),
                BRIDGE_PROXY_ADDRESS.to_address(),
                DCDT_SAFE_ETH_TX_GAS_LIMIT,
            )
            .code(DCDT_SAFE_CODE_PATH)
            .run();

        self
    }

    fn bridged_tokens_wrapper_deploy(&mut self) -> &mut Self {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
            .init()
            .code(BRIDGED_TOKENS_WRAPPER_CODE_PATH)
            .new_address(BRIDGED_TOKENS_WRAPPER_ADDRESS)
            .run();

        self
    }

    fn config_multi_transfer(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(MULTI_TRANSFER_ADDRESS)
            .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
            .set_wrapping_contract_address(OptionalValue::Some(
                BRIDGED_TOKENS_WRAPPER_ADDRESS.to_address(),
            ))
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(MULTI_TRANSFER_ADDRESS)
            .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
            .set_bridge_proxy_contract_address(OptionalValue::Some(
                BRIDGE_PROXY_ADDRESS.to_address(),
            ))
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(MULTI_TRANSFER_ADDRESS)
            .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
            .set_dcdt_safe_contract_address(OptionalValue::Some(DCDT_SAFE_ADDRESS.to_address()))
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .set_multi_transfer_contract_address(OptionalValue::Some(
                MULTI_TRANSFER_ADDRESS.to_address(),
            ))
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .add_token_to_whitelist(
                TokenIdentifier::from_dcdt_bytes("BRIDGE-123456"),
                "BRIDGE",
                true,
                false,
                BigUint::zero(),
                BigUint::zero(),
                BigUint::zero(),
                OptionalValue::Some(BigUint::from(DCDT_SAFE_ETH_TX_GAS_LIMIT)),
            )
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .add_token_to_whitelist(
                TokenIdentifier::from_dcdt_bytes("TOKEN"),
                "TOKEN",
                false,
                true,
                BigUint::from(MAX_AMOUNT),
                BigUint::zero(),
                BigUint::zero(),
                OptionalValue::Some(BigUint::from(DCDT_SAFE_ETH_TX_GAS_LIMIT)),
            )
            .single_dcdt(
                &TokenIdentifier::from_dcdt_bytes("TOKEN"),
                0,
                &BigUint::from(MAX_AMOUNT),
            )
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(MULTI_TRANSFER_ADDRESS)
            .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
            .set_max_bridged_amount(TOKEN_ID, MAX_AMOUNT - 1)
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .add_token_to_whitelist(
                TokenIdentifier::from_dcdt_bytes("WRAPPED-123456"),
                "BRIDGE2",
                true,
                false,
                BigUint::zero(),
                BigUint::zero(),
                BigUint::zero(),
                OptionalValue::Some(BigUint::from(DCDT_SAFE_ETH_TX_GAS_LIMIT)),
            )
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .unpause_endpoint()
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
            .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
            .unpause_endpoint()
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BRIDGE_PROXY_ADDRESS)
            .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
            .unpause_endpoint()
            .run();
    }

    fn config_bridged_tokens_wrapper(&mut self) {

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .set_bridge_proxy_contract_address(OptionalValue::Some(
                BRIDGE_PROXY_ADDRESS.to_address(),
            ))
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .add_token_to_whitelist(
                TokenIdentifier::from_dcdt_bytes("UNIV-abc123"),
                "BRIDGE1",
                true,
                false,
                BigUint::zero(),
                BigUint::zero(),
                BigUint::zero(),
                OptionalValue::Some(BigUint::from(DCDT_SAFE_ETH_TX_GAS_LIMIT)),
            )
            .run();
        self.world.set_dcdt_balance(
            BRIDGE_PROXY_ADDRESS,
            b"UNIV-abc123",
            BigUint::from(10_000_000u64),
        );

        self.world.set_dcdt_balance(
            BRIDGE_PROXY_ADDRESS,
            b"WRAPPED-123456",
            BigUint::from(10_000_000u64),
        );

        self.world.set_dcdt_balance(
            BRIDGED_TOKENS_WRAPPER_ADDRESS,
            b"WRAPPED-123456",
            BigUint::from(10_000_000u64),
        );

        self.world.set_dcdt_balance(
            BRIDGE_PROXY_ADDRESS,
            b"BRIDGE-123456",
            BigUint::from(10_000_000u64),
        );

        self.world.set_dcdt_local_roles(
            BRIDGED_TOKENS_WRAPPER_ADDRESS,
            b"UNIV-abc123",
            &[DcdtLocalRole::Mint, DcdtLocalRole::Burn],
        );

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .set_eth_tx_gas_limit(0u64)
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .init_supply_mint_burn(
                UNIVERSAL_TOKEN_IDENTIFIER,
                BigUint::from(600_000u64),
                BigUint::from(0u64),
            )
            .run();
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .add_token_to_whitelist(
                TokenIdentifier::from_dcdt_bytes("WRAPPED-123456"),
                "BRIDGE2",
                true,
                false,
                BigUint::zero(),
                BigUint::zero(),
                BigUint::zero(),
                OptionalValue::Some(BigUint::from(DCDT_SAFE_ETH_TX_GAS_LIMIT)),
            )
            .run();
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .init_supply_mint_burn(
                WRAPPED_TOKEN_ID,
                BigUint::from(600_000u64),
                BigUint::from(0u64),
            )
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BRIDGE_PROXY_ADDRESS)
            .typed(bridge_proxy_contract_proxy::BridgeProxyContractProxy)
            .set_bridged_tokens_wrapper_contract_address(OptionalValue::Some(
                BRIDGED_TOKENS_WRAPPER_ADDRESS.to_address(),
            ))
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .set_bridged_tokens_wrapper_contract_address(OptionalValue::Some(
                BRIDGED_TOKENS_WRAPPER_ADDRESS.to_address(),
            ))
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
            .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
            .add_wrapped_token(TokenIdentifier::from(UNIVERSAL_TOKEN_IDENTIFIER), 18u32)
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
            .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
            .whitelist_token(
                TokenIdentifier::from(WRAPPED_TOKEN_ID),
                18u32,
                TokenIdentifier::from(UNIVERSAL_TOKEN_IDENTIFIER),
            )
            .run();
    }

    fn check_balances_on_safe(
        &mut self,
        token_id: TestTokenIdentifier,
        total_supply: BigUint<StaticApi>,
        total_minted: BigUint<StaticApi>,
        total_burned: BigUint<StaticApi>,
    ) {
        let actual_total_supply = self
            .world
            .query()
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .total_balances(token_id)
            .returns(ReturnsResult)
            .run();

        assert_eq!(
            actual_total_supply, total_supply,
            "Total supply balance is wrong"
        );
        let actual_total_burned = self
            .world
            .query()
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .burn_balances(token_id)
            .returns(ReturnsResult)
            .run();

        assert_eq!(
            actual_total_burned, total_burned,
            "Total burned balance is wrong"
        );

        let actual_total_minted = self
            .world
            .query()
            .to(DCDT_SAFE_ADDRESS)
            .typed(dcdt_safe_proxy::DcdtSafeProxy)
            .mint_balances(token_id)
            .returns(ReturnsResult)
            .run();

        assert_eq!(
            actual_total_minted, total_minted,
            "Total minted balance is wrong"
        );
    }

    fn deploy_contracts(&mut self) {
        self.multi_transfer_deploy();
        self.bridge_proxy_deploy();
        self.safe_deploy(Address::zero());
        self.bridged_tokens_wrapper_deploy();
    }
}

#[test]
fn basic_transfer_test() {
    let mut state = MultiTransferTestState::new();
    let token_amount = BigUint::from(500u64);

    state.deploy_contracts();
    state.config_multi_transfer();

    let call_data = ManagedBuffer::from(b"add");
    call_data
        .clone()
        .concat(ManagedBuffer::from(GAS_LIMIT.to_string()));
    call_data.clone().concat(ManagedBuffer::default());

    let eth_tx = EthTransaction {
        from: EthAddress {
            raw_addr: ManagedByteArray::default(),
        },
        to: ManagedAddress::from(USER1_ADDRESS.eval_to_array()),
        token_id: TokenIdentifier::from(BRIDGE_TOKEN_ID),
        amount: token_amount.clone(),
        tx_nonce: 1u64,
        call_data: ManagedOption::some(call_data),
    };

    let mut transfers: MultiValueEncoded<StaticApi, EthTransaction<StaticApi>> =
        MultiValueEncoded::new();
    transfers.push(eth_tx);

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .batch_transfer_dcdt_token(1u32, transfers)
        .run();

    state
        .world
        .check_account(USER1_ADDRESS)
        .dcdt_balance(BRIDGE_TOKEN_ID, token_amount);
}

#[test]
fn batch_transfer_both_executed_test() {
    let mut state = MultiTransferTestState::new();
    let token_amount = BigUint::from(500u64);

    state.deploy_contracts();
    state.config_multi_transfer();

    let mut args = ManagedVec::new();
    args.push(ManagedBuffer::from(&[5u8]));

    let call_data: CallData<StaticApi> = CallData {
        endpoint: ManagedBuffer::from("add"),
        gas_limit: GAS_LIMIT,
        args: ManagedOption::some(args),
    };

    let call_data: ManagedBuffer<StaticApi> =
        ManagedSerializer::new().top_encode_to_managed_buffer(&call_data);

    let eth_tx1 = EthTransaction {
        from: EthAddress {
            raw_addr: ManagedByteArray::new_from_bytes(b"01020304050607080910"),
        },
        to: ManagedAddress::from(USER2_ADDRESS.eval_to_array()),
        token_id: TokenIdentifier::from(BRIDGE_TOKEN_ID),
        amount: token_amount.clone(),
        tx_nonce: 1u64,
        call_data: ManagedOption::some(call_data.clone()),
    };

    let eth_tx2 = EthTransaction {
        from: EthAddress {
            raw_addr: ManagedByteArray::new_from_bytes(b"01020304050607080910"),
        },
        to: ManagedAddress::from(USER1_ADDRESS.eval_to_array()),
        token_id: TokenIdentifier::from(WRAPPED_TOKEN_ID),
        amount: token_amount.clone(),
        tx_nonce: 2u64,
        call_data: ManagedOption::some(call_data),
    };

    let mut transfers: MultiValueEncoded<StaticApi, EthTransaction<StaticApi>> =
        MultiValueEncoded::new();
    transfers.push(eth_tx1);
    transfers.push(eth_tx2);

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .batch_transfer_dcdt_token(1u32, transfers)
        .run();

    state
        .world
        .check_account(USER1_ADDRESS)
        .dcdt_balance(WRAPPED_TOKEN_ID, token_amount.clone());

    state
        .world
        .check_account(USER2_ADDRESS)
        .dcdt_balance(BRIDGE_TOKEN_ID, token_amount);
}

#[test]
fn batch_two_transfers_same_token_test() {
    let mut state = MultiTransferTestState::new();
    let token_amount = BigUint::from(500u64);

    state.deploy_contracts();
    state.config_multi_transfer();

    let mut args = ManagedVec::new();
    args.push(ManagedBuffer::from(&[5u8]));

    let call_data: CallData<StaticApi> = CallData {
        endpoint: ManagedBuffer::from("add"),
        gas_limit: GAS_LIMIT,
        args: ManagedOption::some(args),
    };

    let call_data: ManagedBuffer<StaticApi> =
        ManagedSerializer::new().top_encode_to_managed_buffer(&call_data);

    let eth_tx1 = EthTransaction {
        from: EthAddress {
            raw_addr: ManagedByteArray::new_from_bytes(b"01020304050607080910"),
        },
        to: ManagedAddress::from(USER2_ADDRESS.eval_to_array()),
        token_id: TokenIdentifier::from(BRIDGE_TOKEN_ID),
        amount: token_amount.clone(),
        tx_nonce: 1u64,
        call_data: ManagedOption::some(call_data.clone()),
    };

    let eth_tx2 = EthTransaction {
        from: EthAddress {
            raw_addr: ManagedByteArray::new_from_bytes(b"01020304050607080910"),
        },
        to: ManagedAddress::from(USER1_ADDRESS.eval_to_array()),
        token_id: TokenIdentifier::from(BRIDGE_TOKEN_ID),
        amount: token_amount.clone(),
        tx_nonce: 2u64,
        call_data: ManagedOption::some(call_data),
    };

    let mut transfers: MultiValueEncoded<StaticApi, EthTransaction<StaticApi>> =
        MultiValueEncoded::new();
    transfers.push(eth_tx1);
    transfers.push(eth_tx2);

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .batch_transfer_dcdt_token(1u32, transfers)
        .run();

    state
        .world
        .check_account(USER1_ADDRESS)
        .dcdt_balance(BRIDGE_TOKEN_ID, token_amount.clone());

    state
        .world
        .check_account(USER2_ADDRESS)
        .dcdt_balance(BRIDGE_TOKEN_ID, token_amount);
}

#[test]
fn batch_transfer_both_failed_test() {
    let mut state = MultiTransferTestState::new();
    let token_amount = BigUint::from(500u64);

    state.deploy_contracts();
    state.config_multi_transfer();

    let mut args = ManagedVec::new();
    args.push(ManagedBuffer::from(&[5u8]));

    let call_data: CallData<StaticApi> = CallData {
        endpoint: ManagedBuffer::from("add"),
        gas_limit: GAS_LIMIT,
        args: ManagedOption::some(args),
    };

    let call_data: ManagedBuffer<StaticApi> =
        ManagedSerializer::new().top_encode_to_managed_buffer(&call_data);

    let eth_tx1 = EthTransaction {
        from: EthAddress {
            raw_addr: ManagedByteArray::new_from_bytes(b"01020304050607080910"),
        },
        to: ManagedAddress::from(BRIDGE_PROXY_ADDRESS.eval_to_array()),
        token_id: TokenIdentifier::from(BRIDGE_TOKEN_ID),
        amount: token_amount.clone(),
        tx_nonce: 1u64,
        call_data: ManagedOption::some(call_data.clone()),
    };

    let eth_tx2 = EthTransaction {
        from: EthAddress {
            raw_addr: ManagedByteArray::new_from_bytes(b"01020304050607080910"),
        },
        to: ManagedAddress::from(BRIDGE_PROXY_ADDRESS.eval_to_array()),
        token_id: TokenIdentifier::from(BRIDGE_TOKEN_ID),
        amount: token_amount.clone(),
        tx_nonce: 2u64,
        call_data: ManagedOption::some(call_data),
    };

    let mut transfers: MultiValueEncoded<StaticApi, EthTransaction<StaticApi>> =
        MultiValueEncoded::new();
    transfers.push(eth_tx1);
    transfers.push(eth_tx2);

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .batch_transfer_dcdt_token(1u32, transfers)
        .run();

    let first_batch = state
        .world
        .query()
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .get_first_batch_any_status()
        .returns(ReturnsResult)
        .run();

    assert!(first_batch.is_none());

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .move_refund_batch_to_safe()
        .run();

    let first_batch = state
        .world
        .query()
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .get_first_batch_any_status()
        .returns(ReturnsResult)
        .run();

    assert!(first_batch.is_none());
}

#[test]
fn test_unwrap_token_create_transaction_paused() {
    let mut state = MultiTransferTestState::new();

    state.deploy_contracts();

    state.config_bridged_tokens_wrapper();

    state
        .world
        .tx()
        .from(BRIDGE_PROXY_ADDRESS)
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .unwrap_token_create_transaction(
            TokenIdentifier::from(UNIVERSAL_TOKEN_IDENTIFIER),
            DCDT_SAFE_ADDRESS.to_address(),
            EthAddress::zero(),
        )
        .rewa_or_single_dcdt(
            &RewaOrDcdtTokenIdentifier::dcdt(UNIVERSAL_TOKEN_IDENTIFIER),
            0u64,
            &BigUint::from(10u64),
        )
        .returns(ExpectError(ERROR, "Contract is paused"))
        .run();
}

#[test]
fn test_unwrap_token_create_transaction_insufficient_liquidity() {
    let mut state = MultiTransferTestState::new();
    state.deploy_contracts();
    state.config_multi_transfer();
    state.config_bridged_tokens_wrapper();

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .unpause_endpoint()
        .run();

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .deposit_liquidity()
        .rewa_or_single_dcdt(
            &RewaOrDcdtTokenIdentifier::dcdt(WRAPPED_TOKEN_ID),
            0u64,
            &BigUint::from(1_000u64),
        )
        .run();

    state
        .world
        .set_dcdt_balance(USER1_ADDRESS, b"UNIV-abc123", BigUint::from(5_000u64));

    state
        .world
        .tx()
        .from(USER1_ADDRESS)
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .unwrap_token_create_transaction(WRAPPED_TOKEN_ID, DCDT_SAFE_ADDRESS.to_address(), EthAddress::zero())
        .rewa_or_single_dcdt(
            &RewaOrDcdtTokenIdentifier::dcdt(UNIVERSAL_TOKEN_IDENTIFIER),
            0u64,
            &BigUint::from(2_000u64),
        )
        .returns(ExpectError(ERROR, "Contract does not have enough funds"))
        .run();
}

#[test]
fn test_unwrap_token_create_transaction_should_work() {
    let mut state = MultiTransferTestState::new();

    state.deploy_contracts();

    state.config_multi_transfer();
    state.config_bridged_tokens_wrapper();

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .deposit_liquidity()
        .rewa_or_single_dcdt(
            &RewaOrDcdtTokenIdentifier::dcdt(WRAPPED_TOKEN_ID),
            0u64,
            &BigUint::from(1_000u64),
        )
        .run();

    state
        .world
        .set_dcdt_balance(USER1_ADDRESS, b"UNIV-abc123", BigUint::from(5_000u64));

    state.check_balances_on_safe(
        WRAPPED_TOKEN_ID,
        BigUint::zero(),
        BigUint::from(600000u64),
        BigUint::zero(),
    );
    
    state
        .world
        .query()
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .token_liquidity(WRAPPED_TOKEN_ID)
        .returns(ExpectValue(BigUint::from(1000u64)))
        .run();

    state
        .world
        .tx()
        .from(USER1_ADDRESS)
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .unwrap_token_create_transaction(WRAPPED_TOKEN_ID, DCDT_SAFE_ADDRESS.to_address(), EthAddress::zero())
        .rewa_or_single_dcdt(
            &RewaOrDcdtTokenIdentifier::dcdt(UNIVERSAL_TOKEN_IDENTIFIER),
            0u64,
            &BigUint::from(900u64),
        )
        .run();

    state
        .world
        .query()
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .token_liquidity(WRAPPED_TOKEN_ID)
        .returns(ExpectValue(BigUint::from(100u64)))
        .run();

    state.check_balances_on_safe(
        WRAPPED_TOKEN_ID,
        BigUint::zero(),
        BigUint::from(600000u64),
        BigUint::from(900u64),
    );
}

#[test]
fn test_unwrap_token_create_transaction_should_fail() {
    let mut state = MultiTransferTestState::new();

    state.deploy_contracts();

    state.config_multi_transfer();
    state.config_bridged_tokens_wrapper();

    state
        .world
        .set_dcdt_balance(USER1_ADDRESS, b"TOKEN", BigUint::from(5_000u64));

    state
        .world
        .tx()
        .from(USER1_ADDRESS)
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .unwrap_token_create_transaction(WRAPPED_TOKEN_ID, DCDT_SAFE_ADDRESS.to_address(), EthAddress::zero())
        .rewa_or_single_dcdt(
            &RewaOrDcdtTokenIdentifier::dcdt(TOKEN_ID),
            0u64,
            &BigUint::from(1_000u64),
        )
        .returns(ExpectError(ERROR, "Dcdt token unavailable"))
        .run();
}

#[test]
fn test_unwrap_token_create_transaction_amount_zero() {
    let mut state = MultiTransferTestState::new();

    state.deploy_contracts();

    state.config_multi_transfer();
    state.config_bridged_tokens_wrapper();
    state
        .world
        .tx()
        .from(BRIDGE_PROXY_ADDRESS)
        .to(BRIDGED_TOKENS_WRAPPER_ADDRESS)
        .typed(bridged_tokens_wrapper_proxy::BridgedTokensWrapperProxy)
        .unwrap_token_create_transaction(
            TokenIdentifier::from(WRAPPED_TOKEN_ID),
            DCDT_SAFE_ADDRESS.to_address(),
            EthAddress::zero(),
        )
        .rewa_or_single_dcdt(
            &RewaOrDcdtTokenIdentifier::dcdt(UNIVERSAL_TOKEN_IDENTIFIER),
            0u64,
            &BigUint::from(0u64),
        )
        .returns(ExpectError(ERROR, "Must pay more than 0 tokens!"))
        .run();
}

#[test]
fn add_refund_batch_test() {
    let mut state = MultiTransferTestState::new();

    state.multi_transfer_deploy();
    state.bridge_proxy_deploy();
    state.safe_deploy(Address::zero());
    state.bridged_tokens_wrapper_deploy();
    state.config_multi_transfer();

    let eth_tx = EthTransaction {
        from: EthAddress::zero(),
        to: ManagedAddress::from(USER1_ADDRESS.eval_to_array()),
        token_id: TokenIdentifier::from(TOKEN_ID),
        amount: BigUint::from(MAX_AMOUNT),
        tx_nonce: 1u64,
        call_data: ManagedOption::none(),
    };

    let mut transfers: MultiValueEncoded<StaticApi, EthTransaction<StaticApi>> =
        MultiValueEncoded::new();
    transfers.push(eth_tx.clone());

    let fee = state
        .world
        .query()
        .to(DCDT_SAFE_ADDRESS)
        .typed(dcdt_safe_proxy::DcdtSafeProxy)
        .calculate_required_fee(TOKEN_ID)
        .returns(ReturnsResult)
        .run();

    state.check_balances_on_safe(
        TOKEN_ID,
        BigUint::from(MAX_AMOUNT),
        BigUint::zero(),
        BigUint::zero(),
    );

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .batch_transfer_dcdt_token(1u32, transfers)
        .run();
    state.check_balances_on_safe(TOKEN_ID, BigUint::zero(), BigUint::zero(), BigUint::zero());

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(MULTI_TRANSFER_ADDRESS)
        .typed(multi_transfer_proxy::MultiTransferDcdtProxy)
        .move_refund_batch_to_safe()
        .run();

    state.check_balances_on_safe(
        TOKEN_ID,
        BigUint::from(MAX_AMOUNT) - fee,
        BigUint::zero(),
        BigUint::zero(),
    );
}
