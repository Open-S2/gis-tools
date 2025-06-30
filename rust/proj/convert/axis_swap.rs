use crate::proj::{CoordinateStep, Proj, TransformCoordinates};
use alloc::rc::Rc;
use core::cell::RefCell;

/// An axis swapping guide
#[derive(Debug, Clone, Copy)]
pub struct AxisSwap {
    /// The axis guide itself. A 4-tuple of axis directions to swap
    pub axis: [i64; 4],
    /// The sign guide for faster comparisons
    pub sign: [i32; 4],
}

// static int sign(int x) { return (x > 0) - (x < 0); }
// fn sign(x: i32) -> i32 {
//     (x > 0) as i32 - (x < 0) as i32
// }

/// Swap x and y
pub fn swap_xy_4d<P: TransformCoordinates>(coo: &mut P) {
    let tmp = coo.x();
    coo.set_x(coo.y());
    coo.set_y(tmp);
}

// // static void pj_axisswap_forward_4d(PJ_COORD &coo, PJ *P) {
// fn axisswap_forward_4d<P: TransformCoordinates>(proj: &Proj, coords: &mut P) {
//     // struct pj_axisswap_data *Q = (struct pj_axisswap_data *)P->opaque;
//     // unsigned int i;
//     // PJ_COORD out;
//     let out = Coords::default();
//     let swap = &proj.axis_swap;
//     let mut i;

//     for (i = 0; i < 4; i++) {
//         out.v[i] = coo.v[Q->axis[i]] * Q->sign[i];
//     }
//     coo = out;
// }

/// An axis swapping converter
#[derive(Debug, Clone, PartialEq)]
pub struct AxisSwapConverter {
    proj: Rc<RefCell<Proj>>,
}
impl CoordinateStep for AxisSwapConverter {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        // proj.borrow_mut().left = IoUnits::RADIANS;
        // proj.borrow_mut().right = IoUnits::RADIANS;
        // proj.is_ll = true;
        AxisSwapConverter { proj }
    }
    /// Handle the axis swap
    fn forward<P: TransformCoordinates>(&self, _coords: &mut P) {
        unimplemented!()
    }
    /// Handle the axis swap
    fn inverse<P: TransformCoordinates>(&self, _coords: &mut P) {
        unimplemented!()
    }
}

// static void pj_axisswap_reverse_4d(PJ_COORD &coo, PJ *P) {
//     struct pj_axisswap_data *Q = (struct pj_axisswap_data *)P->opaque;
//     unsigned int i;
//     PJ_COORD out;

//     for (i = 0; i < 4; i++)
//         out.v[Q->axis[i]] = coo.v[i] * Q->sign[i];

//     coo = out;
// }

// /***********************************************************************/
// PJ *PJ_CONVERSION(axisswap, 0) {
//     /***********************************************************************/
//     struct pj_axisswap_data *Q = static_cast<struct pj_axisswap_data *>(
//         calloc(1, sizeof(struct pj_axisswap_data)));
//     char *s;
//     unsigned int i, j, n = 0;

//     if (nullptr == Q)
//         return pj_default_destructor(P, PROJ_ERR_OTHER /*ENOMEM*/);
//     P->opaque = (void *)Q;

//     /* +order and +axis are mutually exclusive */
//     if (!pj_param_exists(P->params, "order") ==
//         !pj_param_exists(P->params, "axis")) {
//         proj_log_error(P,
//                        _("must provide EITHER 'order' OR 'axis' parameter."));
//         return pj_default_destructor(
//             P, PROJ_ERR_INVALID_OP_MUTUALLY_EXCLUSIVE_ARGS);
//     }

//     /* fill axis list with indices from 4-7 to simplify duplicate search further
//      * down */
//     for (i = 0; i < 4; i++) {
//         Q->axis[i] = i + 4;
//         Q->sign[i] = 1;
//     }

//     /* if the "order" parameter is used */
//     if (pj_param_exists(P->params, "order")) {
//         /* read axis order */
//         char *order = pj_param(P->ctx, P->params, "sorder").s;

//         /* check that all characters are valid */
//         for (i = 0; i < strlen(order); i++)
//             if (strchr("1234-,", order[i]) == nullptr) {
//                 proj_log_error(P, _("unknown axis '%c'"), order[i]);
//                 return pj_default_destructor(
//                     P, PROJ_ERR_INVALID_OP_ILLEGAL_ARG_VALUE);
//             }

//         /* read axes numbers and signs */
//         s = order;
//         n = 0;
//         while (*s != '\0' && n < 4) {
//             Q->axis[n] = abs(atoi(s)) - 1;
//             if (Q->axis[n] > 3) {
//                 proj_log_error(P, _("invalid axis '%d'"), Q->axis[n]);
//                 return pj_default_destructor(
//                     P, PROJ_ERR_INVALID_OP_ILLEGAL_ARG_VALUE);
//             }
//             Q->sign[n++] = sign(atoi(s));
//             while (*s != '\0' && *s != ',')
//                 s++;
//             if (*s == ',')
//                 s++;
//         }
//     }

//     /* if the "axis" parameter is used */
//     if (pj_param_exists(P->params, "axis")) {
//         /* parse the classic PROJ.4 enu axis specification */
//         for (i = 0; i < 3; i++) {
//             switch (P->axis[i]) {
//             case 'w':
//                 Q->sign[i] = -1;
//                 Q->axis[i] = 0;
//                 break;
//             case 'e':
//                 Q->sign[i] = 1;
//                 Q->axis[i] = 0;
//                 break;
//             case 's':
//                 Q->sign[i] = -1;
//                 Q->axis[i] = 1;
//                 break;
//             case 'n':
//                 Q->sign[i] = 1;
//                 Q->axis[i] = 1;
//                 break;
//             case 'd':
//                 Q->sign[i] = -1;
//                 Q->axis[i] = 2;
//                 break;
//             case 'u':
//                 Q->sign[i] = 1;
//                 Q->axis[i] = 2;
//                 break;
//             default:
//                 proj_log_error(P, _("unknown axis '%c'"), P->axis[i]);
//                 return pj_default_destructor(
//                     P, PROJ_ERR_INVALID_OP_ILLEGAL_ARG_VALUE);
//             }
//         }
//         n = 3;
//     }

//     /* check for duplicate axes */
//     for (i = 0; i < 4; i++)
//         for (j = 0; j < 4; j++) {
//             if (i == j)
//                 continue;
//             if (Q->axis[i] == Q->axis[j]) {
//                 proj_log_error(P, _("axisswap: duplicate axes specified"));
//                 return pj_default_destructor(
//                     P, PROJ_ERR_INVALID_OP_ILLEGAL_ARG_VALUE);
//             }
//         }

//     /* only map fwd/inv functions that are possible with the given axis setup */
//     if (n == 4) {
//         P->fwd4d = pj_axisswap_forward_4d;
//         P->inv4d = pj_axisswap_reverse_4d;
//     }
//     if (n == 3 && Q->axis[0] < 3 && Q->axis[1] < 3 && Q->axis[2] < 3) {
//         P->fwd3d = pj_axisswap_forward_3d;
//         P->inv3d = pj_axisswap_reverse_3d;
//     }
//     if (n == 2) {
//         if (Q->axis[0] == 1 && Q->sign[0] == 1 && Q->axis[1] == 0 &&
//             Q->sign[1] == 1) {
//             P->fwd4d = swap_xy_4d;
//             P->inv4d = swap_xy_4d;
//         } else if (Q->axis[0] < 2 && Q->axis[1] < 2) {
//             P->fwd = pj_axisswap_forward_2d;
//             P->inv = pj_axisswap_reverse_2d;
//         }
//     }

//     if (P->fwd4d == nullptr && P->fwd3d == nullptr && P->fwd == nullptr) {
//         proj_log_error(P, _("axisswap: bad axis order"));
//         return pj_default_destructor(P, PROJ_ERR_INVALID_OP_ILLEGAL_ARG_VALUE);
//     }

//     if (pj_param(P->ctx, P->params, "tangularunits").i) {
//         P->left = PJ_IO_UNITS_RADIANS;
//         P->right = PJ_IO_UNITS_RADIANS;
//     } else {
//         P->left = PJ_IO_UNITS_WHATEVER;
//         P->right = PJ_IO_UNITS_WHATEVER;
//     }

//     /* Preparation and finalization steps are skipped, since the reason   */
//     /* d'etre of axisswap is to bring input coordinates in line with the  */
//     /* the internally expected order (ENU), such that handling of offsets */
//     /* etc. can be done correctly in a later step of a pipeline */
//     P->skip_fwd_prepare = 1;
//     P->skip_fwd_finalize = 1;
//     P->skip_inv_prepare = 1;
//     P->skip_inv_finalize = 1;

//     return P;
// }
