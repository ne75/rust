use crate::abi::call::{ArgAbi, FnAbi};

// Match LLVM. 
// for 32 or 64 bit non-aggregate types, we can pass via registers. less than 32 bits should be extended to 32 bits
// for aggegrate types, pass them indirectly.
// for non-aggregate types > 64 bits, pass them indirectly
fn classify_ret<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.layout.is_aggregate() || ret.layout.size.bits() > 64 {
        ret.make_indirect();
    }
}

fn classify_arg<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    if arg.layout.is_aggregate() || arg.layout.size.bits() > 64 {
        arg.make_indirect();
    }
}

pub fn compute_abi_info<Ty>(fn_abi: &mut FnAbi<'_, Ty>) {
    if !fn_abi.ret.is_ignore() {
        classify_ret(&mut fn_abi.ret);
    }

    for arg in &mut fn_abi.args {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(arg);
    }
}