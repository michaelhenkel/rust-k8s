/// APIGroup contains the name, the supported versions, and the preferred version
/// of a group.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiGroup {
    /// name is the name of the group.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// versions are the versions supported in this group.
    #[prost(message, repeated, tag="2")]
    pub versions: ::prost::alloc::vec::Vec<GroupVersionForDiscovery>,
    /// preferredVersion is the version preferred by the API server, which
    /// probably is the storage version.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub preferred_version: ::core::option::Option<GroupVersionForDiscovery>,
    /// a map of client CIDR to server address that is serving this group.
    /// This is to help clients reach servers in the most network-efficient way possible.
    /// Clients can use the appropriate server address as per the CIDR that they match.
    /// In case of multiple matches, clients should use the longest matching CIDR.
    /// The server returns only those CIDRs that it thinks that the client can match.
    /// For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP.
    /// Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.
    /// +optional
    #[prost(message, repeated, tag="4")]
    pub server_address_by_client_cid_rs: ::prost::alloc::vec::Vec<ServerAddressByClientCidr>,
}
/// APIGroupList is a list of APIGroup, to allow clients to discover the API at
/// /apis.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiGroupList {
    /// groups is a list of APIGroup.
    #[prost(message, repeated, tag="1")]
    pub groups: ::prost::alloc::vec::Vec<ApiGroup>,
}
/// APIResource specifies the name of a resource and whether it is namespaced.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiResource {
    /// name is the plural name of the resource.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// singularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely.
    /// The singularName is more correct for reporting status on a single item and both singular and plural are allowed
    /// from the kubectl CLI interface.
    #[prost(string, optional, tag="6")]
    pub singular_name: ::core::option::Option<::prost::alloc::string::String>,
    /// namespaced indicates if a resource is namespaced or not.
    #[prost(bool, optional, tag="2")]
    pub namespaced: ::core::option::Option<bool>,
    /// group is the preferred group of the resource.  Empty implies the group of the containing resource list.
    /// For subresources, this may have a different value, for example: Scale".
    #[prost(string, optional, tag="8")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// version is the preferred version of the resource.  Empty implies the version of the containing resource list
    /// For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)".
    #[prost(string, optional, tag="9")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// kind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo')
    #[prost(string, optional, tag="3")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// verbs is a list of supported kube verbs (this includes get, list, watch, create,
    /// update, patch, delete, deletecollection, and proxy)
    #[prost(message, optional, tag="4")]
    pub verbs: ::core::option::Option<Verbs>,
    /// shortNames is a list of suggested short names of the resource.
    #[prost(string, repeated, tag="5")]
    pub short_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// categories is a list of the grouped resources this resource belongs to (e.g. 'all')
    #[prost(string, repeated, tag="7")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The hash value of the storage version, the version this resource is
    /// converted to when written to the data store. Value must be treated
    /// as opaque by clients. Only equality comparison on the value is valid.
    /// This is an alpha feature and may change or be removed in the future.
    /// The field is populated by the apiserver only if the
    /// StorageVersionHash feature gate is enabled.
    /// This field will remain optional even if it graduates.
    /// +optional
    #[prost(string, optional, tag="10")]
    pub storage_version_hash: ::core::option::Option<::prost::alloc::string::String>,
}
/// APIResourceList is a list of APIResource, it is used to expose the name of the
/// resources supported in a specific group and version, and if the resource
/// is namespaced.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiResourceList {
    /// groupVersion is the group and version this APIResourceList is for.
    #[prost(string, optional, tag="1")]
    pub group_version: ::core::option::Option<::prost::alloc::string::String>,
    /// resources contains the name of the resources and if they are namespaced.
    #[prost(message, repeated, tag="2")]
    pub resources: ::prost::alloc::vec::Vec<ApiResource>,
}
/// APIVersions lists the versions that are available, to allow clients to
/// discover the API at /api, which is the root path of the legacy v1 API.
///
/// +protobuf.options.(gogoproto.goproto_stringer)=false
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiVersions {
    /// versions are the api versions that are available.
    #[prost(string, repeated, tag="1")]
    pub versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// a map of client CIDR to server address that is serving this group.
    /// This is to help clients reach servers in the most network-efficient way possible.
    /// Clients can use the appropriate server address as per the CIDR that they match.
    /// In case of multiple matches, clients should use the longest matching CIDR.
    /// The server returns only those CIDRs that it thinks that the client can match.
    /// For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP.
    /// Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.
    #[prost(message, repeated, tag="2")]
    pub server_address_by_client_cid_rs: ::prost::alloc::vec::Vec<ServerAddressByClientCidr>,
}
/// Condition contains details for one aspect of the current state of this API Resource.
/// ---
/// This struct is intended for direct use as an array at the field path .status.conditions.  For example,
/// type FooStatus struct{
///     // Represents the observations of a foo's current state.
///     // Known .status.conditions.type are: "Available", "Progressing", and "Degraded"
///     // +patchMergeKey=type
///     // +patchStrategy=merge
///     // +listType=map
///     // +listMapKey=type
///     Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"`
///
///     // other fields
/// }
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// type of condition in CamelCase or in foo.example.com/CamelCase.
    /// ---
    /// Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be
    /// useful (see .node.status.conditions), the ability to deconflict is important.
    /// The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    /// +required
    /// +kubebuilder:validation:Required
    /// +kubebuilder:validation:Pattern=`^(\[a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*/)?(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9\])$`
    /// +kubebuilder:validation:MaxLength=316
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// status of the condition, one of True, False, Unknown.
    /// +required
    /// +kubebuilder:validation:Required
    /// +kubebuilder:validation:Enum=True;False;Unknown
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon.
    /// For instance, if .metadata.generation is currently 12, but the .status.conditions\[x\].observedGeneration is 9, the condition is out of date
    /// with respect to the current state of the instance.
    /// +optional
    /// +kubebuilder:validation:Minimum=0
    #[prost(int64, optional, tag="3")]
    pub observed_generation: ::core::option::Option<i64>,
    /// lastTransitionTime is the last time the condition transitioned from one status to another.
    /// This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    /// +required
    /// +kubebuilder:validation:Required
    /// +kubebuilder:validation:Type=string
    /// +kubebuilder:validation:Format=date-time
    #[prost(message, optional, tag="4")]
    pub last_transition_time: ::core::option::Option<Time>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition.
    /// Producers of specific condition types may define expected values and meanings for this field,
    /// and whether the values are considered a guaranteed API.
    /// The value should be a CamelCase string.
    /// This field may not be empty.
    /// +required
    /// +kubebuilder:validation:Required
    /// +kubebuilder:validation:MaxLength=1024
    /// +kubebuilder:validation:MinLength=1
    /// +kubebuilder:validation:Pattern=`^\[A-Za-z]([A-Za-z0-9_,:]*[A-Za-z0-9_\])?$`
    #[prost(string, optional, tag="5")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// message is a human readable message indicating details about the transition.
    /// This may be an empty string.
    /// +required
    /// +kubebuilder:validation:Required
    /// +kubebuilder:validation:MaxLength=32768
    #[prost(string, optional, tag="6")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// CreateOptions may be provided when creating an API object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOptions {
    /// When present, indicates that modifications should not be
    /// persisted. An invalid or unrecognized dryRun directive will
    /// result in an error response and no further processing of the
    /// request. Valid values are:
    /// - All: all dry run stages will be processed
    /// +optional
    #[prost(string, repeated, tag="1")]
    pub dry_run: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// fieldManager is a name associated with the actor or entity
    /// that is making these changes. The value must be less than or
    /// 128 characters long, and only contain printable characters,
    /// as defined by <https://golang.org/pkg/unicode/#IsPrint.>
    /// +optional
    #[prost(string, optional, tag="3")]
    pub field_manager: ::core::option::Option<::prost::alloc::string::String>,
}
/// DeleteOptions may be provided when deleting an API object.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOptions {
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer.
    /// The value zero indicates delete immediately. If this value is nil, the default grace period for the
    /// specified type will be used.
    /// Defaults to a per object value if not specified. zero means delete immediately.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub grace_period_seconds: ::core::option::Option<i64>,
    /// Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be
    /// returned.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(message, optional, tag="2")]
    pub preconditions: ::core::option::Option<Preconditions>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7.
    /// Should the dependent objects be orphaned. If true/false, the "orphan"
    /// finalizer will be added to/removed from the object's finalizers list.
    /// Either this field or PropagationPolicy may be set, but not both.
    /// +optional
    #[prost(bool, optional, tag="3")]
    pub orphan_dependents: ::core::option::Option<bool>,
    /// Whether and how garbage collection will be performed.
    /// Either this field or OrphanDependents may be set, but not both.
    /// The default policy is decided by the existing finalizer set in the
    /// metadata.finalizers and the resource-specific default policy.
    /// Acceptable values are: 'Orphan' - orphan the dependents; 'Background' -
    /// allow the garbage collector to delete the dependents in the background;
    /// 'Foreground' - a cascading policy that deletes all dependents in the
    /// foreground.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub propagation_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// When present, indicates that modifications should not be
    /// persisted. An invalid or unrecognized dryRun directive will
    /// result in an error response and no further processing of the
    /// request. Valid values are:
    /// - All: all dry run stages will be processed
    /// +optional
    #[prost(string, repeated, tag="5")]
    pub dry_run: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Duration is a wrapper around time.Duration which supports correct
/// marshaling to YAML and JSON. In particular, it marshals into strings, which
/// can be used as map keys in json.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Duration {
    #[prost(int64, optional, tag="1")]
    pub duration: ::core::option::Option<i64>,
}
/// ExportOptions is the query options to the standard REST get call.
/// Deprecated. Planned for removal in 1.18.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportOptions {
    /// Should this value be exported.  Export strips fields that a user can not specify.
    /// Deprecated. Planned for removal in 1.18.
    #[prost(bool, optional, tag="1")]
    pub export: ::core::option::Option<bool>,
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
    /// Deprecated. Planned for removal in 1.18.
    #[prost(bool, optional, tag="2")]
    pub exact: ::core::option::Option<bool>,
}
/// FieldsV1 stores a set of fields in a data structure like a Trie, in JSON format.
///
/// Each key is either a '.' representing the field itself, and will always map to an empty set,
/// or a string representing a sub-field or item. The string will follow one of these four formats:
/// 'f:<name>', where <name> is the name of a field in a struct, or key in a map
/// 'v:<value>', where <value> is the exact json formatted value of a list item
/// 'i:<index>', where <index> is position of a item in a list
/// 'k:<keys>', where <keys> is a map of  a list item's key fields to their unique values
/// If a key maps to an empty Fields value, the field that key represents is part of the set.
///
/// The exact format is defined in sigs.k8s.io/structured-merge-diff
/// +protobuf.options.(gogoproto.goproto_stringer)=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldsV1 {
    /// Raw is the underlying serialization of this object.
    #[prost(bytes="vec", optional, tag="1")]
    pub raw: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// GetOptions is the standard query options to the standard REST get call.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOptions {
    /// resourceVersion sets a constraint on what resource versions a request may be served from.
    /// See <https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions> for
    /// details.
    ///
    /// Defaults to unset
    /// +optional
    #[prost(string, optional, tag="1")]
    pub resource_version: ::core::option::Option<::prost::alloc::string::String>,
}
/// GroupKind specifies a Group and a Kind, but does not force a version.  This is useful for identifying
/// concepts during lookup stages without having partially valid types
///
/// +protobuf.options.(gogoproto.goproto_stringer)=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupKind {
    #[prost(string, optional, tag="1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
}
/// GroupResource specifies a Group and a Resource, but does not force a version.  This is useful for identifying
/// concepts during lookup stages without having partially valid types
///
/// +protobuf.options.(gogoproto.goproto_stringer)=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupResource {
    #[prost(string, optional, tag="1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
}
/// GroupVersion contains the "group" and the "version", which uniquely identifies the API.
///
/// +protobuf.options.(gogoproto.goproto_stringer)=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupVersion {
    #[prost(string, optional, tag="1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
}
/// GroupVersion contains the "group/version" and "version" string of a version.
/// It is made a struct to keep extensibility.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupVersionForDiscovery {
    /// groupVersion specifies the API group and version in the form "group/version"
    #[prost(string, optional, tag="1")]
    pub group_version: ::core::option::Option<::prost::alloc::string::String>,
    /// version specifies the version in the form of "version". This is to save
    /// the clients the trouble of splitting the GroupVersion.
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
}
/// GroupVersionKind unambiguously identifies a kind.  It doesn't anonymously include GroupVersion
/// to avoid automatic coersion.  It doesn't use a GroupVersion to avoid custom marshalling
///
/// +protobuf.options.(gogoproto.goproto_stringer)=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupVersionKind {
    #[prost(string, optional, tag="1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
}
/// GroupVersionResource unambiguously identifies a resource.  It doesn't anonymously include GroupVersion
/// to avoid automatic coersion.  It doesn't use a GroupVersion to avoid custom marshalling
///
/// +protobuf.options.(gogoproto.goproto_stringer)=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupVersionResource {
    #[prost(string, optional, tag="1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
}
/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null
/// label selector matches no objects.
/// +structType=atomic
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelSelector {
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    /// +optional
    #[prost(map="string, string", tag="1")]
    pub match_labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub match_expressions: ::prost::alloc::vec::Vec<LabelSelectorRequirement>,
}
/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelSelectorRequirement {
    /// key is the label key that the selector applies to.
    /// +patchMergeKey=key
    /// +patchStrategy=merge
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    #[prost(string, optional, tag="2")]
    pub operator: ::core::option::Option<::prost::alloc::string::String>,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    /// +optional
    #[prost(string, repeated, tag="3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// List holds a list of objects, which may not be known by the server.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<ListMeta>,
    /// List of objects
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<super::super::super::runtime::RawExtension>,
}
/// ListMeta describes metadata that synthetic resources must have, including lists and
/// various status objects. A resource may have only one of {ObjectMeta, ListMeta}.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeta {
    /// selfLink is a URL representing this object.
    /// Populated by the system.
    /// Read-only.
    ///
    /// DEPRECATED
    /// Kubernetes will stop propagating this field in 1.20 release and the field is planned
    /// to be removed in 1.21 release.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub self_link: ::core::option::Option<::prost::alloc::string::String>,
    /// String that identifies the server's internal version of this object that
    /// can be used by clients to determine when objects have changed.
    /// Value must be treated as opaque by clients and passed unmodified back to the server.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency>
    /// +optional
    #[prost(string, optional, tag="2")]
    pub resource_version: ::core::option::Option<::prost::alloc::string::String>,
    /// continue may be set if the user set a limit on the number of items returned, and indicates that
    /// the server has more data available. The value is opaque and may be used to issue another request
    /// to the endpoint that served this list to retrieve the next set of available objects. Continuing a
    /// consistent list may not be possible if the server configuration has changed or more than a few
    /// minutes have passed. The resourceVersion field returned when using this continue value will be
    /// identical to the value in the first response, unless you have received this token from an error
    /// message.
    #[prost(string, optional, tag="3")]
    pub r#continue: ::core::option::Option<::prost::alloc::string::String>,
    /// remainingItemCount is the number of subsequent items in the list which are not included in this
    /// list response. If the list request contained label or field selectors, then the number of
    /// remaining items is unknown and the field will be left unset and omitted during serialization.
    /// If the list is complete (either because it is not chunking or because this is the last chunk),
    /// then there are no more remaining items and this field will be left unset and omitted during
    /// serialization.
    /// Servers older than v1.15 do not set this field.
    /// The intended use of the remainingItemCount is *estimating* the size of a collection. Clients
    /// should not rely on the remainingItemCount to be set or to be exact.
    /// +optional
    #[prost(int64, optional, tag="4")]
    pub remaining_item_count: ::core::option::Option<i64>,
}
/// ListOptions is the query options to a standard REST list call.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOptions {
    /// A selector to restrict the list of returned objects by their labels.
    /// Defaults to everything.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub label_selector: ::core::option::Option<::prost::alloc::string::String>,
    /// A selector to restrict the list of returned objects by their fields.
    /// Defaults to everything.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub field_selector: ::core::option::Option<::prost::alloc::string::String>,
    /// Watch for changes to the described resources and return them as a stream of
    /// add, update, and remove notifications. Specify resourceVersion.
    /// +optional
    #[prost(bool, optional, tag="3")]
    pub watch: ::core::option::Option<bool>,
    /// allowWatchBookmarks requests watch events with type "BOOKMARK".
    /// Servers that do not implement bookmarks may ignore this flag and
    /// bookmarks are sent at the server's discretion. Clients should not
    /// assume bookmarks are returned at any specific interval, nor may they
    /// assume the server will send any BOOKMARK event during a session.
    /// If this is not a watch, this field is ignored.
    /// If the feature gate WatchBookmarks is not enabled in apiserver,
    /// this field is ignored.
    /// +optional
    #[prost(bool, optional, tag="9")]
    pub allow_watch_bookmarks: ::core::option::Option<bool>,
    /// resourceVersion sets a constraint on what resource versions a request may be served from.
    /// See <https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions> for
    /// details.
    ///
    /// Defaults to unset
    /// +optional
    #[prost(string, optional, tag="4")]
    pub resource_version: ::core::option::Option<::prost::alloc::string::String>,
    /// resourceVersionMatch determines how resourceVersion is applied to list calls.
    /// It is highly recommended that resourceVersionMatch be set for list calls where
    /// resourceVersion is set
    /// See <https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions> for
    /// details.
    ///
    /// Defaults to unset
    /// +optional
    #[prost(string, optional, tag="10")]
    pub resource_version_match: ::core::option::Option<::prost::alloc::string::String>,
    /// Timeout for the list/watch call.
    /// This limits the duration of the call, regardless of any activity or inactivity.
    /// +optional
    #[prost(int64, optional, tag="5")]
    pub timeout_seconds: ::core::option::Option<i64>,
    /// limit is a maximum number of responses to return for a list call. If more items exist, the
    /// server will set the `continue` field on the list metadata to a value that can be used with the
    /// same initial query to retrieve the next set of results. Setting a limit may return fewer than
    /// the requested amount of items (up to zero items) in the event all requested objects are
    /// filtered out and clients should only use the presence of the continue field to determine whether
    /// more results are available. Servers may choose not to support the limit argument and will return
    /// all of the available results. If limit is specified and the continue field is empty, clients may
    /// assume that no more results are available. This field is not supported if watch is true.
    ///
    /// The server guarantees that the objects returned when using continue will be identical to issuing
    /// a single list call without a limit - that is, no objects created, modified, or deleted after the
    /// first request is issued will be included in any subsequent continued requests. This is sometimes
    /// referred to as a consistent snapshot, and ensures that a client that is using limit to receive
    /// smaller chunks of a very large result can ensure they see all possible objects. If objects are
    /// updated during a chunked list the version of the object that was present at the time the first list
    /// result was calculated is returned.
    #[prost(int64, optional, tag="7")]
    pub limit: ::core::option::Option<i64>,
    /// The continue option should be set when retrieving more results from the server. Since this value is
    /// server defined, clients may only use the continue value from a previous query result with identical
    /// query parameters (except for the value of continue) and the server may reject a continue value it
    /// does not recognize. If the specified continue value is no longer valid whether due to expiration
    /// (generally five to fifteen minutes) or a configuration change on the server, the server will
    /// respond with a 410 ResourceExpired error together with a continue token. If the client needs a
    /// consistent list, it must restart their list without the continue field. Otherwise, the client may
    /// send another list request with the token received with the 410 error, the server will respond with
    /// a list starting from the next key, but from the latest snapshot, which is inconsistent from the
    /// previous list results - objects that are created, modified, or deleted after the first list request
    /// will be included in the response, as long as their keys are after the "next key".
    ///
    /// This field is not supported when watch is true. Clients may start a watch from the last
    /// resourceVersion value returned by the server and not miss any modifications.
    #[prost(string, optional, tag="8")]
    pub r#continue: ::core::option::Option<::prost::alloc::string::String>,
}
/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource
/// that the fieldset applies to.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedFieldsEntry {
    /// Manager is an identifier of the workflow managing these fields.
    #[prost(string, optional, tag="1")]
    pub manager: ::core::option::Option<::prost::alloc::string::String>,
    /// Operation is the type of operation which lead to this ManagedFieldsEntry being created.
    /// The only valid values for this field are 'Apply' and 'Update'.
    #[prost(string, optional, tag="2")]
    pub operation: ::core::option::Option<::prost::alloc::string::String>,
    /// APIVersion defines the version of this resource that this field set
    /// applies to. The format is "group/version" just like the top-level
    /// APIVersion field. It is necessary to track the version of a field
    /// set because it cannot be automatically converted.
    #[prost(string, optional, tag="3")]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
    /// Time is timestamp of when these fields were set. It should always be empty if Operation is 'Apply'
    /// +optional
    #[prost(message, optional, tag="4")]
    pub time: ::core::option::Option<Time>,
    /// FieldsType is the discriminator for the different fields format and version.
    /// There is currently only one possible value: "FieldsV1"
    #[prost(string, optional, tag="6")]
    pub fields_type: ::core::option::Option<::prost::alloc::string::String>,
    /// FieldsV1 holds the first JSON version format as described in the "FieldsV1" type.
    /// +optional
    #[prost(message, optional, tag="7")]
    pub fields_v1: ::core::option::Option<FieldsV1>,
}
/// MicroTime is version of Time with microsecond level precision.
///
/// +protobuf.options.marshal=false
/// +protobuf.as=Timestamp
/// +protobuf.options.(gogoproto.goproto_stringer)=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MicroTime {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, optional, tag="1")]
    pub seconds: ::core::option::Option<i64>,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive. This field may be limited in precision depending on context.
    #[prost(int32, optional, tag="2")]
    pub nanos: ::core::option::Option<i32>,
}
/// ObjectMeta is metadata that all persisted resources must have, which includes all objects
/// users must create.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMeta {
    /// Name must be unique within a namespace. Is required when creating resources, although
    /// some resources may allow a client to request the generation of an appropriate name
    /// automatically. Name is primarily intended for creation idempotence and configuration
    /// definition.
    /// Cannot be updated.
    /// More info: <http://kubernetes.io/docs/user-guide/identifiers#names>
    /// +optional
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// GenerateName is an optional prefix, used by the server, to generate a unique
    /// name ONLY IF the Name field has not been provided.
    /// If this field is used, the name returned to the client will be different
    /// than the name passed. This value will also be combined with a unique suffix.
    /// The provided value has the same validation rules as the Name field,
    /// and may be truncated by the length of the suffix required to make the value
    /// unique on the server.
    ///
    /// If this field is specified and the generated name exists, the server will
    /// NOT return a 409 - instead, it will either return 201 Created or 500 with Reason
    /// ServerTimeout indicating a unique name could not be found in the time allotted, and the client
    /// should retry (optionally after the time indicated in the Retry-After header).
    ///
    /// Applied only if Name is not specified.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency>
    /// +optional
    #[prost(string, optional, tag="2")]
    pub generate_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace defines the space within which each name must be unique. An empty namespace is
    /// equivalent to the "default" namespace, but "default" is the canonical representation.
    /// Not all objects are required to be scoped to a namespace - the value of this field for
    /// those objects will be empty.
    ///
    /// Must be a DNS_LABEL.
    /// Cannot be updated.
    /// More info: <http://kubernetes.io/docs/user-guide/namespaces>
    /// +optional
    #[prost(string, optional, tag="3")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// SelfLink is a URL representing this object.
    /// Populated by the system.
    /// Read-only.
    ///
    /// DEPRECATED
    /// Kubernetes will stop propagating this field in 1.20 release and the field is planned
    /// to be removed in 1.21 release.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub self_link: ::core::option::Option<::prost::alloc::string::String>,
    /// UID is the unique in time and space value for this object. It is typically generated by
    /// the server on successful creation of a resource and is not allowed to change on PUT
    /// operations.
    ///
    /// Populated by the system.
    /// Read-only.
    /// More info: <http://kubernetes.io/docs/user-guide/identifiers#uids>
    /// +optional
    #[prost(string, optional, tag="5")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// An opaque value that represents the internal version of this object that can
    /// be used by clients to determine when objects have changed. May be used for optimistic
    /// concurrency, change detection, and the watch operation on a resource or set of resources.
    /// Clients must treat these values as opaque and passed unmodified back to the server.
    /// They may only be valid for a particular resource or set of resources.
    ///
    /// Populated by the system.
    /// Read-only.
    /// Value must be treated as opaque by clients and .
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency>
    /// +optional
    #[prost(string, optional, tag="6")]
    pub resource_version: ::core::option::Option<::prost::alloc::string::String>,
    /// A sequence number representing a specific generation of the desired state.
    /// Populated by the system. Read-only.
    /// +optional
    #[prost(int64, optional, tag="7")]
    pub generation: ::core::option::Option<i64>,
    /// CreationTimestamp is a timestamp representing the server time when this object was
    /// created. It is not guaranteed to be set in happens-before order across separate operations.
    /// Clients may not set this value. It is represented in RFC3339 form and is in UTC.
    ///
    /// Populated by the system.
    /// Read-only.
    /// Null for lists.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="8")]
    pub creation_timestamp: ::core::option::Option<Time>,
    /// DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This
    /// field is set by the server when a graceful deletion is requested by the user, and is not
    /// directly settable by a client. The resource is expected to be deleted (no longer visible
    /// from resource lists, and not reachable by name) after the time in this field, once the
    /// finalizers list is empty. As long as the finalizers list contains items, deletion is blocked.
    /// Once the deletionTimestamp is set, this value may not be unset or be set further into the
    /// future, although it may be shortened or the resource may be deleted prior to this time.
    /// For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react
    /// by sending a graceful termination signal to the containers in the pod. After that 30 seconds,
    /// the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup,
    /// remove the pod from the API. In the presence of network partitions, this object may still
    /// exist after this timestamp, until an administrator or automated process can determine the
    /// resource is fully terminated.
    /// If not set, graceful deletion of the object has not been requested.
    ///
    /// Populated by the system when a graceful deletion is requested.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="9")]
    pub deletion_timestamp: ::core::option::Option<Time>,
    /// Number of seconds allowed for this object to gracefully terminate before
    /// it will be removed from the system. Only set when deletionTimestamp is also set.
    /// May only be shortened.
    /// Read-only.
    /// +optional
    #[prost(int64, optional, tag="10")]
    pub deletion_grace_period_seconds: ::core::option::Option<i64>,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: <http://kubernetes.io/docs/user-guide/labels>
    /// +optional
    #[prost(map="string, string", tag="11")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// More info: <http://kubernetes.io/docs/user-guide/annotations>
    /// +optional
    #[prost(map="string, string", tag="12")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// List of objects depended by this object. If ALL objects in the list have
    /// been deleted, this object will be garbage collected. If this object is managed by a controller,
    /// then an entry in this list will point to this controller, with the controller field set to true.
    /// There cannot be more than one managing controller.
    /// +optional
    /// +patchMergeKey=uid
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="13")]
    pub owner_references: ::prost::alloc::vec::Vec<OwnerReference>,
    /// Must be empty before the object is deleted from the registry. Each entry
    /// is an identifier for the responsible component that will remove the entry
    /// from the list. If the deletionTimestamp of the object is non-nil, entries
    /// in this list can only be removed.
    /// Finalizers may be processed and removed in any order.  Order is NOT enforced
    /// because it introduces significant risk of stuck finalizers.
    /// finalizers is a shared field, any actor with permission can reorder it.
    /// If the finalizer list is processed in order, then this can lead to a situation
    /// in which the component responsible for the first finalizer in the list is
    /// waiting for a signal (field value, external system, or other) produced by a
    /// component responsible for a finalizer later in the list, resulting in a deadlock.
    /// Without enforced ordering finalizers are free to order amongst themselves and
    /// are not vulnerable to ordering changes in the list.
    /// +optional
    /// +patchStrategy=merge
    #[prost(string, repeated, tag="14")]
    pub finalizers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The name of the cluster which the object belongs to.
    /// This is used to distinguish resources with same name and namespace in different clusters.
    /// This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.
    /// +optional
    #[prost(string, optional, tag="15")]
    pub cluster_name: ::core::option::Option<::prost::alloc::string::String>,
    /// ManagedFields maps workflow-id and version to the set of fields
    /// that are managed by that workflow. This is mostly for internal
    /// housekeeping, and users typically shouldn't need to set or
    /// understand this field. A workflow can be the user's name, a
    /// controller's name, or the name of a specific apply path like
    /// "ci-cd". The set of fields is always in the version that the
    /// workflow used when modifying the object.
    ///
    /// +optional
    #[prost(message, repeated, tag="17")]
    pub managed_fields: ::prost::alloc::vec::Vec<ManagedFieldsEntry>,
}
/// OwnerReference contains enough information to let you identify an owning
/// object. An owning object must be in the same namespace as the dependent, or
/// be cluster-scoped, so there is no namespace field.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerReference {
    /// API version of the referent.
    #[prost(string, optional, tag="5")]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
    /// Kind of the referent.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    #[prost(string, optional, tag="1")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the referent.
    /// More info: <http://kubernetes.io/docs/user-guide/identifiers#names>
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// UID of the referent.
    /// More info: <http://kubernetes.io/docs/user-guide/identifiers#uids>
    #[prost(string, optional, tag="4")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// If true, this reference points to the managing controller.
    /// +optional
    #[prost(bool, optional, tag="6")]
    pub controller: ::core::option::Option<bool>,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then
    /// the owner cannot be deleted from the key-value store until this
    /// reference is removed.
    /// Defaults to false.
    /// To set this field, a user needs "delete" permission of the owner,
    /// otherwise 422 (Unprocessable Entity) will be returned.
    /// +optional
    #[prost(bool, optional, tag="7")]
    pub block_owner_deletion: ::core::option::Option<bool>,
}
/// PartialObjectMetadata is a generic representation of any object with ObjectMeta. It allows clients
/// to get access to a particular ObjectMeta schema without knowing the details of the version.
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialObjectMetadata {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<ObjectMeta>,
}
/// PartialObjectMetadataList contains a list of objects containing only their metadata
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialObjectMetadataList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<ListMeta>,
    /// items contains each of the included items.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<PartialObjectMetadata>,
}
/// Patch is provided to give a concrete name and type to the Kubernetes PATCH request body.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Patch {
}
/// PatchOptions may be provided when patching an API object.
/// PatchOptions is meant to be a superset of UpdateOptions.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchOptions {
    /// When present, indicates that modifications should not be
    /// persisted. An invalid or unrecognized dryRun directive will
    /// result in an error response and no further processing of the
    /// request. Valid values are:
    /// - All: all dry run stages will be processed
    /// +optional
    #[prost(string, repeated, tag="1")]
    pub dry_run: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Force is going to "force" Apply requests. It means user will
    /// re-acquire conflicting fields owned by other people. Force
    /// flag must be unset for non-apply patch requests.
    /// +optional
    #[prost(bool, optional, tag="2")]
    pub force: ::core::option::Option<bool>,
    /// fieldManager is a name associated with the actor or entity
    /// that is making these changes. The value must be less than or
    /// 128 characters long, and only contain printable characters,
    /// as defined by <https://golang.org/pkg/unicode/#IsPrint.> This
    /// field is required for apply requests
    /// (application/apply-patch) but optional for non-apply patch
    /// types (JsonPatch, MergePatch, StrategicMergePatch).
    /// +optional
    #[prost(string, optional, tag="3")]
    pub field_manager: ::core::option::Option<::prost::alloc::string::String>,
}
/// Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Preconditions {
    /// Specifies the target UID.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies the target ResourceVersion
    /// +optional
    #[prost(string, optional, tag="2")]
    pub resource_version: ::core::option::Option<::prost::alloc::string::String>,
}
/// RootPaths lists the paths available at root.
/// For example: "/healthz", "/apis".
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RootPaths {
    /// paths are the paths available at root.
    #[prost(string, repeated, tag="1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerAddressByClientCidr {
    /// The CIDR with which clients can match their IP to figure out the server address that they should use.
    #[prost(string, optional, tag="1")]
    pub client_cidr: ::core::option::Option<::prost::alloc::string::String>,
    /// Address of this server, suitable for a client that matches the above CIDR.
    /// This can be a hostname, hostname:port, IP or IP:port.
    #[prost(string, optional, tag="2")]
    pub server_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Status is a return value for calls that don't return other objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<ListMeta>,
    /// Status of the operation.
    /// One of: "Success" or "Failure".
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// A human-readable description of the status of this operation.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// A machine-readable description of why this operation is in the
    /// "Failure" status. If this value is empty there
    /// is no information available. A Reason clarifies an HTTP status
    /// code but does not override it.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Extended data associated with the reason.  Each reason may define its
    /// own extended details. This field is optional and the data returned
    /// is not guaranteed to conform to any schema except that defined by
    /// the reason type.
    /// +optional
    #[prost(message, optional, tag="5")]
    pub details: ::core::option::Option<StatusDetails>,
    /// Suggested HTTP return code for this status, 0 if not set.
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub code: ::core::option::Option<i32>,
}
/// StatusCause provides more information about an api.Status failure, including
/// cases when multiple errors are encountered.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusCause {
    /// A machine-readable description of the cause of the error. If this value is
    /// empty there is no information available.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human-readable description of the cause of the error.  This field may be
    /// presented as-is to a reader.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// The field of the resource that has caused this error, as named by its JSON
    /// serialization. May include dot and postfix notation for nested attributes.
    /// Arrays are zero-indexed.  Fields may appear more than once in an array of
    /// causes due to fields having multiple errors.
    /// Optional.
    ///
    /// Examples:
    ///   "name" - the field "name" on the current resource
    ///   "items\[0\].name" - the field "name" on the first array entry in "items"
    /// +optional
    #[prost(string, optional, tag="3")]
    pub field: ::core::option::Option<::prost::alloc::string::String>,
}
/// StatusDetails is a set of additional properties that MAY be set by the
/// server to provide additional information about a response. The Reason
/// field of a Status object defines what attributes will be set. Clients
/// must ignore fields that do not match the defined type of each attribute,
/// and should assume that any attribute may be empty, invalid, or under
/// defined.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusDetails {
    /// The name attribute of the resource associated with the status StatusReason
    /// (when there is a single name which can be described).
    /// +optional
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The group attribute of the resource associated with the status StatusReason.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// The kind attribute of the resource associated with the status StatusReason.
    /// On some operations may differ from the requested resource Kind.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(string, optional, tag="3")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// UID of the resource.
    /// (when there is a single resource which can be described).
    /// More info: <http://kubernetes.io/docs/user-guide/identifiers#uids>
    /// +optional
    #[prost(string, optional, tag="6")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// The Causes array includes more details associated with the StatusReason
    /// failure. Not all StatusReasons may provide detailed causes.
    /// +optional
    #[prost(message, repeated, tag="4")]
    pub causes: ::prost::alloc::vec::Vec<StatusCause>,
    /// If specified, the time in seconds before the operation should be retried. Some errors may indicate
    /// the client must take an alternate action - for those errors this field may indicate how long to wait
    /// before taking the alternate action.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub retry_after_seconds: ::core::option::Option<i32>,
}
/// TableOptions are used when a Table is requested by the caller.
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableOptions {
    /// includeObject decides whether to include each object along with its columnar information.
    /// Specifying "None" will return no object, specifying "Object" will return the full object contents, and
    /// specifying "Metadata" (the default) will return the object's metadata in the PartialObjectMetadata kind
    /// in version v1beta1 of the meta.k8s.io API group.
    #[prost(string, optional, tag="1")]
    pub include_object: ::core::option::Option<::prost::alloc::string::String>,
}
/// Time is a wrapper around time.Time which supports correct
/// marshaling to YAML and JSON.  Wrappers are provided for many
/// of the factory methods that the time package offers.
///
/// +protobuf.options.marshal=false
/// +protobuf.as=Timestamp
/// +protobuf.options.(gogoproto.goproto_stringer)=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Time {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, optional, tag="1")]
    pub seconds: ::core::option::Option<i64>,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive. This field may be limited in precision depending on context.
    #[prost(int32, optional, tag="2")]
    pub nanos: ::core::option::Option<i32>,
}
/// Timestamp is a struct that is equivalent to Time, but intended for
/// protobuf marshalling/unmarshalling. It is generated into a serialization
/// that matches Time. Do not use in Go structs.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, optional, tag="1")]
    pub seconds: ::core::option::Option<i64>,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive. This field may be limited in precision depending on context.
    #[prost(int32, optional, tag="2")]
    pub nanos: ::core::option::Option<i32>,
}
/// TypeMeta describes an individual object in an API response or request
/// with strings representing the type of the object and its API schema version.
/// Structures that are versioned or persisted should inline TypeMeta.
///
/// +k8s:deepcopy-gen=false
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeMeta {
    /// Kind is a string value representing the REST resource this object represents.
    /// Servers may infer this from the endpoint the client submits requests to.
    /// Cannot be updated.
    /// In CamelCase.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(string, optional, tag="1")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// APIVersion defines the versioned schema of this representation of an object.
    /// Servers should convert recognized schemas to the latest internal value, and
    /// may reject unrecognized values.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources>
    /// +optional
    #[prost(string, optional, tag="2")]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
}
/// UpdateOptions may be provided when updating an API object.
/// All fields in UpdateOptions should also be present in PatchOptions.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOptions {
    /// When present, indicates that modifications should not be
    /// persisted. An invalid or unrecognized dryRun directive will
    /// result in an error response and no further processing of the
    /// request. Valid values are:
    /// - All: all dry run stages will be processed
    /// +optional
    #[prost(string, repeated, tag="1")]
    pub dry_run: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// fieldManager is a name associated with the actor or entity
    /// that is making these changes. The value must be less than or
    /// 128 characters long, and only contain printable characters,
    /// as defined by <https://golang.org/pkg/unicode/#IsPrint.>
    /// +optional
    #[prost(string, optional, tag="2")]
    pub field_manager: ::core::option::Option<::prost::alloc::string::String>,
}
/// Verbs masks the value so protobuf can generate
///
/// +protobuf.nullable=true
/// +protobuf.options.(gogoproto.goproto_stringer)=false
///
/// items, if empty, will result in an empty slice
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Verbs {
    #[prost(string, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Event represents a single event to a watched resource.
///
/// +protobuf=true
/// +k8s:deepcopy-gen=true
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchEvent {
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Object is:
    ///  * If Type is Added or Modified: the new state of the object.
    ///  * If Type is Deleted: the state of the object immediately before deletion.
    ///  * If Type is Error: *Status is recommended; other types may make sense
    ///    depending on context.
    #[prost(message, optional, tag="2")]
    pub object: ::core::option::Option<super::super::super::runtime::RawExtension>,
}
