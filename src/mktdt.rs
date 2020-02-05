use chrono::{Utc, DateTime, TimeZone, Timelike, NaiveTime};
use std::io::Result;
use encoding_rs::GB18030;
use std::str;

use crate::ftcodec;
use crate::ftcodec::snapshot::{Snapshot_Field, ObjSnapshot};
use std::collections::HashMap;
use crate::ftcodec::{symbol_str, create_obj_snapshot};
use std::ptr::{null_mut};


pub struct MktdtParser {
    file_time: DateTime<Utc>,

    cached : HashMap<String, (ftcodec::snapshot::ObjSnapshot, bool)>
}

impl Default for MktdtParser {
    fn default() -> Self {
        MktdtParser{
            file_time: Utc::now(),
            cached : HashMap::new(),
        }
    }
}

impl MktdtParser {

    pub fn parse_file(&mut self, data : &[u8]) -> Result<bool> {
        let splitter = data.split(|c| *c == b'\n');
        for line in splitter {
            self.parse_line(line);
        }
        Ok(true)
    }

    fn parse_line(&mut self, data: &[u8]) {
        const HEADER: &[u8] = b"HEADER";
        const MD001: &[u8] = b"MD001";
        const MD002: &[u8] = b"MD002";
        const MD003: &[u8] = b"MD003";
        const MD004: &[u8] = b"MD004";
        if data.len() < MD001.len() {
            return;
        }
        if data[0..HEADER.len()].eq(HEADER) {
            self.parse_header(data);
        }else if data[0..MD001.len()].eq(MD001) {
            self.parse_md001(data);
        }else if data[0..MD002.len()].eq(MD002) {
            self.parse_md002(data);
        }else if data[0..MD003.len()].eq(MD003) {
            self.parse_md003(data);
        }else if data[0..MD004.len()].eq(MD004) {
            self.parse_md004(data);
        }
    }

    fn parse_header(&mut self, data: &[u8]) {
        let splitter = data.split(|c| *c == b'|');
        for (sn, field) in splitter.enumerate() {
            if sn == 6 {
                if let Ok(dt) = self.parse_datetime(field) {
                    self.file_time = dt;
                }
            }
        }
    }


    fn set_field(&self, fid: usize, num: Result<(i64, i8)>, s: &mut ObjSnapshot ) -> bool {
        s.mut_lists().first_mut().map_or(false, |ss| -> bool {
            let fields = ss.mut_fields().mut_value();
            match num {
                Ok(n) => {
                    let new_val =  ftcodec::build_num(n);
                    let old_val = fields[fid];
                    fields[fid as usize] = new_val;
                    old_val != new_val
                }
                Err(err) => {
                    print!("{:?}", err);
                    false
                },
            }
        })
    }

    fn parse_md00(&mut self, data: &[u8], fields_map: &[usize], time_field: usize) -> (bool, &ObjSnapshot) {
        let mut obj_ptr: *mut ObjSnapshot = null_mut() ;
        let mut changed = false;

        let splitter = data.split(|c| *c == b'|');
        for (sn, field) in splitter.enumerate() {
            match sn {
                1 => {
                    let text = unsafe { str::from_utf8_unchecked(field) };
                    let mut symbol = ftcodec::common::Symbol::new();
                    symbol.set_market(0);
                    symbol.set_code(protobuf::Chars::from(text));
                    let symbol_id = symbol_str(&symbol);
                    obj_ptr = self.get_mut(symbol_id);
                    unsafe {
                        let s = &mut *obj_ptr;
                        s.set_symbol(symbol);
                    };
                },
                2 => {
                    let name = GB18030.decode(field).0;
                    unsafe {
                        let s = &mut *obj_ptr;
                        s.set_name(protobuf::Chars::from(name.to_string()));
                    };
                },
                n => {
                    if n == time_field {
                        match self.parse_time(field) {
                            Ok(dt) => {
                                let s = unsafe {&mut *obj_ptr };
                                s.mut_lists()
                                    .first_mut()
                                    .map(|ss| ss.mut_timestamp().set_value(dt.timestamp_millis()));
                            },
                            Err(err) => println!("Time {:?} {:?}", err, String::from_utf8_lossy(field))
                        }
                    }else if n >=3 && n < (fields_map.len()/2){
                        let i = (n - 3) * 2;
                        let fid = fields_map[i+1];
                        let num = self.parse_number(field);
                        let s = unsafe{&mut *obj_ptr};
                        if self.set_field(fid, num, s) {
                            changed = true;
                        }

                        /*
                        for i in (0..fields_map.len()).step_by(2) {
                            if n == fields_map[i] {
                                let fid = fields_map[i];
                                let num = self.parse_number(field);
                                let s = unsafe{&mut *obj_ptr};
                                if self.set_field(fid, num, s) {
                                    changed = true;
                                }
                            }
                        }
                        */

                    }
                },
            }
        }
        let obj = unsafe {&mut *obj_ptr};
        (changed, obj)
    }


    fn parse_md001(&mut self, data: &[u8]) -> (bool, &ObjSnapshot){
        const FIELDS_MAP :[usize; 16] =[
            3, Snapshot_Field::VOLUME as usize,
            4, Snapshot_Field::AMOUNT as usize,
            5, Snapshot_Field::PRE_CLOSE_PRICE as usize,
            6, Snapshot_Field::OPEN_PRICE as usize,
            7, Snapshot_Field::HIGH_PRICE as usize,
            8, Snapshot_Field::LOW_PRICE as usize,
            9, Snapshot_Field::PRICE as usize,
            10, Snapshot_Field::CLOSE_PRICE as usize,
        ];

        self.parse_md00(data, &FIELDS_MAP, 12)
    }

    fn parse_md002(&mut self, data: &[u8])-> (bool, &ObjSnapshot) {
        const FIELDS_MAP :[usize; 56] =[
            3, Snapshot_Field::VOLUME as usize,
            4, Snapshot_Field::AMOUNT as usize,
            5, Snapshot_Field::PRE_CLOSE_PRICE as usize,
            6, Snapshot_Field::OPEN_PRICE as usize,
            7, Snapshot_Field::HIGH_PRICE as usize,
            8, Snapshot_Field::LOW_PRICE as usize,
            9, Snapshot_Field::PRICE as usize,
            10, Snapshot_Field::CLOSE_PRICE as usize,
            11, Snapshot_Field::BID1_PRICE as usize,
            12, Snapshot_Field::BID1_VOLUME as usize,
            13, Snapshot_Field::ASK1_PRICE as usize,
            14, Snapshot_Field::ASK1_VOLUME as usize,
            15, Snapshot_Field::BID2_PRICE as usize,
            16, Snapshot_Field::BID2_VOLUME as usize,
            17, Snapshot_Field::ASK2_PRICE as usize,
            18, Snapshot_Field::ASK2_VOLUME as usize,
            19, Snapshot_Field::BID3_PRICE as usize,
            20, Snapshot_Field::BID3_VOLUME as usize,
            21, Snapshot_Field::ASK3_PRICE as usize,
            22, Snapshot_Field::ASK3_VOLUME as usize,
            23, Snapshot_Field::BID4_PRICE as usize,
            24, Snapshot_Field::BID4_VOLUME as usize,
            25, Snapshot_Field::ASK4_PRICE as usize,
            26, Snapshot_Field::ASK4_VOLUME as usize,
            27, Snapshot_Field::BID5_PRICE as usize,
            28, Snapshot_Field::BID5_VOLUME as usize,
            29, Snapshot_Field::ASK5_PRICE as usize,
            30, Snapshot_Field::ASK5_VOLUME as usize,
        ];
        self.parse_md00(data, &FIELDS_MAP, 32)
    }

    fn parse_md003(&mut self, data: &[u8])-> (bool, &ObjSnapshot) {
        self.parse_md002(data)
    }

    fn parse_md004(&mut self, data: &[u8])-> (bool, &ObjSnapshot) {
        const FIELDS_MAP :[usize; 60] =[
            3, Snapshot_Field::VOLUME as usize,
            4, Snapshot_Field::AMOUNT as usize,
            5, Snapshot_Field::PRE_CLOSE_PRICE as usize,
            6, Snapshot_Field::OPEN_PRICE as usize,
            7, Snapshot_Field::HIGH_PRICE as usize,
            8, Snapshot_Field::LOW_PRICE as usize,
            9, Snapshot_Field::PRICE as usize,
            10, Snapshot_Field::CLOSE_PRICE as usize,
            11, Snapshot_Field::BID1_PRICE as usize,
            12, Snapshot_Field::BID1_VOLUME as usize,
            13, Snapshot_Field::ASK1_PRICE as usize,
            14, Snapshot_Field::ASK1_VOLUME as usize,
            15, Snapshot_Field::BID2_PRICE as usize,
            16, Snapshot_Field::BID2_VOLUME as usize,
            17, Snapshot_Field::ASK2_PRICE as usize,
            18, Snapshot_Field::ASK2_VOLUME as usize,
            19, Snapshot_Field::BID3_PRICE as usize,
            20, Snapshot_Field::BID3_VOLUME as usize,
            21, Snapshot_Field::ASK3_PRICE as usize,
            22, Snapshot_Field::ASK3_VOLUME as usize,
            23, Snapshot_Field::BID4_PRICE as usize,
            24, Snapshot_Field::BID4_VOLUME as usize,
            25, Snapshot_Field::ASK4_PRICE as usize,
            26, Snapshot_Field::ASK4_VOLUME as usize,
            27, Snapshot_Field::BID5_PRICE as usize,
            28, Snapshot_Field::BID5_VOLUME as usize,
            29, Snapshot_Field::ASK5_PRICE as usize,
            30, Snapshot_Field::ASK5_VOLUME as usize,
            31, Snapshot_Field::PRE_IOPV as usize,
            32, Snapshot_Field::IOPV as usize,
        ];
        self.parse_md00(data, &FIELDS_MAP, 34)
    }

    fn parse_datetime(&self, data: &[u8]) -> Result<DateTime<Utc>>{
        const DATETIME_FORMAT : &str = "%Y%m%d-%H:%M:%S%.3f";
        let dt = unsafe {
            str::from_utf8_unchecked(data)
        };
        Utc.datetime_from_str(dt, DATETIME_FORMAT)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    }

    fn parse_time(&self, data: &[u8]) -> Result<DateTime<Utc>>{
        const TIME_FORMAT: &str = "%H:%M:%S%.3f";
        let dt = unsafe {
            str::from_utf8_unchecked(data)
        };
        NaiveTime::parse_from_str(dt, TIME_FORMAT)
            .map(|dt| self.file_time.date().and_hms_nano(dt.hour(), dt.minute(), dt.second(),dt.nanosecond()))
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    }

    fn parse_number(&self, data: &[u8]) -> Result<(i64,i8)> {
        let mut dec : i8 = -1;
        let mut val : i64 = 0;
        let mut minus = false;
        for b in data.iter() {
            match b {
                b'.' => dec = 0,
                b'-' => minus = true,
                b'0'..=b'9' => {
                    val = val * 10 + (b - b'0') as i64;
                    if dec >= 0 {
                        dec = dec + 1;
                    }
                },
                _ => (),
            }
        }
        if minus {
            val = -val;
        }
        if dec < 0 {
            dec = 0;
        }
        Ok((val, dec))
    }


    fn get_mut(&mut self, s: String) -> *mut ObjSnapshot  {
        let found = self.cached
            .entry(s)
            .or_insert_with(|| (create_obj_snapshot(1), true));
        &mut found.0 as *mut ObjSnapshot
    }
}