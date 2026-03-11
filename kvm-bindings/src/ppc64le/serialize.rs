// Copyright 2024 © Institute of Software, CAS. All rights reserved.
// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::bindings::{
    kvm_allocate_rma, kvm_book3e_206_tlb_entry, kvm_book3e_206_tlb_params,
    kvm_create_spapr_tce, kvm_create_spapr_tce_64, kvm_debug_exit_arch, kvm_fpu,
    kvm_get_htab_fd, kvm_get_htab_header, kvm_guest_debug_arch, kvm_irq_routing,
    kvm_irq_routing_entry, kvm_irq_routing_entry__bindgen_ty_1, kvm_irq_routing_msi,
    kvm_irq_routing_msi__bindgen_ty_1, kvm_mp_state, kvm_one_reg, kvm_ppc_cpu_char,
    kvm_ppc_mmuv3_cfg, kvm_ppc_one_page_size, kvm_ppc_one_seg_page_size, kvm_ppc_pvinfo,
    kvm_ppc_resize_hpt, kvm_ppc_rmmu_info, kvm_ppc_rmmu_info_kvm_ppc_radix_geom,
    kvm_ppc_smmu_info, kvm_ppc_xive_eq, kvm_regs, kvm_rtas_token_args, kvm_sregs,
    kvm_sync_regs,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zerocopy::{IntoBytes, transmute};

serde_impls! {
    kvm_regs,
    kvm_sregs,
    kvm_fpu,
    kvm_debug_exit_arch,
    kvm_guest_debug_arch,
    kvm_sync_regs,
    kvm_mp_state,
    kvm_one_reg,
    kvm_create_spapr_tce,
    kvm_create_spapr_tce_64,
    kvm_allocate_rma,
    kvm_rtas_token_args,
    kvm_book3e_206_tlb_entry,
    kvm_book3e_206_tlb_params,
    kvm_get_htab_fd,
    kvm_get_htab_header,
    kvm_ppc_mmuv3_cfg,
    kvm_ppc_rmmu_info,
    kvm_ppc_rmmu_info_kvm_ppc_radix_geom,
    kvm_ppc_cpu_char,
    kvm_ppc_xive_eq,
    kvm_ppc_pvinfo,
    kvm_ppc_one_page_size,
    kvm_ppc_one_seg_page_size,
    kvm_ppc_smmu_info,
    kvm_ppc_resize_hpt,
    kvm_irq_routing,
    kvm_irq_routing_entry,
    kvm_irq_routing_msi
}

// SAFETY: zerocopy's derives explicitly disallow deriving for unions where
// the fields have different sizes, due to the smaller fields having padding.
// Miri however does not complain about these implementations (e.g. about
// reading the "padding" for one union field as valid data for a bigger one)
unsafe impl IntoBytes for kvm_irq_routing_msi__bindgen_ty_1 {
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized,
    {
    }
}

// SAFETY: zerocopy's derives explicitly disallow deriving for unions where
// the fields have different sizes, due to the smaller fields having padding.
// Miri however does not complain about these implementations (e.g. about
// reading the "padding" for one union field as valid data for a bigger one)
unsafe impl IntoBytes for kvm_irq_routing_entry__bindgen_ty_1 {
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized,
    {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    fn is_serde<T: Serialize + for<'de> Deserialize<'de> + Default>() {
        let config = bincode::config::standard();
        let serialized = bincode::serde::encode_to_vec(T::default(), config).unwrap();
        let (deserialized, _): (T, _) =
            bincode::serde::decode_from_slice(&serialized, config).unwrap();
        let serialized_again = bincode::serde::encode_to_vec(&deserialized, config).unwrap();
        // Compare the serialized state after a roundtrip, to work around issues with
        // bindings not implementing `PartialEq`.
        assert_eq!(serialized, serialized_again);
    }

    #[test]
    fn static_assert_serde_implementations() {
        // This test statically (= at compile-time) asserts that various bindgen generated
        // structures implement serde's `Serialize` and `Deserialize` traits.
        // This is to make sure that we do not accidentally remove those implementations
        // when regenerating bindings. If this test fails to compile, please add
        //
        // #[cfg_attr(
        //     feature = "serde",
        //     derive(zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::FromBytes)
        // )]
        //
        // to all structures causing compilation errors (we need the zerocopy traits, as the
        // `Serialize` and `Deserialize` implementations are provided by the `serde_impls!` macro
        // above, which implements serialization based on zerocopy's `FromBytes` and `IntoBytes`
        // traits that it expects to be derived).
        //
        // NOTE: This only include "top-level" items, and does not list out bindgen-anonymous types
        // (e.g. types like `kvm_vcpu_events__bindgen_ty_5`). These types can change name across
        // bindgen versions. If after re-adding the derives to all the below items you can compile
        // errors about anonymous types not implementing `Serialize`/`Deserialize`, please also add
        // the derives to all anonymous types references in the definitions of the below items.

        is_serde::<kvm_regs>();
        is_serde::<kvm_sregs>();
        is_serde::<kvm_fpu>();
        is_serde::<kvm_debug_exit_arch>();
        is_serde::<kvm_guest_debug_arch>();
        is_serde::<kvm_sync_regs>();
        is_serde::<kvm_mp_state>();
        is_serde::<kvm_one_reg>();
        is_serde::<kvm_create_spapr_tce>();
        is_serde::<kvm_create_spapr_tce_64>();
        is_serde::<kvm_allocate_rma>();
        is_serde::<kvm_rtas_token_args>();
        is_serde::<kvm_book3e_206_tlb_entry>();
        is_serde::<kvm_book3e_206_tlb_params>();
        is_serde::<kvm_get_htab_fd>();
        is_serde::<kvm_get_htab_header>();
        is_serde::<kvm_ppc_mmuv3_cfg>();
        is_serde::<kvm_ppc_rmmu_info>();
        is_serde::<kvm_ppc_rmmu_info_kvm_ppc_radix_geom>();
        is_serde::<kvm_ppc_cpu_char>();
        is_serde::<kvm_ppc_xive_eq>();
        is_serde::<kvm_ppc_pvinfo>();
        is_serde::<kvm_ppc_one_page_size>();
        is_serde::<kvm_ppc_one_seg_page_size>();
        is_serde::<kvm_ppc_smmu_info>();
        is_serde::<kvm_ppc_resize_hpt>();
        is_serde::<kvm_irq_routing>();
        is_serde::<kvm_irq_routing_entry>();
        is_serde::<kvm_irq_routing_msi>();
    }

    fn is_serde_json<T: Serialize + for<'de> Deserialize<'de> + Default>() {
        let serialized = serde_json::to_string(&T::default()).unwrap();
        let deserialized = serde_json::from_str::<T>(serialized.as_ref()).unwrap();
        let serialized_again = serde_json::to_string(&deserialized).unwrap();
        // Compare the serialized state after a roundtrip, to work around issues with
        // bindings not implementing `PartialEq`.
        assert_eq!(serialized, serialized_again);
    }

    #[test]
    fn test_json_serde() {
        is_serde_json::<kvm_regs>();
        is_serde_json::<kvm_sregs>();
        is_serde_json::<kvm_fpu>();
        is_serde_json::<kvm_debug_exit_arch>();
        is_serde_json::<kvm_guest_debug_arch>();
        is_serde_json::<kvm_sync_regs>();
        is_serde_json::<kvm_mp_state>();
        is_serde_json::<kvm_one_reg>();
        is_serde_json::<kvm_create_spapr_tce>();
        is_serde_json::<kvm_create_spapr_tce_64>();
        is_serde_json::<kvm_allocate_rma>();
        is_serde_json::<kvm_rtas_token_args>();
        is_serde_json::<kvm_book3e_206_tlb_entry>();
        is_serde_json::<kvm_book3e_206_tlb_params>();
        is_serde_json::<kvm_get_htab_fd>();
        is_serde_json::<kvm_get_htab_header>();
        is_serde_json::<kvm_ppc_mmuv3_cfg>();
        is_serde_json::<kvm_ppc_rmmu_info>();
        is_serde_json::<kvm_ppc_rmmu_info_kvm_ppc_radix_geom>();
        is_serde_json::<kvm_ppc_cpu_char>();
        is_serde_json::<kvm_ppc_xive_eq>();
        is_serde_json::<kvm_ppc_pvinfo>();
        is_serde_json::<kvm_ppc_one_page_size>();
        is_serde_json::<kvm_ppc_one_seg_page_size>();
        is_serde_json::<kvm_ppc_smmu_info>();
        is_serde_json::<kvm_ppc_resize_hpt>();
        is_serde_json::<kvm_irq_routing>();
        is_serde_json::<kvm_irq_routing_entry>();
        is_serde_json::<kvm_irq_routing_msi>();
    }
}

