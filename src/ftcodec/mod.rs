use crate::ftcodec::snapshot::{Snapshot, ObjSnapshot, Snapshot_Field};

pub mod snapshot;
pub mod kline;
pub mod common;


pub fn build_num(n : (i64, i8)) -> i64 {
    n.0 << 8 | (n.1 as i64)
}


pub fn symbol_str(s : &common::Symbol) -> String {
    match s.market {
        0 => s.code.to_string() + ".SH",
        _ => s.code.to_string()
    }
}

pub fn create_obj_snapshot(n : usize) -> ObjSnapshot {

    let mut objs = ObjSnapshot::new();
    for _i in 0..n {
        let mut s = Snapshot::new();
        s.mut_fields().mut_value().resize(Snapshot_Field::TOTAL as usize + 1, 0);
        objs.mut_lists().push(s);
    }
    objs
}


