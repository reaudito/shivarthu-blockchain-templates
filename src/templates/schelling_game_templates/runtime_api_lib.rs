#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::sp_std::{vec::Vec};
//  or
use frame_support::sp_std::{prelude::*};
use sp_api::codec::Codec;
{% if schelling_game_name is containing("profile-validation") %}
type ChallengePostId = u64;
{% endif %}
{% if params_type is containing("number") %}
type {{params_variable_type}} = {{param_type_value}};
{% endif %}

sp_api::decl_runtime_apis! {
	pub trait {{runtime_pallet_name}}Api<AccountId> where AccountId: Codec {
		{% if schelling_game_name is containing("profile-validation") %}
		fn get_challengers_evidence({{params_variable}}: {{params_variable_type}}, offset: u64, limit: u16) -> Vec<ChallengePostId>;
		{% endif %}
		fn get_evidence_period_end_block({{params_variable}}: {{params_variable_type}}) -> Option<u32>;
		fn get_staking_period_end_block({{params_variable}}: {{params_variable_type}}) -> Option<u32>;
		fn get_drawing_period_end({{params_variable}}: {{params_variable_type}}) -> (u64, u64, bool);
		fn get_commit_period_end_block({{params_variable}}: {{params_variable_type}}) -> Option<u32>;
		fn get_vote_period_end_block({{params_variable}}: {{params_variable_type}}) -> Option<u32>;
		fn selected_as_juror({{params_variable}}: {{params_variable_type}}, who: AccountId) -> bool;
	}
}