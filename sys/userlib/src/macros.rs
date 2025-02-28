// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use bstringify;
pub use paste;

#[cfg(feature = "log-itm")]
#[macro_export]
macro_rules! sys_log {
    ($s:expr) => {
        unsafe {
            let stim = &mut (*cortex_m::peripheral::ITM::ptr()).stim[1];
            cortex_m::iprintln!(stim, $s);
        }
    };
    ($s:expr, $($tt:tt)*) => {
        unsafe {
            let stim = &mut (*cortex_m::peripheral::ITM::ptr()).stim[1];
            cortex_m::iprintln!(stim, $s, $($tt)*);
        }
    };
}

#[cfg(feature = "log-semihosting")]
#[macro_export]
macro_rules! sys_log {
    ($s:expr) => {
        { let _ = cortex_m_semihosting::hprintln!($s); }
    };
    ($s:expr, $($tt:tt)*) => {
        { let _ = cortex_m_semihosting::hprintln!($s, $($tt)*); }
    };
}

#[cfg(not(any(feature = "log-semihosting", feature = "log-itm")))]
#[macro_export]
macro_rules! sys_log {
    ($s:expr) => {
        compile_error!(concat!(
            "to use sys_log! must enable either ",
            "'log-semihosting' or 'log-itm' feature"
        ))
    };
    ($s:expr, $($tt:tt)*) => {
        compile_error!(concat!(
            "to use sys_log! must enable either ",
            "'log-semihosting' or 'log-itm' feature"
        ))
    };
}

#[macro_export]
macro_rules! task_slot {
    ($var:ident, $task_name:ident) => {
        $crate::macros::paste::paste! {
            #[used]
            static $var: $crate::task_slot::TaskSlot =
                $crate::task_slot::TaskSlot::UNBOUND;

            #[used]
            #[link_section = ".task_slot_table"]
            static [< _TASK_SLOT_TABLE_ $var >]: $crate::task_slot::TaskSlotTableEntry<
                { $crate::macros::bstringify::bstringify!($task_name).len() },
            > = $crate::task_slot::TaskSlotTableEntry::for_task_slot(
                $crate::macros::bstringify::bstringify!($task_name),
                &$var,
            );
        }
    };
}
