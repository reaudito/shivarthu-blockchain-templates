
use shivarthu_blockchain_templates::modules::department_funding::department_funding;
use shivarthu_blockchain_templates::modules::profile_validation::profile_validation;
use shivarthu_blockchain_templates::modules::positive_externality::positive_externality;
use shivarthu_blockchain_templates::modules::project_tips::project_tips;
fn main() {
    profile_validation();
    department_funding();
    positive_externality();
    project_tips();
}
