impl {{underscore_name}}_runtime_api::{{trait_name}}<Block, AccountId> for Runtime {

    fn get_challengers_evidence({{params_variable}}: {{params_variable_type}}, offset: u64, limit: u16) -> Vec<ChallengePostId> {
        {{runtime_pallet_name}}::get_challengers_evidence({{params_variable}}, offset, limit)
    }

    fn get_evidence_period_end_block({{params_variable}}: {{params_variable_type}}) -> Option<u32> {
        {{runtime_pallet_name}}::get_evidence_period_end_block({{params_variable}})
    }

    fn get_staking_period_end_block({{params_variable}}: {{params_variable_type}}) -> Option<u32> {
        {{runtime_pallet_name}}::get_staking_period_end_block({{params_variable}})
    }
    fn get_drawing_period_end({{params_variable}}: {{params_variable_type}}) -> (u64, u64, bool) {
        {{runtime_pallet_name}}::get_drawing_period_end({{params_variable}})
    }
    fn get_commit_period_end_block({{params_variable}}: {{params_variable_type}}) -> Option<u32> {
        {{runtime_pallet_name}}::get_commit_period_end_block({{params_variable}})
    }

    fn get_vote_period_end_block({{params_variable}}: {{params_variable_type}}) -> Option<u32> {
        {{runtime_pallet_name}}::get_vote_period_end_block({{params_variable}})
    }
    fn selected_as_juror({{params_variable}}: {{params_variable_type}}, who: AccountId) -> bool {
        {{runtime_pallet_name}}::selected_as_juror({{params_variable}}, who)
    }
}