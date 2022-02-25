// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `k8s.io_apimachinery_pkg_util_intstr_generated.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct IntOrString {
    // message fields
    field_type: ::std::option::Option<i64>,
    intVal: ::std::option::Option<i32>,
    strVal: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a IntOrString {
    fn default() -> &'a IntOrString {
        <IntOrString as ::protobuf::Message>::default_instance()
    }
}

impl IntOrString {
    pub fn new() -> IntOrString {
        ::std::default::Default::default()
    }

    // optional int64 type = 1;


    pub fn get_field_type(&self) -> i64 {
        self.field_type.unwrap_or(0)
    }
    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i64) {
        self.field_type = ::std::option::Option::Some(v);
    }

    // optional int32 intVal = 2;


    pub fn get_intVal(&self) -> i32 {
        self.intVal.unwrap_or(0)
    }
    pub fn clear_intVal(&mut self) {
        self.intVal = ::std::option::Option::None;
    }

    pub fn has_intVal(&self) -> bool {
        self.intVal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_intVal(&mut self, v: i32) {
        self.intVal = ::std::option::Option::Some(v);
    }

    // optional string strVal = 3;


    pub fn get_strVal(&self) -> &str {
        match self.strVal.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_strVal(&mut self) {
        self.strVal.clear();
    }

    pub fn has_strVal(&self) -> bool {
        self.strVal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_strVal(&mut self, v: ::std::string::String) {
        self.strVal = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_strVal(&mut self) -> &mut ::std::string::String {
        if self.strVal.is_none() {
            self.strVal.set_default();
        }
        self.strVal.as_mut().unwrap()
    }

    // Take field
    pub fn take_strVal(&mut self) -> ::std::string::String {
        self.strVal.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for IntOrString {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.intVal = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.strVal)?;
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.intVal {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.strVal.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.intVal {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.strVal.as_ref() {
            os.write_string(3, &v)?;
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
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> IntOrString {
        IntOrString::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "type",
                |m: &IntOrString| { &m.field_type },
                |m: &mut IntOrString| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "intVal",
                |m: &IntOrString| { &m.intVal },
                |m: &mut IntOrString| { &mut m.intVal },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "strVal",
                |m: &IntOrString| { &m.strVal },
                |m: &mut IntOrString| { &mut m.strVal },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<IntOrString>(
                "IntOrString",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static IntOrString {
        static instance: ::protobuf::rt::LazyV2<IntOrString> = ::protobuf::rt::LazyV2::INIT;
        instance.get(IntOrString::new)
    }
}

impl ::protobuf::Clear for IntOrString {
    fn clear(&mut self) {
        self.field_type = ::std::option::Option::None;
        self.intVal = ::std::option::Option::None;
        self.strVal.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IntOrString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IntOrString {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n3k8s.io_apimachinery_pkg_util_intstr_generated.proto\x12#k8s.io.apimac\
    hinery.pkg.util.intstr\"Q\n\x0bIntOrString\x12\x12\n\x04type\x18\x01\x20\
    \x01(\x03R\x04type\x12\x16\n\x06intVal\x18\x02\x20\x01(\x05R\x06intVal\
    \x12\x16\n\x06strVal\x18\x03\x20\x01(\tR\x06strValB\x08Z\x06intstrJ\xfd\
    \n\n\x06\x12\x04\x13\0)\x01\n\x82\x05\n\x01\x0c\x12\x03\x13\0\x122\xac\
    \x04\nCopyright\x20The\x20Kubernetes\x20Authors.\n\nLicensed\x20under\
    \x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\
    \nyou\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\
    \x20with\x20the\x20License.\nYou\x20may\x20obtain\x20a\x20copy\x20of\x20\
    the\x20License\x20at\n\nhttp://www.apache.org/licenses/LICENSE-2.0\n\nUn\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\ndistributed\x20under\x20the\x20License\x20is\
    \x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\nWITHOUT\x20WARRAN\
    TIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20o\
    r\x20implied.\nSee\x20the\x20License\x20for\x20the\x20specific\x20langua\
    ge\x20governing\x20permissions\x20and\nlimitations\x20under\x20the\x20Li\
    cense.\n2I\x20This\x20file\x20was\x20autogenerated\x20by\x20go-to-protob\
    uf.\x20Do\x20not\x20edit\x20it\x20manually!\n\n\x08\n\x01\x02\x12\x03\
    \x15\x08+\n\x08\n\x01\x08\x12\x03\x18\0\x1d\nA\n\x04\x08\xe7\x07\0\x12\
    \x03\x18\0\x1d\x1a4\x20Package-wide\x20variables\x20from\x20generator\
    \x20\"generated\".\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x18\x07\x11\n\
    \r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x18\x07\x11\n\x0e\n\x07\x08\xe7\x07\
    \0\x02\0\x01\x12\x03\x18\x07\x11\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\
    \x18\x14\x1c\n\x80\x03\n\x02\x04\0\x12\x04#\0)\x01\x1a\xf3\x02\x20IntOrS\
    tring\x20is\x20a\x20type\x20that\x20can\x20hold\x20an\x20int32\x20or\x20\
    a\x20string.\x20\x20When\x20used\x20in\n\x20JSON\x20or\x20YAML\x20marsha\
    lling\x20and\x20unmarshalling,\x20it\x20produces\x20or\x20consumes\x20th\
    e\n\x20inner\x20type.\x20\x20This\x20allows\x20you\x20to\x20have,\x20for\
    \x20example,\x20a\x20JSON\x20field\x20that\x20can\n\x20accept\x20a\x20na\
    me\x20or\x20number.\n\x20TODO:\x20Rename\x20to\x20Int32OrString\n\n\x20+\
    protobuf=true\n\x20+protobuf.options.(gogoproto.goproto_stringer)=false\
    \n\x20+k8s:openapi-gen=true\n\n\n\n\x03\x04\0\x01\x12\x03#\x08\x13\n\x0b\
    \n\x04\x04\0\x02\0\x12\x03$\x02\x1a\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03$\
    \x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03$\x0b\x10\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03$\x11\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03$\x18\x19\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03&\x02\x1c\n\x0c\n\x05\x04\0\x02\x01\
    \x04\x12\x03&\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03&\x0b\x10\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03&\x11\x17\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03&\x1a\x1b\n\x0b\n\x04\x04\0\x02\x02\x12\x03(\x02\x1d\n\x0c\n\x05\
    \x04\0\x02\x02\x04\x12\x03(\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03(\
    \x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03(\x12\x18\n\x0c\n\x05\x04\
    \0\x02\x02\x03\x12\x03(\x1b\x1c\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
