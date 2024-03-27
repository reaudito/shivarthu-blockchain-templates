use crate::*;


impl<T: Config> Pallet<T> {
	
	// Block code start

	pub fn get_evidence_period_end_block({{params_variable}}: {{params_variable_pallet_function_type}}) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		{{get_key}}


		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_evidence_period_end_block_helper_link(
			key, phase_data, now,
		);
		result


	}

	
	pub fn get_staking_period_end_block({{params_variable}}: {{params_variable_pallet_function_type}}) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		
		{{get_key}}

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_staking_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn get_drawing_period_end({{params_variable}}: {{params_variable_pallet_function_type}}) -> (u64, u64, bool) {
		{{get_key}}
		let phase_data = Self::get_phase_data();

		let result =
			T::SchellingGameSharedSource::get_drawing_period_end_helper_link(key, phase_data);
		result
	}

	pub fn get_commit_period_end_block({{params_variable}}: {{params_variable_pallet_function_type}}) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();
		
		{{get_key}}

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_commit_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn get_vote_period_end_block({{params_variable}}: {{params_variable_pallet_function_type}}) -> Option<u32> {
		let now = <frame_system::Pallet<T>>::block_number();

		{{get_key}}

		let phase_data = Self::get_phase_data();

		let result = T::SchellingGameSharedSource::get_vote_period_end_block_helper_link(
			key, phase_data, now,
		);
		result
	}

	pub fn selected_as_juror({{params_variable}}: {{params_variable_pallet_function_type}}, who: T::AccountId) -> bool {
		{{get_key}}

		let result = T::SchellingGameSharedSource::selected_as_juror_helper_link(key, who);
		result
	}

	// Block code end


}
