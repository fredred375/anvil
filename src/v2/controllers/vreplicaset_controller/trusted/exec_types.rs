// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::error::UnmarshalError;
use crate::kubernetes_api_objects::exec::{
    api_resource::*, label_selector::*, pod_template_spec::*, prelude::*,
};
use crate::kubernetes_api_objects::spec::resource::*;
use crate::vreplicaset_controller::trusted::spec_types;
use crate::vstd_ext::string_map::StringMap;
use deps_hack::kube::Resource;
use vstd::prelude::*;
verus! {

#[verifier(external_body)]
pub struct VReplicaSet {
    inner: deps_hack::VReplicaSet
}

impl View for VReplicaSet {
    type V = spec_types::VReplicaSetView;

    spec fn view(&self) -> spec_types::VReplicaSetView;
}

impl VReplicaSet {
    #[verifier(external_body)]
    pub fn metadata(&self) -> (metadata: ObjectMeta)
        ensures metadata@ == self@.metadata,
    {
        ObjectMeta::from_kube(self.inner.metadata.clone())
    }

    #[verifier(external_body)]
    pub fn spec(&self) -> (spec: VReplicaSetSpec)
        ensures spec@ == self@.spec,
    {
        VReplicaSetSpec { inner: self.inner.spec.clone() }
    }

    #[verifier(external_body)]
    pub fn api_resource() -> (res: ApiResource)
        ensures res@.kind == spec_types::VReplicaSetView::kind(),
    {
        ApiResource::from_kube(deps_hack::kube::api::ApiResource::erase::<deps_hack::VReplicaSet>(&()))
    }

    #[verifier(external_body)]
    pub fn controller_owner_ref(&self) -> (owner_reference: OwnerReference)
        ensures owner_reference@ == self@.controller_owner_ref(),
    {
        // We can safely unwrap here because the trait method implementation always returns a Some(...)
        OwnerReference::from_kube(self.inner.controller_owner_ref(&()).unwrap())
    }

    // NOTE: This function assumes serde_json::to_string won't fail!
    #[verifier(external_body)]
    pub fn marshal(self) -> (obj: DynamicObject)
        ensures obj@ == self@.marshal(),
    {
        // TODO: this might be unnecessarily slow
        DynamicObject::from_kube(deps_hack::k8s_openapi::serde_json::from_str(&deps_hack::k8s_openapi::serde_json::to_string(&self.inner).unwrap()).unwrap())
    }

    #[verifier(external_body)]
    pub fn unmarshal(obj: DynamicObject) -> (res: Result<VReplicaSet, UnmarshalError>)
        ensures
            res.is_Ok() == spec_types::VReplicaSetView::unmarshal(obj@).is_Ok(),
            res.is_Ok() ==> res.get_Ok_0()@ == spec_types::VReplicaSetView::unmarshal(obj@).get_Ok_0(),
    {
        let parse_result = obj.into_kube().try_parse::<deps_hack::VReplicaSet>();
        if parse_result.is_ok() {
            let res = VReplicaSet { inner: parse_result.unwrap() };
            Ok(res)
        } else {
            Err(())
        }
    }

    pub fn state_validation(&self) -> (res: bool) 
        ensures
            res == self@.state_validation()
    {
        
        // replicas exists and non-negative
        if let Some(replicas) = self.spec().replicas() {
            if replicas < 0 {
                return false;
            }
        }

        // Check if selector's match_labels exist and are non-empty
        if let Some(match_labels) = self.spec().selector().match_labels() {
            if match_labels.len() <= 0 {
                return false;
            }
        } else {
            return false;
        }

        // template, metadata, and spec exist
        if let Some(template) = self.spec().template() {
            if template.metadata().is_none() || template.spec().is_none() {
                return false;
            }
            
            // Map::empty() did not compile
            if !self.spec().selector().matches(template.metadata().unwrap().labels().unwrap_or(StringMap::empty())) {
                return false;
            }

        } else {
            return false;
        }
    
        true
    }
}

#[verifier(external)]
impl ResourceWrapper<deps_hack::VReplicaSet> for VReplicaSet {
    fn from_kube(inner: deps_hack::VReplicaSet) -> VReplicaSet { VReplicaSet { inner: inner } }

    fn into_kube(self) -> deps_hack::VReplicaSet { self.inner }
}

#[verifier(external_body)]
pub struct VReplicaSetSpec {
    inner: deps_hack::VReplicaSetSpec,
}

impl VReplicaSetSpec {
    pub spec fn view(&self) -> spec_types::VReplicaSetSpecView;

    #[verifier(external_body)]
    pub fn replicas(&self) -> (replicas: Option<i32>)
        ensures
            replicas.is_Some() == self@.replicas.is_Some(),
            replicas.is_Some() ==> replicas.get_Some_0() as int == self@.replicas.get_Some_0(),
    {
        self.inner.replicas
    }

    #[verifier(external_body)]
    pub fn selector(&self) -> (selector: LabelSelector)
        ensures selector@ == self@.selector
    {
        LabelSelector::from_kube(self.inner.selector.clone())
    }

    #[verifier(external_body)]
    pub fn template(&self) -> (template: Option<PodTemplateSpec>)
        ensures
            template.is_Some() == self@.template.is_Some(),
            template.is_Some() ==> template.get_Some_0()@ == self@.template.get_Some_0(),
    {
        match &self.inner.template {
            Some(t) => Some(PodTemplateSpec::from_kube(t.clone())),
            None => None
        }
    }
}

}
