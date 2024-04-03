use multiversx_sc::proxy_imports::*;

pub struct KittyOwnershipProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for KittyOwnershipProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = KittyOwnershipProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        KittyOwnershipProxyMethods { wrapped_tx: tx }
    }
}

pub struct KittyOwnershipProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> KittyOwnershipProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: CodecInto<BigUint<Env::Api>>,
        Arg1: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
        Arg2: CodecInto<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        birth_fee: Arg0,
        opt_gene_science_contract_address: Arg1,
        opt_kitty_auction_contract_address: Arg2,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&birth_fee)
            .argument(&opt_gene_science_contract_address)
            .argument(&opt_kitty_auction_contract_address)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> KittyOwnershipProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_gene_science_contract_address_endpoint<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setGeneScienceContractAddress")
            .argument(&address)
            .original_result()
    }

    pub fn set_kitty_auction_contract_address_endpoint<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setKittyAuctionContractAddress")
            .argument(&address)
            .original_result()
    }

    pub fn claim(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("claim")
            .original_result()
    }

    pub fn total_supply(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u32> {
        self.wrapped_tx
            .raw_call("totalSupply")
            .original_result()
    }

    pub fn balance_of<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, u32> {
        self.wrapped_tx
            .raw_call("balanceOf")
            .argument(&address)
            .original_result()
    }

    pub fn owner_of<
        Arg0: CodecInto<u32>,
    >(
        self,
        kitty_id: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("ownerOf")
            .argument(&kitty_id)
            .original_result()
    }

    pub fn approve<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<u32>,
    >(
        self,
        to: Arg0,
        kitty_id: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("approve")
            .argument(&to)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn transfer<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<u32>,
    >(
        self,
        to: Arg0,
        kitty_id: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("transfer")
            .argument(&to)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn transfer_from<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
        Arg2: CodecInto<u32>,
    >(
        self,
        from: Arg0,
        to: Arg1,
        kitty_id: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("transfer_from")
            .argument(&from)
            .argument(&to)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn tokens_of_owner<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValueEncoded<Env::Api, u32>> {
        self.wrapped_tx
            .raw_call("tokensOfOwner")
            .argument(&address)
            .original_result()
    }

    pub fn allow_auctioning<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<u32>,
    >(
        self,
        by: Arg0,
        kitty_id: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("allowAuctioning")
            .argument(&by)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn approve_siring_and_return_kitty<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
        Arg2: CodecInto<u32>,
    >(
        self,
        approved_address: Arg0,
        kitty_owner: Arg1,
        kitty_id: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("approveSiringAndReturnKitty")
            .argument(&approved_address)
            .argument(&kitty_owner)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn create_gen_zero_kitty(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u32> {
        self.wrapped_tx
            .raw_call("createGenZeroKitty")
            .original_result()
    }

    pub fn get_kitty_by_id_endpoint<
        Arg0: CodecInto<u32>,
    >(
        self,
        kitty_id: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, Kitty> {
        self.wrapped_tx
            .raw_call("getKittyById")
            .argument(&kitty_id)
            .original_result()
    }

    pub fn is_ready_to_breed<
        Arg0: CodecInto<u32>,
    >(
        self,
        kitty_id: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, bool> {
        self.wrapped_tx
            .raw_call("isReadyToBreed")
            .argument(&kitty_id)
            .original_result()
    }

    pub fn is_pregnant<
        Arg0: CodecInto<u32>,
    >(
        self,
        kitty_id: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, bool> {
        self.wrapped_tx
            .raw_call("isPregnant")
            .argument(&kitty_id)
            .original_result()
    }

    pub fn can_breed_with<
        Arg0: CodecInto<u32>,
        Arg1: CodecInto<u32>,
    >(
        self,
        matron_id: Arg0,
        sire_id: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, bool> {
        self.wrapped_tx
            .raw_call("canBreedWith")
            .argument(&matron_id)
            .argument(&sire_id)
            .original_result()
    }

    pub fn approve_siring<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<u32>,
    >(
        self,
        address: Arg0,
        kitty_id: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("approveSiring")
            .argument(&address)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn breed_with<
        Arg0: CodecInto<u32>,
        Arg1: CodecInto<u32>,
    >(
        self,
        matron_id: Arg0,
        sire_id: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("breedWith")
            .argument(&matron_id)
            .argument(&sire_id)
            .original_result()
    }

    pub fn give_birth<
        Arg0: CodecInto<u32>,
    >(
        self,
        matron_id: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("giveBirth")
            .argument(&matron_id)
            .original_result()
    }

    pub fn birth_fee(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("birthFee")
            .original_result()
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Kitty {
    pub genes: KittyGenes,
    pub birth_time: u64,
    pub cooldown_end: u64,
    pub matron_id: u32,
    pub sire_id: u32,
    pub siring_with_id: u32,
    pub nr_children: u16,
    pub generation: u16,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, Default)]
pub struct KittyGenes {
    pub fur_color: Color,
    pub eye_color: Color,
    pub meow_power: u8,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl KittyGenes {
    pub fn get_as_u64(&self) -> u64 {
        (self.fur_color.as_u64() << 12 | self.eye_color.as_u64()) << 4
            | self.meow_power.to_be() as u64
    }
}

impl Color {
    pub fn as_u64(&self) -> u64 {
        ((self.r.to_be() as u64) << 4 | self.r.to_be() as u64) << 4 | self.r.to_be() as u64
    }
}
