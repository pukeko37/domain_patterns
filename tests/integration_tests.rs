use domain_patterns::collections::*;
mod common;
use common::*;

#[test]
#[allow(unused)]
fn test_add_user() {
    let user_id = "test_id".to_string();
    let test_user = common::create_test_user(&user_id);
    let mut user_repo = MockUserRepository::new();
    user_repo.insert(&test_user);
    let success_result = user_repo.get(&user_id).unwrap();

    assert_eq!(&success_result.unwrap().first_name, &test_user.first_name)
}

#[test]
#[allow(unused)]
fn test_cant_add_duplicate() {
    let user_id = "test_id".to_string();
    let test_user = common::create_test_user(&user_id);
    let mut user_repo = MockUserRepository::new();
    let returned_entity = user_repo.insert(&test_user).unwrap();
    assert!(returned_entity.is_some());

    let success_result = user_repo.get(&user_id).unwrap();
    assert_eq!(&success_result.unwrap().first_name, &test_user.first_name);

    let failure_result = user_repo.insert(&test_user).unwrap();
    assert!(failure_result.is_none());
}

#[test]
#[allow(unused)]
fn test_update_user() {
    let user_id = "test_id".to_string();
    let mut test_user = common::create_test_user(&user_id);
    let mut user_repo = MockUserRepository::new();
    let returned_entity = user_repo.insert(&test_user).unwrap();
    assert!(returned_entity.is_some());

    let updated_name = "new_name".to_string();
    test_user.first_name = updated_name.clone();
    let mut updated_user = user_repo.update(&test_user).unwrap();
    // check that we get back Some() which implies updating worked.
    assert!(returned_entity.is_some());
    // Check that our name is correct in the returned (updated) user.
    assert_eq!(&updated_user.unwrap().first_name, &updated_name);

    // sanity check with fresh get and check that name was updated;
    updated_user = user_repo.get(&user_id).unwrap();
    assert_eq!(&updated_user.unwrap().first_name, &updated_name);
}

#[test]
#[allow(unused)]
fn test_remove_user() {
    let user_id = "test_id".to_string();
    let test_user = common::create_test_user(&user_id);
    let mut user_repo = MockUserRepository::new();
    user_repo.insert(&test_user);

    // we first check that user is in repo
    assert!(user_repo.contains_key(&user_id).unwrap());

    user_repo.remove(&user_id);
    assert!(!user_repo.contains_key(&user_id).unwrap())
}

#[test]
#[allow(unused)]
fn test_get_paged() {
    let user_id1 = "test_id1".to_string();
    let test_user1 = common::create_test_user(&user_id1);

    let user_id2 = "test_id2".to_string();
    let test_user2 = common::create_test_user(&user_id2);
    let mut user_repo = MockUserRepository::new();

    user_repo.insert(&test_user1);
    assert!(user_repo.contains_key(&user_id1).unwrap());
    user_repo.insert(&test_user2);
    assert!(user_repo.contains_key(&user_id2).unwrap());

    let results = user_repo.get_paged(1, 2).unwrap();
    assert_eq!(results.len(), 2)
}
