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
//! Generated file from `k8s.io_apimachinery_pkg_runtime_generated.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct RawExtension {
    // message fields
    raw: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RawExtension {
    fn default() -> &'a RawExtension {
        <RawExtension as ::protobuf::Message>::default_instance()
    }
}

impl RawExtension {
    pub fn new() -> RawExtension {
        ::std::default::Default::default()
    }

    // optional bytes raw = 1;


    pub fn get_raw(&self) -> &[u8] {
        match self.raw.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_raw(&mut self) {
        self.raw.clear();
    }

    pub fn has_raw(&self) -> bool {
        self.raw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw(&mut self, v: ::std::vec::Vec<u8>) {
        self.raw = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.raw.is_none() {
            self.raw.set_default();
        }
        self.raw.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw(&mut self) -> ::std::vec::Vec<u8> {
        self.raw.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for RawExtension {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.raw)?;
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
        if let Some(ref v) = self.raw.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.raw.as_ref() {
            os.write_bytes(1, &v)?;
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

    fn new() -> RawExtension {
        RawExtension::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "raw",
                |m: &RawExtension| { &m.raw },
                |m: &mut RawExtension| { &mut m.raw },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RawExtension>(
                "RawExtension",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RawExtension {
        static instance: ::protobuf::rt::LazyV2<RawExtension> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RawExtension::new)
    }
}

impl ::protobuf::Clear for RawExtension {
    fn clear(&mut self) {
        self.raw.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawExtension {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawExtension {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TypeMeta {
    // message fields
    apiVersion: ::protobuf::SingularField<::std::string::String>,
    kind: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TypeMeta {
    fn default() -> &'a TypeMeta {
        <TypeMeta as ::protobuf::Message>::default_instance()
    }
}

impl TypeMeta {
    pub fn new() -> TypeMeta {
        ::std::default::Default::default()
    }

    // optional string apiVersion = 1;


    pub fn get_apiVersion(&self) -> &str {
        match self.apiVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_apiVersion(&mut self) {
        self.apiVersion.clear();
    }

    pub fn has_apiVersion(&self) -> bool {
        self.apiVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_apiVersion(&mut self, v: ::std::string::String) {
        self.apiVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_apiVersion(&mut self) -> &mut ::std::string::String {
        if self.apiVersion.is_none() {
            self.apiVersion.set_default();
        }
        self.apiVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_apiVersion(&mut self) -> ::std::string::String {
        self.apiVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string kind = 2;


    pub fn get_kind(&self) -> &str {
        match self.kind.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_kind(&mut self) {
        self.kind.clear();
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kind(&mut self) -> &mut ::std::string::String {
        if self.kind.is_none() {
            self.kind.set_default();
        }
        self.kind.as_mut().unwrap()
    }

    // Take field
    pub fn take_kind(&mut self) -> ::std::string::String {
        self.kind.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for TypeMeta {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.apiVersion)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.kind)?;
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
        if let Some(ref v) = self.apiVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.kind.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.apiVersion.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.kind.as_ref() {
            os.write_string(2, &v)?;
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

    fn new() -> TypeMeta {
        TypeMeta::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "apiVersion",
                |m: &TypeMeta| { &m.apiVersion },
                |m: &mut TypeMeta| { &mut m.apiVersion },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "kind",
                |m: &TypeMeta| { &m.kind },
                |m: &mut TypeMeta| { &mut m.kind },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TypeMeta>(
                "TypeMeta",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TypeMeta {
        static instance: ::protobuf::rt::LazyV2<TypeMeta> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TypeMeta::new)
    }
}

impl ::protobuf::Clear for TypeMeta {
    fn clear(&mut self) {
        self.apiVersion.clear();
        self.kind.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TypeMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TypeMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Unknown {
    // message fields
    pub typeMeta: ::protobuf::SingularPtrField<TypeMeta>,
    raw: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    contentEncoding: ::protobuf::SingularField<::std::string::String>,
    contentType: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Unknown {
    fn default() -> &'a Unknown {
        <Unknown as ::protobuf::Message>::default_instance()
    }
}

impl Unknown {
    pub fn new() -> Unknown {
        ::std::default::Default::default()
    }

    // optional .k8s.io.apimachinery.pkg.runtime.TypeMeta typeMeta = 1;


    pub fn get_typeMeta(&self) -> &TypeMeta {
        self.typeMeta.as_ref().unwrap_or_else(|| <TypeMeta as ::protobuf::Message>::default_instance())
    }
    pub fn clear_typeMeta(&mut self) {
        self.typeMeta.clear();
    }

    pub fn has_typeMeta(&self) -> bool {
        self.typeMeta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typeMeta(&mut self, v: TypeMeta) {
        self.typeMeta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typeMeta(&mut self) -> &mut TypeMeta {
        if self.typeMeta.is_none() {
            self.typeMeta.set_default();
        }
        self.typeMeta.as_mut().unwrap()
    }

    // Take field
    pub fn take_typeMeta(&mut self) -> TypeMeta {
        self.typeMeta.take().unwrap_or_else(|| TypeMeta::new())
    }

    // optional bytes raw = 2;


    pub fn get_raw(&self) -> &[u8] {
        match self.raw.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_raw(&mut self) {
        self.raw.clear();
    }

    pub fn has_raw(&self) -> bool {
        self.raw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw(&mut self, v: ::std::vec::Vec<u8>) {
        self.raw = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.raw.is_none() {
            self.raw.set_default();
        }
        self.raw.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw(&mut self) -> ::std::vec::Vec<u8> {
        self.raw.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional string contentEncoding = 3;


    pub fn get_contentEncoding(&self) -> &str {
        match self.contentEncoding.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_contentEncoding(&mut self) {
        self.contentEncoding.clear();
    }

    pub fn has_contentEncoding(&self) -> bool {
        self.contentEncoding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contentEncoding(&mut self, v: ::std::string::String) {
        self.contentEncoding = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contentEncoding(&mut self) -> &mut ::std::string::String {
        if self.contentEncoding.is_none() {
            self.contentEncoding.set_default();
        }
        self.contentEncoding.as_mut().unwrap()
    }

    // Take field
    pub fn take_contentEncoding(&mut self) -> ::std::string::String {
        self.contentEncoding.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string contentType = 4;


    pub fn get_contentType(&self) -> &str {
        match self.contentType.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_contentType(&mut self) {
        self.contentType.clear();
    }

    pub fn has_contentType(&self) -> bool {
        self.contentType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contentType(&mut self, v: ::std::string::String) {
        self.contentType = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contentType(&mut self) -> &mut ::std::string::String {
        if self.contentType.is_none() {
            self.contentType.set_default();
        }
        self.contentType.as_mut().unwrap()
    }

    // Take field
    pub fn take_contentType(&mut self) -> ::std::string::String {
        self.contentType.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for Unknown {
    fn is_initialized(&self) -> bool {
        for v in &self.typeMeta {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.typeMeta)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.raw)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.contentEncoding)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.contentType)?;
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
        if let Some(ref v) = self.typeMeta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.raw.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.contentEncoding.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.contentType.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.typeMeta.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.raw.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.contentEncoding.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.contentType.as_ref() {
            os.write_string(4, &v)?;
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

    fn new() -> Unknown {
        Unknown::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TypeMeta>>(
                "typeMeta",
                |m: &Unknown| { &m.typeMeta },
                |m: &mut Unknown| { &mut m.typeMeta },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "raw",
                |m: &Unknown| { &m.raw },
                |m: &mut Unknown| { &mut m.raw },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "contentEncoding",
                |m: &Unknown| { &m.contentEncoding },
                |m: &mut Unknown| { &mut m.contentEncoding },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "contentType",
                |m: &Unknown| { &m.contentType },
                |m: &mut Unknown| { &mut m.contentType },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Unknown>(
                "Unknown",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Unknown {
        static instance: ::protobuf::rt::LazyV2<Unknown> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Unknown::new)
    }
}

impl ::protobuf::Clear for Unknown {
    fn clear(&mut self) {
        self.typeMeta.clear();
        self.raw.clear();
        self.contentEncoding.clear();
        self.contentType.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Unknown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Unknown {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/k8s.io_apimachinery_pkg_runtime_generated.proto\x12\x1fk8s.io.apimach\
    inery.pkg.runtime\"\x20\n\x0cRawExtension\x12\x10\n\x03raw\x18\x01\x20\
    \x01(\x0cR\x03raw\">\n\x08TypeMeta\x12\x1e\n\napiVersion\x18\x01\x20\x01\
    (\tR\napiVersion\x12\x12\n\x04kind\x18\x02\x20\x01(\tR\x04kind\"\xae\x01\
    \n\x07Unknown\x12E\n\x08typeMeta\x18\x01\x20\x01(\x0b2).k8s.io.apimachin\
    ery.pkg.runtime.TypeMetaR\x08typeMeta\x12\x10\n\x03raw\x18\x02\x20\x01(\
    \x0cR\x03raw\x12(\n\x0fcontentEncoding\x18\x03\x20\x01(\tR\x0fcontentEnc\
    oding\x12\x20\n\x0bcontentType\x18\x04\x20\x01(\tR\x0bcontentTypeB\tZ\
    \x07runtimeJ\x96\"\n\x06\x12\x04\x13\0}\x01\n\x82\x05\n\x01\x0c\x12\x03\
    \x13\0\x122\xac\x04\nCopyright\x20The\x20Kubernetes\x20Authors.\n\nLicen\
    sed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\
    \"License\");\nyou\x20may\x20not\x20use\x20this\x20file\x20except\x20in\
    \x20compliance\x20with\x20the\x20License.\nYou\x20may\x20obtain\x20a\x20\
    copy\x20of\x20the\x20License\x20at\n\nhttp://www.apache.org/licenses/LIC\
    ENSE-2.0\n\nUnless\x20required\x20by\x20applicable\x20law\x20or\x20agree\
    d\x20to\x20in\x20writing,\x20software\ndistributed\x20under\x20the\x20Li\
    cense\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\nWITHOU\
    T\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20e\
    xpress\x20or\x20implied.\nSee\x20the\x20License\x20for\x20the\x20specifi\
    c\x20language\x20governing\x20permissions\x20and\nlimitations\x20under\
    \x20the\x20License.\n2I\x20This\x20file\x20was\x20autogenerated\x20by\
    \x20go-to-protobuf.\x20Do\x20not\x20edit\x20it\x20manually!\n\n\x08\n\
    \x01\x02\x12\x03\x15\x08'\n\x08\n\x01\x08\x12\x03\x18\0\x1e\nA\n\x04\x08\
    \xe7\x07\0\x12\x03\x18\0\x1e\x1a4\x20Package-wide\x20variables\x20from\
    \x20generator\x20\"generated\".\n\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\
    \x18\x07\x11\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x18\x07\x11\n\x0e\n\
    \x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x18\x07\x11\n\x0c\n\x05\x08\xe7\x07\
    \0\x07\x12\x03\x18\x14\x1d\n\xab\x0b\n\x02\x04\0\x12\x04G\0L\x01\x1a\x9e\
    \x0b\x20RawExtension\x20is\x20used\x20to\x20hold\x20extensions\x20in\x20\
    external\x20versions.\n\n\x20To\x20use\x20this,\x20make\x20a\x20field\
    \x20which\x20has\x20RawExtension\x20as\x20its\x20type\x20in\x20your\x20e\
    xternal,\x20versioned\n\x20struct,\x20and\x20Object\x20in\x20your\x20int\
    ernal\x20struct.\x20You\x20also\x20need\x20to\x20register\x20your\n\x20v\
    arious\x20plugin\x20types.\n\n\x20//\x20Internal\x20package:\n\x20type\
    \x20MyAPIObject\x20struct\x20{\n\x20\truntime.TypeMeta\x20`json:\",inlin\
    e\"`\n\x20\tMyPlugin\x20runtime.Object\x20`json:\"myPlugin\"`\n\x20}\n\
    \x20type\x20PluginA\x20struct\x20{\n\x20\tAOption\x20string\x20`json:\"a\
    Option\"`\n\x20}\n\n\x20//\x20External\x20package:\n\x20type\x20MyAPIObj\
    ect\x20struct\x20{\n\x20\truntime.TypeMeta\x20`json:\",inline\"`\n\x20\t\
    MyPlugin\x20runtime.RawExtension\x20`json:\"myPlugin\"`\n\x20}\n\x20type\
    \x20PluginA\x20struct\x20{\n\x20\tAOption\x20string\x20`json:\"aOption\"\
    `\n\x20}\n\n\x20//\x20On\x20the\x20wire,\x20the\x20JSON\x20will\x20look\
    \x20something\x20like\x20this:\n\x20{\n\x20\t\"kind\":\"MyAPIObject\",\n\
    \x20\t\"apiVersion\":\"v1\",\n\x20\t\"myPlugin\":\x20{\n\x20\t\t\"kind\"\
    :\"PluginA\",\n\x20\t\t\"aOption\":\"foo\",\n\x20\t},\n\x20}\n\n\x20So\
    \x20what\x20happens?\x20Decode\x20first\x20uses\x20json\x20or\x20yaml\
    \x20to\x20unmarshal\x20the\x20serialized\x20data\x20into\n\x20your\x20ex\
    ternal\x20MyAPIObject.\x20That\x20causes\x20the\x20raw\x20JSON\x20to\x20\
    be\x20stored,\x20but\x20not\x20unpacked.\n\x20The\x20next\x20step\x20is\
    \x20to\x20copy\x20(using\x20pkg/conversion)\x20into\x20the\x20internal\
    \x20struct.\x20The\x20runtime\n\x20package's\x20DefaultScheme\x20has\x20\
    conversion\x20functions\x20installed\x20which\x20will\x20unpack\x20the\n\
    \x20JSON\x20stored\x20in\x20RawExtension,\x20turning\x20it\x20into\x20th\
    e\x20correct\x20object\x20type,\x20and\x20storing\x20it\n\x20in\x20the\
    \x20Object.\x20(TODO:\x20In\x20the\x20case\x20where\x20the\x20object\x20\
    is\x20of\x20an\x20unknown\x20type,\x20a\n\x20runtime.Unknown\x20object\
    \x20will\x20be\x20created\x20and\x20stored.)\n\n\x20+k8s:deepcopy-gen=tr\
    ue\n\x20+protobuf=true\n\x20+k8s:openapi-gen=true\n\n\n\n\x03\x04\0\x01\
    \x12\x03G\x08\x14\n\x92\x01\n\x04\x04\0\x02\0\x12\x03K\x02\x19\x1a\x84\
    \x01\x20Raw\x20is\x20the\x20underlying\x20serialization\x20of\x20this\
    \x20object.\n\n\x20TODO:\x20Determine\x20how\x20to\x20detect\x20ContentT\
    ype\x20and\x20ContentEncoding\x20of\x20'Raw'\x20data.\n\n\x0c\n\x05\x04\
    \0\x02\0\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03K\x0b\x10\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03K\x11\x14\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03K\x17\x18\n\xc5\x04\n\x02\x04\x01\x12\x04\\\0b\x01\x1a\xb8\
    \x04\x20TypeMeta\x20is\x20shared\x20by\x20all\x20top\x20level\x20objects\
    .\x20The\x20proper\x20way\x20to\x20use\x20it\x20is\x20to\x20inline\x20it\
    \x20in\x20your\x20type,\n\x20like\x20this:\n\x20type\x20MyAwesomeAPIObje\
    ct\x20struct\x20{\n\x20\x20\x20\x20\x20\x20runtime.TypeMeta\x20\x20\x20\
    \x20`json:\",inline\"`\n\x20\x20\x20\x20\x20\x20...\x20//\x20other\x20fi\
    elds\n\x20}\n\x20func\x20(obj\x20*MyAwesomeAPIObject)\x20SetGroupVersion\
    Kind(gvk\x20*metav1.GroupVersionKind)\x20{\x20metav1.UpdateTypeMeta(obj,\
    gvk)\x20};\x20GroupVersionKind()\x20*GroupVersionKind\n\n\x20TypeMeta\
    \x20is\x20provided\x20here\x20for\x20convenience.\x20You\x20may\x20use\
    \x20it\x20directly\x20from\x20this\x20package\x20or\x20define\n\x20your\
    \x20own\x20with\x20the\x20same\x20fields.\n\n\x20+k8s:deepcopy-gen=false\
    \n\x20+protobuf=true\n\x20+k8s:openapi-gen=true\n\n\n\n\x03\x04\x01\x01\
    \x12\x03\\\x08\x10\n\x18\n\x04\x04\x01\x02\0\x12\x03^\x02!\x1a\x0b\x20+o\
    ptional\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03^\x02\n\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03^\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03^\
    \x12\x1c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03^\x1f\x20\n\x18\n\x04\x04\
    \x01\x02\x01\x12\x03a\x02\x1b\x1a\x0b\x20+optional\n\n\x0c\n\x05\x04\x01\
    \x02\x01\x04\x12\x03a\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03a\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03a\x12\x16\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03a\x19\x1a\n\xd8\x03\n\x02\x04\x02\x12\x04n\0}\
    \x01\x1a\xcb\x03\x20Unknown\x20allows\x20api\x20objects\x20with\x20unkno\
    wn\x20types\x20to\x20be\x20passed-through.\x20This\x20can\x20be\x20used\
    \n\x20to\x20deal\x20with\x20the\x20API\x20objects\x20from\x20a\x20plug-i\
    n.\x20Unknown\x20objects\x20still\x20have\x20functioning\n\x20TypeMeta\
    \x20features--\x20kind,\x20version,\x20etc.\n\x20TODO:\x20Make\x20this\
    \x20object\x20have\x20easy\x20access\x20to\x20field\x20based\x20accessor\
    s\x20and\x20settors\x20for\n\x20metadata\x20and\x20field\x20mutatation.\
    \n\n\x20+k8s:deepcopy-gen=true\n\x20+k8s:deepcopy-gen:interfaces=k8s.io/\
    apimachinery/pkg/runtime.Object\n\x20+protobuf=true\n\x20+k8s:openapi-ge\
    n=true\n\n\n\n\x03\x04\x02\x01\x12\x03n\x08\x0f\n\x0b\n\x04\x04\x02\x02\
    \0\x12\x03o\x02!\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03o\x02\n\n\x0c\n\
    \x05\x04\x02\x02\0\x06\x12\x03o\x0b\x13\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03o\x14\x1c\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03o\x1f\x20\n\xc8\
    \x01\n\x04\x04\x02\x02\x01\x12\x03t\x02\x19\x1a\xba\x01\x20Raw\x20will\
    \x20hold\x20the\x20complete\x20serialized\x20object\x20which\x20couldn't\
    \x20be\x20matched\n\x20with\x20a\x20registered\x20type.\x20Most\x20likel\
    y,\x20nothing\x20should\x20be\x20done\x20with\x20this\n\x20except\x20for\
    \x20passing\x20it\x20through\x20the\x20system.\n\n\x0c\n\x05\x04\x02\x02\
    \x01\x04\x12\x03t\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03t\x0b\x10\
    \n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03t\x11\x14\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03t\x17\x18\ne\n\x04\x04\x02\x02\x02\x12\x03x\x02&\x1a\
    X\x20ContentEncoding\x20is\x20encoding\x20used\x20to\x20encode\x20'Raw'\
    \x20data.\n\x20Unspecified\x20means\x20no\x20encoding.\n\n\x0c\n\x05\x04\
    \x02\x02\x02\x04\x12\x03x\x02\n\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03x\
    \x0b\x11\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03x\x12!\n\x0c\n\x05\x04\
    \x02\x02\x02\x03\x12\x03x$%\np\n\x04\x04\x02\x02\x03\x12\x03|\x02\"\x1ac\
    \x20ContentType\x20\x20is\x20serialization\x20method\x20used\x20to\x20se\
    rialize\x20'Raw'.\n\x20Unspecified\x20means\x20ContentTypeJSON.\n\n\x0c\
    \n\x05\x04\x02\x02\x03\x04\x12\x03|\x02\n\n\x0c\n\x05\x04\x02\x02\x03\
    \x05\x12\x03|\x0b\x11\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03|\x12\x1d\n\
    \x0c\n\x05\x04\x02\x02\x03\x03\x12\x03|\x20!\
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