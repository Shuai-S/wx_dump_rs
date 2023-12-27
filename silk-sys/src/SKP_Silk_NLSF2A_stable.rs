#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::{SKP_Silk_LPC_inv_pred_gain::SKP_Silk_LPC_inverse_pred_gain, skp_silk_bwexpander::skp_silk_bwexpander, SKP_Silk_NLSF2A::SKP_Silk_NLSF2A};
#[no_mangle]
pub unsafe fn SKP_Silk_NLSF2A_stable(
    pAR_Q12: &mut [i16],
    pNLSF: &[i32],
    LPC_order: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut invGain_Q30: libc::c_int = 0;
    SKP_Silk_NLSF2A(pAR_Q12, pNLSF, LPC_order);
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        if !(SKP_Silk_LPC_inverse_pred_gain(
            &mut invGain_Q30,
            pAR_Q12,
            LPC_order,
        ) == 1 as libc::c_int)
        {
            break;
        }
        skp_silk_bwexpander(
            pAR_Q12,
            LPC_order as usize,
            65536 as libc::c_int
                - (10 as libc::c_int + i) as libc::c_short as libc::c_int
                    * i as libc::c_short as libc::c_int,
        );
        i += 1;
    }
    if i == 20 as libc::c_int {
        i = 0 as libc::c_int;
        while i < LPC_order {
            pAR_Q12[i as usize] = 0 as libc::c_int as libc::c_short;
            i += 1;
        }
    }
}
