use std::vec;

use oci_spec::runtime::{
    LinuxBuilder, LinuxIdMapping, LinuxIdMappingBuilder, LinuxNamespace, LinuxNamespaceType,
    ProcessBuilder, Spec, SpecBuilder,
};
use test_framework::{Test, TestGroup, TestResult};

use crate::utils::test_inside_container;

fn create_spec(uid_mapping: Vec<LinuxIdMapping>, gid_mapping: Vec<LinuxIdMapping>) -> Spec {
    // Get default namespaces and filter them to optional exclude network or user namespaces
    let default_namespaces: Vec<LinuxNamespace> = oci_spec::runtime::get_default_namespaces()
        .into_iter()
        .filter(|ns| match ns.typ() {
            LinuxNamespaceType::User => true,
            _ => true,
        })
        .collect();

    SpecBuilder::default()
        .linux(
            LinuxBuilder::default()
                .namespaces(default_namespaces)
                .uid_mappings(uid_mapping)
                .gid_mappings(gid_mapping)
                .build()
                .expect("error in building linux config"),
        )
        .process(
            ProcessBuilder::default()
                .args(vec![
                    "runtimetest".to_string(),
                    "uid_mappings".to_string(),
                    "gid_mappings".to_string(),
                ])
                .build()
                .expect("error in creating process config"),
        )
        .build()
        .unwrap()
}

fn uid_mappings_test() -> TestResult {
    let uid_mapping = vec![LinuxIdMappingBuilder::default()
        .host_id(1000u32)
        .container_id(0u32)
        .size(2000u32)
        .build()
        .unwrap()];

    let gid_mapping = vec![LinuxIdMappingBuilder::default()
        .container_id(1000u32)
        .host_id(0u32)
        .size(3000u32)
        .build()
        .unwrap()];

    let spec = create_spec(uid_mapping, gid_mapping);
    test_inside_container(spec, &|_| Ok(()))
}

pub fn get_uid_mappings_test() -> TestGroup {
    let mut test_group = TestGroup::new("uid_mappings");
    let uid_mappings_test = Test::new("uid_mappings_test", Box::new(uid_mappings_test));
    test_group.add(vec![Box::new(uid_mappings_test)]);

    test_group
}
