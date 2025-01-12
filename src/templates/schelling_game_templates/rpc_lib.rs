use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use {{underscore_name}}_runtime_api::{{runtime_pallet_name}}Api as {{runtime_pallet_name}}RuntimeApi;
use sp_api::codec::Codec;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;
{% if schelling_game_name is containing("profile-validation") %}
type ChallengePostId = u64;
{% endif %}
{% if params_type is containing("number") %}
type {{params_variable_type}} = {{param_type_value}};
{% endif %}
#[rpc(client, server)]
pub trait {{runtime_pallet_name}}Api<BlockHash, AccountId> {
	{% if schelling_game_name is containing("profile-validation") %}
	#[method(name = "{{rpc_url}}_challengerevidence")]
	fn get_challengers_evidence(
		&self,
		{{params_variable}}: {{params_variable_type}},
		offset: u64,
		limit: u16,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<ChallengePostId>>;
	{% endif %}
	
	#[method(name = "{{rpc_url}}_evidenceperiodendblock")]
	fn get_evidence_period_end_block(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<BlockHash>,
	) -> RpcResult<Option<u32>>;
	#[method(name = "{{rpc_url}}_stakingperiodendblock")]
	fn get_staking_period_end_block(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<BlockHash>,
	) -> RpcResult<Option<u32>>;
	#[method(name = "{{rpc_url}}_drawingperiodend")]
	fn get_drawing_period_end(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<BlockHash>,
	) -> RpcResult<(u64, u64, bool)>;
	#[method(name = "{{rpc_url}}_commitendblock")]
	fn get_commit_period_end_block(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<BlockHash>,
	) -> RpcResult<Option<u32>>;
	#[method(name = "{{rpc_url}}_voteendblock")]
	fn get_vote_period_end_block(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<BlockHash>,
	) -> RpcResult<Option<u32>>;
	#[method(name = "{{rpc_url}}_selectedjuror")]
	fn selected_as_juror(
		&self,
		{{params_variable}}: {{params_variable_type}},
		who: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<bool>;
}

/// A struct that implements the `SumStorageApi`.
pub struct {{runtime_pallet_name}}<C, M> {
	// If you have more generics, no need to SumStorage<C, M, N, P, ...>
	// just use a tuple like SumStorage<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> {{runtime_pallet_name}}<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}


/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}


impl<C, Block, AccountId> {{runtime_pallet_name}}ApiServer<<Block as BlockT>::Hash, AccountId> for {{runtime_pallet_name}}<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: {{runtime_pallet_name}}RuntimeApi<Block, AccountId>,
{
	{% if schelling_game_name is containing("profile-validation") %}
	fn get_challengers_evidence(
		&self,
		{{params_variable}}: {{params_variable_type}},
		offset: u64,
		limit: u16,
		at: Option<Block::Hash>,
	) -> RpcResult<Vec<ChallengePostId>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

		let runtime_api_result =
			api.get_challengers_evidence(at, {{params_variable}}, offset, limit);
			fn map_err(error: impl ToString, desc: &'static str) -> CallError {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					desc,
					Some(error.to_string()),
				))
			}
			let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
			Ok(res)
	}

	{% endif %}
	fn get_evidence_period_end_block(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<Block::Hash>,
	) -> RpcResult<Option<u32>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

		let runtime_api_result = api.get_evidence_period_end_block(at, {{params_variable}});
		fn map_err(error: impl ToString, desc: &'static str) -> CallError {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				desc,
				Some(error.to_string()),
			))
		}
		let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
			Ok(res)
	}
	fn get_staking_period_end_block(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<Block::Hash>,
	) -> RpcResult<Option<u32>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

		let runtime_api_result = api.get_staking_period_end_block(at, {{params_variable}});
		fn map_err(error: impl ToString, desc: &'static str) -> CallError {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				desc,
				Some(error.to_string()),
			))
		}
		let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
			Ok(res)
	}
	fn get_drawing_period_end(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<Block::Hash>,
	) -> RpcResult<(u64, u64, bool)> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

		let runtime_api_result = api.get_drawing_period_end(at, {{params_variable}});
		fn map_err(error: impl ToString, desc: &'static str) -> CallError {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				desc,
				Some(error.to_string()),
			))
		}
		let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
			Ok(res)
	}

	fn get_commit_period_end_block(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<Block::Hash>,
	) -> RpcResult<Option<u32>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

		let runtime_api_result = api.get_commit_period_end_block(at, {{params_variable}});
		fn map_err(error: impl ToString, desc: &'static str) -> CallError {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				desc,
				Some(error.to_string()),
			))
		}
		let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
			Ok(res)
	}

	fn get_vote_period_end_block(
		&self,
		{{params_variable}}: {{params_variable_type}},
		at: Option<Block::Hash>,
	) -> RpcResult<Option<u32>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

		let runtime_api_result = api.get_vote_period_end_block(at, {{params_variable}});
		fn map_err(error: impl ToString, desc: &'static str) -> CallError {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				desc,
				Some(error.to_string()),
			))
		}
		let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
			Ok(res)
	}

	fn selected_as_juror(
		&self,
		{{params_variable}}: {{params_variable_type}},
		who: AccountId,
		at: Option<Block::Hash>,
	) -> RpcResult<bool> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash);

		let runtime_api_result = api.selected_as_juror(at, {{params_variable}}, who);
		fn map_err(error: impl ToString, desc: &'static str) -> CallError {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				desc,
				Some(error.to_string()),
			))
		}
		let res = runtime_api_result.map_err(|e| map_err(e, "Unable to query dispatch info."))?;
			Ok(res)
	}
}