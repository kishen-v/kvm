// Copyright 2024 © Institute of Software, CAS. All rights reserved.
// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use vmm_sys_util::fam::{FamStruct, FamStructWrapper};
use vmm_sys_util::generate_fam_struct_impl;

use super::bindings::*;

// There is no constant in the kernel as far as the maximum number
// of registers on PPC64, but KVM_GET_REG_LIST typically returns a reasonable number.
// Setting a conservative upper bound.
const PPC64_REGS_MAX: usize = 500;

// Implement the FamStruct trait for kvm_reg_list.
generate_fam_struct_impl!(kvm_reg_list, u64, reg, u64, n, PPC64_REGS_MAX);

// Implement the PartialEq trait for kvm_reg_list.
impl PartialEq for kvm_reg_list {
    fn eq(&self, other: &kvm_reg_list) -> bool {
        // No need to call entries's eq, FamStructWrapper's PartialEq will do it for you
        self.n == other.n
    }
}

/// Wrapper over the `kvm_reg_list` structure.
///
/// The `kvm_reg_list` structure contains a flexible array member. For details check the
/// [KVM API KVM_GET_REG_LIST](https://docs.kernel.org/virt/kvm/api.html#kvm-get-reg-list)
/// documentation. To provide safe access to the array elements, this type is
/// implemented using [FamStructWrapper](../vmm_sys_util/fam/struct.FamStructWrapper.html).
pub type RegList = FamStructWrapper<kvm_reg_list>;

// Implement the FamStruct trait for kvm_irq_routing
generate_fam_struct_impl!(
    kvm_irq_routing,
    kvm_irq_routing_entry,
    entries,
    u32,
    nr,
    1024
);

// Implement the PartialEq trait for kvm_irq_routing.
impl PartialEq for kvm_irq_routing {
    fn eq(&self, other: &kvm_irq_routing) -> bool {
        // No need to call entries's eq, FamStructWrapper's PartialEq will do it for you
        self.nr == other.nr && self.flags == other.flags
    }
}

/// Wrapper over the `kvm_irq_routing` structure.
///
/// The `kvm_irq_routing` structure contains a flexible array member. For details check the
/// [KVM API](https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt)
/// documentation on `kvm_irq_routing`. To provide safe access to
/// the array elements, this type is implemented using
/// [FamStructWrapper](../vmm_sys_util/fam/struct.FamStructWrapper.html).
pub type IrqRouting = FamStructWrapper<kvm_irq_routing>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg_list() {
        let mut reg_list = RegList::new(0).unwrap();
        assert_eq!(reg_list.as_slice().len(), 0);
        assert_eq!(reg_list.len(), 0);

        let mut reg_list = RegList::new(10).unwrap();
        assert_eq!(reg_list.as_slice().len(), 10);
        assert_eq!(reg_list.len(), 10);

        let regs = reg_list.as_mut_slice();
        for (i, reg) in regs.iter_mut().enumerate() {
            *reg = i as u64;
        }

        for (i, reg) in reg_list.as_slice().iter().enumerate() {
            assert_eq!(*reg, i as u64);
        }
    }

    #[test]
    fn test_irq_routing() {
        let irq_routing = IrqRouting::new(0).unwrap();
        assert_eq!(irq_routing.as_slice().len(), 0);
        assert_eq!(irq_routing.len(), 0);

        let irq_routing = IrqRouting::new(10).unwrap();
        assert_eq!(irq_routing.as_slice().len(), 10);
        assert_eq!(irq_routing.len(), 10);
    }

    #[test]
    fn test_reg_list_partial_eq() {
        let reg_list1 = RegList::new(5).unwrap();
        let reg_list2 = RegList::new(5).unwrap();
        assert_eq!(reg_list1, reg_list2);

        let reg_list3 = RegList::new(10).unwrap();
        assert_ne!(reg_list1, reg_list3);
    }

    #[test]
    fn test_irq_routing_partial_eq() {
        let irq_routing1 = IrqRouting::new(5).unwrap();
        let irq_routing2 = IrqRouting::new(5).unwrap();
        assert_eq!(irq_routing1, irq_routing2);

        let irq_routing3 = IrqRouting::new(10).unwrap();
        assert_ne!(irq_routing1, irq_routing3);
    }
}

