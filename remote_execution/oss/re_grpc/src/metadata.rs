/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

pub type TPlatform = crate::grpc::Platform;
pub type TProperty = crate::grpc::Property;

#[derive(Clone, Default)]
pub struct ActionHistoryInfo {
    pub action_key: String,
    pub disable_retry_on_oom: bool,
    pub _dot_dot: (),
}

#[derive(Clone, Default)]
pub struct HostResourceRequirements {
    pub affinity_keys: Vec<String>,
    pub input_files_bytes: i64,
    pub _dot_dot: (),
}

#[derive(Clone, Default)]
pub struct BuckInfo {
    pub build_id: String,
    pub _dot_dot: (),
}

#[derive(Clone, Default)]
pub struct TDependency {
    pub smc_tier: String,
    pub id: String,
    pub _dot_dot: (),
}

#[derive(Clone, Default)]
pub struct RemoteExecutionMetadata {
    pub action_history_info: Option<ActionHistoryInfo>,
    pub buck_info: Option<BuckInfo>,
    pub host_resource_requirements: Option<HostResourceRequirements>,
    pub platform: Option<TPlatform>,
    pub use_case_id: String,
    pub do_not_cache: bool,
    pub dependencies: Vec<TDependency>,
    pub _dot_dot: (),
}
