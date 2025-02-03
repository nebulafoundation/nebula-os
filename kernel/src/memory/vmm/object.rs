use core::ptr::NonNull;

use alloc::boxed::Box;
use bitflags::bitflags;
use mem::{paging::PageEntryFlags, VirtualAddress};

#[derive(Debug)]
pub(super) struct VmObject {
    pub(super) base: VirtualAddress,
    pub(super) length: usize,
    pub(super) flags: VmFlags,
    pub(super) next: Option<NonNull<VmObject>>,
    pub(super) prev: Option<NonNull<VmObject>>,
}

impl VmObject {
    /// Allocates new `VmObject` struct on the heap. Returns a non-null pointer to the object.
    ///
    /// # Safety
    /// The caller must ensure that the new allocated vm object is valid.
    pub(super) unsafe fn alloc_new(
        base: VirtualAddress,
        length: usize,
        flags: VmFlags,
        next: Option<NonNull<VmObject>>,
        prev: Option<NonNull<VmObject>>,
    ) -> NonNull<VmObject> {
        let new_object = Box::into_raw(Box::new(VmObject {
            base,
            length,
            flags,
            next,
            prev,
        }));
        NonNull::new_unchecked(new_object)
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct VmFlags: u8 {
        /// If set, the object can be written to
        const WRITE = 1 << 0;
        /// If set, the object can be executed
        const EXECUTABLE = 1 << 1;
        /// If set, the object can be accessed by the lowest privilege ring, otherwise, just the kernel
        const USER = 1 << 2;
        /// If set, the objects is mapped to MMIO and therefore does not need to request pages when allocated.
        const MMIO = 1 << 3;
        /// If set, the CPU does not cache any data in that memory region.
        const NO_CACHE = 1 << 4;
    }
}

impl From<VmFlags> for PageEntryFlags {
    fn from(value: VmFlags) -> Self {
        let mut flags = PageEntryFlags::PRESENT;

        if value.contains(VmFlags::WRITE) {
            flags |= PageEntryFlags::READ_WRITE;
        }
        if !value.contains(VmFlags::EXECUTABLE) {
            flags |= PageEntryFlags::EXECUTE_DISABLE;
        }
        if value.contains(VmFlags::USER) {
            flags |= PageEntryFlags::USER_SUPER;
        }
        if value.contains(VmFlags::NO_CACHE) {
            // todo: maybe add PAT configuration for strong ordering
            flags |= PageEntryFlags::CACHE_DISABLED;
        }
        flags
    }
}
