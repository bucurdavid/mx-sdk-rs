#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

// factorial.wasm
const CONTRACT_CODE: &[u8] = &[
	0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x22, 0x07, 0x60, 0x01, 0x7e, 0x01, 0x7f,
	0x60, 0x02, 0x7f, 0x7f, 0x00, 0x60, 0x00, 0x01, 0x7f, 0x60, 0x00, 0x00, 0x60, 0x02, 0x7f, 0x7f,
	0x01, 0x7f, 0x60, 0x03, 0x7f, 0x7f, 0x7f, 0x00, 0x60, 0x01, 0x7f, 0x00, 0x02, 0xb9, 0x01, 0x09,
	0x03, 0x65, 0x6e, 0x76, 0x09, 0x62, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x4e, 0x65, 0x77, 0x00, 0x00,
	0x03, 0x65, 0x6e, 0x76, 0x0b, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x45, 0x72, 0x72, 0x6f, 0x72,
	0x00, 0x01, 0x03, 0x65, 0x6e, 0x76, 0x0f, 0x67, 0x65, 0x74, 0x4e, 0x75, 0x6d, 0x41, 0x72, 0x67,
	0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x00, 0x02, 0x03, 0x65, 0x6e, 0x76, 0x0e, 0x63, 0x68, 0x65,
	0x63, 0x6b, 0x4e, 0x6f, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x00, 0x03, 0x03, 0x65, 0x6e,
	0x76, 0x19, 0x62, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x47, 0x65, 0x74, 0x55, 0x6e, 0x73, 0x69, 0x67,
	0x6e, 0x65, 0x64, 0x41, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x00, 0x01, 0x03, 0x65, 0x6e,
	0x76, 0x09, 0x62, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x43, 0x6d, 0x70, 0x00, 0x04, 0x03, 0x65, 0x6e,
	0x76, 0x09, 0x62, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x4d, 0x75, 0x6c, 0x00, 0x05, 0x03, 0x65, 0x6e,
	0x76, 0x09, 0x62, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x00, 0x05, 0x03, 0x65, 0x6e,
	0x76, 0x14, 0x62, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x46, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x55, 0x6e,
	0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x00, 0x06, 0x03, 0x08, 0x07, 0x02, 0x03, 0x03, 0x06, 0x03,
	0x03, 0x03, 0x04, 0x05, 0x01, 0x70, 0x01, 0x01, 0x01, 0x05, 0x03, 0x01, 0x00, 0x11, 0x06, 0x19,
	0x03, 0x7f, 0x01, 0x41, 0x80, 0x80, 0xc0, 0x00, 0x0b, 0x7f, 0x00, 0x41, 0x99, 0x80, 0xc0, 0x00,
	0x0b, 0x7f, 0x00, 0x41, 0x99, 0x80, 0xc0, 0x00, 0x0b, 0x07, 0x43, 0x06, 0x06, 0x6d, 0x65, 0x6d,
	0x6f, 0x72, 0x79, 0x02, 0x00, 0x04, 0x69, 0x6e, 0x69, 0x74, 0x00, 0x0d, 0x09, 0x66, 0x61, 0x63,
	0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x00, 0x0e, 0x08, 0x63, 0x61, 0x6c, 0x6c, 0x42, 0x61, 0x63,
	0x6b, 0x00, 0x0f, 0x0a, 0x5f, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x65, 0x6e, 0x64, 0x03, 0x01,
	0x0b, 0x5f, 0x5f, 0x68, 0x65, 0x61, 0x70, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x03, 0x02, 0x0a, 0xf1,
	0x01, 0x07, 0x0a, 0x00, 0x42, 0x01, 0x10, 0x80, 0x80, 0x80, 0x80, 0x00, 0x0b, 0x09, 0x00, 0x10,
	0x8b, 0x80, 0x80, 0x80, 0x00, 0x00, 0x0b, 0x11, 0x00, 0x41, 0x80, 0x80, 0xc0, 0x80, 0x00, 0x41,
	0x19, 0x10, 0x81, 0x80, 0x80, 0x80, 0x00, 0x00, 0x0b, 0x18, 0x00, 0x02, 0x40, 0x10, 0x82, 0x80,
	0x80, 0x80, 0x00, 0x20, 0x00, 0x47, 0x0d, 0x00, 0x0f, 0x0b, 0x10, 0x8a, 0x80, 0x80, 0x80, 0x00,
	0x00, 0x0b, 0x10, 0x00, 0x10, 0x83, 0x80, 0x80, 0x80, 0x00, 0x41, 0x00, 0x10, 0x8c, 0x80, 0x80,
	0x80, 0x00, 0x0b, 0x9a, 0x01, 0x01, 0x05, 0x7f, 0x10, 0x83, 0x80, 0x80, 0x80, 0x00, 0x41, 0x01,
	0x10, 0x8c, 0x80, 0x80, 0x80, 0x00, 0x41, 0x00, 0x42, 0x00, 0x10, 0x80, 0x80, 0x80, 0x80, 0x00,
	0x22, 0x00, 0x10, 0x84, 0x80, 0x80, 0x80, 0x00, 0x20, 0x00, 0x42, 0x00, 0x10, 0x80, 0x80, 0x80,
	0x80, 0x00, 0x10, 0x85, 0x80, 0x80, 0x80, 0x00, 0x21, 0x01, 0x10, 0x89, 0x80, 0x80, 0x80, 0x00,
	0x21, 0x02, 0x02, 0x40, 0x20, 0x01, 0x45, 0x0d, 0x00, 0x10, 0x89, 0x80, 0x80, 0x80, 0x00, 0x21,
	0x03, 0x10, 0x89, 0x80, 0x80, 0x80, 0x00, 0x21, 0x01, 0x03, 0x40, 0x20, 0x01, 0x20, 0x00, 0x10,
	0x85, 0x80, 0x80, 0x80, 0x00, 0x22, 0x04, 0x41, 0x00, 0x47, 0x41, 0x7f, 0x20, 0x04, 0x41, 0x7f,
	0x4a, 0x1b, 0x41, 0x01, 0x6a, 0x41, 0x01, 0x4b, 0x0d, 0x01, 0x20, 0x02, 0x20, 0x02, 0x20, 0x01,
	0x10, 0x86, 0x80, 0x80, 0x80, 0x00, 0x20, 0x01, 0x20, 0x01, 0x20, 0x03, 0x10, 0x87, 0x80, 0x80,
	0x80, 0x00, 0x0c, 0x00, 0x0b, 0x0b, 0x20, 0x02, 0x10, 0x88, 0x80, 0x80, 0x80, 0x00, 0x0b, 0x02,
	0x00, 0x0b, 0x0b, 0x22, 0x01, 0x00, 0x41, 0x80, 0x80, 0xc0, 0x00, 0x0b, 0x19, 0x77, 0x72, 0x6f,
	0x6e, 0x67, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x72, 0x67,
	0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73,
];

#[elrond_wasm_derive::contract(DeployTwoContractsImpl)]
pub trait DeployTwoContracts {
	#[init]
	fn init() {}

	#[endpoint(deployContract)]
	fn deploy_contract(&self) -> SCResult<Address> {
		let deployed_contract_address = self.deploy();
		if deployed_contract_address.is_zero() {
			return sc_error!("Deploy failed");
		}

		Ok(deployed_contract_address)
	}

	#[endpoint(deployTwoContracts)]
	fn deploy_two_contracts(&self) -> SCResult<(Address, Address)> {
		let first_deployed_contract_address = self.deploy();
		if first_deployed_contract_address.is_zero() {
			return sc_error!("First deploy failed");
		}

		let second_deployed_contract_address = self.deploy();
		if second_deployed_contract_address.is_zero() {
			return sc_error!("Second deploy failed");
		}

		Ok((
			first_deployed_contract_address,
			second_deployed_contract_address,
		))
	}

	fn deploy(&self) -> Address {
		self.send().deploy_contract(
			self.get_gas_left(),
			&BigUint::zero(),
			&BoxedBytes::from(CONTRACT_CODE),
			CodeMetadata::DEFAULT,
			&ArgBuffer::new(),
		)
	}
}