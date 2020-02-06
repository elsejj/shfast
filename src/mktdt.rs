use chrono::{Utc, DateTime, TimeZone, Timelike, NaiveTime};
use std::io::Result;
use encoding_rs::GB18030;
use std::str;

use crate::ftcodec;
use crate::ftcodec::snapshot::{Snapshot_Field, ObjSnapshot};
use std::collections::HashMap;
use crate::ftcodec::{symbol_str, create_obj_snapshot};



const FIELDS_MAP :[usize; 30] =[
    Snapshot_Field::VOLUME as usize,
    Snapshot_Field::AMOUNT as usize,
    Snapshot_Field::PRE_CLOSE_PRICE as usize,
    Snapshot_Field::OPEN_PRICE as usize,
    Snapshot_Field::HIGH_PRICE as usize,
    Snapshot_Field::LOW_PRICE as usize,
    Snapshot_Field::PRICE as usize,
    Snapshot_Field::CLOSE_PRICE as usize,
    Snapshot_Field::BID1_PRICE as usize,
    Snapshot_Field::BID1_VOLUME as usize,
    Snapshot_Field::ASK1_PRICE as usize,
    Snapshot_Field::ASK1_VOLUME as usize,
    Snapshot_Field::BID2_PRICE as usize,
    Snapshot_Field::BID2_VOLUME as usize,
    Snapshot_Field::ASK2_PRICE as usize,
    Snapshot_Field::ASK2_VOLUME as usize,
    Snapshot_Field::BID3_PRICE as usize,
    Snapshot_Field::BID3_VOLUME as usize,
    Snapshot_Field::ASK3_PRICE as usize,
    Snapshot_Field::ASK3_VOLUME as usize,
    Snapshot_Field::BID4_PRICE as usize,
    Snapshot_Field::BID4_VOLUME as usize,
    Snapshot_Field::ASK4_PRICE as usize,
    Snapshot_Field::ASK4_VOLUME as usize,
    Snapshot_Field::BID5_PRICE as usize,
    Snapshot_Field::BID5_VOLUME as usize,
    Snapshot_Field::ASK5_PRICE as usize,
    Snapshot_Field::ASK5_VOLUME as usize,
    Snapshot_Field::PRE_IOPV as usize,
    Snapshot_Field::IOPV as usize,
    ];

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
        if data.len() < 5 {
            return;
        }

        if data[0] == b'H'{
            self.parse_header(data);
        }else if data[0] == b'M' {
            match data[4] {
                b'1' => self.parse_md001(data),
                b'2' => self.parse_md002(data),
                b'3' => self.parse_md003(data),
                b'4' => self.parse_md004(data),
                _ => (),
            }
        }
    }

    fn parse_header(&mut self, data: &[u8]) {
        let splitter = data.split(|c| *c == b'|');
        for (sn, field) in splitter.enumerate() {
            if sn == 6 {
                if let Ok(dt) = MktdtParser::parse_datetime(field) {
                    self.file_time = dt;
                }
            }
        }
    }


    fn parse_md00(&mut self, data: &[u8], value_fields: &[usize], time_field: usize) {
        let now = self.file_time;
        let mut splitter = data.split(|c| *c == b'|').enumerate();
        // skip type
        splitter.next();
        // code
        let val = splitter.next().map(|(_,field)| {
            let text = unsafe { str::from_utf8_unchecked(field) };
            self.cached.entry(text.to_string()).or_insert_with(|| {
                let mut symbol = ftcodec::common::Symbol::new();
                symbol.set_market(0);
                symbol.set_code(protobuf::Chars::from(text));
                let mut new = create_obj_snapshot(1);
                new.set_symbol(symbol);
                (new, true)
            })
        }).unwrap();

        // name
        splitter.next().map(|(_, field)| {
            let name = GB18030.decode(field).0;
            val.0.set_name(protobuf::Chars::from(name.to_string()));
        });

        // value fields
        for i in 0..value_fields.len() {
            if i < Snapshot_Field::TOTAL as usize {
                let fid = value_fields[i];
                splitter.next().map(|(_, field)|{ let num = MktdtParser::parse_number(field);
                    MktdtParser::set_field(fid, num, &mut val.0);
                });
            }
        }


        // set time and flags
        for (sn, field) in splitter {
            if sn == time_field {
                let tt = MktdtParser::parse_time(&now, field);
                match tt {
                    Ok(dt) => {
                        val.0.mut_lists()
                            .first_mut()
                            .map(|ss| ss.mut_timestamp().set_value(dt.timestamp_millis()));
                    },
                    Err(err) => println!("Time {:?} {:?}", err, String::from_utf8_lossy(field))
                }
            }
        }
    }


    fn parse_md001(&mut self, data: &[u8]){
        self.parse_md00(data, &FIELDS_MAP[0..8], 12)
    }

    fn parse_md002(&mut self, data: &[u8]) {
        self.parse_md00(data, &FIELDS_MAP[0..28], 32)
    }

    fn parse_md003(&mut self, data: &[u8]) {
        self.parse_md002(data)
    }

    fn parse_md004(&mut self, data: &[u8]) {
        self.parse_md00(data, &FIELDS_MAP[0..30], 34)
    }

    fn set_field(fid: usize, num: Result<(i64, i8)>, s: &mut ObjSnapshot ) -> bool {
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

    fn parse_datetime(data: &[u8]) -> Result<DateTime<Utc>>{
        const DATETIME_FORMAT : &str = "%Y%m%d-%H:%M:%S%.3f";
        let dt = unsafe {
            str::from_utf8_unchecked(data)
        };
        Utc.datetime_from_str(dt, DATETIME_FORMAT)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    }

    fn parse_time(base: &DateTime<Utc>, data: &[u8]) -> Result<DateTime<Utc>>{
        const TIME_FORMAT: &str = "%H:%M:%S%.3f";
        let dt = unsafe {
            str::from_utf8_unchecked(data)
        };
        NaiveTime::parse_from_str(dt, TIME_FORMAT)
            .map(|dt| base.date().and_hms_nano(dt.hour(), dt.minute(), dt.second(),dt.nanosecond()))
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    }

    fn parse_number(data: &[u8]) -> Result<(i64,i8)> {
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
}