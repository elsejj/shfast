// This file is generated by rust-protobuf 2.10.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `snapshot.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct Snapshot {
    // message fields
    pub timestamp: ::protobuf::SingularPtrField<super::common::DateTime>,
    pub fields: ::protobuf::SingularPtrField<super::common::Fields>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Snapshot {
    fn default() -> &'a Snapshot {
        <Snapshot as ::protobuf::Message>::default_instance()
    }
}

impl Snapshot {
    pub fn new() -> Snapshot {
        ::std::default::Default::default()
    }

    // .ftcodec.DateTime timestamp = 1;


    pub fn get_timestamp(&self) -> &super::common::DateTime {
        self.timestamp.as_ref().unwrap_or_else(|| super::common::DateTime::default_instance())
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: super::common::DateTime) {
        self.timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timestamp(&mut self) -> &mut super::common::DateTime {
        if self.timestamp.is_none() {
            self.timestamp.set_default();
        }
        self.timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_timestamp(&mut self) -> super::common::DateTime {
        self.timestamp.take().unwrap_or_else(|| super::common::DateTime::new())
    }

    // .ftcodec.Fields fields = 2;


    pub fn get_fields(&self) -> &super::common::Fields {
        self.fields.as_ref().unwrap_or_else(|| super::common::Fields::default_instance())
    }
    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    pub fn has_fields(&self) -> bool {
        self.fields.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: super::common::Fields) {
        self.fields = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fields(&mut self) -> &mut super::common::Fields {
        if self.fields.is_none() {
            self.fields.set_default();
        }
        self.fields.as_mut().unwrap()
    }

    // Take field
    pub fn take_fields(&mut self) -> super::common::Fields {
        self.fields.take().unwrap_or_else(|| super::common::Fields::new())
    }
}

impl ::protobuf::Message for Snapshot {
    fn is_initialized(&self) -> bool {
        for v in &self.timestamp {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fields {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timestamp)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fields)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.fields.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.timestamp.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.fields.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Snapshot {
        Snapshot::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::DateTime>>(
                    "timestamp",
                    |m: &Snapshot| { &m.timestamp },
                    |m: &mut Snapshot| { &mut m.timestamp },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Fields>>(
                    "fields",
                    |m: &Snapshot| { &m.fields },
                    |m: &mut Snapshot| { &mut m.fields },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Snapshot>(
                    "Snapshot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Snapshot {
        static mut instance: ::protobuf::lazy::Lazy<Snapshot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snapshot,
        };
        unsafe {
            instance.get(Snapshot::new)
        }
    }
}

impl ::protobuf::Clear for Snapshot {
    fn clear(&mut self) {
        self.timestamp.clear();
        self.fields.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Snapshot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Snapshot_Field {
    PRICE = 0,
    VOLUME = 1,
    AMOUNT = 2,
    PRE_CLOSE_PRICE = 3,
    OPEN_PRICE = 4,
    HIGH_PRICE = 5,
    LOW_PRICE = 6,
    CLOSE_PRICE = 7,
    IOPV = 8,
    PRE_IOPV = 9,
    SETTLEMENT = 10,
    PRE_SETTLEMENT = 11,
    BID1_PRICE = 12,
    BID1_VOLUME = 13,
    ASK1_PRICE = 14,
    ASK1_VOLUME = 15,
    BID2_PRICE = 16,
    BID2_VOLUME = 17,
    ASK2_PRICE = 18,
    ASK2_VOLUME = 19,
    BID3_PRICE = 20,
    BID3_VOLUME = 21,
    ASK3_PRICE = 22,
    ASK3_VOLUME = 23,
    BID4_PRICE = 24,
    BID4_VOLUME = 25,
    ASK4_PRICE = 26,
    ASK4_VOLUME = 27,
    BID5_PRICE = 28,
    BID5_VOLUME = 29,
    ASK5_PRICE = 30,
    ASK5_VOLUME = 31,
    TOTAL = 32,
}

impl ::protobuf::ProtobufEnum for Snapshot_Field {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Snapshot_Field> {
        match value {
            0 => ::std::option::Option::Some(Snapshot_Field::PRICE),
            1 => ::std::option::Option::Some(Snapshot_Field::VOLUME),
            2 => ::std::option::Option::Some(Snapshot_Field::AMOUNT),
            3 => ::std::option::Option::Some(Snapshot_Field::PRE_CLOSE_PRICE),
            4 => ::std::option::Option::Some(Snapshot_Field::OPEN_PRICE),
            5 => ::std::option::Option::Some(Snapshot_Field::HIGH_PRICE),
            6 => ::std::option::Option::Some(Snapshot_Field::LOW_PRICE),
            7 => ::std::option::Option::Some(Snapshot_Field::CLOSE_PRICE),
            8 => ::std::option::Option::Some(Snapshot_Field::IOPV),
            9 => ::std::option::Option::Some(Snapshot_Field::PRE_IOPV),
            10 => ::std::option::Option::Some(Snapshot_Field::SETTLEMENT),
            11 => ::std::option::Option::Some(Snapshot_Field::PRE_SETTLEMENT),
            12 => ::std::option::Option::Some(Snapshot_Field::BID1_PRICE),
            13 => ::std::option::Option::Some(Snapshot_Field::BID1_VOLUME),
            14 => ::std::option::Option::Some(Snapshot_Field::ASK1_PRICE),
            15 => ::std::option::Option::Some(Snapshot_Field::ASK1_VOLUME),
            16 => ::std::option::Option::Some(Snapshot_Field::BID2_PRICE),
            17 => ::std::option::Option::Some(Snapshot_Field::BID2_VOLUME),
            18 => ::std::option::Option::Some(Snapshot_Field::ASK2_PRICE),
            19 => ::std::option::Option::Some(Snapshot_Field::ASK2_VOLUME),
            20 => ::std::option::Option::Some(Snapshot_Field::BID3_PRICE),
            21 => ::std::option::Option::Some(Snapshot_Field::BID3_VOLUME),
            22 => ::std::option::Option::Some(Snapshot_Field::ASK3_PRICE),
            23 => ::std::option::Option::Some(Snapshot_Field::ASK3_VOLUME),
            24 => ::std::option::Option::Some(Snapshot_Field::BID4_PRICE),
            25 => ::std::option::Option::Some(Snapshot_Field::BID4_VOLUME),
            26 => ::std::option::Option::Some(Snapshot_Field::ASK4_PRICE),
            27 => ::std::option::Option::Some(Snapshot_Field::ASK4_VOLUME),
            28 => ::std::option::Option::Some(Snapshot_Field::BID5_PRICE),
            29 => ::std::option::Option::Some(Snapshot_Field::BID5_VOLUME),
            30 => ::std::option::Option::Some(Snapshot_Field::ASK5_PRICE),
            31 => ::std::option::Option::Some(Snapshot_Field::ASK5_VOLUME),
            32 => ::std::option::Option::Some(Snapshot_Field::TOTAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Snapshot_Field] = &[
            Snapshot_Field::PRICE,
            Snapshot_Field::VOLUME,
            Snapshot_Field::AMOUNT,
            Snapshot_Field::PRE_CLOSE_PRICE,
            Snapshot_Field::OPEN_PRICE,
            Snapshot_Field::HIGH_PRICE,
            Snapshot_Field::LOW_PRICE,
            Snapshot_Field::CLOSE_PRICE,
            Snapshot_Field::IOPV,
            Snapshot_Field::PRE_IOPV,
            Snapshot_Field::SETTLEMENT,
            Snapshot_Field::PRE_SETTLEMENT,
            Snapshot_Field::BID1_PRICE,
            Snapshot_Field::BID1_VOLUME,
            Snapshot_Field::ASK1_PRICE,
            Snapshot_Field::ASK1_VOLUME,
            Snapshot_Field::BID2_PRICE,
            Snapshot_Field::BID2_VOLUME,
            Snapshot_Field::ASK2_PRICE,
            Snapshot_Field::ASK2_VOLUME,
            Snapshot_Field::BID3_PRICE,
            Snapshot_Field::BID3_VOLUME,
            Snapshot_Field::ASK3_PRICE,
            Snapshot_Field::ASK3_VOLUME,
            Snapshot_Field::BID4_PRICE,
            Snapshot_Field::BID4_VOLUME,
            Snapshot_Field::ASK4_PRICE,
            Snapshot_Field::ASK4_VOLUME,
            Snapshot_Field::BID5_PRICE,
            Snapshot_Field::BID5_VOLUME,
            Snapshot_Field::ASK5_PRICE,
            Snapshot_Field::ASK5_VOLUME,
            Snapshot_Field::TOTAL,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Snapshot_Field", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Snapshot_Field {
}

impl ::std::default::Default for Snapshot_Field {
    fn default() -> Self {
        Snapshot_Field::PRICE
    }
}

impl ::protobuf::reflect::ProtobufValue for Snapshot_Field {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ObjSnapshot {
    // message fields
    pub symbol: ::protobuf::SingularPtrField<super::common::Symbol>,
    pub name: ::protobuf::Chars,
    pub lists: ::protobuf::RepeatedField<Snapshot>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ObjSnapshot {
    fn default() -> &'a ObjSnapshot {
        <ObjSnapshot as ::protobuf::Message>::default_instance()
    }
}

impl ObjSnapshot {
    pub fn new() -> ObjSnapshot {
        ::std::default::Default::default()
    }

    // .ftcodec.Symbol symbol = 1;


    pub fn get_symbol(&self) -> &super::common::Symbol {
        self.symbol.as_ref().unwrap_or_else(|| super::common::Symbol::default_instance())
    }
    pub fn clear_symbol(&mut self) {
        self.symbol.clear();
    }

    pub fn has_symbol(&self) -> bool {
        self.symbol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_symbol(&mut self, v: super::common::Symbol) {
        self.symbol = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symbol(&mut self) -> &mut super::common::Symbol {
        if self.symbol.is_none() {
            self.symbol.set_default();
        }
        self.symbol.as_mut().unwrap()
    }

    // Take field
    pub fn take_symbol(&mut self) -> super::common::Symbol {
        self.symbol.take().unwrap_or_else(|| super::common::Symbol::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        ::protobuf::Clear::clear(&mut self.name);
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::protobuf::Chars) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::protobuf::Chars {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::protobuf::Chars {
        ::std::mem::replace(&mut self.name, ::protobuf::Chars::new())
    }

    // repeated .ftcodec.Snapshot lists = 3;


    pub fn get_lists(&self) -> &[Snapshot] {
        &self.lists
    }
    pub fn clear_lists(&mut self) {
        self.lists.clear();
    }

    // Param is passed by value, moved
    pub fn set_lists(&mut self, v: ::protobuf::RepeatedField<Snapshot>) {
        self.lists = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lists(&mut self) -> &mut ::protobuf::RepeatedField<Snapshot> {
        &mut self.lists
    }

    // Take field
    pub fn take_lists(&mut self) -> ::protobuf::RepeatedField<Snapshot> {
        ::std::mem::replace(&mut self.lists, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ObjSnapshot {
    fn is_initialized(&self) -> bool {
        for v in &self.symbol {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lists {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.symbol)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lists)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.symbol.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        for value in &self.lists {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.symbol.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        for v in &self.lists {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ObjSnapshot {
        ObjSnapshot::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Symbol>>(
                    "symbol",
                    |m: &ObjSnapshot| { &m.symbol },
                    |m: &mut ObjSnapshot| { &mut m.symbol },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "name",
                    |m: &ObjSnapshot| { &m.name },
                    |m: &mut ObjSnapshot| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Snapshot>>(
                    "lists",
                    |m: &ObjSnapshot| { &m.lists },
                    |m: &mut ObjSnapshot| { &mut m.lists },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjSnapshot>(
                    "ObjSnapshot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ObjSnapshot {
        static mut instance: ::protobuf::lazy::Lazy<ObjSnapshot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjSnapshot,
        };
        unsafe {
            instance.get(ObjSnapshot::new)
        }
    }
}

impl ::protobuf::Clear for ObjSnapshot {
    fn clear(&mut self) {
        self.symbol.clear();
        ::protobuf::Clear::clear(&mut self.name);
        self.lists.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObjSnapshot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObjSnapshot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ObjSnapshotList {
    // message fields
    pub lists: ::protobuf::RepeatedField<ObjSnapshot>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ObjSnapshotList {
    fn default() -> &'a ObjSnapshotList {
        <ObjSnapshotList as ::protobuf::Message>::default_instance()
    }
}

impl ObjSnapshotList {
    pub fn new() -> ObjSnapshotList {
        ::std::default::Default::default()
    }

    // repeated .ftcodec.ObjSnapshot lists = 1;


    pub fn get_lists(&self) -> &[ObjSnapshot] {
        &self.lists
    }
    pub fn clear_lists(&mut self) {
        self.lists.clear();
    }

    // Param is passed by value, moved
    pub fn set_lists(&mut self, v: ::protobuf::RepeatedField<ObjSnapshot>) {
        self.lists = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lists(&mut self) -> &mut ::protobuf::RepeatedField<ObjSnapshot> {
        &mut self.lists
    }

    // Take field
    pub fn take_lists(&mut self) -> ::protobuf::RepeatedField<ObjSnapshot> {
        ::std::mem::replace(&mut self.lists, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ObjSnapshotList {
    fn is_initialized(&self) -> bool {
        for v in &self.lists {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lists)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.lists {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.lists {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ObjSnapshotList {
        ObjSnapshotList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ObjSnapshot>>(
                    "lists",
                    |m: &ObjSnapshotList| { &m.lists },
                    |m: &mut ObjSnapshotList| { &mut m.lists },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjSnapshotList>(
                    "ObjSnapshotList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ObjSnapshotList {
        static mut instance: ::protobuf::lazy::Lazy<ObjSnapshotList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjSnapshotList,
        };
        unsafe {
            instance.get(ObjSnapshotList::new)
        }
    }
}

impl ::protobuf::Clear for ObjSnapshotList {
    fn clear(&mut self) {
        self.lists.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObjSnapshotList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObjSnapshotList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0esnapshot.proto\x12\x07ftcodec\x1a\x0ccommon.proto\"\xf7\x04\n\x08S\
    napshot\x12/\n\ttimestamp\x18\x01\x20\x01(\x0b2\x11.ftcodec.DateTimeR\tt\
    imestamp\x12'\n\x06fields\x18\x02\x20\x01(\x0b2\x0f.ftcodec.FieldsR\x06f\
    ields\"\x90\x04\n\x05Field\x12\t\n\x05PRICE\x10\0\x12\n\n\x06VOLUME\x10\
    \x01\x12\n\n\x06AMOUNT\x10\x02\x12\x13\n\x0fPRE_CLOSE_PRICE\x10\x03\x12\
    \x0e\n\nOPEN_PRICE\x10\x04\x12\x0e\n\nHIGH_PRICE\x10\x05\x12\r\n\tLOW_PR\
    ICE\x10\x06\x12\x0f\n\x0bCLOSE_PRICE\x10\x07\x12\x08\n\x04IOPV\x10\x08\
    \x12\x0c\n\x08PRE_IOPV\x10\t\x12\x0e\n\nSETTLEMENT\x10\n\x12\x12\n\x0ePR\
    E_SETTLEMENT\x10\x0b\x12\x0e\n\nBID1_PRICE\x10\x0c\x12\x0f\n\x0bBID1_VOL\
    UME\x10\r\x12\x0e\n\nASK1_PRICE\x10\x0e\x12\x0f\n\x0bASK1_VOLUME\x10\x0f\
    \x12\x0e\n\nBID2_PRICE\x10\x10\x12\x0f\n\x0bBID2_VOLUME\x10\x11\x12\x0e\
    \n\nASK2_PRICE\x10\x12\x12\x0f\n\x0bASK2_VOLUME\x10\x13\x12\x0e\n\nBID3_\
    PRICE\x10\x14\x12\x0f\n\x0bBID3_VOLUME\x10\x15\x12\x0e\n\nASK3_PRICE\x10\
    \x16\x12\x0f\n\x0bASK3_VOLUME\x10\x17\x12\x0e\n\nBID4_PRICE\x10\x18\x12\
    \x0f\n\x0bBID4_VOLUME\x10\x19\x12\x0e\n\nASK4_PRICE\x10\x1a\x12\x0f\n\
    \x0bASK4_VOLUME\x10\x1b\x12\x0e\n\nBID5_PRICE\x10\x1c\x12\x0f\n\x0bBID5_\
    VOLUME\x10\x1d\x12\x0e\n\nASK5_PRICE\x10\x1e\x12\x0f\n\x0bASK5_VOLUME\
    \x10\x1f\x12\t\n\x05TOTAL\x10\x20\"s\n\x0bObjSnapshot\x12'\n\x06symbol\
    \x18\x01\x20\x01(\x0b2\x0f.ftcodec.SymbolR\x06symbol\x12\x12\n\x04name\
    \x18\x02\x20\x01(\tR\x04name\x12'\n\x05lists\x18\x03\x20\x03(\x0b2\x11.f\
    tcodec.SnapshotR\x05lists\"=\n\x0fObjSnapshotList\x12*\n\x05lists\x18\
    \x01\x20\x03(\x0b2\x14.ftcodec.ObjSnapshotR\x05listsb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
