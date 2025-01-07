use crate::constant::INVALID;
// 双线性插值
pub fn bilinear(
    lt_val: f64,
    rt_val: f64,
    lb_val: f64,
    rb_val: f64,
    left_scale_factor: f64,
    top_scale_factor: f64
) -> f64 {
    let lr_liner_val_top: f64;
    let lr_liner_val_bottom: f64;

    if lt_val == INVALID {
        if rt_val == INVALID {
            lr_liner_val_top = INVALID;
        } else {
            lr_liner_val_top = rt_val as f64;
        }
    } else {
        if rt_val == INVALID {
            lr_liner_val_top = lt_val as f64;
        } else {
            // 插值
            lr_liner_val_top =
                (1.0 - left_scale_factor) * (lt_val as f64) + left_scale_factor * (rt_val as f64);
        }
    }

    if lb_val == INVALID {
        if rb_val == INVALID {
            lr_liner_val_bottom = INVALID;
        } else {
            lr_liner_val_bottom = rb_val as f64;
        }
    } else {
        if rb_val == INVALID {
            lr_liner_val_bottom = lb_val as f64;
        } else {
            // 插值
            lr_liner_val_bottom =
                (1.0 - left_scale_factor) * (lb_val as f64) + left_scale_factor * (rb_val as f64);
        }
    }

    if lr_liner_val_top == INVALID {
        if lr_liner_val_bottom == INVALID {
            INVALID
        } else {
            lr_liner_val_bottom
        }
    } else {
        if lr_liner_val_bottom == INVALID {
            lr_liner_val_top
        } else {
            (1.0 - top_scale_factor) * lr_liner_val_top + top_scale_factor * lr_liner_val_bottom
        }
    }
}