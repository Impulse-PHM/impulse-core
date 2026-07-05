//! The integration tests for [`impulse_core::model::bioactive`]

mod common;

use tempfile::NamedTempFile;

use impulse_core::{
    BioactiveAgentBuilder, BioactiveAgentCategory, CoreDatabase, GetByName, ImpulseCore, ManageBioactiveAgent, ManageUser, Unit, UserBuilder, UserDatabase, environment, model::bioactive::{BIOACTIVE_AGENT_CATEGORY_OTC, DEFAULT_CREATED_AT, DEFAULT_ID, DEFAULT_IS_DELETED}
};


/// Verify that a bioactive agent can be saved with all required fields set
#[test]
fn save_bioactive_agent_with_required_fields() {
    common::setup_logging();

    let core_db_temp_file: NamedTempFile = common::create_core_database_temp_file();
    let user_db_temp_file: NamedTempFile = common::create_user_database_temp_file();

    let core_database: CoreDatabase = common::create_test_core_database_with_defaults(core_db_temp_file.path().to_owned())
        .expect("Failed to create the core database");

    let user_database: UserDatabase = common::create_test_user_database_with_defaults(user_db_temp_file.path().to_owned())
        .expect("Failed to create the user database");

    environment::setup_user_database(&user_database)
        .expect("Failed to setup the user database");
    
    let mut impulse_core = ImpulseCore::new(core_database, user_database);

    let user = UserBuilder::new()
        .with_first_name("Tony")
        .with_last_name("Stark")
        .with_birth_year(1970)
        .with_birth_month(5)
        .with_birth_day(29)
        .with_is_primary(true)
        .build()
        .expect("Failed to build a user");

    let saved_user = impulse_core
        .save_user(&user)
        .expect("Failed to save a new user in the database");
    

    let quantity_unit_id = Unit::get_by_abbreviation(impulse_core.get_user_database(), "mg")
        .expect("Failed to get a unit by its abbreviation")
        .id;

    let frequency_unit_id = Unit::get_by_name(impulse_core.get_user_database(), "as needed")
        .expect("Failed to get a unit by its frequency")
        .id;

    let agent_category_id = BioactiveAgentCategory::get_by_name(
        impulse_core.get_user_database(), BIOACTIVE_AGENT_CATEGORY_OTC
    )
        .expect("Failed to get a bioactive agent category by its name")
        .id;

    let agent = BioactiveAgentBuilder::new()
        .with_user_id(saved_user.id)
        .with_name("fexofenadine")
        .with_quantity(180.0)
        .with_quantity_unit_id(quantity_unit_id)
        .with_frequency_unit_id(frequency_unit_id)
        .with_agent_category_id(agent_category_id)
        .build()
        .expect("Failed to build a bioactive agent");

    // Expecting default values for the id, created_at, and is_deleted fields
    assert_eq!(
        agent.id, DEFAULT_ID, 
        "The ID should be 0 since it has not been assigned in the database yet"
    );
    
    assert_eq!(
        agent.created_at, DEFAULT_CREATED_AT, 
        "created_at should be 0 since it has not been assigned in the database yet"
    );

    assert_eq!(
        agent.is_deleted, DEFAULT_IS_DELETED, 
        "is_deleted should be false"
    );

    let saved_agent = impulse_core
        .save_bioactive_agent(&agent)
        .expect("Failed to save a new bioactive agent in the database");

    assert_ne!(
        saved_agent, agent, 
        "Should not be equal as the id and created_at fields should have different values"
    );

    let mut expected_agent = agent.clone();
    expected_agent.id = saved_agent.id;
    expected_agent.created_at = saved_agent.created_at;

    assert_eq!(
        saved_agent, expected_agent,
        "Both should be equal now since the database assigned real values for the internal id and \
        created_at columns."
    );

}

