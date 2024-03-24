impl profile_validation_runtime_api::ProfileValidationApi<Block, AccountId> for Runtime {

    fn get_challengers_evidence(profile_user_account: AccountId, offset: u64, limit: u16) -> Vec<ChallengePostId> {
        ProfileValidation::get_challengers_evidence(profile_user_account, offset, limit)
    }

    fn get_evidence_period_end_block(profile_user_account: AccountId) -> Option<u32> {
        ProfileValidation::get_evidence_period_end_block(profile_user_account)
    }

    fn get_staking_period_end_block(profile_user_account: AccountId) -> Option<u32> {
        ProfileValidation::get_staking_period_end_block(profile_user_account)
    }
    fn get_drawing_period_end(profile_user_account: AccountId) -> (u64, u64, bool) {
        ProfileValidation::get_drawing_period_end(profile_user_account)
    }
    fn get_commit_period_end_block(profile_user_account: AccountId) -> Option<u32> {
        ProfileValidation::get_commit_period_end_block(profile_user_account)
    }

    fn get_vote_period_end_block(profile_user_account: AccountId) -> Option<u32> {
        ProfileValidation::get_vote_period_end_block(profile_user_account)
    }
    fn selected_as_juror(profile_user_account: AccountId, who: AccountId) -> bool {
        ProfileValidation::selected_as_juror(profile_user_account, who)
    }
}