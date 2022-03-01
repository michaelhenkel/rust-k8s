/// Represents a Persistent Disk resource in AWS.
///
/// An AWS EBS disk must exist before mounting to a container. The disk
/// must also be in the same AWS zone as the kubelet. An AWS EBS disk
/// can only be mounted as read/write once. AWS EBS volumes support
/// ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsElasticBlockStoreVolumeSource {
    /// Unique ID of the persistent disk resource in AWS (Amazon EBS volume).
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub volume_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type of the volume that you want to mount.
    /// Tip: Ensure that the filesystem type is supported by the host operating system.
    /// Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore>
    /// TODO: how do we prevent errors in the filesystem from compromising the machine
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// The partition in the volume that you want to mount.
    /// If omitted, the default is to mount by volume name.
    /// Examples: For volume /dev/sda1, you specify the partition as "1".
    /// Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
    /// +optional
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub partition: ::core::option::Option<i32>,
    /// Specify "true" to force and set the ReadOnly property in VolumeMounts to "true".
    /// If omitted, the default is "false".
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore>
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Affinity is a group of affinity scheduling rules.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Affinity {
    /// Describes node affinity scheduling rules for the pod.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub node_affinity: ::core::option::Option<NodeAffinity>,
    /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub pod_affinity: ::core::option::Option<PodAffinity>,
    /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub pod_anti_affinity: ::core::option::Option<PodAntiAffinity>,
}
/// AttachedVolume describes a volume attached to a node
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachedVolume {
    /// Name of the attached volume
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// DevicePath represents the device path where the volume should be available
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub device_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// AvoidPods describes pods that should avoid this node. This is the value for a
/// Node annotation with key scheduler.alpha.kubernetes.io/preferAvoidPods and
/// will eventually become a field of NodeStatus.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvoidPods {
    /// Bounded-sized list of signatures of pods that should avoid this node, sorted
    /// in timestamp order from oldest to newest. Size of the slice is unspecified.
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub prefer_avoid_pods: ::prost::alloc::vec::Vec<PreferAvoidPodsEntry>,
}
/// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureDiskVolumeSource {
    /// The Name of the data disk in the blob storage
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub disk_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The URI the data disk in the blob storage
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub disk_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// Host Caching mode: None, Read Only, Read Write.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub caching_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="5")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
}
/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureFilePersistentVolumeSource {
    /// the name of secret that contains Azure Storage Account Name and Key
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub secret_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Share Name
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub share_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// the namespace of the secret that contains Azure Storage Account Name and Key
    /// default is the same as the Pod
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub secret_namespace: ::core::option::Option<::prost::alloc::string::String>,
}
/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureFileVolumeSource {
    /// the name of secret that contains Azure Storage Account Name and Key
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub secret_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Share Name
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub share_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Binding ties one object to another; for example, a pod is bound to a node by a scheduler.
/// Deprecated in 1.7, please use the bindings subresource of pods instead.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Binding {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// The target object that you want to bind to the standard object.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub target: ::core::option::Option<ObjectReference>,
}
/// Represents storage that is managed by an external CSI volume driver (Beta feature)
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiPersistentVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    /// Required.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub driver: ::core::option::Option<::prost::alloc::string::String>,
    /// VolumeHandle is the unique volume name returned by the CSI volume
    /// plugin’s CreateVolume to refer to the volume on all subsequent calls.
    /// Required.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub volume_handle: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: The value to pass to ControllerPublishVolumeRequest.
    /// Defaults to false (read/write).
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs".
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Attributes of the volume to publish.
    /// +optional
    #[prost(map="string, string", tag="5")]
    #[serde(default)]
    pub volume_attributes: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// ControllerPublishSecretRef is a reference to the secret object containing
    /// sensitive information to pass to the CSI driver to complete the CSI
    /// ControllerPublishVolume and ControllerUnpublishVolume calls.
    /// This field is optional, and may be empty if no secret is required. If the
    /// secret object contains more than one secret, all secrets are passed.
    /// +optional
    #[prost(message, optional, tag="6")]
    #[serde(default)]
    pub controller_publish_secret_ref: ::core::option::Option<SecretReference>,
    /// NodeStageSecretRef is a reference to the secret object containing sensitive
    /// information to pass to the CSI driver to complete the CSI NodeStageVolume
    /// and NodeStageVolume and NodeUnstageVolume calls.
    /// This field is optional, and may be empty if no secret is required. If the
    /// secret object contains more than one secret, all secrets are passed.
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub node_stage_secret_ref: ::core::option::Option<SecretReference>,
    /// NodePublishSecretRef is a reference to the secret object containing
    /// sensitive information to pass to the CSI driver to complete the CSI
    /// NodePublishVolume and NodeUnpublishVolume calls.
    /// This field is optional, and may be empty if no secret is required. If the
    /// secret object contains more than one secret, all secrets are passed.
    /// +optional
    #[prost(message, optional, tag="8")]
    #[serde(default)]
    pub node_publish_secret_ref: ::core::option::Option<SecretReference>,
    /// ControllerExpandSecretRef is a reference to the secret object containing
    /// sensitive information to pass to the CSI driver to complete the CSI
    /// ControllerExpandVolume call.
    /// This is an alpha field and requires enabling ExpandCSIVolumes feature gate.
    /// This field is optional, and may be empty if no secret is required. If the
    /// secret object contains more than one secret, all secrets are passed.
    /// +optional
    #[prost(message, optional, tag="9")]
    #[serde(default)]
    pub controller_expand_secret_ref: ::core::option::Option<SecretReference>,
}
/// Represents a source location of a volume to mount, managed by an external CSI driver
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiVolumeSource {
    /// Driver is the name of the CSI driver that handles this volume.
    /// Consult with your admin for the correct name as registered in the cluster.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub driver: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies a read-only configuration for the volume.
    /// Defaults to false (read/write).
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Filesystem type to mount. Ex. "ext4", "xfs", "ntfs".
    /// If not provided, the empty value is passed to the associated CSI driver
    /// which will determine the default filesystem to apply.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// VolumeAttributes stores driver-specific properties that are passed to the CSI
    /// driver. Consult your driver's documentation for supported values.
    /// +optional
    #[prost(map="string, string", tag="4")]
    #[serde(default)]
    pub volume_attributes: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// NodePublishSecretRef is a reference to the secret object containing
    /// sensitive information to pass to the CSI driver to complete the CSI
    /// NodePublishVolume and NodeUnpublishVolume calls.
    /// This field is optional, and  may be empty if no secret is required. If the
    /// secret object contains more than one secret, all secret references are passed.
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub node_publish_secret_ref: ::core::option::Option<LocalObjectReference>,
}
/// Adds and removes POSIX capabilities from running containers.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Capabilities {
    /// Added capabilities
    /// +optional
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Removed capabilities
    /// +optional
    #[prost(string, repeated, tag="2")]
    #[serde(default)]
    pub drop: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a Ceph Filesystem mount that lasts the lifetime of a pod
/// Cephfs volumes do not support ownership management or SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CephFsPersistentVolumeSource {
    /// Required: Monitors is a collection of Ceph monitors
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub monitors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: User is the rados user name, default is admin
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub secret_file: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: SecretRef is reference to the authentication secret for User, default is empty.
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<SecretReference>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    /// +optional
    #[prost(bool, optional, tag="6")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Represents a Ceph Filesystem mount that lasts the lifetime of a pod
/// Cephfs volumes do not support ownership management or SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CephFsVolumeSource {
    /// Required: Monitors is a collection of Ceph monitors
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub monitors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: User is the rados user name, default is admin
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub secret_file: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: SecretRef is reference to the authentication secret for User, default is empty.
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<LocalObjectReference>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// More info: <https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it>
    /// +optional
    #[prost(bool, optional, tag="6")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Represents a cinder volume resource in Openstack.
/// A Cinder volume must exist before mounting to a container.
/// The volume must also be in the same region as the kubelet.
/// Cinder volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CinderPersistentVolumeSource {
    /// volume id used to identify the volume in cinder.
    /// More info: <https://examples.k8s.io/mysql-cinder-pd/README.md>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub volume_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: <https://examples.k8s.io/mysql-cinder-pd/README.md>
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// More info: <https://examples.k8s.io/mysql-cinder-pd/README.md>
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Optional: points to a secret object containing parameters used to connect
    /// to OpenStack.
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<SecretReference>,
}
/// Represents a cinder volume resource in Openstack.
/// A Cinder volume must exist before mounting to a container.
/// The volume must also be in the same region as the kubelet.
/// Cinder volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CinderVolumeSource {
    /// volume id used to identify the volume in cinder.
    /// More info: <https://examples.k8s.io/mysql-cinder-pd/README.md>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub volume_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: <https://examples.k8s.io/mysql-cinder-pd/README.md>
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// More info: <https://examples.k8s.io/mysql-cinder-pd/README.md>
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Optional: points to a secret object containing parameters used to connect
    /// to OpenStack.
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<LocalObjectReference>,
}
/// ClientIPConfig represents the configurations of Client IP based session affinity.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientIpConfig {
    /// timeoutSeconds specifies the seconds of ClientIP type session sticky time.
    /// The value must be >0 && <=86400(for 1 day) if ServiceAffinity == "ClientIP".
    /// Default value is 10800(for 3 hours).
    /// +optional
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub timeout_seconds: ::core::option::Option<i32>,
}
/// Information about the condition of a component.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentCondition {
    /// Type of condition for a component.
    /// Valid value: "Healthy"
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition for a component.
    /// Valid values for "Healthy": "True", "False", or "Unknown".
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Message about the condition for a component.
    /// For example, information about a health check.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// Condition error code for a component.
    /// For example, a health check error code.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// ComponentStatus (and ComponentStatusList) holds the cluster validation info.
/// Deprecated: This API is deprecated in v1.19+
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentStatus {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// List of component conditions observed
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub conditions: ::prost::alloc::vec::Vec<ComponentCondition>,
}
/// Status of all the conditions for the component as a list of ComponentStatus objects.
/// Deprecated: This API is deprecated in v1.19+
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentStatusList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of ComponentStatus objects.
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<ComponentStatus>,
}
/// ConfigMap holds configuration data for pods to consume.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMap {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Immutable, if set to true, ensures that data stored in the ConfigMap cannot
    /// be updated (only object metadata can be modified).
    /// If not set to true, the field can be modified at any time.
    /// Defaulted to nil.
    /// This is a beta field enabled by ImmutableEphemeralVolumes feature gate.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub immutable: ::core::option::Option<bool>,
    /// Data contains the configuration data.
    /// Each key must consist of alphanumeric characters, '-', '_' or '.'.
    /// Values with non-UTF-8 byte sequences must use the BinaryData field.
    /// The keys stored in Data must not overlap with the keys in
    /// the BinaryData field, this is enforced during validation process.
    /// +optional
    #[prost(map="string, string", tag="2")]
    #[serde(default)]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// BinaryData contains the binary data.
    /// Each key must consist of alphanumeric characters, '-', '_' or '.'.
    /// BinaryData can contain byte sequences that are not in the UTF-8 range.
    /// The keys stored in BinaryData must not overlap with the ones in
    /// the Data field, this is enforced during validation process.
    /// Using this field will require 1.10+ apiserver and
    /// kubelet.
    /// +optional
    #[prost(map="string, bytes", tag="3")]
    #[serde(default)]
    pub binary_data: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
/// ConfigMapEnvSource selects a ConfigMap to populate the environment
/// variables with.
///
/// The contents of the target ConfigMap's Data field will represent the
/// key-value pairs as environment variables.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMapEnvSource {
    /// The ConfigMap to select from.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub local_object_reference: ::core::option::Option<LocalObjectReference>,
    /// Specify whether the ConfigMap must be defined
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub optional: ::core::option::Option<bool>,
}
/// Selects a key from a ConfigMap.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMapKeySelector {
    /// The ConfigMap to select from.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub local_object_reference: ::core::option::Option<LocalObjectReference>,
    /// The key to select.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Specify whether the ConfigMap or its key must be defined
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub optional: ::core::option::Option<bool>,
}
/// ConfigMapList is a resource containing a list of ConfigMap objects.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMapList {
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of ConfigMaps.
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<ConfigMap>,
}
/// ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMapNodeConfigSource {
    /// Namespace is the metadata.namespace of the referenced ConfigMap.
    /// This field is required in all cases.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the metadata.name of the referenced ConfigMap.
    /// This field is required in all cases.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// UID is the metadata.UID of the referenced ConfigMap.
    /// This field is forbidden in Node.Spec, and required in Node.Status.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap.
    /// This field is forbidden in Node.Spec, and required in Node.Status.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub resource_version: ::core::option::Option<::prost::alloc::string::String>,
    /// KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure
    /// This field is required in all cases.
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub kubelet_config_key: ::core::option::Option<::prost::alloc::string::String>,
}
/// Adapts a ConfigMap into a projected volume.
///
/// The contents of the target ConfigMap's Data field will be presented in a
/// projected volume as files using the keys in the Data field as the file names,
/// unless the items element is populated with specific mappings of keys to paths.
/// Note that this is identical to a configmap volume source without the default
/// mode.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMapProjection {
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub local_object_reference: ::core::option::Option<LocalObjectReference>,
    /// If unspecified, each key-value pair in the Data field of the referenced
    /// ConfigMap will be projected into the volume as a file whose name is the
    /// key and content is the value. If specified, the listed keys will be
    /// projected into the specified paths, and unlisted keys will not be
    /// present. If a key is specified which is not present in the ConfigMap,
    /// the volume setup will error unless it is marked optional. Paths must be
    /// relative and may not contain the '..' path or start with '..'.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<KeyToPath>,
    /// Specify whether the ConfigMap or its keys must be defined
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub optional: ::core::option::Option<bool>,
}
/// Adapts a ConfigMap into a volume.
///
/// The contents of the target ConfigMap's Data field will be presented in a
/// volume as files using the keys in the Data field as the file names, unless
/// the items element is populated with specific mappings of keys to paths.
/// ConfigMap volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigMapVolumeSource {
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub local_object_reference: ::core::option::Option<LocalObjectReference>,
    /// If unspecified, each key-value pair in the Data field of the referenced
    /// ConfigMap will be projected into the volume as a file whose name is the
    /// key and content is the value. If specified, the listed keys will be
    /// projected into the specified paths, and unlisted keys will not be
    /// present. If a key is specified which is not present in the ConfigMap,
    /// the volume setup will error unless it is marked optional. Paths must be
    /// relative and may not contain the '..' path or start with '..'.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<KeyToPath>,
    /// Optional: mode bits used to set permissions on created files by default.
    /// Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
    /// YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
    /// Defaults to 0644.
    /// Directories within the path are not affected by this setting.
    /// This might be in conflict with other options that affect the file
    /// mode, like fsGroup, and the result can be other mode bits set.
    /// +optional
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub default_mode: ::core::option::Option<i32>,
    /// Specify whether the ConfigMap or its keys must be defined
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub optional: ::core::option::Option<bool>,
}
/// A single application container that you want to run within a pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// Name of the container specified as a DNS_LABEL.
    /// Each container in a pod must have a unique name (DNS_LABEL).
    /// Cannot be updated.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Docker image name.
    /// More info: <https://kubernetes.io/docs/concepts/containers/images>
    /// This field is optional to allow higher level config management to default or override
    /// container images in workload controllers like Deployments and StatefulSets.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
    /// Entrypoint array. Not executed within a shell.
    /// The docker image's ENTRYPOINT is used if this is not provided.
    /// Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
    /// cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax
    /// can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
    /// regardless of whether the variable exists or not.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell>
    /// +optional
    #[prost(string, repeated, tag="3")]
    #[serde(default)]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Arguments to the entrypoint.
    /// The docker image's CMD is used if this is not provided.
    /// Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
    /// cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax
    /// can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
    /// regardless of whether the variable exists or not.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell>
    /// +optional
    #[prost(string, repeated, tag="4")]
    #[serde(default)]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Container's working directory.
    /// If not specified, the container runtime's default will be used, which
    /// might be configured in the container image.
    /// Cannot be updated.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub working_dir: ::core::option::Option<::prost::alloc::string::String>,
    /// List of ports to expose from the container. Exposing a port here gives
    /// the system additional information about the network connections a
    /// container uses, but is primarily informational. Not specifying a port here
    /// DOES NOT prevent that port from being exposed. Any port which is
    /// listening on the default "0.0.0.0" address inside a container will be
    /// accessible from the network.
    /// Cannot be updated.
    /// +optional
    /// +patchMergeKey=containerPort
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=containerPort
    /// +listMapKey=protocol
    #[prost(message, repeated, tag="6")]
    #[serde(default)]
    pub ports: ::prost::alloc::vec::Vec<ContainerPort>,
    /// List of sources to populate environment variables in the container.
    /// The keys defined within a source must be a C_IDENTIFIER. All invalid keys
    /// will be reported as an event when the container is starting. When a key exists in multiple
    /// sources, the value associated with the last source will take precedence.
    /// Values defined by an Env with a duplicate key will take precedence.
    /// Cannot be updated.
    /// +optional
    #[prost(message, repeated, tag="19")]
    #[serde(default)]
    pub env_from: ::prost::alloc::vec::Vec<EnvFromSource>,
    /// List of environment variables to set in the container.
    /// Cannot be updated.
    /// +optional
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="7")]
    #[serde(default)]
    pub env: ::prost::alloc::vec::Vec<EnvVar>,
    /// Compute Resources required by this container.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/>
    /// +optional
    #[prost(message, optional, tag="8")]
    #[serde(default)]
    pub resources: ::core::option::Option<ResourceRequirements>,
    /// Pod volumes to mount into the container's filesystem.
    /// Cannot be updated.
    /// +optional
    /// +patchMergeKey=mountPath
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="9")]
    #[serde(default)]
    pub volume_mounts: ::prost::alloc::vec::Vec<VolumeMount>,
    /// volumeDevices is the list of block devices to be used by the container.
    /// +patchMergeKey=devicePath
    /// +patchStrategy=merge
    /// +optional
    #[prost(message, repeated, tag="21")]
    #[serde(default)]
    pub volume_devices: ::prost::alloc::vec::Vec<VolumeDevice>,
    /// Periodic probe of container liveness.
    /// Container will be restarted if the probe fails.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes>
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub liveness_probe: ::core::option::Option<Probe>,
    /// Periodic probe of container service readiness.
    /// Container will be removed from service endpoints if the probe fails.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes>
    /// +optional
    #[prost(message, optional, tag="11")]
    #[serde(default)]
    pub readiness_probe: ::core::option::Option<Probe>,
    /// StartupProbe indicates that the Pod has successfully initialized.
    /// If specified, no other probes are executed until this completes successfully.
    /// If this probe fails, the Pod will be restarted, just as if the livenessProbe failed.
    /// This can be used to provide different probe parameters at the beginning of a Pod's lifecycle,
    /// when it might take a long time to load data or warm a cache, than during steady-state operation.
    /// This cannot be updated.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes>
    /// +optional
    #[prost(message, optional, tag="22")]
    #[serde(default)]
    pub startup_probe: ::core::option::Option<Probe>,
    /// Actions that the management system should take in response to container lifecycle events.
    /// Cannot be updated.
    /// +optional
    #[prost(message, optional, tag="12")]
    #[serde(default)]
    pub lifecycle: ::core::option::Option<Lifecycle>,
    /// Optional: Path at which the file to which the container's termination message
    /// will be written is mounted into the container's filesystem.
    /// Message written is intended to be brief final status, such as an assertion failure message.
    /// Will be truncated by the node if greater than 4096 bytes. The total message length across
    /// all containers will be limited to 12kb.
    /// Defaults to /dev/termination-log.
    /// Cannot be updated.
    /// +optional
    #[prost(string, optional, tag="13")]
    #[serde(default)]
    pub termination_message_path: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicate how the termination message should be populated. File will use the contents of
    /// terminationMessagePath to populate the container status message on both success and failure.
    /// FallbackToLogsOnError will use the last chunk of container log output if the termination
    /// message file is empty and the container exited with an error.
    /// The log output is limited to 2048 bytes or 80 lines, whichever is smaller.
    /// Defaults to File.
    /// Cannot be updated.
    /// +optional
    #[prost(string, optional, tag="20")]
    #[serde(default)]
    pub termination_message_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// Image pull policy.
    /// One of Always, Never, IfNotPresent.
    /// Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/concepts/containers/images#updating-images>
    /// +optional
    #[prost(string, optional, tag="14")]
    #[serde(default)]
    pub image_pull_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// Security options the pod should run with.
    /// More info: <https://kubernetes.io/docs/concepts/policy/security-context/>
    /// More info: <https://kubernetes.io/docs/tasks/configure-pod-container/security-context/>
    /// +optional
    #[prost(message, optional, tag="15")]
    #[serde(default)]
    pub security_context: ::core::option::Option<SecurityContext>,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If this
    /// is not set, reads from stdin in the container will always result in EOF.
    /// Default is false.
    /// +optional
    #[prost(bool, optional, tag="16")]
    #[serde(default)]
    pub stdin: ::core::option::Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by
    /// a single attach. When stdin is true the stdin stream will remain open across multiple attach
    /// sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the
    /// first client attaches to stdin, and then remains open and accepts data until the client disconnects,
    /// at which time stdin is closed and remains closed until the container is restarted. If this
    /// flag is false, a container processes that reads from stdin will never receive an EOF.
    /// Default is false
    /// +optional
    #[prost(bool, optional, tag="17")]
    #[serde(default)]
    pub stdin_once: ::core::option::Option<bool>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true.
    /// Default is false.
    /// +optional
    #[prost(bool, optional, tag="18")]
    #[serde(default)]
    pub tty: ::core::option::Option<bool>,
}
/// Describe a container image
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerImage {
    /// Names by which this image is known.
    /// e.g. ["k8s.gcr.io/hyperkube:v1.0.7", "dockerhub.io/google_containers/hyperkube:v1.0.7"]
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The size of the image in bytes.
    /// +optional
    #[prost(int64, optional, tag="2")]
    #[serde(default)]
    pub size_bytes: ::core::option::Option<i64>,
}
/// ContainerPort represents a network port in a single container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerPort {
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each
    /// named port in a pod must have a unique name. Name for the port that can be
    /// referred to by services.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Number of port to expose on the host.
    /// If specified, this must be a valid port number, 0 < x < 65536.
    /// If HostNetwork is specified, this must match ContainerPort.
    /// Most containers do not need this.
    /// +optional
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub host_port: ::core::option::Option<i32>,
    /// Number of port to expose on the pod's IP address.
    /// This must be a valid port number, 0 < x < 65536.
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub container_port: ::core::option::Option<i32>,
    /// Protocol for port. Must be UDP, TCP, or SCTP.
    /// Defaults to "TCP".
    /// +optional
    /// +default="TCP"
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// What host IP to bind the external port to.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub host_ip: ::core::option::Option<::prost::alloc::string::String>,
}
/// ContainerState holds a possible state of container.
/// Only one of its members may be specified.
/// If none of them is specified, the default one is ContainerStateWaiting.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerState {
    /// Details about a waiting container
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub waiting: ::core::option::Option<ContainerStateWaiting>,
    /// Details about a running container
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub running: ::core::option::Option<ContainerStateRunning>,
    /// Details about a terminated container
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub terminated: ::core::option::Option<ContainerStateTerminated>,
}
/// ContainerStateRunning is a running state of a container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerStateRunning {
    /// Time at which the container was last (re-)started
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub started_at: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
}
/// ContainerStateTerminated is a terminated state of a container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerStateTerminated {
    /// Exit status from the last termination of the container
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub exit_code: ::core::option::Option<i32>,
    /// Signal from the last termination of the container
    /// +optional
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub signal: ::core::option::Option<i32>,
    /// (brief) reason from the last termination of the container
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Message regarding the last termination of the container
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// Time at which previous execution of the container started
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub started_at: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Time at which the container last terminated
    /// +optional
    #[prost(message, optional, tag="6")]
    #[serde(default)]
    pub finished_at: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Container's ID in the format 'docker://<container_id>'
    /// +optional
    #[prost(string, optional, tag="7")]
    #[serde(default)]
    pub container_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// ContainerStateWaiting is a waiting state of a container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerStateWaiting {
    /// (brief) reason the container is not yet running.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Message regarding why the container is not yet running.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// ContainerStatus contains details for the current status of this container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerStatus {
    /// This must be a DNS_LABEL. Each container in a pod must have a unique name.
    /// Cannot be updated.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Details about the container's current condition.
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub state: ::core::option::Option<ContainerState>,
    /// Details about the container's last termination condition.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub last_state: ::core::option::Option<ContainerState>,
    /// Specifies whether the container has passed its readiness probe.
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub ready: ::core::option::Option<bool>,
    /// The number of times the container has been restarted, currently based on
    /// the number of dead containers that have not yet been removed.
    /// Note that this is calculated from dead containers. But those containers are subject to
    /// garbage collection. This value will get capped at 5 by GC.
    #[prost(int32, optional, tag="5")]
    #[serde(default)]
    pub restart_count: ::core::option::Option<i32>,
    /// The image the container is running.
    /// More info: <https://kubernetes.io/docs/concepts/containers/images>
    /// TODO(dchen1107): Which image the container is running with?
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
    /// ImageID of the container's image.
    #[prost(string, optional, tag="7")]
    #[serde(default)]
    pub image_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Container's ID in the format 'docker://<container_id>'.
    /// +optional
    #[prost(string, optional, tag="8")]
    #[serde(default)]
    pub container_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies whether the container has passed its startup probe.
    /// Initialized as false, becomes true after startupProbe is considered successful.
    /// Resets to false when the container is restarted, or if kubelet loses state temporarily.
    /// Is always true when no startupProbe is defined.
    /// +optional
    #[prost(bool, optional, tag="9")]
    #[serde(default)]
    pub started: ::core::option::Option<bool>,
}
/// DaemonEndpoint contains information about a single Daemon endpoint.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonEndpoint {
    /// Port number of the given endpoint.
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub port: ::core::option::Option<i32>,
}
/// Represents downward API info for projecting into a projected volume.
/// Note that this is identical to a downwardAPI volume source without the default
/// mode.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownwardApiProjection {
    /// Items is a list of DownwardAPIVolume file
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<DownwardApiVolumeFile>,
}
/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownwardApiVolumeFile {
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub field_ref: ::core::option::Option<ObjectFieldSelector>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub resource_field_ref: ::core::option::Option<ResourceFieldSelector>,
    /// Optional: mode bits used to set permissions on this file, must be an octal value
    /// between 0000 and 0777 or a decimal value between 0 and 511.
    /// YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
    /// If not specified, the volume defaultMode will be used.
    /// This might be in conflict with other options that affect the file
    /// mode, like fsGroup, and the result can be other mode bits set.
    /// +optional
    #[prost(int32, optional, tag="4")]
    #[serde(default)]
    pub mode: ::core::option::Option<i32>,
}
/// DownwardAPIVolumeSource represents a volume containing downward API info.
/// Downward API volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownwardApiVolumeSource {
    /// Items is a list of downward API volume file
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<DownwardApiVolumeFile>,
    /// Optional: mode bits to use on created files by default. Must be a
    /// Optional: mode bits used to set permissions on created files by default.
    /// Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
    /// YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
    /// Defaults to 0644.
    /// Directories within the path are not affected by this setting.
    /// This might be in conflict with other options that affect the file
    /// mode, like fsGroup, and the result can be other mode bits set.
    /// +optional
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub default_mode: ::core::option::Option<i32>,
}
/// Represents an empty directory for a pod.
/// Empty directory volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyDirVolumeSource {
    /// What type of storage medium should back this directory.
    /// The default is "" which means to use the node's default medium.
    /// Must be an empty string (default) or Memory.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#emptydir>
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub medium: ::core::option::Option<::prost::alloc::string::String>,
    /// Total amount of local storage required for this EmptyDir volume.
    /// The size limit is also applicable for memory medium.
    /// The maximum usage on memory medium EmptyDir would be the minimum value between
    /// the SizeLimit specified here and the sum of memory limits of all containers in a pod.
    /// The default is nil which means that the limit is undefined.
    /// More info: <http://kubernetes.io/docs/user-guide/volumes#emptydir>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub size_limit: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// EndpointAddress is a tuple that describes single IP address.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointAddress {
    /// The IP of this endpoint.
    /// May not be loopback (127.0.0.0/8), link-local (169.254.0.0/16),
    /// or link-local multicast ((224.0.0.0/24).
    /// IPv6 is also accepted but not fully supported on all platforms. Also, certain
    /// kubernetes components, like kube-proxy, are not IPv6 ready.
    /// TODO: This should allow hostname or IP, See #4447.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    /// The Hostname of this endpoint
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub node_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Reference to object providing the endpoint.
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub target_ref: ::core::option::Option<ObjectReference>,
}
/// EndpointPort is a tuple that describes a single port.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointPort {
    /// The name of this port.  This must match the 'name' field in the
    /// corresponding ServicePort.
    /// Must be a DNS_LABEL.
    /// Optional only if one port is defined.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The port number of the endpoint.
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub port: ::core::option::Option<i32>,
    /// The IP protocol for this port.
    /// Must be UDP, TCP, or SCTP.
    /// Default is TCP.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// The application protocol for this port.
    /// This field follows standard Kubernetes label syntax.
    /// Un-prefixed names are reserved for IANA standard service names (as per
    /// RFC-6335 and <http://www.iana.org/assignments/service-names>).
    /// Non-standard protocols should use prefixed names such as
    /// mycompany.com/my-custom-protocol.
    /// This is a beta field that is guarded by the ServiceAppProtocol feature
    /// gate and enabled by default.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub app_protocol: ::core::option::Option<::prost::alloc::string::String>,
}
/// EndpointSubset is a group of addresses with a common set of ports. The
/// expanded set of endpoints is the Cartesian product of Addresses x Ports.
/// For example, given:
///   {
///     Addresses: [{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}],
///     Ports:     [{"name": "a", "port": 8675}, {"name": "b", "port": 309}]
///   }
/// The resulting set of endpoints can be viewed as:
///     a: [ 10.10.1.1:8675, 10.10.2.2:8675 ],
///     b: [ 10.10.1.1:309, 10.10.2.2:309 ]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointSubset {
    /// IP addresses which offer the related ports that are marked as ready. These endpoints
    /// should be considered safe for load balancers and clients to utilize.
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub addresses: ::prost::alloc::vec::Vec<EndpointAddress>,
    /// IP addresses which offer the related ports but are not currently marked as ready
    /// because they have not yet finished starting, have recently failed a readiness check,
    /// or have recently failed a liveness check.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub not_ready_addresses: ::prost::alloc::vec::Vec<EndpointAddress>,
    /// Port numbers available on the related IP addresses.
    /// +optional
    #[prost(message, repeated, tag="3")]
    #[serde(default)]
    pub ports: ::prost::alloc::vec::Vec<EndpointPort>,
}
/// Endpoints is a collection of endpoints that implement the actual service. Example:
///   Name: "mysvc",
///   Subsets: [
///     {
///       Addresses: [{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}],
///       Ports: [{"name": "a", "port": 8675}, {"name": "b", "port": 309}]
///     },
///     {
///       Addresses: [{"ip": "10.10.3.3"}],
///       Ports: [{"name": "a", "port": 93}, {"name": "b", "port": 76}]
///     },
///  ]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoints {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// The set of all endpoints is the union of all subsets. Addresses are placed into
    /// subsets according to the IPs they share. A single address with multiple ports,
    /// some of which are ready and some of which are not (because they come from
    /// different containers) will result in the address being displayed in different
    /// subsets for the different ports. No address will appear in both Addresses and
    /// NotReadyAddresses in the same subset.
    /// Sets of addresses and ports that comprise a service.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub subsets: ::prost::alloc::vec::Vec<EndpointSubset>,
}
/// EndpointsList is a list of endpoints.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointsList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of endpoints.
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<Endpoints>,
}
/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvFromSource {
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// The ConfigMap to select from
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub config_map_ref: ::core::option::Option<ConfigMapEnvSource>,
    /// The Secret to select from
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<SecretEnvSource>,
}
/// EnvVar represents an environment variable present in a Container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvVar {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Variable references $(VAR_NAME) are expanded
    /// using the previous defined environment variables in the container and
    /// any service environment variables. If a variable cannot be resolved,
    /// the reference in the input string will be unchanged. The $(VAR_NAME)
    /// syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped
    /// references will never be expanded, regardless of whether the variable
    /// exists or not.
    /// Defaults to "".
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub value_from: ::core::option::Option<EnvVarSource>,
}
/// EnvVarSource represents a source for the value of an EnvVar.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvVarSource {
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels\['<KEY>'\]`, `metadata.annotations\['<KEY>'\]`,
    /// spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub field_ref: ::core::option::Option<ObjectFieldSelector>,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub resource_field_ref: ::core::option::Option<ResourceFieldSelector>,
    /// Selects a key of a ConfigMap.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub config_map_key_ref: ::core::option::Option<ConfigMapKeySelector>,
    /// Selects a key of a secret in the pod's namespace
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub secret_key_ref: ::core::option::Option<SecretKeySelector>,
}
/// An EphemeralContainer is a container that may be added temporarily to an existing pod for
/// user-initiated activities such as debugging. Ephemeral containers have no resource or
/// scheduling guarantees, and they will not be restarted when they exit or when a pod is
/// removed or restarted. If an ephemeral container causes a pod to exceed its resource
/// allocation, the pod may be evicted.
/// Ephemeral containers may not be added by directly updating the pod spec. They must be added
/// via the pod's ephemeralcontainers subresource, and they will appear in the pod spec
/// once added.
/// This is an alpha feature enabled by the EphemeralContainers feature flag.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EphemeralContainer {
    /// Ephemeral containers have all of the fields of Container, plus additional fields
    /// specific to ephemeral containers. Fields in common with Container are in the
    /// following inlined struct so than an EphemeralContainer may easily be converted
    /// to a Container.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub ephemeral_container_common: ::core::option::Option<EphemeralContainerCommon>,
    /// If set, the name of the container from PodSpec that this ephemeral container targets.
    /// The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container.
    /// If not set then the ephemeral container is run in whatever namespaces are shared
    /// for the pod. Note that the container runtime must support this feature.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub target_container_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// EphemeralContainerCommon is a copy of all fields in Container to be inlined in
/// EphemeralContainer. This separate type allows easy conversion from EphemeralContainer
/// to Container and allows separate documentation for the fields of EphemeralContainer.
/// When a new field is added to Container it must be added here as well.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EphemeralContainerCommon {
    /// Name of the ephemeral container specified as a DNS_LABEL.
    /// This name must be unique among all containers, init containers and ephemeral containers.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Docker image name.
    /// More info: <https://kubernetes.io/docs/concepts/containers/images>
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
    /// Entrypoint array. Not executed within a shell.
    /// The docker image's ENTRYPOINT is used if this is not provided.
    /// Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
    /// cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax
    /// can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
    /// regardless of whether the variable exists or not.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell>
    /// +optional
    #[prost(string, repeated, tag="3")]
    #[serde(default)]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Arguments to the entrypoint.
    /// The docker image's CMD is used if this is not provided.
    /// Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
    /// cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax
    /// can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
    /// regardless of whether the variable exists or not.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell>
    /// +optional
    #[prost(string, repeated, tag="4")]
    #[serde(default)]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Container's working directory.
    /// If not specified, the container runtime's default will be used, which
    /// might be configured in the container image.
    /// Cannot be updated.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub working_dir: ::core::option::Option<::prost::alloc::string::String>,
    /// Ports are not allowed for ephemeral containers.
    #[prost(message, repeated, tag="6")]
    #[serde(default)]
    pub ports: ::prost::alloc::vec::Vec<ContainerPort>,
    /// List of sources to populate environment variables in the container.
    /// The keys defined within a source must be a C_IDENTIFIER. All invalid keys
    /// will be reported as an event when the container is starting. When a key exists in multiple
    /// sources, the value associated with the last source will take precedence.
    /// Values defined by an Env with a duplicate key will take precedence.
    /// Cannot be updated.
    /// +optional
    #[prost(message, repeated, tag="19")]
    #[serde(default)]
    pub env_from: ::prost::alloc::vec::Vec<EnvFromSource>,
    /// List of environment variables to set in the container.
    /// Cannot be updated.
    /// +optional
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="7")]
    #[serde(default)]
    pub env: ::prost::alloc::vec::Vec<EnvVar>,
    /// Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources
    /// already allocated to the pod.
    /// +optional
    #[prost(message, optional, tag="8")]
    #[serde(default)]
    pub resources: ::core::option::Option<ResourceRequirements>,
    /// Pod volumes to mount into the container's filesystem.
    /// Cannot be updated.
    /// +optional
    /// +patchMergeKey=mountPath
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="9")]
    #[serde(default)]
    pub volume_mounts: ::prost::alloc::vec::Vec<VolumeMount>,
    /// volumeDevices is the list of block devices to be used by the container.
    /// +patchMergeKey=devicePath
    /// +patchStrategy=merge
    /// +optional
    #[prost(message, repeated, tag="21")]
    #[serde(default)]
    pub volume_devices: ::prost::alloc::vec::Vec<VolumeDevice>,
    /// Probes are not allowed for ephemeral containers.
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub liveness_probe: ::core::option::Option<Probe>,
    /// Probes are not allowed for ephemeral containers.
    /// +optional
    #[prost(message, optional, tag="11")]
    #[serde(default)]
    pub readiness_probe: ::core::option::Option<Probe>,
    /// Probes are not allowed for ephemeral containers.
    /// +optional
    #[prost(message, optional, tag="22")]
    #[serde(default)]
    pub startup_probe: ::core::option::Option<Probe>,
    /// Lifecycle is not allowed for ephemeral containers.
    /// +optional
    #[prost(message, optional, tag="12")]
    #[serde(default)]
    pub lifecycle: ::core::option::Option<Lifecycle>,
    /// Optional: Path at which the file to which the container's termination message
    /// will be written is mounted into the container's filesystem.
    /// Message written is intended to be brief final status, such as an assertion failure message.
    /// Will be truncated by the node if greater than 4096 bytes. The total message length across
    /// all containers will be limited to 12kb.
    /// Defaults to /dev/termination-log.
    /// Cannot be updated.
    /// +optional
    #[prost(string, optional, tag="13")]
    #[serde(default)]
    pub termination_message_path: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicate how the termination message should be populated. File will use the contents of
    /// terminationMessagePath to populate the container status message on both success and failure.
    /// FallbackToLogsOnError will use the last chunk of container log output if the termination
    /// message file is empty and the container exited with an error.
    /// The log output is limited to 2048 bytes or 80 lines, whichever is smaller.
    /// Defaults to File.
    /// Cannot be updated.
    /// +optional
    #[prost(string, optional, tag="20")]
    #[serde(default)]
    pub termination_message_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// Image pull policy.
    /// One of Always, Never, IfNotPresent.
    /// Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/concepts/containers/images#updating-images>
    /// +optional
    #[prost(string, optional, tag="14")]
    #[serde(default)]
    pub image_pull_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// SecurityContext is not allowed for ephemeral containers.
    /// +optional
    #[prost(message, optional, tag="15")]
    #[serde(default)]
    pub security_context: ::core::option::Option<SecurityContext>,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If this
    /// is not set, reads from stdin in the container will always result in EOF.
    /// Default is false.
    /// +optional
    #[prost(bool, optional, tag="16")]
    #[serde(default)]
    pub stdin: ::core::option::Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by
    /// a single attach. When stdin is true the stdin stream will remain open across multiple attach
    /// sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the
    /// first client attaches to stdin, and then remains open and accepts data until the client disconnects,
    /// at which time stdin is closed and remains closed until the container is restarted. If this
    /// flag is false, a container processes that reads from stdin will never receive an EOF.
    /// Default is false
    /// +optional
    #[prost(bool, optional, tag="17")]
    #[serde(default)]
    pub stdin_once: ::core::option::Option<bool>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true.
    /// Default is false.
    /// +optional
    #[prost(bool, optional, tag="18")]
    #[serde(default)]
    pub tty: ::core::option::Option<bool>,
}
/// A list of ephemeral containers used with the Pod ephemeralcontainers subresource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EphemeralContainers {
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// A list of ephemeral containers associated with this pod. New ephemeral containers
    /// may be appended to this list, but existing ephemeral containers may not be removed
    /// or modified.
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub ephemeral_containers: ::prost::alloc::vec::Vec<EphemeralContainer>,
}
/// Represents an ephemeral volume that is handled by a normal storage driver.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EphemeralVolumeSource {
    /// Will be used to create a stand-alone PVC to provision the volume.
    /// The pod in which this EphemeralVolumeSource is embedded will be the
    /// owner of the PVC, i.e. the PVC will be deleted together with the
    /// pod.  The name of the PVC will be `<pod name>-<volume name>` where
    /// `<volume name>` is the name from the `PodSpec.Volumes` array
    /// entry. Pod validation will reject the pod if the concatenated name
    /// is not valid for a PVC (for example, too long).
    ///
    /// An existing PVC with that name that is not owned by the pod
    /// will *not* be used for the pod to avoid using an unrelated
    /// volume by mistake. Starting the pod is then blocked until
    /// the unrelated PVC is removed. If such a pre-created PVC is
    /// meant to be used by the pod, the PVC has to updated with an
    /// owner reference to the pod once the pod exists. Normally
    /// this should not be necessary, but it may be useful when
    /// manually reconstructing a broken cluster.
    ///
    /// This field is read-only and no changes will be made by Kubernetes
    /// to the PVC after it has been created.
    ///
    /// Required, must not be nil.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub volume_claim_template: ::core::option::Option<PersistentVolumeClaimTemplate>,
    /// Specifies a read-only configuration for the volume.
    /// Defaults to false (read/write).
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Event is a report of an event somewhere in the cluster.  Events
/// have a limited retention time and triggers and messages may evolve
/// with time.  Event consumers should not rely on the timing of an event
/// with a given Reason reflecting a consistent underlying trigger, or the
/// continued existence of events with that Reason.  Events should be
/// treated as informative, best-effort, supplemental data.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// The object that this event is about.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub involved_object: ::core::option::Option<ObjectReference>,
    /// This should be a short, machine understandable string that gives the reason
    /// for the transition into the object's current status.
    /// TODO: provide exact specification for format.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human-readable description of the status of this operation.
    /// TODO: decide on maximum length.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// The component reporting this event. Should be a short machine understandable string.
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub source: ::core::option::Option<EventSource>,
    /// The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
    /// +optional
    #[prost(message, optional, tag="6")]
    #[serde(default)]
    pub first_timestamp: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The time at which the most recent occurrence of this event was recorded.
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub last_timestamp: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The number of times this event has occurred.
    /// +optional
    #[prost(int32, optional, tag="8")]
    #[serde(default)]
    pub count: ::core::option::Option<i32>,
    /// Type of this event (Normal, Warning), new types could be added in the future
    /// +optional
    #[prost(string, optional, tag="9")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Time when this Event was first observed.
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub event_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime>,
    /// Data about the Event series this event represents or nil if it's a singleton Event.
    /// +optional
    #[prost(message, optional, tag="11")]
    #[serde(default)]
    pub series: ::core::option::Option<EventSeries>,
    /// What action was taken/failed regarding to the Regarding object.
    /// +optional
    #[prost(string, optional, tag="12")]
    #[serde(default)]
    pub action: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional secondary object for more complex actions.
    /// +optional
    #[prost(message, optional, tag="13")]
    #[serde(default)]
    pub related: ::core::option::Option<ObjectReference>,
    /// Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.
    /// +optional
    #[prost(string, optional, tag="14")]
    #[serde(default)]
    pub reporting_component: ::core::option::Option<::prost::alloc::string::String>,
    /// ID of the controller instance, e.g. `kubelet-xyzf`.
    /// +optional
    #[prost(string, optional, tag="15")]
    #[serde(default)]
    pub reporting_instance: ::core::option::Option<::prost::alloc::string::String>,
}
/// EventList is a list of events.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of events
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<Event>,
}
/// EventSeries contain information on series of events, i.e. thing that was/is happening
/// continuously for some time.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSeries {
    /// Number of occurrences in this series up to the last heartbeat time
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub count: ::core::option::Option<i32>,
    /// Time of the last occurrence observed
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub last_observed_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime>,
}
/// EventSource contains information for an event.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSource {
    /// Component from which the event is generated.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub component: ::core::option::Option<::prost::alloc::string::String>,
    /// Node name on which the event is generated.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
}
/// ExecAction describes a "run in container" action.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecAction {
    /// Command is the command line to execute inside the container, the working directory for the
    /// command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
    /// not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
    /// a shell, you need to explicitly call out to that shell.
    /// Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    /// +optional
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a Fibre Channel volume.
/// Fibre Channel volumes can only be mounted as read/write once.
/// Fibre Channel volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FcVolumeSource {
    /// Optional: FC target worldwide names (WWNs)
    /// +optional
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub target_ww_ns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional: FC target lun number
    /// +optional
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub lun: ::core::option::Option<i32>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// TODO: how do we prevent errors in the filesystem from compromising the machine
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Optional: FC volume world wide identifiers (wwids)
    /// Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    /// +optional
    #[prost(string, repeated, tag="5")]
    #[serde(default)]
    pub wwids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// FlexPersistentVolumeSource represents a generic persistent volume resource that is
/// provisioned/attached using an exec based plugin.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexPersistentVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub driver: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: SecretRef is reference to the secret object containing
    /// sensitive information to pass to the plugin scripts. This may be
    /// empty if no secret object is specified. If the secret object
    /// contains more than one secret, all secrets are passed to the plugin
    /// scripts.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<SecretReference>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Optional: Extra command options if any.
    /// +optional
    #[prost(map="string, string", tag="5")]
    #[serde(default)]
    pub options: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// FlexVolume represents a generic volume resource that is
/// provisioned/attached using an exec based plugin.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexVolumeSource {
    /// Driver is the name of the driver to use for this volume.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub driver: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: SecretRef is reference to the secret object containing
    /// sensitive information to pass to the plugin scripts. This may be
    /// empty if no secret object is specified. If the secret object
    /// contains more than one secret, all secrets are passed to the plugin
    /// scripts.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<LocalObjectReference>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Optional: Extra command options if any.
    /// +optional
    #[prost(map="string, string", tag="5")]
    #[serde(default)]
    pub options: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Represents a Flocker volume mounted by the Flocker agent.
/// One and only one of datasetName and datasetUUID should be set.
/// Flocker volumes do not support ownership management or SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlockerVolumeSource {
    /// Name of the dataset stored as metadata -> name on the dataset for Flocker
    /// should be considered as deprecated
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub dataset_name: ::core::option::Option<::prost::alloc::string::String>,
    /// UUID of the dataset. This is unique identifier of a Flocker dataset
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub dataset_uuid: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Persistent Disk resource in Google Compute Engine.
///
/// A GCE PD must exist before mounting to a container. The disk must
/// also be in the same GCE project and zone as the kubelet. A GCE PD
/// can only be mounted as read/write once or read-only many times. GCE
/// PDs support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcePersistentDiskVolumeSource {
    /// Unique name of the PD resource in GCE. Used to identify the disk in GCE.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub pd_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type of the volume that you want to mount.
    /// Tip: Ensure that the filesystem type is supported by the host operating system.
    /// Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk>
    /// TODO: how do we prevent errors in the filesystem from compromising the machine
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// The partition in the volume that you want to mount.
    /// If omitted, the default is to mount by volume name.
    /// Examples: For volume /dev/sda1, you specify the partition as "1".
    /// Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk>
    /// +optional
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub partition: ::core::option::Option<i32>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts.
    /// Defaults to false.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk>
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Represents a volume that is populated with the contents of a git repository.
/// Git repo volumes do not support ownership management.
/// Git repo volumes support SELinux relabeling.
///
/// DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an
/// EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir
/// into the Pod's container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitRepoVolumeSource {
    /// Repository URL
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub repository: ::core::option::Option<::prost::alloc::string::String>,
    /// Commit hash for the specified revision.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub revision: ::core::option::Option<::prost::alloc::string::String>,
    /// Target directory name.
    /// Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the
    /// git repository.  Otherwise, if specified, the volume will contain the git repository in
    /// the subdirectory with the given name.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub directory: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Glusterfs mount that lasts the lifetime of a pod.
/// Glusterfs volumes do not support ownership management or SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlusterfsPersistentVolumeSource {
    /// EndpointsName is the endpoint name that details Glusterfs topology.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub endpoints: ::core::option::Option<::prost::alloc::string::String>,
    /// Path is the Glusterfs volume path.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod>
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions.
    /// Defaults to false.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod>
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// EndpointsNamespace is the namespace that contains Glusterfs endpoint.
    /// If this field is empty, the EndpointNamespace defaults to the same namespace as the bound PVC.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod>
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub endpoints_namespace: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Glusterfs mount that lasts the lifetime of a pod.
/// Glusterfs volumes do not support ownership management or SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlusterfsVolumeSource {
    /// EndpointsName is the endpoint name that details Glusterfs topology.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub endpoints: ::core::option::Option<::prost::alloc::string::String>,
    /// Path is the Glusterfs volume path.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod>
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions.
    /// Defaults to false.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod>
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpGetAction {
    /// Path to access on the HTTP server.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Name or number of the port to access on the container.
    /// Number must be in the range 1 to 65535.
    /// Name must be an IANA_SVC_NAME.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub port: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// Host name to connect to, defaults to the pod IP. You probably want to set
    /// "Host" in httpHeaders instead.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    /// Scheme to use for connecting to the host.
    /// Defaults to HTTP.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub scheme: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    /// +optional
    #[prost(message, repeated, tag="5")]
    #[serde(default)]
    pub http_headers: ::prost::alloc::vec::Vec<HttpHeader>,
}
/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpHeader {
    /// The header field name
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The header field value
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Handler defines a specific action that should be taken
/// TODO: pass structured data to these actions, and document that data here.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Handler {
    /// One and only one of the following should be specified.
    /// Exec specifies the action to take.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub exec: ::core::option::Option<ExecAction>,
    /// HTTPGet specifies the http request to perform.
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub http_get: ::core::option::Option<HttpGetAction>,
    /// TCPSocket specifies an action involving a TCP port.
    /// TCP hooks not yet supported
    /// TODO: implement a realistic TCP lifecycle hook
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub tcp_socket: ::core::option::Option<TcpSocketAction>,
}
/// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the
/// pod's hosts file.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostAlias {
    /// IP address of the host file entry.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    /// Hostnames for the above IP address.
    #[prost(string, repeated, tag="2")]
    #[serde(default)]
    pub hostnames: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a host path mapped into a pod.
/// Host path volumes do not support ownership management or SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostPathVolumeSource {
    /// Path of the directory on the host.
    /// If the path is a symlink, it will follow the link to the real path.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#hostpath>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Type for HostPath Volume
    /// Defaults to ""
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#hostpath>
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
}
/// ISCSIPersistentVolumeSource represents an ISCSI disk.
/// ISCSI volumes can only be mounted as read/write once.
/// ISCSI volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IscsiPersistentVolumeSource {
    /// iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port
    /// is other than default (typically TCP ports 860 and 3260).
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub target_portal: ::core::option::Option<::prost::alloc::string::String>,
    /// Target iSCSI Qualified Name.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub iqn: ::core::option::Option<::prost::alloc::string::String>,
    /// iSCSI Target Lun number.
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub lun: ::core::option::Option<i32>,
    /// iSCSI Interface Name that uses an iSCSI transport.
    /// Defaults to 'default' (tcp).
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub iscsi_interface: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type of the volume that you want to mount.
    /// Tip: Ensure that the filesystem type is supported by the host operating system.
    /// Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#iscsi>
    /// TODO: how do we prevent errors in the filesystem from compromising the machine
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="6")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// iSCSI Target Portal List. The Portal is either an IP or ip_addr:port if the port
    /// is other than default (typically TCP ports 860 and 3260).
    /// +optional
    #[prost(string, repeated, tag="7")]
    #[serde(default)]
    pub portals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// whether support iSCSI Discovery CHAP authentication
    /// +optional
    #[prost(bool, optional, tag="8")]
    #[serde(default)]
    pub chap_auth_discovery: ::core::option::Option<bool>,
    /// whether support iSCSI Session CHAP authentication
    /// +optional
    #[prost(bool, optional, tag="11")]
    #[serde(default)]
    pub chap_auth_session: ::core::option::Option<bool>,
    /// CHAP Secret for iSCSI target and initiator authentication
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<SecretReference>,
    /// Custom iSCSI Initiator Name.
    /// If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface
    /// <target portal>:<volume name> will be created for the connection.
    /// +optional
    #[prost(string, optional, tag="12")]
    #[serde(default)]
    pub initiator_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents an ISCSI disk.
/// ISCSI volumes can only be mounted as read/write once.
/// ISCSI volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IscsiVolumeSource {
    /// iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port
    /// is other than default (typically TCP ports 860 and 3260).
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub target_portal: ::core::option::Option<::prost::alloc::string::String>,
    /// Target iSCSI Qualified Name.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub iqn: ::core::option::Option<::prost::alloc::string::String>,
    /// iSCSI Target Lun number.
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub lun: ::core::option::Option<i32>,
    /// iSCSI Interface Name that uses an iSCSI transport.
    /// Defaults to 'default' (tcp).
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub iscsi_interface: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type of the volume that you want to mount.
    /// Tip: Ensure that the filesystem type is supported by the host operating system.
    /// Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#iscsi>
    /// TODO: how do we prevent errors in the filesystem from compromising the machine
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="6")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port
    /// is other than default (typically TCP ports 860 and 3260).
    /// +optional
    #[prost(string, repeated, tag="7")]
    #[serde(default)]
    pub portals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// whether support iSCSI Discovery CHAP authentication
    /// +optional
    #[prost(bool, optional, tag="8")]
    #[serde(default)]
    pub chap_auth_discovery: ::core::option::Option<bool>,
    /// whether support iSCSI Session CHAP authentication
    /// +optional
    #[prost(bool, optional, tag="11")]
    #[serde(default)]
    pub chap_auth_session: ::core::option::Option<bool>,
    /// CHAP Secret for iSCSI target and initiator authentication
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<LocalObjectReference>,
    /// Custom iSCSI Initiator Name.
    /// If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface
    /// <target portal>:<volume name> will be created for the connection.
    /// +optional
    #[prost(string, optional, tag="12")]
    #[serde(default)]
    pub initiator_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Maps a string key to a path within a volume.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyToPath {
    /// The key to project.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The relative path of the file to map the key to.
    /// May not be an absolute path.
    /// May not contain the path element '..'.
    /// May not start with the string '..'.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional: mode bits used to set permissions on this file.
    /// Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
    /// YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
    /// If not specified, the volume defaultMode will be used.
    /// This might be in conflict with other options that affect the file
    /// mode, like fsGroup, and the result can be other mode bits set.
    /// +optional
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub mode: ::core::option::Option<i32>,
}
/// Lifecycle describes actions that the management system should take in response to container lifecycle
/// events. For the PostStart and PreStop lifecycle handlers, management of the container blocks
/// until the action is complete, unless the container process fails, in which case the handler is aborted.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lifecycle {
    /// PostStart is called immediately after a container is created. If the handler fails,
    /// the container is terminated and restarted according to its restart policy.
    /// Other management of the container blocks until the hook completes.
    /// More info: <https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub post_start: ::core::option::Option<Handler>,
    /// PreStop is called immediately before a container is terminated due to an
    /// API request or management event such as liveness/startup probe failure,
    /// preemption, resource contention, etc. The handler is not called if the
    /// container crashes or exits. The reason for termination is passed to the
    /// handler. The Pod's termination grace period countdown begins before the
    /// PreStop hooked is executed. Regardless of the outcome of the handler, the
    /// container will eventually terminate within the Pod's termination grace
    /// period. Other management of the container blocks until the hook completes
    /// or until the termination grace period is reached.
    /// More info: <https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub pre_stop: ::core::option::Option<Handler>,
}
/// LimitRange sets resource usage limits for each kind of resource in a Namespace.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitRange {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the limits enforced.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<LimitRangeSpec>,
}
/// LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitRangeItem {
    /// Type of resource that this limit applies to.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Max usage constraints on this kind by resource name.
    /// +optional
    #[prost(map="string, message", tag="2")]
    #[serde(default)]
    pub max: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// Min usage constraints on this kind by resource name.
    /// +optional
    #[prost(map="string, message", tag="3")]
    #[serde(default)]
    pub min: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// Default resource requirement limit value by resource name if resource limit is omitted.
    /// +optional
    #[prost(map="string, message", tag="4")]
    #[serde(default)]
    pub default: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
    /// +optional
    #[prost(map="string, message", tag="5")]
    #[serde(default)]
    pub default_request: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
    /// +optional
    #[prost(map="string, message", tag="6")]
    #[serde(default)]
    pub max_limit_request_ratio: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// LimitRangeList is a list of LimitRange items.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitRangeList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of LimitRange objects.
    /// More info: <https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<LimitRange>,
}
/// LimitRangeSpec defines a min/max usage limit for resources that match on kind.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitRangeSpec {
    /// Limits is the list of LimitRangeItem objects that are enforced.
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub limits: ::prost::alloc::vec::Vec<LimitRangeItem>,
}
/// List holds a list of objects, which may not be known by the server.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of objects
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<super::super::super::apimachinery::pkg::runtime::RawExtension>,
}
/// LoadBalancerIngress represents the status of a load-balancer ingress point:
/// traffic intended for the service should be sent to an ingress point.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerIngress {
    /// IP is set for load-balancer ingress points that are IP based
    /// (typically GCE or OpenStack load-balancers)
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    /// Hostname is set for load-balancer ingress points that are DNS based
    /// (typically AWS load-balancers)
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    /// Ports is a list of records of service ports
    /// If used, every port defined in the service should have an entry in it
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag="4")]
    #[serde(default)]
    pub ports: ::prost::alloc::vec::Vec<PortStatus>,
}
/// LoadBalancerStatus represents the status of a load-balancer.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer.
    /// Traffic intended for the service should be sent to these ingress points.
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub ingress: ::prost::alloc::vec::Vec<LoadBalancerIngress>,
}
/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalObjectReference {
    /// Name of the referent.
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names>
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Local represents directly-attached storage with node affinity (Beta feature)
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalVolumeSource {
    /// The full path to the volume on the node.
    /// It can be either a directory or block device (disk, partition, ...).
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// It applies only when the Path is a block device.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". The default value is to auto-select a fileystem if unspecified.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents an NFS mount that lasts the lifetime of a pod.
/// NFS volumes do not support ownership management or SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfsVolumeSource {
    /// Server is the hostname or IP address of the NFS server.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#nfs>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub server: ::core::option::Option<::prost::alloc::string::String>,
    /// Path that is exported by the NFS server.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#nfs>
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// ReadOnly here will force
    /// the NFS export to be mounted with read-only permissions.
    /// Defaults to false.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#nfs>
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Namespace provides a scope for Names.
/// Use of multiple namespaces is optional.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespace {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the behavior of the Namespace.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<NamespaceSpec>,
    /// Status describes the current status of a Namespace.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub status: ::core::option::Option<NamespaceStatus>,
}
/// NamespaceCondition contains details about state of namespace.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamespaceCondition {
    /// Type of namespace controller condition.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// NamespaceList is a list of Namespaces.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamespaceList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of Namespace objects in the list.
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<Namespace>,
}
/// NamespaceSpec describes the attributes on a Namespace.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamespaceSpec {
    /// Finalizers is an opaque list of values that must be empty to permanently remove object from storage.
    /// More info: <https://kubernetes.io/docs/tasks/administer-cluster/namespaces/>
    /// +optional
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub finalizers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// NamespaceStatus is information about the current status of a Namespace.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamespaceStatus {
    /// Phase is the current lifecycle phase of the namespace.
    /// More info: <https://kubernetes.io/docs/tasks/administer-cluster/namespaces/>
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub phase: ::core::option::Option<::prost::alloc::string::String>,
    /// Represents the latest available observations of a namespace's current state.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub conditions: ::prost::alloc::vec::Vec<NamespaceCondition>,
}
/// Node is a worker node in Kubernetes.
/// Each node will have a unique identifier in the cache (i.e. in etcd).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the behavior of a node.
    /// <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<NodeSpec>,
    /// Most recently observed status of the node.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub status: ::core::option::Option<NodeStatus>,
}
/// NodeAddress contains information for the node's address.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeAddress {
    /// Node address type, one of Hostname, ExternalIP or InternalIP.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// The node address.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Node affinity is a group of node affinity scheduling rules.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeAffinity {
    /// If the affinity requirements specified by this field are not met at
    /// scheduling time, the pod will not be scheduled onto the node.
    /// If the affinity requirements specified by this field cease to be met
    /// at some point during pod execution (e.g. due to an update), the system
    /// may or may not try to eventually evict the pod from its node.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub required_during_scheduling_ignored_during_execution: ::core::option::Option<NodeSelector>,
    /// The scheduler will prefer to schedule pods to nodes that satisfy
    /// the affinity expressions specified by this field, but it may choose
    /// a node that violates one or more of the expressions. The node that is
    /// most preferred is the one with the greatest sum of weights, i.e.
    /// for each node that meets all of the scheduling requirements (resource
    /// request, requiredDuringScheduling affinity expressions, etc.),
    /// compute a sum by iterating through the elements of this field and adding
    /// "weight" to the sum if the node matches the corresponding matchExpressions; the
    /// node(s) with the highest sum are the most preferred.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub preferred_during_scheduling_ignored_during_execution: ::prost::alloc::vec::Vec<PreferredSchedulingTerm>,
}
/// NodeCondition contains condition information for a node.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeCondition {
    /// Type of node condition.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time we got an update on a given condition.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub last_heartbeat_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Last time the condition transit from one status to another.
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// (brief) reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Human readable message indicating details about last transition.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfigSource {
    /// ConfigMap is a reference to a Node's ConfigMap
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub config_map: ::core::option::Option<ConfigMapNodeConfigSource>,
}
/// NodeConfigStatus describes the status of the config assigned by Node.Spec.ConfigSource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfigStatus {
    /// Assigned reports the checkpointed config the node will try to use.
    /// When Node.Spec.ConfigSource is updated, the node checkpoints the associated
    /// config payload to local disk, along with a record indicating intended
    /// config. The node refers to this record to choose its config checkpoint, and
    /// reports this record in Assigned. Assigned only updates in the status after
    /// the record has been checkpointed to disk. When the Kubelet is restarted,
    /// it tries to make the Assigned config the Active config by loading and
    /// validating the checkpointed payload identified by Assigned.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub assigned: ::core::option::Option<NodeConfigSource>,
    /// Active reports the checkpointed config the node is actively using.
    /// Active will represent either the current version of the Assigned config,
    /// or the current LastKnownGood config, depending on whether attempting to use the
    /// Assigned config results in an error.
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub active: ::core::option::Option<NodeConfigSource>,
    /// LastKnownGood reports the checkpointed config the node will fall back to
    /// when it encounters an error attempting to use the Assigned config.
    /// The Assigned config becomes the LastKnownGood config when the node determines
    /// that the Assigned config is stable and correct.
    /// This is currently implemented as a 10-minute soak period starting when the local
    /// record of Assigned config is updated. If the Assigned config is Active at the end
    /// of this period, it becomes the LastKnownGood. Note that if Spec.ConfigSource is
    /// reset to nil (use local defaults), the LastKnownGood is also immediately reset to nil,
    /// because the local default config is always assumed good.
    /// You should not make assumptions about the node's method of determining config stability
    /// and correctness, as this may change or become configurable in the future.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub last_known_good: ::core::option::Option<NodeConfigSource>,
    /// Error describes any problems reconciling the Spec.ConfigSource to the Active config.
    /// Errors may occur, for example, attempting to checkpoint Spec.ConfigSource to the local Assigned
    /// record, attempting to checkpoint the payload associated with Spec.ConfigSource, attempting
    /// to load or validate the Assigned config, etc.
    /// Errors may occur at different points while syncing config. Earlier errors (e.g. download or
    /// checkpointing errors) will not result in a rollback to LastKnownGood, and may resolve across
    /// Kubelet retries. Later errors (e.g. loading or validating a checkpointed config) will result in
    /// a rollback to LastKnownGood. In the latter case, it is usually possible to resolve the error
    /// by fixing the config assigned in Spec.ConfigSource.
    /// You can find additional information for debugging by searching the error message in the Kubelet log.
    /// Error is a human-readable description of the error state; machines can check whether or not Error
    /// is empty, but should not rely on the stability of the Error text across Kubelet versions.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// NodeDaemonEndpoints lists ports opened by daemons running on the Node.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeDaemonEndpoints {
    /// Endpoint on which Kubelet is listening.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub kubelet_endpoint: ::core::option::Option<DaemonEndpoint>,
}
/// NodeList is the whole list of all Nodes which have been registered with master.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of nodes
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<Node>,
}
/// NodeProxyOptions is the query options to a Node's proxy call.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeProxyOptions {
    /// Path is the URL path to use for the current proxy request to node.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// NodeResources is an object for conveying resource information about a node.
/// see <https://kubernetes.io/docs/concepts/architecture/nodes/#capacity> for more details.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeResources {
    /// Capacity represents the available resources of a node
    #[prost(map="string, message", tag="1")]
    #[serde(default)]
    pub capacity: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// A node selector represents the union of the results of one or more label queries
/// over a set of nodes; that is, it represents the OR of the selectors represented
/// by the node selector terms.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeSelector {
    /// Required. A list of node selector terms. The terms are ORed.
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub node_selector_terms: ::prost::alloc::vec::Vec<NodeSelectorTerm>,
}
/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeSelectorRequirement {
    /// The label key that the selector applies to.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub operator: ::core::option::Option<::prost::alloc::string::String>,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    /// +optional
    #[prost(string, repeated, tag="3")]
    #[serde(default)]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A null or empty node selector term matches no objects. The requirements of
/// them are ANDed.
/// The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeSelectorTerm {
    /// A list of node selector requirements by node's labels.
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub match_expressions: ::prost::alloc::vec::Vec<NodeSelectorRequirement>,
    /// A list of node selector requirements by node's fields.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub match_fields: ::prost::alloc::vec::Vec<NodeSelectorRequirement>,
}
/// NodeSpec describes the attributes that a node is created with.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeSpec {
    /// PodCIDR represents the pod IP range assigned to the node.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub pod_cidr: ::core::option::Option<::prost::alloc::string::String>,
    /// podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this
    /// field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for
    /// each of IPv4 and IPv6.
    /// +optional
    /// +patchStrategy=merge
    #[prost(string, repeated, tag="7")]
    #[serde(default)]
    pub pod_cid_rs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub provider_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Unschedulable controls node schedulability of new pods. By default, node is schedulable.
    /// More info: <https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration>
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub unschedulable: ::core::option::Option<bool>,
    /// If specified, the node's taints.
    /// +optional
    #[prost(message, repeated, tag="5")]
    #[serde(default)]
    pub taints: ::prost::alloc::vec::Vec<Taint>,
    /// If specified, the source to get node configuration from
    /// The DynamicKubeletConfig feature gate must be enabled for the Kubelet to use this field
    /// +optional
    #[prost(message, optional, tag="6")]
    #[serde(default)]
    pub config_source: ::core::option::Option<NodeConfigSource>,
    /// Deprecated. Not all kubelets will set this field. Remove field after 1.13.
    /// see: <https://issues.k8s.io/61966>
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub external_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// NodeStatus is information about the current status of a node.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeStatus {
    /// Capacity represents the total resources of a node.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity>
    /// +optional
    #[prost(map="string, message", tag="1")]
    #[serde(default)]
    pub capacity: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// Allocatable represents the resources of a node that are available for scheduling.
    /// Defaults to Capacity.
    /// +optional
    #[prost(map="string, message", tag="2")]
    #[serde(default)]
    pub allocatable: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// NodePhase is the recently observed lifecycle phase of the node.
    /// More info: <https://kubernetes.io/docs/concepts/nodes/node/#phase>
    /// The field is never populated, and now is deprecated.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub phase: ::core::option::Option<::prost::alloc::string::String>,
    /// Conditions is an array of current observed node conditions.
    /// More info: <https://kubernetes.io/docs/concepts/nodes/node/#condition>
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="4")]
    #[serde(default)]
    pub conditions: ::prost::alloc::vec::Vec<NodeCondition>,
    /// List of addresses reachable to the node.
    /// Queried from cloud provider, if available.
    /// More info: <https://kubernetes.io/docs/concepts/nodes/node/#addresses>
    /// Note: This field is declared as mergeable, but the merge key is not sufficiently
    /// unique, which can cause data corruption when it is merged. Callers should instead
    /// use a full-replacement patch. See <http://pr.k8s.io/79391> for an example.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="5")]
    #[serde(default)]
    pub addresses: ::prost::alloc::vec::Vec<NodeAddress>,
    /// Endpoints of daemons running on the Node.
    /// +optional
    #[prost(message, optional, tag="6")]
    #[serde(default)]
    pub daemon_endpoints: ::core::option::Option<NodeDaemonEndpoints>,
    /// Set of ids/uuids to uniquely identify the node.
    /// More info: <https://kubernetes.io/docs/concepts/nodes/node/#info>
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub node_info: ::core::option::Option<NodeSystemInfo>,
    /// List of container images on this node
    /// +optional
    #[prost(message, repeated, tag="8")]
    #[serde(default)]
    pub images: ::prost::alloc::vec::Vec<ContainerImage>,
    /// List of attachable volumes in use (mounted) by the node.
    /// +optional
    #[prost(string, repeated, tag="9")]
    #[serde(default)]
    pub volumes_in_use: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of volumes that are attached to the node.
    /// +optional
    #[prost(message, repeated, tag="10")]
    #[serde(default)]
    pub volumes_attached: ::prost::alloc::vec::Vec<AttachedVolume>,
    /// Status of the config assigned to the node via the dynamic Kubelet config feature.
    /// +optional
    #[prost(message, optional, tag="11")]
    #[serde(default)]
    pub config: ::core::option::Option<NodeConfigStatus>,
}
/// NodeSystemInfo is a set of ids/uuids to uniquely identify the node.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeSystemInfo {
    /// MachineID reported by the node. For unique machine identification
    /// in the cluster this field is preferred. Learn more from man(5)
    /// machine-id: <http://man7.org/linux/man-pages/man5/machine-id.5.html>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub machine_id: ::core::option::Option<::prost::alloc::string::String>,
    /// SystemUUID reported by the node. For unique machine identification
    /// MachineID is preferred. This field is specific to Red Hat hosts
    /// <https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid>
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub system_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// Boot ID reported by the node.
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub boot_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub kernel_version: ::core::option::Option<::prost::alloc::string::String>,
    /// OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub os_image: ::core::option::Option<::prost::alloc::string::String>,
    /// ContainerRuntime Version reported by the node through runtime remote API (e.g. docker://1.5.0).
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub container_runtime_version: ::core::option::Option<::prost::alloc::string::String>,
    /// Kubelet Version reported by the node.
    #[prost(string, optional, tag="7")]
    #[serde(default)]
    pub kubelet_version: ::core::option::Option<::prost::alloc::string::String>,
    /// KubeProxy Version reported by the node.
    #[prost(string, optional, tag="8")]
    #[serde(default)]
    pub kube_proxy_version: ::core::option::Option<::prost::alloc::string::String>,
    /// The Operating System reported by the node
    #[prost(string, optional, tag="9")]
    #[serde(default)]
    pub operating_system: ::core::option::Option<::prost::alloc::string::String>,
    /// The Architecture reported by the node
    #[prost(string, optional, tag="10")]
    #[serde(default)]
    pub architecture: ::core::option::Option<::prost::alloc::string::String>,
}
/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectFieldSelector {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
    /// Path of the field to select in the specified API version.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub field_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// ObjectReference contains enough information to let you inspect or modify the referred object.
/// ---
/// New uses of this type are discouraged because of difficulty describing its usage when embedded in APIs.
///  1. Ignored fields.  It includes many fields which are not generally honored.  For instance, ResourceVersion and FieldPath are both very rarely valid in actual usage.
///  2. Invalid usage help.  It is impossible to add specific help for individual usage.  In most embedded usages, there are particular
///     restrictions like, "must refer only to types A and B" or "UID not honored" or "name must be restricted".
///     Those cannot be well described when embedded.
///  3. Inconsistent validation.  Because the usages are different, the validation rules are different by usage, which makes it hard for users to predict what will happen.
///  4. The fields are both imprecise and overly precise.  Kind is not a precise mapping to a URL. This can produce ambiguity
///     during interpretation and require a REST mapping.  In most cases, the dependency is on the group,resource tuple
///     and the version of the actual struct is irrelevant.
///  5. We cannot easily change it.  Because this type is embedded in many locations, updates to this type
///     will affect numerous schemas.  Don't make new APIs embed an underspecified API type they do not control.
/// Instead of using this type, create a locally provided and used type that is well-focused on your reference.
/// For example, ServiceReferences for admission registration: <https://github.com/kubernetes/api/blob/release-1.17/admissionregistration/v1/types.go#L533> .
/// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectReference {
    /// Kind of the referent.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace of the referent.
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/>
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the referent.
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names>
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// UID of the referent.
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids>
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// API version of the referent.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency>
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub resource_version: ::core::option::Option<::prost::alloc::string::String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers\[2\].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers\[2\]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    /// +optional
    #[prost(string, optional, tag="7")]
    #[serde(default)]
    pub field_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// PersistentVolume (PV) is a storage resource provisioned by an administrator.
/// It is analogous to a node.
/// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes>
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolume {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines a specification of a persistent volume owned by the cluster.
    /// Provisioned by an administrator.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistent-volumes>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<PersistentVolumeSpec>,
    /// Status represents the current information/status for the persistent volume.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistent-volumes>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub status: ::core::option::Option<PersistentVolumeStatus>,
}
/// PersistentVolumeClaim is a user's request for and claim to a persistent volume
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeClaim {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the desired characteristics of a volume requested by a pod author.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<PersistentVolumeClaimSpec>,
    /// Status represents the current information/status of a persistent volume claim.
    /// Read-only.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub status: ::core::option::Option<PersistentVolumeClaimStatus>,
}
/// PersistentVolumeClaimCondition contails details about state of pvc
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeClaimCondition {
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time we probed the condition.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub last_probe_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Unique, this should be a short, machine understandable string that gives the reason
    /// for condition's last transition. If it reports "ResizeStarted" that means the underlying
    /// persistent volume is being resized.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Human-readable message indicating details about last transition.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// PersistentVolumeClaimList is a list of PersistentVolumeClaim items.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeClaimList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// A list of persistent volume claims.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<PersistentVolumeClaim>,
}
/// PersistentVolumeClaimSpec describes the common attributes of storage devices
/// and allows a Source for provider-specific attributes
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeClaimSpec {
    /// AccessModes contains the desired access modes the volume should have.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1>
    /// +optional
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub access_modes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A label query over volumes to consider for binding.
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// Resources represents the minimum resources the volume should have.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub resources: ::core::option::Option<ResourceRequirements>,
    /// VolumeName is the binding reference to the PersistentVolume backing this claim.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub volume_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the StorageClass required by the claim.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1>
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub storage_class_name: ::core::option::Option<::prost::alloc::string::String>,
    /// volumeMode defines what type of volume is required by the claim.
    /// Value of Filesystem is implied when not included in claim spec.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub volume_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// This field can be used to specify either:
    /// * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot)
    /// * An existing PVC (PersistentVolumeClaim)
    /// * An existing custom resource that implements data population (Alpha)
    /// In order to use custom resource types that implement data population,
    /// the AnyVolumeDataSource feature gate must be enabled.
    /// If the provisioner or an external controller can support the specified data source,
    /// it will create a new volume based on the contents of the specified data source.
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub data_source: ::core::option::Option<TypedLocalObjectReference>,
}
/// PersistentVolumeClaimStatus is the current status of a persistent volume claim.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeClaimStatus {
    /// Phase represents the current phase of PersistentVolumeClaim.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub phase: ::core::option::Option<::prost::alloc::string::String>,
    /// AccessModes contains the actual access modes the volume backing the PVC has.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1>
    /// +optional
    #[prost(string, repeated, tag="2")]
    #[serde(default)]
    pub access_modes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Represents the actual resources of the underlying volume.
    /// +optional
    #[prost(map="string, message", tag="3")]
    #[serde(default)]
    pub capacity: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// Current Condition of persistent volume claim. If underlying persistent volume is being
    /// resized then the Condition will be set to 'ResizeStarted'.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="4")]
    #[serde(default)]
    pub conditions: ::prost::alloc::vec::Vec<PersistentVolumeClaimCondition>,
}
/// PersistentVolumeClaimTemplate is used to produce
/// PersistentVolumeClaim objects as part of an EphemeralVolumeSource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeClaimTemplate {
    /// May contain labels and annotations that will be copied into the PVC
    /// when creating it. No other fields are allowed and will be rejected during
    /// validation.
    ///
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// The specification for the PersistentVolumeClaim. The entire content is
    /// copied unchanged into the PVC that gets created from this
    /// template. The same fields as in a PersistentVolumeClaim
    /// are also valid here.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<PersistentVolumeClaimSpec>,
}
/// PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace.
/// This volume finds the bound PV and mounts that volume for the pod. A
/// PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another
/// type of volume that is owned by someone else (the system).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeClaimVolumeSource {
    /// ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub claim_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Will force the ReadOnly setting in VolumeMounts.
    /// Default false.
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// PersistentVolumeList is a list of PersistentVolume items.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of persistent volumes.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<PersistentVolume>,
}
/// PersistentVolumeSource is similar to VolumeSource but meant for the
/// administrator who creates PVs. Exactly one of its members must be set.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeSource {
    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a
    /// kubelet's host machine and then exposed to the pod. Provisioned by an admin.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub gce_persistent_disk: ::core::option::Option<GcePersistentDiskVolumeSource>,
    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a
    /// kubelet's host machine and then exposed to the pod.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub aws_elastic_block_store: ::core::option::Option<AwsElasticBlockStoreVolumeSource>,
    /// HostPath represents a directory on the host.
    /// Provisioned by a developer or tester.
    /// This is useful for single-node development and testing only!
    /// On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#hostpath>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub host_path: ::core::option::Option<HostPathVolumeSource>,
    /// Glusterfs represents a Glusterfs volume that is attached to a host and
    /// exposed to the pod. Provisioned by an admin.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md>
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub glusterfs: ::core::option::Option<GlusterfsPersistentVolumeSource>,
    /// NFS represents an NFS mount on the host. Provisioned by an admin.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#nfs>
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub nfs: ::core::option::Option<NfsVolumeSource>,
    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md>
    /// +optional
    #[prost(message, optional, tag="6")]
    #[serde(default)]
    pub rbd: ::core::option::Option<RbdPersistentVolumeSource>,
    /// ISCSI represents an ISCSI Disk resource that is attached to a
    /// kubelet's host machine and then exposed to the pod. Provisioned by an admin.
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub iscsi: ::core::option::Option<IscsiPersistentVolumeSource>,
    /// Cinder represents a cinder volume attached and mounted on kubelets host machine.
    /// More info: <https://examples.k8s.io/mysql-cinder-pd/README.md>
    /// +optional
    #[prost(message, optional, tag="8")]
    #[serde(default)]
    pub cinder: ::core::option::Option<CinderPersistentVolumeSource>,
    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    /// +optional
    #[prost(message, optional, tag="9")]
    #[serde(default)]
    pub cephfs: ::core::option::Option<CephFsPersistentVolumeSource>,
    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub fc: ::core::option::Option<FcVolumeSource>,
    /// Flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running
    /// +optional
    #[prost(message, optional, tag="11")]
    #[serde(default)]
    pub flocker: ::core::option::Option<FlockerVolumeSource>,
    /// FlexVolume represents a generic volume resource that is
    /// provisioned/attached using an exec based plugin.
    /// +optional
    #[prost(message, optional, tag="12")]
    #[serde(default)]
    pub flex_volume: ::core::option::Option<FlexPersistentVolumeSource>,
    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    /// +optional
    #[prost(message, optional, tag="13")]
    #[serde(default)]
    pub azure_file: ::core::option::Option<AzureFilePersistentVolumeSource>,
    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    /// +optional
    #[prost(message, optional, tag="14")]
    #[serde(default)]
    pub vsphere_volume: ::core::option::Option<VsphereVirtualDiskVolumeSource>,
    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    /// +optional
    #[prost(message, optional, tag="15")]
    #[serde(default)]
    pub quobyte: ::core::option::Option<QuobyteVolumeSource>,
    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    /// +optional
    #[prost(message, optional, tag="16")]
    #[serde(default)]
    pub azure_disk: ::core::option::Option<AzureDiskVolumeSource>,
    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    #[prost(message, optional, tag="17")]
    #[serde(default)]
    pub photon_persistent_disk: ::core::option::Option<PhotonPersistentDiskVolumeSource>,
    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    /// +optional
    #[prost(message, optional, tag="18")]
    #[serde(default)]
    pub portworx_volume: ::core::option::Option<PortworxVolumeSource>,
    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    /// +optional
    #[prost(message, optional, tag="19")]
    #[serde(default)]
    pub scale_io: ::core::option::Option<ScaleIoPersistentVolumeSource>,
    /// Local represents directly-attached storage with node affinity
    /// +optional
    #[prost(message, optional, tag="20")]
    #[serde(default)]
    pub local: ::core::option::Option<LocalVolumeSource>,
    /// StorageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod
    /// More info: <https://examples.k8s.io/volumes/storageos/README.md>
    /// +optional
    #[prost(message, optional, tag="21")]
    #[serde(default)]
    pub storageos: ::core::option::Option<StorageOsPersistentVolumeSource>,
    /// CSI represents storage that is handled by an external CSI driver (Beta feature).
    /// +optional
    #[prost(message, optional, tag="22")]
    #[serde(default)]
    pub csi: ::core::option::Option<CsiPersistentVolumeSource>,
}
/// PersistentVolumeSpec is the specification of a persistent volume.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeSpec {
    /// A description of the persistent volume's resources and capacity.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity>
    /// +optional
    #[prost(map="string, message", tag="1")]
    #[serde(default)]
    pub capacity: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// The actual volume backing the persistent volume.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub persistent_volume_source: ::core::option::Option<PersistentVolumeSource>,
    /// AccessModes contains all ways the volume can be mounted.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes>
    /// +optional
    #[prost(string, repeated, tag="3")]
    #[serde(default)]
    pub access_modes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ClaimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim.
    /// Expected to be non-nil when bound.
    /// claim.VolumeName is the authoritative bind between PV and PVC.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding>
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub claim_ref: ::core::option::Option<ObjectReference>,
    /// What happens to a persistent volume when released from its claim.
    /// Valid options are Retain (default for manually created PersistentVolumes), Delete (default
    /// for dynamically provisioned PersistentVolumes), and Recycle (deprecated).
    /// Recycle must be supported by the volume plugin underlying this PersistentVolume.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming>
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub persistent_volume_reclaim_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of StorageClass to which this persistent volume belongs. Empty value
    /// means that this volume does not belong to any StorageClass.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub storage_class_name: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of mount options, e.g. ["ro", "soft"]. Not validated - mount will
    /// simply fail if one is invalid.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options>
    /// +optional
    #[prost(string, repeated, tag="7")]
    #[serde(default)]
    pub mount_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// volumeMode defines if a volume is intended to be used with a formatted filesystem
    /// or to remain in raw block state. Value of Filesystem is implied when not included in spec.
    /// +optional
    #[prost(string, optional, tag="8")]
    #[serde(default)]
    pub volume_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// NodeAffinity defines constraints that limit what nodes this volume can be accessed from.
    /// This field influences the scheduling of pods that use this volume.
    /// +optional
    #[prost(message, optional, tag="9")]
    #[serde(default)]
    pub node_affinity: ::core::option::Option<VolumeNodeAffinity>,
}
/// PersistentVolumeStatus is the current status of a persistent volume.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentVolumeStatus {
    /// Phase indicates if a volume is available, bound to a claim, or released by a claim.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase>
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub phase: ::core::option::Option<::prost::alloc::string::String>,
    /// A human-readable message indicating details about why the volume is in this state.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// Reason is a brief CamelCase string that describes any failure and is meant
    /// for machine parsing and tidy display in the CLI.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Photon Controller persistent disk resource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotonPersistentDiskVolumeSource {
    /// ID that identifies Photon Controller persistent disk
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub pd_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
}
/// Pod is a collection of containers that can run on a host. This resource is created
/// by clients and scheduled onto hosts.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pod {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the pod.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<PodSpec>,
    /// Most recently observed status of the pod.
    /// This data may not be up to date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub status: ::core::option::Option<PodStatus>,
}
/// Pod affinity is a group of inter pod affinity scheduling rules.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodAffinity {
    /// If the affinity requirements specified by this field are not met at
    /// scheduling time, the pod will not be scheduled onto the node.
    /// If the affinity requirements specified by this field cease to be met
    /// at some point during pod execution (e.g. due to a pod label update), the
    /// system may or may not try to eventually evict the pod from its node.
    /// When there are multiple elements, the lists of nodes corresponding to each
    /// podAffinityTerm are intersected, i.e. all terms must be satisfied.
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub required_during_scheduling_ignored_during_execution: ::prost::alloc::vec::Vec<PodAffinityTerm>,
    /// The scheduler will prefer to schedule pods to nodes that satisfy
    /// the affinity expressions specified by this field, but it may choose
    /// a node that violates one or more of the expressions. The node that is
    /// most preferred is the one with the greatest sum of weights, i.e.
    /// for each node that meets all of the scheduling requirements (resource
    /// request, requiredDuringScheduling affinity expressions, etc.),
    /// compute a sum by iterating through the elements of this field and adding
    /// "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the
    /// node(s) with the highest sum are the most preferred.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub preferred_during_scheduling_ignored_during_execution: ::prost::alloc::vec::Vec<WeightedPodAffinityTerm>,
}
/// Defines a set of pods (namely those matching the labelSelector
/// relative to the given namespace(s)) that this pod should be
/// co-located (affinity) or not co-located (anti-affinity) with,
/// where co-located is defined as running on a node whose value of
/// the label with key <topologyKey> matches that of any node on which
/// a pod of the set of pods is running
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodAffinityTerm {
    /// A label query over a set of resources, in this case pods.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub label_selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// namespaces specifies which namespaces the labelSelector applies to (matches against);
    /// null or empty list means "this pod's namespace"
    /// +optional
    #[prost(string, repeated, tag="2")]
    #[serde(default)]
    pub namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
    /// the labelSelector in the specified namespaces, where co-located is defined as running on a node
    /// whose value of the label with key topologyKey matches that of any node on which any of the
    /// selected pods is running.
    /// Empty topologyKey is not allowed.
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub topology_key: ::core::option::Option<::prost::alloc::string::String>,
}
/// Pod anti affinity is a group of inter pod anti affinity scheduling rules.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodAntiAffinity {
    /// If the anti-affinity requirements specified by this field are not met at
    /// scheduling time, the pod will not be scheduled onto the node.
    /// If the anti-affinity requirements specified by this field cease to be met
    /// at some point during pod execution (e.g. due to a pod label update), the
    /// system may or may not try to eventually evict the pod from its node.
    /// When there are multiple elements, the lists of nodes corresponding to each
    /// podAffinityTerm are intersected, i.e. all terms must be satisfied.
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub required_during_scheduling_ignored_during_execution: ::prost::alloc::vec::Vec<PodAffinityTerm>,
    /// The scheduler will prefer to schedule pods to nodes that satisfy
    /// the anti-affinity expressions specified by this field, but it may choose
    /// a node that violates one or more of the expressions. The node that is
    /// most preferred is the one with the greatest sum of weights, i.e.
    /// for each node that meets all of the scheduling requirements (resource
    /// request, requiredDuringScheduling anti-affinity expressions, etc.),
    /// compute a sum by iterating through the elements of this field and adding
    /// "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the
    /// node(s) with the highest sum are the most preferred.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub preferred_during_scheduling_ignored_during_execution: ::prost::alloc::vec::Vec<WeightedPodAffinityTerm>,
}
/// PodAttachOptions is the query options to a Pod's remote attach call.
/// ---
/// TODO: merge w/ PodExecOptions below for stdin, stdout, etc
/// and also when we cut V2, we should export a "StreamOptions" or somesuch that contains Stdin, Stdout, Stder and TTY
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodAttachOptions {
    /// Stdin if true, redirects the standard input stream of the pod for this call.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="1")]
    #[serde(default)]
    pub stdin: ::core::option::Option<bool>,
    /// Stdout if true indicates that stdout is to be redirected for the attach call.
    /// Defaults to true.
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub stdout: ::core::option::Option<bool>,
    /// Stderr if true indicates that stderr is to be redirected for the attach call.
    /// Defaults to true.
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub stderr: ::core::option::Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the attach call.
    /// This is passed through the container runtime so the tty
    /// is allocated on the worker node by the container runtime.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub tty: ::core::option::Option<bool>,
    /// The container in which to execute the command.
    /// Defaults to only container if there is only one container in the pod.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub container: ::core::option::Option<::prost::alloc::string::String>,
}
/// PodCondition contains details for the current condition of this pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodCondition {
    /// Type is the type of the condition.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status is the status of the condition.
    /// Can be True, False, Unknown.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions>
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time we probed the condition.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub last_probe_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Human-readable message indicating details about last transition.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// PodDNSConfig defines the DNS parameters of a pod in addition to
/// those generated from DNSPolicy.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDnsConfig {
    /// A list of DNS name server IP addresses.
    /// This will be appended to the base nameservers generated from DNSPolicy.
    /// Duplicated nameservers will be removed.
    /// +optional
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub nameservers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of DNS search domains for host-name lookup.
    /// This will be appended to the base search paths generated from DNSPolicy.
    /// Duplicated search paths will be removed.
    /// +optional
    #[prost(string, repeated, tag="2")]
    #[serde(default)]
    pub searches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of DNS resolver options.
    /// This will be merged with the base options generated from DNSPolicy.
    /// Duplicated entries will be removed. Resolution options given in Options
    /// will override those that appear in the base DNSPolicy.
    /// +optional
    #[prost(message, repeated, tag="3")]
    #[serde(default)]
    pub options: ::prost::alloc::vec::Vec<PodDnsConfigOption>,
}
/// PodDNSConfigOption defines DNS resolver options of a pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDnsConfigOption {
    /// Required.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// PodExecOptions is the query options to a Pod's remote exec call.
/// ---
/// TODO: This is largely identical to PodAttachOptions above, make sure they stay in sync and see about merging
/// and also when we cut V2, we should export a "StreamOptions" or somesuch that contains Stdin, Stdout, Stder and TTY
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodExecOptions {
    /// Redirect the standard input stream of the pod for this call.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="1")]
    #[serde(default)]
    pub stdin: ::core::option::Option<bool>,
    /// Redirect the standard output stream of the pod for this call.
    /// Defaults to true.
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub stdout: ::core::option::Option<bool>,
    /// Redirect the standard error stream of the pod for this call.
    /// Defaults to true.
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub stderr: ::core::option::Option<bool>,
    /// TTY if true indicates that a tty will be allocated for the exec call.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub tty: ::core::option::Option<bool>,
    /// Container in which to execute the command.
    /// Defaults to only container if there is only one container in the pod.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub container: ::core::option::Option<::prost::alloc::string::String>,
    /// Command is the remote command to execute. argv array. Not executed within a shell.
    #[prost(string, repeated, tag="6")]
    #[serde(default)]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// IP address information for entries in the (plural) PodIPs field.
/// Each entry includes:
///    IP: An IP address allocated to the pod. Routable at least within the cluster.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodIp {
    /// ip is an IP address (IPv4 or IPv6) assigned to the pod
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
}
/// PodList is a list of Pods.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of pods.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<Pod>,
}
/// PodLogOptions is the query options for a Pod's logs REST call.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodLogOptions {
    /// The container for which to stream logs. Defaults to only container if there is one container in the pod.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub container: ::core::option::Option<::prost::alloc::string::String>,
    /// Follow the log stream of the pod. Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub follow: ::core::option::Option<bool>,
    /// Return previous terminated container logs. Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub previous: ::core::option::Option<bool>,
    /// A relative time in seconds before the current time from which to show logs. If this value
    /// precedes the time a pod was started, only logs since the pod start will be returned.
    /// If this value is in the future, no logs will be returned.
    /// Only one of sinceSeconds or sinceTime may be specified.
    /// +optional
    #[prost(int64, optional, tag="4")]
    #[serde(default)]
    pub since_seconds: ::core::option::Option<i64>,
    /// An RFC3339 timestamp from which to show logs. If this value
    /// precedes the time a pod was started, only logs since the pod start will be returned.
    /// If this value is in the future, no logs will be returned.
    /// Only one of sinceSeconds or sinceTime may be specified.
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub since_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// If true, add an RFC3339 or RFC3339Nano timestamp at the beginning of every line
    /// of log output. Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="6")]
    #[serde(default)]
    pub timestamps: ::core::option::Option<bool>,
    /// If set, the number of lines from the end of the logs to show. If not specified,
    /// logs are shown from the creation of the container or sinceSeconds or sinceTime
    /// +optional
    #[prost(int64, optional, tag="7")]
    #[serde(default)]
    pub tail_lines: ::core::option::Option<i64>,
    /// If set, the number of bytes to read from the server before terminating the
    /// log output. This may not display a complete final line of logging, and may return
    /// slightly more or slightly less than the specified limit.
    /// +optional
    #[prost(int64, optional, tag="8")]
    #[serde(default)]
    pub limit_bytes: ::core::option::Option<i64>,
    /// insecureSkipTLSVerifyBackend indicates that the apiserver should not confirm the validity of the
    /// serving certificate of the backend it is connecting to.  This will make the HTTPS connection between the apiserver
    /// and the backend insecure. This means the apiserver cannot verify the log data it is receiving came from the real
    /// kubelet.  If the kubelet is configured to verify the apiserver's TLS credentials, it does not mean the
    /// connection to the real kubelet is vulnerable to a man in the middle attack (e.g. an attacker could not intercept
    /// the actual log data coming from the real kubelet).
    /// +optional
    #[prost(bool, optional, tag="9")]
    #[serde(default)]
    pub insecure_skip_tls_verify_backend: ::core::option::Option<bool>,
}
/// PodPortForwardOptions is the query options to a Pod's port forward call
/// when using WebSockets.
/// The `port` query parameter must specify the port or
/// ports (comma separated) to forward over.
/// Port forwarding over SPDY does not use these options. It requires the port
/// to be passed in the `port` header as part of request.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodPortForwardOptions {
    /// List of ports to forward
    /// Required when using WebSockets
    /// +optional
    #[prost(int32, repeated, packed="false", tag="1")]
    #[serde(default)]
    pub ports: ::prost::alloc::vec::Vec<i32>,
}
/// PodProxyOptions is the query options to a Pod's proxy call.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodProxyOptions {
    /// Path is the URL path to use for the current proxy request to pod.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// PodReadinessGate contains the reference to a pod condition
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodReadinessGate {
    /// ConditionType refers to a condition in the pod's condition list with matching type.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub condition_type: ::core::option::Option<::prost::alloc::string::String>,
}
/// PodSecurityContext holds pod-level security attributes and common container settings.
/// Some fields are also present in container.securityContext.  Field values of
/// container.securityContext take precedence over field values of PodSecurityContext.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSecurityContext {
    /// The SELinux context to be applied to all containers.
    /// If unspecified, the container runtime will allocate a random SELinux context for each
    /// container.  May also be set in SecurityContext.  If set in
    /// both SecurityContext and PodSecurityContext, the value specified in SecurityContext
    /// takes precedence for that container.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub se_linux_options: ::core::option::Option<SeLinuxOptions>,
    /// The Windows specific settings applied to all containers.
    /// If unspecified, the options within a container's SecurityContext will be used.
    /// If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// +optional
    #[prost(message, optional, tag="8")]
    #[serde(default)]
    pub windows_options: ::core::option::Option<WindowsSecurityContextOptions>,
    /// The UID to run the entrypoint of the container process.
    /// Defaults to user specified in image metadata if unspecified.
    /// May also be set in SecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence
    /// for that container.
    /// +optional
    #[prost(int64, optional, tag="2")]
    #[serde(default)]
    pub run_as_user: ::core::option::Option<i64>,
    /// The GID to run the entrypoint of the container process.
    /// Uses runtime default if unset.
    /// May also be set in SecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence
    /// for that container.
    /// +optional
    #[prost(int64, optional, tag="6")]
    #[serde(default)]
    pub run_as_group: ::core::option::Option<i64>,
    /// Indicates that the container must run as a non-root user.
    /// If true, the Kubelet will validate the image at runtime to ensure that it
    /// does not run as UID 0 (root) and fail to start the container if it does.
    /// If unset or false, no such validation will be performed.
    /// May also be set in SecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub run_as_non_root: ::core::option::Option<bool>,
    /// A list of groups applied to the first process run in each container, in addition
    /// to the container's primary GID.  If unspecified, no groups will be added to
    /// any container.
    /// +optional
    #[prost(int64, repeated, packed="false", tag="4")]
    #[serde(default)]
    pub supplemental_groups: ::prost::alloc::vec::Vec<i64>,
    /// A special supplemental group that applies to all containers in a pod.
    /// Some volume types allow the Kubelet to change the ownership of that volume
    /// to be owned by the pod:
    ///
    /// 1. The owning GID will be the FSGroup
    /// 2. The setgid bit is set (new files created in the volume will be owned by FSGroup)
    /// 3. The permission bits are OR'd with rw-rw----
    ///
    /// If unset, the Kubelet will not modify the ownership and permissions of any volume.
    /// +optional
    #[prost(int64, optional, tag="5")]
    #[serde(default)]
    pub fs_group: ::core::option::Option<i64>,
    /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported
    /// sysctls (by the container runtime) might fail to launch.
    /// +optional
    #[prost(message, repeated, tag="7")]
    #[serde(default)]
    pub sysctls: ::prost::alloc::vec::Vec<Sysctl>,
    /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume
    /// before being exposed inside Pod. This field will only apply to
    /// volume types which support fsGroup based ownership(and permissions).
    /// It will have no effect on ephemeral volume types such as: secret, configmaps
    /// and emptydir.
    /// Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
    /// +optional
    #[prost(string, optional, tag="9")]
    #[serde(default)]
    pub fs_group_change_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// The seccomp options to use by the containers in this pod.
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub seccomp_profile: ::core::option::Option<SeccompProfile>,
}
/// Describes the class of pods that should avoid this node.
/// Exactly one field should be set.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSignature {
    /// Reference to controller whose pods should avoid this node.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub pod_controller: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::OwnerReference>,
}
/// PodSpec is a description of a pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSpec {
    /// List of volumes that can be mounted by containers belonging to the pod.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes>
    /// +optional
    /// +patchMergeKey=name
    /// +patchStrategy=merge,retainKeys
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// List of initialization containers belonging to the pod.
    /// Init containers are executed in order prior to containers being started. If any
    /// init container fails, the pod is considered to have failed and is handled according
    /// to its restartPolicy. The name for an init container or normal container must be
    /// unique among all containers.
    /// Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes.
    /// The resourceRequirements of an init container are taken into account during scheduling
    /// by finding the highest request/limit for each resource type, and then using the max of
    /// of that value or the sum of the normal containers. Limits are applied to init containers
    /// in a similar fashion.
    /// Init containers cannot currently be added or removed.
    /// Cannot be updated.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/init-containers/>
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="20")]
    #[serde(default)]
    pub init_containers: ::prost::alloc::vec::Vec<Container>,
    /// List of containers belonging to the pod.
    /// Containers cannot currently be added or removed.
    /// There must be at least one container in a Pod.
    /// Cannot be updated.
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing
    /// pod to perform user-initiated actions such as debugging. This list cannot be specified when
    /// creating a pod, and it cannot be modified by updating the pod spec. In order to add an
    /// ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.
    /// This field is alpha-level and is only honored by servers that enable the EphemeralContainers feature.
    /// +optional
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="34")]
    #[serde(default)]
    pub ephemeral_containers: ::prost::alloc::vec::Vec<EphemeralContainer>,
    /// Restart policy for all containers within the pod.
    /// One of Always, OnFailure, Never.
    /// Default to Always.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy>
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub restart_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request.
    /// Value must be non-negative integer. The value zero indicates delete immediately.
    /// If this value is nil, the default grace period will be used instead.
    /// The grace period is the duration in seconds after the processes running in the pod are sent
    /// a termination signal and the time when the processes are forcibly halted with a kill signal.
    /// Set this value longer than the expected cleanup time for your process.
    /// Defaults to 30 seconds.
    /// +optional
    #[prost(int64, optional, tag="4")]
    #[serde(default)]
    pub termination_grace_period_seconds: ::core::option::Option<i64>,
    /// Optional duration in seconds the pod may be active on the node relative to
    /// StartTime before the system will actively try to mark it failed and kill associated containers.
    /// Value must be a positive integer.
    /// +optional
    #[prost(int64, optional, tag="5")]
    #[serde(default)]
    pub active_deadline_seconds: ::core::option::Option<i64>,
    /// Set DNS policy for the pod.
    /// Defaults to "ClusterFirst".
    /// Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'.
    /// DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy.
    /// To have DNS options set along with hostNetwork, you have to specify DNS policy
    /// explicitly to 'ClusterFirstWithHostNet'.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub dns_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node.
    /// Selector which must match a node's labels for the pod to be scheduled on that node.
    /// More info: <https://kubernetes.io/docs/concepts/configuration/assign-pod-node/>
    /// +optional
    #[prost(map="string, string", tag="7")]
    #[serde(default)]
    pub node_selector: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod.
    /// More info: <https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/>
    /// +optional
    #[prost(string, optional, tag="8")]
    #[serde(default)]
    pub service_account_name: ::core::option::Option<::prost::alloc::string::String>,
    /// DeprecatedServiceAccount is a depreciated alias for ServiceAccountName.
    /// Deprecated: Use serviceAccountName instead.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(string, optional, tag="9")]
    #[serde(default)]
    pub service_account: ::core::option::Option<::prost::alloc::string::String>,
    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    /// +optional
    #[prost(bool, optional, tag="21")]
    #[serde(default)]
    pub automount_service_account_token: ::core::option::Option<bool>,
    /// NodeName is a request to schedule this pod onto a specific node. If it is non-empty,
    /// the scheduler simply schedules this pod onto that node, assuming that it fits resource
    /// requirements.
    /// +optional
    #[prost(string, optional, tag="10")]
    #[serde(default)]
    pub node_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Host networking requested for this pod. Use the host's network namespace.
    /// If this option is set, the ports that will be used must be specified.
    /// Default to false.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(bool, optional, tag="11")]
    #[serde(default)]
    pub host_network: ::core::option::Option<bool>,
    /// Use the host's pid namespace.
    /// Optional: Default to false.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(bool, optional, tag="12")]
    #[serde(default)]
    pub host_pid: ::core::option::Option<bool>,
    /// Use the host's ipc namespace.
    /// Optional: Default to false.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(bool, optional, tag="13")]
    #[serde(default)]
    pub host_ipc: ::core::option::Option<bool>,
    /// Share a single process namespace between all of the containers in a pod.
    /// When this is set containers will be able to view and signal processes from other containers
    /// in the same pod, and the first process in each container will not be assigned PID 1.
    /// HostPID and ShareProcessNamespace cannot both be set.
    /// Optional: Default to false.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(bool, optional, tag="27")]
    #[serde(default)]
    pub share_process_namespace: ::core::option::Option<bool>,
    /// SecurityContext holds pod-level security attributes and common container settings.
    /// Optional: Defaults to empty.  See type description for default values of each field.
    /// +optional
    #[prost(message, optional, tag="14")]
    #[serde(default)]
    pub security_context: ::core::option::Option<PodSecurityContext>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec.
    /// If specified, these secrets will be passed to individual puller implementations for them to use. For example,
    /// in the case of docker, only DockerConfig type secrets are honored.
    /// More info: <https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod>
    /// +optional
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="15")]
    #[serde(default)]
    pub image_pull_secrets: ::prost::alloc::vec::Vec<LocalObjectReference>,
    /// Specifies the hostname of the Pod
    /// If not specified, the pod's hostname will be set to a system-defined value.
    /// +optional
    #[prost(string, optional, tag="16")]
    #[serde(default)]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>".
    /// If not specified, the pod will not have a domainname at all.
    /// +optional
    #[prost(string, optional, tag="17")]
    #[serde(default)]
    pub subdomain: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified, the pod's scheduling constraints
    /// +optional
    #[prost(message, optional, tag="18")]
    #[serde(default)]
    pub affinity: ::core::option::Option<Affinity>,
    /// If specified, the pod will be dispatched by specified scheduler.
    /// If not specified, the pod will be dispatched by default scheduler.
    /// +optional
    #[prost(string, optional, tag="19")]
    #[serde(default)]
    pub scheduler_name: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified, the pod's tolerations.
    /// +optional
    #[prost(message, repeated, tag="22")]
    #[serde(default)]
    pub tolerations: ::prost::alloc::vec::Vec<Toleration>,
    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts
    /// file if specified. This is only valid for non-hostNetwork pods.
    /// +optional
    /// +patchMergeKey=ip
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="23")]
    #[serde(default)]
    pub host_aliases: ::prost::alloc::vec::Vec<HostAlias>,
    /// If specified, indicates the pod's priority. "system-node-critical" and
    /// "system-cluster-critical" are two special keywords which indicate the
    /// highest priorities with the former being the highest priority. Any other
    /// name must be defined by creating a PriorityClass object with that name.
    /// If not specified, the pod priority will be default or zero if there is no
    /// default.
    /// +optional
    #[prost(string, optional, tag="24")]
    #[serde(default)]
    pub priority_class_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The priority value. Various system components use this field to find the
    /// priority of the pod. When Priority Admission Controller is enabled, it
    /// prevents users from setting this field. The admission controller populates
    /// this field from PriorityClassName.
    /// The higher the value, the higher the priority.
    /// +optional
    #[prost(int32, optional, tag="25")]
    #[serde(default)]
    pub priority: ::core::option::Option<i32>,
    /// Specifies the DNS parameters of a pod.
    /// Parameters specified here will be merged to the generated DNS
    /// configuration based on DNSPolicy.
    /// +optional
    #[prost(message, optional, tag="26")]
    #[serde(default)]
    pub dns_config: ::core::option::Option<PodDnsConfig>,
    /// If specified, all readiness gates will be evaluated for pod readiness.
    /// A pod is ready when all its containers are ready AND
    /// all conditions specified in the readiness gates have status equal to "True"
    /// More info: <https://git.k8s.io/enhancements/keps/sig-network/0007-pod-ready%2B%2B.md>
    /// +optional
    #[prost(message, repeated, tag="28")]
    #[serde(default)]
    pub readiness_gates: ::prost::alloc::vec::Vec<PodReadinessGate>,
    /// RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used
    /// to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run.
    /// If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an
    /// empty definition that uses the default runtime handler.
    /// More info: <https://git.k8s.io/enhancements/keps/sig-node/runtime-class.md>
    /// This is a beta feature as of Kubernetes v1.14.
    /// +optional
    #[prost(string, optional, tag="29")]
    #[serde(default)]
    pub runtime_class_name: ::core::option::Option<::prost::alloc::string::String>,
    /// EnableServiceLinks indicates whether information about services should be injected into pod's
    /// environment variables, matching the syntax of Docker links.
    /// Optional: Defaults to true.
    /// +optional
    #[prost(bool, optional, tag="30")]
    #[serde(default)]
    pub enable_service_links: ::core::option::Option<bool>,
    /// PreemptionPolicy is the Policy for preempting pods with lower priority.
    /// One of Never, PreemptLowerPriority.
    /// Defaults to PreemptLowerPriority if unset.
    /// This field is beta-level, gated by the NonPreemptingPriority feature-gate.
    /// +optional
    #[prost(string, optional, tag="31")]
    #[serde(default)]
    pub preemption_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass.
    /// This field will be autopopulated at admission time by the RuntimeClass admission controller. If
    /// the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests.
    /// The RuntimeClass admission controller will reject Pod create requests which have the overhead already
    /// set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value
    /// defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero.
    /// More info: <https://git.k8s.io/enhancements/keps/sig-node/20190226-pod-overhead.md>
    /// This field is alpha-level as of Kubernetes v1.16, and is only honored by servers that enable the PodOverhead feature.
    /// +optional
    #[prost(map="string, message", tag="32")]
    #[serde(default)]
    pub overhead: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// TopologySpreadConstraints describes how a group of pods ought to spread across topology
    /// domains. Scheduler will schedule pods in a way which abides by the constraints.
    /// All topologySpreadConstraints are ANDed.
    /// +optional
    /// +patchMergeKey=topologyKey
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=topologyKey
    /// +listMapKey=whenUnsatisfiable
    #[prost(message, repeated, tag="33")]
    #[serde(default)]
    pub topology_spread_constraints: ::prost::alloc::vec::Vec<TopologySpreadConstraint>,
    /// If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default).
    /// In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname).
    /// In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters to FQDN.
    /// If a pod does not have FQDN, this has no effect.
    /// Default to false.
    /// +optional
    #[prost(bool, optional, tag="35")]
    #[serde(default)]
    pub set_hostname_as_fqdn: ::core::option::Option<bool>,
}
/// PodStatus represents information about the status of a pod. Status may trail the actual
/// state of a system, especially if the node that hosts the pod cannot contact the control
/// plane.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodStatus {
    /// The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle.
    /// The conditions array, the reason and message fields, and the individual container status
    /// arrays contain more detail about the pod's status.
    /// There are five possible phase values:
    ///
    /// Pending: The pod has been accepted by the Kubernetes system, but one or more of the
    /// container images has not been created. This includes time before being scheduled as
    /// well as time spent downloading images over the network, which could take a while.
    /// Running: The pod has been bound to a node, and all of the containers have been created.
    /// At least one container is still running, or is in the process of starting or restarting.
    /// Succeeded: All containers in the pod have terminated in success, and will not be restarted.
    /// Failed: All containers in the pod have terminated, and at least one container has
    /// terminated in failure. The container either exited with non-zero status or was terminated
    /// by the system.
    /// Unknown: For some reason the state of the pod could not be obtained, typically due to an
    /// error in communicating with the host of the pod.
    ///
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase>
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub phase: ::core::option::Option<::prost::alloc::string::String>,
    /// Current service state of pod.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions>
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub conditions: ::prost::alloc::vec::Vec<PodCondition>,
    /// A human readable message indicating details about why the pod is in this condition.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// A brief CamelCase message indicating details about why the pod is in this state.
    /// e.g. 'Evicted'
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be
    /// scheduled right away as preemption victims receive their graceful termination periods.
    /// This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide
    /// to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to
    /// give the resources on this node to a higher priority pod that is created after preemption.
    /// As a result, this field may be different than PodSpec.nodeName when the pod is
    /// scheduled.
    /// +optional
    #[prost(string, optional, tag="11")]
    #[serde(default)]
    pub nominated_node_name: ::core::option::Option<::prost::alloc::string::String>,
    /// IP address of the host to which the pod is assigned. Empty if not yet scheduled.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub host_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// IP address allocated to the pod. Routable at least within the cluster.
    /// Empty if not yet allocated.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub pod_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must
    /// match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list
    /// is empty if no IPs have been allocated yet.
    /// +optional
    /// +patchStrategy=merge
    /// +patchMergeKey=ip
    #[prost(message, repeated, tag="12")]
    #[serde(default)]
    pub pod_i_ps: ::prost::alloc::vec::Vec<PodIp>,
    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet.
    /// This is before the Kubelet pulled the container image(s) for the pod.
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub start_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The list has one entry per init container in the manifest. The most recent successful
    /// init container will have ready = true, the most recently started container will have
    /// startTime set.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status>
    #[prost(message, repeated, tag="10")]
    #[serde(default)]
    pub init_container_statuses: ::prost::alloc::vec::Vec<ContainerStatus>,
    /// The list has one entry per container in the manifest. Each entry is currently the output
    /// of `docker inspect`.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status>
    /// +optional
    #[prost(message, repeated, tag="8")]
    #[serde(default)]
    pub container_statuses: ::prost::alloc::vec::Vec<ContainerStatus>,
    /// The Quality of Service (QOS) classification assigned to the pod based on resource requirements
    /// See PodQOSClass type for available QOS classes
    /// More info: <https://git.k8s.io/community/contributors/design-proposals/node/resource-qos.md>
    /// +optional
    #[prost(string, optional, tag="9")]
    #[serde(default)]
    pub qos_class: ::core::option::Option<::prost::alloc::string::String>,
    /// Status for any ephemeral containers that have run in this pod.
    /// This field is alpha-level and is only populated by servers that enable the EphemeralContainers feature.
    /// +optional
    #[prost(message, repeated, tag="13")]
    #[serde(default)]
    pub ephemeral_container_statuses: ::prost::alloc::vec::Vec<ContainerStatus>,
}
/// PodStatusResult is a wrapper for PodStatus returned by kubelet that can be encode/decoded
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodStatusResult {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Most recently observed status of the pod.
    /// This data may not be up to date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub status: ::core::option::Option<PodStatus>,
}
/// PodTemplate describes a template for creating copies of a predefined pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodTemplate {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Template defines the pods that will be created from this pod template.
    /// <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub template: ::core::option::Option<PodTemplateSpec>,
}
/// PodTemplateList is a list of PodTemplates.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodTemplateList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of pod templates
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<PodTemplate>,
}
/// PodTemplateSpec describes the data a pod should have when created from a template
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodTemplateSpec {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the pod.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<PodSpec>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortStatus {
    /// Port is the port number of the service port of which status is recorded here
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub port: ::core::option::Option<i32>,
    /// Protocol is the protocol of the service port of which status is recorded here
    /// The supported values are: "TCP", "UDP", "SCTP"
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// Error is to record the problem with the service port
    /// The format of the error shall comply with the following rules:
    /// - built-in error values shall be specified in this file and those shall use
    ///   CamelCase names
    /// - cloud provider specific error values must have names that comply with the
    ///   format foo.example.com/CamelCase.
    /// ---
    /// The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    /// +optional
    /// +kubebuilder:validation:Required
    /// +kubebuilder:validation:Pattern=`^(\[a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*/)?(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9\])$`
    /// +kubebuilder:validation:MaxLength=316
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// PortworxVolumeSource represents a Portworx volume resource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortworxVolumeSource {
    /// VolumeID uniquely identifies a Portworx volume
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub volume_id: ::core::option::Option<::prost::alloc::string::String>,
    /// FSType represents the filesystem type to mount
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
/// +k8s:openapi-gen=false
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Preconditions {
    /// Specifies the target UID.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
}
/// Describes a class of pods that should avoid this node.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreferAvoidPodsEntry {
    /// The class of pods.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub pod_signature: ::core::option::Option<PodSignature>,
    /// Time at which this entry was added to the list.
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub eviction_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// (brief) reason why this entry was added to the list.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Human readable message indicating why this entry was added to the list.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// An empty preferred scheduling term matches all objects with implicit weight 0
/// (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreferredSchedulingTerm {
    /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub weight: ::core::option::Option<i32>,
    /// A node selector term, associated with the corresponding weight.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub preference: ::core::option::Option<NodeSelectorTerm>,
}
/// Probe describes a health check to be performed against a container to determine whether it is
/// alive or ready to receive traffic.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Probe {
    /// The action taken to determine the health of a container
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub handler: ::core::option::Option<Handler>,
    /// Number of seconds after the container has started before liveness probes are initiated.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes>
    /// +optional
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub initial_delay_seconds: ::core::option::Option<i32>,
    /// Number of seconds after which the probe times out.
    /// Defaults to 1 second. Minimum value is 1.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes>
    /// +optional
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub timeout_seconds: ::core::option::Option<i32>,
    /// How often (in seconds) to perform the probe.
    /// Default to 10 seconds. Minimum value is 1.
    /// +optional
    #[prost(int32, optional, tag="4")]
    #[serde(default)]
    pub period_seconds: ::core::option::Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed.
    /// Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    /// +optional
    #[prost(int32, optional, tag="5")]
    #[serde(default)]
    pub success_threshold: ::core::option::Option<i32>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded.
    /// Defaults to 3. Minimum value is 1.
    /// +optional
    #[prost(int32, optional, tag="6")]
    #[serde(default)]
    pub failure_threshold: ::core::option::Option<i32>,
}
/// Represents a projected volume source
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectedVolumeSource {
    /// list of volume projections
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub sources: ::prost::alloc::vec::Vec<VolumeProjection>,
    /// Mode bits used to set permissions on created files by default.
    /// Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
    /// YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
    /// Directories within the path are not affected by this setting.
    /// This might be in conflict with other options that affect the file
    /// mode, like fsGroup, and the result can be other mode bits set.
    /// +optional
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub default_mode: ::core::option::Option<i32>,
}
/// Represents a Quobyte mount that lasts the lifetime of a pod.
/// Quobyte volumes do not support ownership management or SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuobyteVolumeSource {
    /// Registry represents a single or multiple Quobyte Registry services
    /// specified as a string as host:port pair (multiple entries are separated with commas)
    /// which acts as the central registry for volumes
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub registry: ::core::option::Option<::prost::alloc::string::String>,
    /// Volume is a string that references an already created Quobyte volume by name.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub volume: ::core::option::Option<::prost::alloc::string::String>,
    /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// User to map volume access to
    /// Defaults to serivceaccount user
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// Group to map volume access to
    /// Default is no group
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// Tenant owning the given Quobyte volume in the Backend
    /// Used with dynamically provisioned Quobyte volumes, value is set by the plugin
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub tenant: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Rados Block Device mount that lasts the lifetime of a pod.
/// RBD volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RbdPersistentVolumeSource {
    /// A collection of Ceph monitors.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub monitors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The rados image name.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type of the volume that you want to mount.
    /// Tip: Ensure that the filesystem type is supported by the host operating system.
    /// Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#rbd>
    /// TODO: how do we prevent errors in the filesystem from compromising the machine
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// The rados pool name.
    /// Default is rbd.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub pool: ::core::option::Option<::prost::alloc::string::String>,
    /// The rados user name.
    /// Default is admin.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// Keyring is the path to key ring for RBDUser.
    /// Default is /etc/ceph/keyring.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub keyring: ::core::option::Option<::prost::alloc::string::String>,
    /// SecretRef is name of the authentication secret for RBDUser. If provided
    /// overrides keyring.
    /// Default is nil.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<SecretReference>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts.
    /// Defaults to false.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(bool, optional, tag="8")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// Represents a Rados Block Device mount that lasts the lifetime of a pod.
/// RBD volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RbdVolumeSource {
    /// A collection of Ceph monitors.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    #[prost(string, repeated, tag="1")]
    #[serde(default)]
    pub monitors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The rados image name.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type of the volume that you want to mount.
    /// Tip: Ensure that the filesystem type is supported by the host operating system.
    /// Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#rbd>
    /// TODO: how do we prevent errors in the filesystem from compromising the machine
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// The rados pool name.
    /// Default is rbd.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub pool: ::core::option::Option<::prost::alloc::string::String>,
    /// The rados user name.
    /// Default is admin.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// Keyring is the path to key ring for RBDUser.
    /// Default is /etc/ceph/keyring.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub keyring: ::core::option::Option<::prost::alloc::string::String>,
    /// SecretRef is name of the authentication secret for RBDUser. If provided
    /// overrides keyring.
    /// Default is nil.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<LocalObjectReference>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts.
    /// Defaults to false.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it>
    /// +optional
    #[prost(bool, optional, tag="8")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// RangeAllocation is not a public type.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeAllocation {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Range is string that identifies the range represented by 'data'.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub range: ::core::option::Option<::prost::alloc::string::String>,
    /// Data is a bit array containing all allocated addresses in the previous segment.
    #[prost(bytes="vec", optional, tag="3")]
    #[serde(default)]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// ReplicationController represents the configuration of a replication controller.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationController {
    /// If the Labels of a ReplicationController are empty, they are defaulted to
    /// be the same as the Pod(s) that the replication controller manages.
    /// Standard object's metadata. More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the specification of the desired behavior of the replication controller.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<ReplicationControllerSpec>,
    /// Status is the most recently observed status of the replication controller.
    /// This data may be out of date by some window of time.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub status: ::core::option::Option<ReplicationControllerStatus>,
}
/// ReplicationControllerCondition describes the state of a replication controller at a certain point.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationControllerCondition {
    /// Type of replication controller condition.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// The last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// ReplicationControllerList is a collection of replication controllers.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationControllerList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of replication controllers.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<ReplicationController>,
}
/// ReplicationControllerSpec is the specification of a replication controller.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationControllerSpec {
    /// Replicas is the number of desired replicas.
    /// This is a pointer to distinguish between explicit zero and unspecified.
    /// Defaults to 1.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller>
    /// +optional
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub replicas: ::core::option::Option<i32>,
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing, for it to be considered available.
    /// Defaults to 0 (pod will be considered available as soon as it is ready)
    /// +optional
    #[prost(int32, optional, tag="4")]
    #[serde(default)]
    pub min_ready_seconds: ::core::option::Option<i32>,
    /// Selector is a label query over pods that should match the Replicas count.
    /// If Selector is empty, it is defaulted to the labels present on the Pod template.
    /// Label keys and values that must match in order to be controlled by this replication
    /// controller, if empty defaulted to labels on Pod template.
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors>
    /// +optional
    #[prost(map="string, string", tag="2")]
    #[serde(default)]
    pub selector: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Template is the object that describes the pod that will be created if
    /// insufficient replicas are detected. This takes precedence over a TemplateRef.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub template: ::core::option::Option<PodTemplateSpec>,
}
/// ReplicationControllerStatus represents the current status of a replication
/// controller.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationControllerStatus {
    /// Replicas is the most recently oberved number of replicas.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller>
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub replicas: ::core::option::Option<i32>,
    /// The number of pods that have labels matching the labels of the pod template of the replication controller.
    /// +optional
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub fully_labeled_replicas: ::core::option::Option<i32>,
    /// The number of ready replicas for this replication controller.
    /// +optional
    #[prost(int32, optional, tag="4")]
    #[serde(default)]
    pub ready_replicas: ::core::option::Option<i32>,
    /// The number of available replicas (ready for at least minReadySeconds) for this replication controller.
    /// +optional
    #[prost(int32, optional, tag="5")]
    #[serde(default)]
    pub available_replicas: ::core::option::Option<i32>,
    /// ObservedGeneration reflects the generation of the most recently observed replication controller.
    /// +optional
    #[prost(int64, optional, tag="3")]
    #[serde(default)]
    pub observed_generation: ::core::option::Option<i64>,
    /// Represents the latest available observations of a replication controller's current state.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="6")]
    #[serde(default)]
    pub conditions: ::prost::alloc::vec::Vec<ReplicationControllerCondition>,
}
/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub container_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Required: resource to select
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub divisor: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// ResourceQuota sets aggregate quota restrictions enforced per namespace
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceQuota {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the desired quota.
    /// <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<ResourceQuotaSpec>,
    /// Status defines the actual enforced quota and its current usage.
    /// <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub status: ::core::option::Option<ResourceQuotaStatus>,
}
/// ResourceQuotaList is a list of ResourceQuota items.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceQuotaList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of ResourceQuota objects.
    /// More info: <https://kubernetes.io/docs/concepts/policy/resource-quotas/>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<ResourceQuota>,
}
/// ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceQuotaSpec {
    /// hard is the set of desired hard limits for each named resource.
    /// More info: <https://kubernetes.io/docs/concepts/policy/resource-quotas/>
    /// +optional
    #[prost(map="string, message", tag="1")]
    #[serde(default)]
    pub hard: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// A collection of filters that must match each object tracked by a quota.
    /// If not specified, the quota matches all objects.
    /// +optional
    #[prost(string, repeated, tag="2")]
    #[serde(default)]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota
    /// but expressed using ScopeSelectorOperator in combination with possible values.
    /// For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub scope_selector: ::core::option::Option<ScopeSelector>,
}
/// ResourceQuotaStatus defines the enforced hard limits and observed use.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource.
    /// More info: <https://kubernetes.io/docs/concepts/policy/resource-quotas/>
    /// +optional
    #[prost(map="string, message", tag="1")]
    #[serde(default)]
    pub hard: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// Used is the current observed total usage of the resource in the namespace.
    /// +optional
    #[prost(map="string, message", tag="2")]
    #[serde(default)]
    pub used: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// ResourceRequirements describes the compute resource requirements.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceRequirements {
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: <https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/>
    /// +optional
    #[prost(map="string, message", tag="1")]
    #[serde(default)]
    pub limits: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value.
    /// More info: <https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/>
    /// +optional
    #[prost(map="string, message", tag="2")]
    #[serde(default)]
    pub requests: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// SELinuxOptions are the labels to be applied to the container
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeLinuxOptions {
    /// User is a SELinux user label that applies to the container.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// Role is a SELinux role label that applies to the container.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub role: ::core::option::Option<::prost::alloc::string::String>,
    /// Type is a SELinux type label that applies to the container.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Level is SELinux level label that applies to the container.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub level: ::core::option::Option<::prost::alloc::string::String>,
}
/// ScaleIOPersistentVolumeSource represents a persistent ScaleIO volume
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleIoPersistentVolumeSource {
    /// The host address of the ScaleIO API Gateway.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub gateway: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the storage system as configured in ScaleIO.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub system: ::core::option::Option<::prost::alloc::string::String>,
    /// SecretRef references to the secret for ScaleIO user and other
    /// sensitive information. If this is not provided, Login operation will fail.
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<SecretReference>,
    /// Flag to enable/disable SSL communication with Gateway, default false
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub ssl_enabled: ::core::option::Option<bool>,
    /// The name of the ScaleIO Protection Domain for the configured storage.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub protection_domain: ::core::option::Option<::prost::alloc::string::String>,
    /// The ScaleIO Storage Pool associated with the protection domain.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub storage_pool: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned.
    /// Default is ThinProvisioned.
    /// +optional
    #[prost(string, optional, tag="7")]
    #[serde(default)]
    pub storage_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of a volume already created in the ScaleIO system
    /// that is associated with this volume source.
    #[prost(string, optional, tag="8")]
    #[serde(default)]
    pub volume_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs".
    /// Default is "xfs"
    /// +optional
    #[prost(string, optional, tag="9")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="10")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// ScaleIOVolumeSource represents a persistent ScaleIO volume
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleIoVolumeSource {
    /// The host address of the ScaleIO API Gateway.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub gateway: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the storage system as configured in ScaleIO.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub system: ::core::option::Option<::prost::alloc::string::String>,
    /// SecretRef references to the secret for ScaleIO user and other
    /// sensitive information. If this is not provided, Login operation will fail.
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<LocalObjectReference>,
    /// Flag to enable/disable SSL communication with Gateway, default false
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub ssl_enabled: ::core::option::Option<bool>,
    /// The name of the ScaleIO Protection Domain for the configured storage.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub protection_domain: ::core::option::Option<::prost::alloc::string::String>,
    /// The ScaleIO Storage Pool associated with the protection domain.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub storage_pool: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned.
    /// Default is ThinProvisioned.
    /// +optional
    #[prost(string, optional, tag="7")]
    #[serde(default)]
    pub storage_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of a volume already created in the ScaleIO system
    /// that is associated with this volume source.
    #[prost(string, optional, tag="8")]
    #[serde(default)]
    pub volume_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs".
    /// Default is "xfs".
    /// +optional
    #[prost(string, optional, tag="9")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="10")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
}
/// A scope selector represents the AND of the selectors represented
/// by the scoped-resource selector requirements.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeSelector {
    /// A list of scope selector requirements by scope of the resources.
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub match_expressions: ::prost::alloc::vec::Vec<ScopedResourceSelectorRequirement>,
}
/// A scoped-resource selector requirement is a selector that contains values, a scope name, and an operator
/// that relates the scope name and values.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopedResourceSelectorRequirement {
    /// The name of the scope that the selector applies to.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub scope_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Represents a scope's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub operator: ::core::option::Option<::prost::alloc::string::String>,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty.
    /// This array is replaced during a strategic merge patch.
    /// +optional
    #[prost(string, repeated, tag="3")]
    #[serde(default)]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// SeccompProfile defines a pod/container's seccomp profile settings.
/// Only one profile source may be set.
/// +union
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeccompProfile {
    /// type indicates which kind of seccomp profile will be applied.
    /// Valid options are:
    ///
    /// Localhost - a profile defined in a file on the node should be used.
    /// RuntimeDefault - the container runtime default profile should be used.
    /// Unconfined - no profile should be applied.
    /// +unionDiscriminator
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// localhostProfile indicates a profile defined in a file on the node should be used.
    /// The profile must be preconfigured on the node to work.
    /// Must be a descending path, relative to the kubelet's configured seccomp profile location.
    /// Must only be set if type is "Localhost".
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub localhost_profile: ::core::option::Option<::prost::alloc::string::String>,
}
/// Secret holds secret data of a certain type. The total bytes of the values in
/// the Data field must be less than MaxSecretSize bytes.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Immutable, if set to true, ensures that data stored in the Secret cannot
    /// be updated (only object metadata can be modified).
    /// If not set to true, the field can be modified at any time.
    /// Defaulted to nil.
    /// This is a beta field enabled by ImmutableEphemeralVolumes feature gate.
    /// +optional
    #[prost(bool, optional, tag="5")]
    #[serde(default)]
    pub immutable: ::core::option::Option<bool>,
    /// Data contains the secret data. Each key must consist of alphanumeric
    /// characters, '-', '_' or '.'. The serialized form of the secret data is a
    /// base64 encoded string, representing the arbitrary (possibly non-string)
    /// data value here. Described in <https://tools.ietf.org/html/rfc4648#section-4>
    /// +optional
    #[prost(map="string, bytes", tag="2")]
    #[serde(default)]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    /// stringData allows specifying non-binary secret data in string form.
    /// It is provided as a write-only convenience method.
    /// All keys and values are merged into the data field on write, overwriting any existing values.
    /// It is never output when reading from the API.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(map="string, string", tag="4")]
    #[serde(default)]
    pub string_data: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Used to facilitate programmatic handling of secret data.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
}
/// SecretEnvSource selects a Secret to populate the environment
/// variables with.
///
/// The contents of the target Secret's Data field will represent the
/// key-value pairs as environment variables.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretEnvSource {
    /// The Secret to select from.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub local_object_reference: ::core::option::Option<LocalObjectReference>,
    /// Specify whether the Secret must be defined
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub optional: ::core::option::Option<bool>,
}
/// SecretKeySelector selects a key of a Secret.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretKeySelector {
    /// The name of the secret in the pod's namespace to select from.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub local_object_reference: ::core::option::Option<LocalObjectReference>,
    /// The key of the secret to select from.  Must be a valid secret key.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Specify whether the Secret or its key must be defined
    /// +optional
    #[prost(bool, optional, tag="3")]
    #[serde(default)]
    pub optional: ::core::option::Option<bool>,
}
/// SecretList is a list of Secret.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of secret objects.
    /// More info: <https://kubernetes.io/docs/concepts/configuration/secret>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<Secret>,
}
/// Adapts a secret into a projected volume.
///
/// The contents of the target Secret's Data field will be presented in a
/// projected volume as files using the keys in the Data field as the file names.
/// Note that this is identical to a secret volume source without the default
/// mode.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretProjection {
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub local_object_reference: ::core::option::Option<LocalObjectReference>,
    /// If unspecified, each key-value pair in the Data field of the referenced
    /// Secret will be projected into the volume as a file whose name is the
    /// key and content is the value. If specified, the listed keys will be
    /// projected into the specified paths, and unlisted keys will not be
    /// present. If a key is specified which is not present in the Secret,
    /// the volume setup will error unless it is marked optional. Paths must be
    /// relative and may not contain the '..' path or start with '..'.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<KeyToPath>,
    /// Specify whether the Secret or its key must be defined
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub optional: ::core::option::Option<bool>,
}
/// SecretReference represents a Secret Reference. It has enough information to retrieve secret
/// in any namespace
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretReference {
    /// Name is unique within a namespace to reference a secret resource.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace defines the space within which the secret name must be unique.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
}
/// Adapts a Secret into a volume.
///
/// The contents of the target Secret's Data field will be presented in a volume
/// as files using the keys in the Data field as the file names.
/// Secret volumes support ownership management and SELinux relabeling.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVolumeSource {
    /// Name of the secret in the pod's namespace to use.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#secret>
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub secret_name: ::core::option::Option<::prost::alloc::string::String>,
    /// If unspecified, each key-value pair in the Data field of the referenced
    /// Secret will be projected into the volume as a file whose name is the
    /// key and content is the value. If specified, the listed keys will be
    /// projected into the specified paths, and unlisted keys will not be
    /// present. If a key is specified which is not present in the Secret,
    /// the volume setup will error unless it is marked optional. Paths must be
    /// relative and may not contain the '..' path or start with '..'.
    /// +optional
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<KeyToPath>,
    /// Optional: mode bits used to set permissions on created files by default.
    /// Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
    /// YAML accepts both octal and decimal values, JSON requires decimal values
    /// for mode bits. Defaults to 0644.
    /// Directories within the path are not affected by this setting.
    /// This might be in conflict with other options that affect the file
    /// mode, like fsGroup, and the result can be other mode bits set.
    /// +optional
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub default_mode: ::core::option::Option<i32>,
    /// Specify whether the Secret or its keys must be defined
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub optional: ::core::option::Option<bool>,
}
/// SecurityContext holds security configuration that will be applied to a container.
/// Some fields are present in both SecurityContext and PodSecurityContext.  When both
/// are set, the values in SecurityContext take precedence.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityContext {
    /// The capabilities to add/drop when running containers.
    /// Defaults to the default set of capabilities granted by the container runtime.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub capabilities: ::core::option::Option<Capabilities>,
    /// Run container in privileged mode.
    /// Processes in privileged containers are essentially equivalent to root on the host.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub privileged: ::core::option::Option<bool>,
    /// The SELinux context to be applied to the container.
    /// If unspecified, the container runtime will allocate a random SELinux context for each
    /// container.  May also be set in PodSecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub se_linux_options: ::core::option::Option<SeLinuxOptions>,
    /// The Windows specific settings applied to all containers.
    /// If unspecified, the options from the PodSecurityContext will be used.
    /// If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub windows_options: ::core::option::Option<WindowsSecurityContextOptions>,
    /// The UID to run the entrypoint of the container process.
    /// Defaults to user specified in image metadata if unspecified.
    /// May also be set in PodSecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// +optional
    #[prost(int64, optional, tag="4")]
    #[serde(default)]
    pub run_as_user: ::core::option::Option<i64>,
    /// The GID to run the entrypoint of the container process.
    /// Uses runtime default if unset.
    /// May also be set in PodSecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// +optional
    #[prost(int64, optional, tag="8")]
    #[serde(default)]
    pub run_as_group: ::core::option::Option<i64>,
    /// Indicates that the container must run as a non-root user.
    /// If true, the Kubelet will validate the image at runtime to ensure that it
    /// does not run as UID 0 (root) and fail to start the container if it does.
    /// If unset or false, no such validation will be performed.
    /// May also be set in PodSecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// +optional
    #[prost(bool, optional, tag="5")]
    #[serde(default)]
    pub run_as_non_root: ::core::option::Option<bool>,
    /// Whether this container has a read-only root filesystem.
    /// Default is false.
    /// +optional
    #[prost(bool, optional, tag="6")]
    #[serde(default)]
    pub read_only_root_filesystem: ::core::option::Option<bool>,
    /// AllowPrivilegeEscalation controls whether a process can gain more
    /// privileges than its parent process. This bool directly controls if
    /// the no_new_privs flag will be set on the container process.
    /// AllowPrivilegeEscalation is true always when the container is:
    /// 1) run as Privileged
    /// 2) has CAP_SYS_ADMIN
    /// +optional
    #[prost(bool, optional, tag="7")]
    #[serde(default)]
    pub allow_privilege_escalation: ::core::option::Option<bool>,
    /// procMount denotes the type of proc mount to use for the containers.
    /// The default is DefaultProcMount which uses the container runtime defaults for
    /// readonly paths and masked paths.
    /// This requires the ProcMountType feature flag to be enabled.
    /// +optional
    #[prost(string, optional, tag="9")]
    #[serde(default)]
    pub proc_mount: ::core::option::Option<::prost::alloc::string::String>,
    /// The seccomp options to use by this container. If seccomp options are
    /// provided at both the pod & container level, the container options
    /// override the pod options.
    /// +optional
    #[prost(message, optional, tag="11")]
    #[serde(default)]
    pub seccomp_profile: ::core::option::Option<SeccompProfile>,
}
/// SerializedReference is a reference to serialized object.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedReference {
    /// The reference to an object in the system.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub reference: ::core::option::Option<ObjectReference>,
}
/// Service is a named abstraction of software service (for example, mysql) consisting of local port
/// (for example 3306) that the proxy listens on, and the selector that determines which pods
/// will answer requests sent through the proxy.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the behavior of a service.
    /// <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub spec: ::core::option::Option<ServiceSpec>,
    /// Most recently observed status of the service.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub status: ::core::option::Option<ServiceStatus>,
}
/// ServiceAccount binds together:
/// * a name, understood by users, and perhaps by peripheral systems, for an identity
/// * a principal that can be authenticated and authorized
/// * a set of secrets
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Secrets is the list of secrets allowed to be used by pods running using this ServiceAccount.
    /// More info: <https://kubernetes.io/docs/concepts/configuration/secret>
    /// +optional
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub secrets: ::prost::alloc::vec::Vec<ObjectReference>,
    /// ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images
    /// in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets
    /// can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet.
    /// More info: <https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod>
    /// +optional
    #[prost(message, repeated, tag="3")]
    #[serde(default)]
    pub image_pull_secrets: ::prost::alloc::vec::Vec<LocalObjectReference>,
    /// AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted.
    /// Can be overridden at the pod level.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub automount_service_account_token: ::core::option::Option<bool>,
}
/// ServiceAccountList is a list of ServiceAccount objects
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccountList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of ServiceAccounts.
    /// More info: <https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/>
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<ServiceAccount>,
}
/// ServiceAccountTokenProjection represents a projected service account token
/// volume. This projection can be used to insert a service account token into
/// the pods runtime filesystem for use against APIs (Kubernetes API Server or
/// otherwise).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccountTokenProjection {
    /// Audience is the intended audience of the token. A recipient of a token
    /// must identify itself with an identifier specified in the audience of the
    /// token, and otherwise should reject the token. The audience defaults to the
    /// identifier of the apiserver.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub audience: ::core::option::Option<::prost::alloc::string::String>,
    /// ExpirationSeconds is the requested duration of validity of the service
    /// account token. As the token approaches expiration, the kubelet volume
    /// plugin will proactively rotate the service account token. The kubelet will
    /// start trying to rotate the token if the token is older than 80 percent of
    /// its time to live or if the token is older than 24 hours.Defaults to 1 hour
    /// and must be at least 10 minutes.
    /// +optional
    #[prost(int64, optional, tag="2")]
    #[serde(default)]
    pub expiration_seconds: ::core::option::Option<i64>,
    /// Path is the path relative to the mount point of the file to project the
    /// token into.
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// ServiceList holds a list of services.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of services
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub items: ::prost::alloc::vec::Vec<Service>,
}
/// ServicePort contains information on service's port.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServicePort {
    /// The name of this port within the service. This must be a DNS_LABEL.
    /// All ports within a ServiceSpec must have unique names. When considering
    /// the endpoints for a Service, this must match the 'name' field in the
    /// EndpointPort.
    /// Optional if only one ServicePort is defined on this service.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The IP protocol for this port. Supports "TCP", "UDP", and "SCTP".
    /// Default is TCP.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// The application protocol for this port.
    /// This field follows standard Kubernetes label syntax.
    /// Un-prefixed names are reserved for IANA standard service names (as per
    /// RFC-6335 and <http://www.iana.org/assignments/service-names>).
    /// Non-standard protocols should use prefixed names such as
    /// mycompany.com/my-custom-protocol.
    /// This is a beta field that is guarded by the ServiceAppProtocol feature
    /// gate and enabled by default.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub app_protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// The port that will be exposed by this service.
    #[prost(int32, optional, tag="3")]
    #[serde(default)]
    pub port: ::core::option::Option<i32>,
    /// Number or name of the port to access on the pods targeted by the service.
    /// Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    /// If this is a string, it will be looked up as a named port in the
    /// target Pod's container ports. If this is not specified, the value
    /// of the 'port' field is used (an identity map).
    /// This field is ignored for services with clusterIP=None, and should be
    /// omitted or set equal to the 'port' field.
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service>
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub target_port: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// The port on each node on which this service is exposed when type is
    /// NodePort or LoadBalancer.  Usually assigned by the system. If a value is
    /// specified, in-range, and not in use it will be used, otherwise the
    /// operation will fail.  If not specified, a port will be allocated if this
    /// Service requires one.  If this field is specified when creating a
    /// Service which does not need it, creation will fail. This field will be
    /// wiped when updating a Service to no longer need it (e.g. changing type
    /// from NodePort to ClusterIP).
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport>
    /// +optional
    #[prost(int32, optional, tag="5")]
    #[serde(default)]
    pub node_port: ::core::option::Option<i32>,
}
/// ServiceProxyOptions is the query options to a Service's proxy call.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceProxyOptions {
    /// Path is the part of URLs that include service endpoints, suffixes,
    /// and parameters to use for the current proxy request to service.
    /// For example, the whole request URL is
    /// <http://localhost/api/v1/namespaces/kube-system/services/elasticsearch-logging/_search?q=user:kimchy.>
    /// Path is _search?q=user:kimchy.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// ServiceSpec describes the attributes that a user creates on a service.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceSpec {
    /// The list of ports that are exposed by this service.
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies>
    /// +patchMergeKey=port
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=port
    /// +listMapKey=protocol
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub ports: ::prost::alloc::vec::Vec<ServicePort>,
    /// Route service traffic to pods with label keys and values matching this
    /// selector. If empty or not present, the service is assumed to have an
    /// external process managing its endpoints, which Kubernetes will not
    /// modify. Only applies to types ClusterIP, NodePort, and LoadBalancer.
    /// Ignored if type is ExternalName.
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/>
    /// +optional
    #[prost(map="string, string", tag="2")]
    #[serde(default)]
    pub selector: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// clusterIP is the IP address of the service and is usually assigned
    /// randomly. If an address is specified manually, is in-range (as per
    /// system configuration), and is not in use, it will be allocated to the
    /// service; otherwise creation of the service will fail. This field may not
    /// be changed through updates unless the type field is also being changed
    /// to ExternalName (which requires this field to be blank) or the type
    /// field is being changed from ExternalName (in which case this field may
    /// optionally be specified, as describe above).  Valid values are "None",
    /// empty string (""), or a valid IP address. Setting this to "None" makes a
    /// "headless service" (no virtual IP), which is useful when direct endpoint
    /// connections are preferred and proxying is not required.  Only applies to
    /// types ClusterIP, NodePort, and LoadBalancer. If this field is specified
    /// when creating a Service of type ExternalName, creation will fail. This
    /// field will be wiped when updating a Service to type ExternalName.
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies>
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub cluster_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// ClusterIPs is a list of IP addresses assigned to this service, and are
    /// usually assigned randomly.  If an address is specified manually, is
    /// in-range (as per system configuration), and is not in use, it will be
    /// allocated to the service; otherwise creation of the service will fail.
    /// This field may not be changed through updates unless the type field is
    /// also being changed to ExternalName (which requires this field to be
    /// empty) or the type field is being changed from ExternalName (in which
    /// case this field may optionally be specified, as describe above).  Valid
    /// values are "None", empty string (""), or a valid IP address.  Setting
    /// this to "None" makes a "headless service" (no virtual IP), which is
    /// useful when direct endpoint connections are preferred and proxying is
    /// not required.  Only applies to types ClusterIP, NodePort, and
    /// LoadBalancer. If this field is specified when creating a Service of type
    /// ExternalName, creation will fail. This field will be wiped when updating
    /// a Service to type ExternalName.  If this field is not specified, it will
    /// be initialized from the clusterIP field.  If this field is specified,
    /// clients must ensure that clusterIPs\[0\] and clusterIP have the same
    /// value.
    ///
    /// Unless the "IPv6DualStack" feature gate is enabled, this field is
    /// limited to one value, which must be the same as the clusterIP field.  If
    /// the feature gate is enabled, this field may hold a maximum of two
    /// entries (dual-stack IPs, in either order).  These IPs must correspond to
    /// the values of the ipFamilies field. Both clusterIPs and ipFamilies are
    /// governed by the ipFamilyPolicy field.
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies>
    /// +listType=atomic
    /// +optional
    #[prost(string, repeated, tag="18")]
    #[serde(default)]
    pub cluster_i_ps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// type determines how the Service is exposed. Defaults to ClusterIP. Valid
    /// options are ExternalName, ClusterIP, NodePort, and LoadBalancer.
    /// "ClusterIP" allocates a cluster-internal IP address for load-balancing
    /// to endpoints. Endpoints are determined by the selector or if that is not
    /// specified, by manual construction of an Endpoints object or
    /// EndpointSlice objects. If clusterIP is "None", no virtual IP is
    /// allocated and the endpoints are published as a set of endpoints rather
    /// than a virtual IP.
    /// "NodePort" builds on ClusterIP and allocates a port on every node which
    /// routes to the same endpoints as the clusterIP.
    /// "LoadBalancer" builds on NodePort and creates an external load-balancer
    /// (if supported in the current cloud) which routes to the same endpoints
    /// as the clusterIP.
    /// "ExternalName" aliases this service to the specified externalName.
    /// Several other fields do not apply to ExternalName services.
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types>
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// externalIPs is a list of IP addresses for which nodes in the cluster
    /// will also accept traffic for this service.  These IPs are not managed by
    /// Kubernetes.  The user is responsible for ensuring that traffic arrives
    /// at a node with this IP.  A common example is external load-balancers
    /// that are not part of the Kubernetes system.
    /// +optional
    #[prost(string, repeated, tag="5")]
    #[serde(default)]
    pub external_i_ps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Supports "ClientIP" and "None". Used to maintain session affinity.
    /// Enable client IP based session affinity.
    /// Must be ClientIP or None.
    /// Defaults to None.
    /// More info: <https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies>
    /// +optional
    #[prost(string, optional, tag="7")]
    #[serde(default)]
    pub session_affinity: ::core::option::Option<::prost::alloc::string::String>,
    /// Only applies to Service Type: LoadBalancer
    /// LoadBalancer will get created with the IP specified in this field.
    /// This feature depends on whether the underlying cloud-provider supports specifying
    /// the loadBalancerIP when a load balancer is created.
    /// This field will be ignored if the cloud-provider does not support the feature.
    /// +optional
    #[prost(string, optional, tag="8")]
    #[serde(default)]
    pub load_balancer_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified and supported by the platform, this will restrict traffic through the cloud-provider
    /// load-balancer will be restricted to the specified client IPs. This field will be ignored if the
    /// cloud-provider does not support the feature."
    /// More info: <https://kubernetes.io/docs/tasks/access-application-cluster/configure-cloud-provider-firewall/>
    /// +optional
    #[prost(string, repeated, tag="9")]
    #[serde(default)]
    pub load_balancer_source_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// externalName is the external reference that discovery mechanisms will
    /// return as an alias for this service (e.g. a DNS CNAME record). No
    /// proxying will be involved.  Must be a lowercase RFC-1123 hostname
    /// (<https://tools.ietf.org/html/rfc1123>) and requires Type to be
    /// +optional
    #[prost(string, optional, tag="10")]
    #[serde(default)]
    pub external_name: ::core::option::Option<::prost::alloc::string::String>,
    /// externalTrafficPolicy denotes if this Service desires to route external
    /// traffic to node-local or cluster-wide endpoints. "Local" preserves the
    /// client source IP and avoids a second hop for LoadBalancer and Nodeport
    /// type services, but risks potentially imbalanced traffic spreading.
    /// "Cluster" obscures the client source IP and may cause a second hop to
    /// another node, but should have good overall load-spreading.
    /// +optional
    #[prost(string, optional, tag="11")]
    #[serde(default)]
    pub external_traffic_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// healthCheckNodePort specifies the healthcheck nodePort for the service.
    /// This only applies when type is set to LoadBalancer and
    /// externalTrafficPolicy is set to Local. If a value is specified, is
    /// in-range, and is not in use, it will be used.  If not specified, a value
    /// will be automatically allocated.  External systems (e.g. load-balancers)
    /// can use this port to determine if a given node holds endpoints for this
    /// service or not.  If this field is specified when creating a Service
    /// which does not need it, creation will fail. This field will be wiped
    /// when updating a Service to no longer need it (e.g. changing type).
    /// +optional
    #[prost(int32, optional, tag="12")]
    #[serde(default)]
    pub health_check_node_port: ::core::option::Option<i32>,
    /// publishNotReadyAddresses indicates that any agent which deals with endpoints for this
    /// Service should disregard any indications of ready/not-ready.
    /// The primary use case for setting this field is for a StatefulSet's Headless Service to
    /// propagate SRV DNS records for its Pods for the purpose of peer discovery.
    /// The Kubernetes controllers that generate Endpoints and EndpointSlice resources for
    /// Services interpret this to mean that all endpoints are considered "ready" even if the
    /// Pods themselves are not. Agents which consume only Kubernetes generated endpoints
    /// through the Endpoints or EndpointSlice resources can safely assume this behavior.
    /// +optional
    #[prost(bool, optional, tag="13")]
    #[serde(default)]
    pub publish_not_ready_addresses: ::core::option::Option<bool>,
    /// sessionAffinityConfig contains the configurations of session affinity.
    /// +optional
    #[prost(message, optional, tag="14")]
    #[serde(default)]
    pub session_affinity_config: ::core::option::Option<SessionAffinityConfig>,
    /// topologyKeys is a preference-order list of topology keys which
    /// implementations of services should use to preferentially sort endpoints
    /// when accessing this Service, it can not be used at the same time as
    /// externalTrafficPolicy=Local.
    /// Topology keys must be valid label keys and at most 16 keys may be specified.
    /// Endpoints are chosen based on the first topology key with available backends.
    /// If this field is specified and all entries have no backends that match
    /// the topology of the client, the service has no backends for that client
    /// and connections should fail.
    /// The special value "*" may be used to mean "any topology". This catch-all
    /// value, if used, only makes sense as the last value in the list.
    /// If this is not specified or empty, no topology constraints will be applied.
    /// This field is alpha-level and is only honored by servers that enable the ServiceTopology feature.
    /// +optional
    #[prost(string, repeated, tag="16")]
    #[serde(default)]
    pub topology_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this
    /// service, and is gated by the "IPv6DualStack" feature gate.  This field
    /// is usually assigned automatically based on cluster configuration and the
    /// ipFamilyPolicy field. If this field is specified manually, the requested
    /// family is available in the cluster, and ipFamilyPolicy allows it, it
    /// will be used; otherwise creation of the service will fail.  This field
    /// is conditionally mutable: it allows for adding or removing a secondary
    /// IP family, but it does not allow changing the primary IP family of the
    /// Service.  Valid values are "IPv4" and "IPv6".  This field only applies
    /// to Services of types ClusterIP, NodePort, and LoadBalancer, and does
    /// apply to "headless" services.  This field will be wiped when updating a
    /// Service to type ExternalName.
    ///
    /// This field may hold a maximum of two entries (dual-stack families, in
    /// either order).  These families must correspond to the values of the
    /// clusterIPs field, if specified. Both clusterIPs and ipFamilies are
    /// governed by the ipFamilyPolicy field.
    /// +listType=atomic
    /// +optional
    #[prost(string, repeated, tag="19")]
    #[serde(default)]
    pub ip_families: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// IPFamilyPolicy represents the dual-stack-ness requested or required by
    /// this Service, and is gated by the "IPv6DualStack" feature gate.  If
    /// there is no value provided, then this field will be set to SingleStack.
    /// Services can be "SingleStack" (a single IP family), "PreferDualStack"
    /// (two IP families on dual-stack configured clusters or a single IP family
    /// on single-stack clusters), or "RequireDualStack" (two IP families on
    /// dual-stack configured clusters, otherwise fail). The ipFamilies and
    /// clusterIPs fields depend on the value of this field.  This field will be
    /// wiped when updating a service to type ExternalName.
    /// +optional
    #[prost(string, optional, tag="17")]
    #[serde(default)]
    pub ip_family_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// allocateLoadBalancerNodePorts defines if NodePorts will be automatically
    /// allocated for services with type LoadBalancer.  Default is "true". It may be
    /// set to "false" if the cluster load-balancer does not rely on NodePorts.
    /// allocateLoadBalancerNodePorts may only be set for services with type LoadBalancer
    /// and will be cleared if the type is changed to any other type.
    /// This field is alpha-level and is only honored by servers that enable the ServiceLBNodePortControl feature.
    /// +optional
    #[prost(bool, optional, tag="20")]
    #[serde(default)]
    pub allocate_load_balancer_node_ports: ::core::option::Option<bool>,
}
/// ServiceStatus represents the current status of a service.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceStatus {
    /// LoadBalancer contains the current status of the load-balancer,
    /// if one is present.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub load_balancer: ::core::option::Option<LoadBalancerStatus>,
    /// Current service state
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag="2")]
    #[serde(default)]
    pub conditions: ::prost::alloc::vec::Vec<super::super::super::apimachinery::pkg::apis::meta::v1::Condition>,
}
/// SessionAffinityConfig represents the configurations of session affinity.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionAffinityConfig {
    /// clientIP contains the configurations of Client IP based session affinity.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub client_ip: ::core::option::Option<ClientIpConfig>,
}
/// Represents a StorageOS persistent volume resource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageOsPersistentVolumeSource {
    /// VolumeName is the human-readable name of the StorageOS volume.  Volume
    /// names are only unique within a namespace.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub volume_name: ::core::option::Option<::prost::alloc::string::String>,
    /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no
    /// namespace is specified then the Pod's namespace will be used.  This allows the
    /// Kubernetes name scoping to be mirrored within StorageOS for tighter integration.
    /// Set VolumeName to any name to override the default behaviour.
    /// Set to "default" if you are not using namespaces within StorageOS.
    /// Namespaces that do not pre-exist within StorageOS will be created.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub volume_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// SecretRef specifies the secret to use for obtaining the StorageOS API
    /// credentials.  If not specified, default values will be attempted.
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<ObjectReference>,
}
/// Represents a StorageOS persistent volume resource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageOsVolumeSource {
    /// VolumeName is the human-readable name of the StorageOS volume.  Volume
    /// names are only unique within a namespace.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub volume_name: ::core::option::Option<::prost::alloc::string::String>,
    /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no
    /// namespace is specified then the Pod's namespace will be used.  This allows the
    /// Kubernetes name scoping to be mirrored within StorageOS for tighter integration.
    /// Set VolumeName to any name to override the default behaviour.
    /// Set to "default" if you are not using namespaces within StorageOS.
    /// Namespaces that do not pre-exist within StorageOS will be created.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub volume_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Defaults to false (read/write). ReadOnly here will force
    /// the ReadOnly setting in VolumeMounts.
    /// +optional
    #[prost(bool, optional, tag="4")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// SecretRef specifies the secret to use for obtaining the StorageOS API
    /// credentials.  If not specified, default values will be attempted.
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub secret_ref: ::core::option::Option<LocalObjectReference>,
}
/// Sysctl defines a kernel parameter to be set
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sysctl {
    /// Name of a property to set
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Value of a property to set
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// TCPSocketAction describes an action based on opening a socket
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpSocketAction {
    /// Number or name of the port to access on the container.
    /// Number must be in the range 1 to 65535.
    /// Name must be an IANA_SVC_NAME.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub port: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// Optional: Host name to connect to, defaults to the pod IP.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
}
/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Taint {
    /// Required. The taint key to be applied to a node.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The taint value corresponding to the taint key.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub effect: ::core::option::Option<::prost::alloc::string::String>,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub time_added: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
}
/// The pod this Toleration is attached to tolerates any taint that matches
/// the triple <key,value,effect> using the matching operator <operator>.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Toleration {
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys.
    /// If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Operator represents a key's relationship to the value.
    /// Valid operators are Exists and Equal. Defaults to Equal.
    /// Exists is equivalent to wildcard for value, so that a pod can
    /// tolerate all taints of a particular category.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub operator: ::core::option::Option<::prost::alloc::string::String>,
    /// Value is the taint value the toleration matches to.
    /// If the operator is Exists, the value should be empty, otherwise just a regular string.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Effect indicates the taint effect to match. Empty means match all taint effects.
    /// When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub effect: ::core::option::Option<::prost::alloc::string::String>,
    /// TolerationSeconds represents the period of time the toleration (which must be
    /// of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default,
    /// it is not set, which means tolerate the taint forever (do not evict). Zero and
    /// negative values will be treated as 0 (evict immediately) by the system.
    /// +optional
    #[prost(int64, optional, tag="5")]
    #[serde(default)]
    pub toleration_seconds: ::core::option::Option<i64>,
}
/// A topology selector requirement is a selector that matches given label.
/// This is an alpha feature and may change in the future.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopologySelectorLabelRequirement {
    /// The label key that the selector applies to.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// An array of string values. One value must match the label to be selected.
    /// Each entry in Values is ORed.
    #[prost(string, repeated, tag="2")]
    #[serde(default)]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A topology selector term represents the result of label queries.
/// A null or empty topology selector term matches no objects.
/// The requirements of them are ANDed.
/// It provides a subset of functionality as NodeSelectorTerm.
/// This is an alpha feature and may change in the future.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopologySelectorTerm {
    /// A list of topology selector requirements by labels.
    /// +optional
    #[prost(message, repeated, tag="1")]
    #[serde(default)]
    pub match_label_expressions: ::prost::alloc::vec::Vec<TopologySelectorLabelRequirement>,
}
/// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopologySpreadConstraint {
    /// MaxSkew describes the degree to which pods may be unevenly distributed.
    /// When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference
    /// between the number of matching pods in the target topology and the global minimum.
    /// For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same
    /// labelSelector spread as 1/1/0:
    /// +-------+-------+-------+
    /// | zone1 | zone2 | zone3 |
    /// +-------+-------+-------+
    /// |   P   |   P   |       |
    /// +-------+-------+-------+
    /// - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 1/1/1;
    /// scheduling it onto zone1(zone2) would make the ActualSkew(2-0) on zone1(zone2)
    /// violate MaxSkew(1).
    /// - if MaxSkew is 2, incoming pod can be scheduled onto any zone.
    /// When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence
    /// to topologies that satisfy it.
    /// It's a required field. Default value is 1 and 0 is not allowed.
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub max_skew: ::core::option::Option<i32>,
    /// TopologyKey is the key of node labels. Nodes that have a label with this key
    /// and identical values are considered to be in the same topology.
    /// We consider each <key, value> as a "bucket", and try to put balanced number
    /// of pods into each bucket.
    /// It's a required field.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub topology_key: ::core::option::Option<::prost::alloc::string::String>,
    /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy
    /// the spread constraint.
    /// - DoNotSchedule (default) tells the scheduler not to schedule it.
    /// - ScheduleAnyway tells the scheduler to schedule the pod in any location,
    ///   but giving higher precedence to topologies that would help reduce the
    ///   skew.
    /// A constraint is considered "Unsatisfiable" for an incoming pod
    /// if and only if every possible node assigment for that pod would violate
    /// "MaxSkew" on some topology.
    /// For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same
    /// labelSelector spread as 3/1/1:
    /// +-------+-------+-------+
    /// | zone1 | zone2 | zone3 |
    /// +-------+-------+-------+
    /// | P P P |   P   |   P   |
    /// +-------+-------+-------+
    /// If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled
    /// to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies
    /// MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler
    /// won't make it *more* imbalanced.
    /// It's a required field.
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub when_unsatisfiable: ::core::option::Option<::prost::alloc::string::String>,
    /// LabelSelector is used to find matching pods.
    /// Pods that match this label selector are counted to determine the number of pods
    /// in their corresponding topology domain.
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub label_selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
/// TypedLocalObjectReference contains enough information to let you locate the
/// typed referenced object inside the same namespace.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedLocalObjectReference {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub api_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Kind is the type of resource being referenced
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of resource being referenced
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Volume's name.
    /// Must be a DNS_LABEL and unique within the pod.
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names>
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// VolumeSource represents the location and type of the mounted volume.
    /// If not specified, the Volume is implied to be an EmptyDir.
    /// This implied behavior is deprecated and will be removed in a future version.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub volume_source: ::core::option::Option<VolumeSource>,
}
/// volumeDevice describes a mapping of a raw block device within a container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeDevice {
    /// name must match the name of a persistentVolumeClaim in the pod
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// devicePath is the path inside of the container that the device will be mapped to.
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub device_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// VolumeMount describes a mounting of a Volume within a container.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeMount {
    /// This must match the Name of a Volume.
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Mounted read-only if true, read-write otherwise (false or unspecified).
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="2")]
    #[serde(default)]
    pub read_only: ::core::option::Option<bool>,
    /// Path within the container at which the volume should be mounted.  Must
    /// not contain ':'.
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub mount_path: ::core::option::Option<::prost::alloc::string::String>,
    /// Path within the volume from which the container's volume should be mounted.
    /// Defaults to "" (volume's root).
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub sub_path: ::core::option::Option<::prost::alloc::string::String>,
    /// mountPropagation determines how mounts are propagated from the host
    /// to container and the other way around.
    /// When not set, MountPropagationNone is used.
    /// This field is beta in 1.10.
    /// +optional
    #[prost(string, optional, tag="5")]
    #[serde(default)]
    pub mount_propagation: ::core::option::Option<::prost::alloc::string::String>,
    /// Expanded path within the volume from which the container's volume should be mounted.
    /// Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment.
    /// Defaults to "" (volume's root).
    /// SubPathExpr and SubPath are mutually exclusive.
    /// +optional
    #[prost(string, optional, tag="6")]
    #[serde(default)]
    pub sub_path_expr: ::core::option::Option<::prost::alloc::string::String>,
}
/// VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeNodeAffinity {
    /// Required specifies hard node constraints that must be met.
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub required: ::core::option::Option<NodeSelector>,
}
/// Projection that may be projected along with other supported volume types
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeProjection {
    /// information about the secret data to project
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub secret: ::core::option::Option<SecretProjection>,
    /// information about the downwardAPI data to project
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub downward_api: ::core::option::Option<DownwardApiProjection>,
    /// information about the configMap data to project
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub config_map: ::core::option::Option<ConfigMapProjection>,
    /// information about the serviceAccountToken data to project
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub service_account_token: ::core::option::Option<ServiceAccountTokenProjection>,
}
/// Represents the source of a volume to mount.
/// Only one of its members may be specified.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeSource {
    /// HostPath represents a pre-existing file or directory on the host
    /// machine that is directly exposed to the container. This is generally
    /// used for system agents or other privileged things that are allowed
    /// to see the host machine. Most containers will NOT need this.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#hostpath>
    /// ---
    /// TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not
    /// mount host directories as read/write.
    /// +optional
    #[prost(message, optional, tag="1")]
    #[serde(default)]
    pub host_path: ::core::option::Option<HostPathVolumeSource>,
    /// EmptyDir represents a temporary directory that shares a pod's lifetime.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#emptydir>
    /// +optional
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub empty_dir: ::core::option::Option<EmptyDirVolumeSource>,
    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a
    /// kubelet's host machine and then exposed to the pod.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk>
    /// +optional
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub gce_persistent_disk: ::core::option::Option<GcePersistentDiskVolumeSource>,
    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a
    /// kubelet's host machine and then exposed to the pod.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore>
    /// +optional
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub aws_elastic_block_store: ::core::option::Option<AwsElasticBlockStoreVolumeSource>,
    /// GitRepo represents a git repository at a particular revision.
    /// DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an
    /// EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir
    /// into the Pod's container.
    /// +optional
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub git_repo: ::core::option::Option<GitRepoVolumeSource>,
    /// Secret represents a secret that should populate this volume.
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#secret>
    /// +optional
    #[prost(message, optional, tag="6")]
    #[serde(default)]
    pub secret: ::core::option::Option<SecretVolumeSource>,
    /// NFS represents an NFS mount on the host that shares a pod's lifetime
    /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#nfs>
    /// +optional
    #[prost(message, optional, tag="7")]
    #[serde(default)]
    pub nfs: ::core::option::Option<NfsVolumeSource>,
    /// ISCSI represents an ISCSI Disk resource that is attached to a
    /// kubelet's host machine and then exposed to the pod.
    /// More info: <https://examples.k8s.io/volumes/iscsi/README.md>
    /// +optional
    #[prost(message, optional, tag="8")]
    #[serde(default)]
    pub iscsi: ::core::option::Option<IscsiVolumeSource>,
    /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime.
    /// More info: <https://examples.k8s.io/volumes/glusterfs/README.md>
    /// +optional
    #[prost(message, optional, tag="9")]
    #[serde(default)]
    pub glusterfs: ::core::option::Option<GlusterfsVolumeSource>,
    /// PersistentVolumeClaimVolumeSource represents a reference to a
    /// PersistentVolumeClaim in the same namespace.
    /// More info: <https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims>
    /// +optional
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub persistent_volume_claim: ::core::option::Option<PersistentVolumeClaimVolumeSource>,
    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime.
    /// More info: <https://examples.k8s.io/volumes/rbd/README.md>
    /// +optional
    #[prost(message, optional, tag="11")]
    #[serde(default)]
    pub rbd: ::core::option::Option<RbdVolumeSource>,
    /// FlexVolume represents a generic volume resource that is
    /// provisioned/attached using an exec based plugin.
    /// +optional
    #[prost(message, optional, tag="12")]
    #[serde(default)]
    pub flex_volume: ::core::option::Option<FlexVolumeSource>,
    /// Cinder represents a cinder volume attached and mounted on kubelets host machine.
    /// More info: <https://examples.k8s.io/mysql-cinder-pd/README.md>
    /// +optional
    #[prost(message, optional, tag="13")]
    #[serde(default)]
    pub cinder: ::core::option::Option<CinderVolumeSource>,
    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    /// +optional
    #[prost(message, optional, tag="14")]
    #[serde(default)]
    pub cephfs: ::core::option::Option<CephFsVolumeSource>,
    /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
    /// +optional
    #[prost(message, optional, tag="15")]
    #[serde(default)]
    pub flocker: ::core::option::Option<FlockerVolumeSource>,
    /// DownwardAPI represents downward API about the pod that should populate this volume
    /// +optional
    #[prost(message, optional, tag="16")]
    #[serde(default)]
    pub downward_api: ::core::option::Option<DownwardApiVolumeSource>,
    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    /// +optional
    #[prost(message, optional, tag="17")]
    #[serde(default)]
    pub fc: ::core::option::Option<FcVolumeSource>,
    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    /// +optional
    #[prost(message, optional, tag="18")]
    #[serde(default)]
    pub azure_file: ::core::option::Option<AzureFileVolumeSource>,
    /// ConfigMap represents a configMap that should populate this volume
    /// +optional
    #[prost(message, optional, tag="19")]
    #[serde(default)]
    pub config_map: ::core::option::Option<ConfigMapVolumeSource>,
    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    /// +optional
    #[prost(message, optional, tag="20")]
    #[serde(default)]
    pub vsphere_volume: ::core::option::Option<VsphereVirtualDiskVolumeSource>,
    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    /// +optional
    #[prost(message, optional, tag="21")]
    #[serde(default)]
    pub quobyte: ::core::option::Option<QuobyteVolumeSource>,
    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    /// +optional
    #[prost(message, optional, tag="22")]
    #[serde(default)]
    pub azure_disk: ::core::option::Option<AzureDiskVolumeSource>,
    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    #[prost(message, optional, tag="23")]
    #[serde(default)]
    pub photon_persistent_disk: ::core::option::Option<PhotonPersistentDiskVolumeSource>,
    /// Items for all in one resources secrets, configmaps, and downward API
    #[prost(message, optional, tag="26")]
    #[serde(default)]
    pub projected: ::core::option::Option<ProjectedVolumeSource>,
    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    /// +optional
    #[prost(message, optional, tag="24")]
    #[serde(default)]
    pub portworx_volume: ::core::option::Option<PortworxVolumeSource>,
    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    /// +optional
    #[prost(message, optional, tag="25")]
    #[serde(default)]
    pub scale_io: ::core::option::Option<ScaleIoVolumeSource>,
    /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    /// +optional
    #[prost(message, optional, tag="27")]
    #[serde(default)]
    pub storageos: ::core::option::Option<StorageOsVolumeSource>,
    /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
    /// +optional
    #[prost(message, optional, tag="28")]
    #[serde(default)]
    pub csi: ::core::option::Option<CsiVolumeSource>,
    /// Ephemeral represents a volume that is handled by a cluster storage driver (Alpha feature).
    /// The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts,
    /// and deleted when the pod is removed.
    ///
    /// Use this if:
    /// a) the volume is only needed while the pod runs,
    /// b) features of normal volumes like restoring from snapshot or capacity
    ///    tracking are needed,
    /// c) the storage driver is specified through a storage class, and
    /// d) the storage driver supports dynamic volume provisioning through
    ///    a PersistentVolumeClaim (see EphemeralVolumeSource for more
    ///    information on the connection between this volume type
    ///    and PersistentVolumeClaim).
    ///
    /// Use PersistentVolumeClaim or one of the vendor-specific
    /// APIs for volumes that persist for longer than the lifecycle
    /// of an individual pod.
    ///
    /// Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to
    /// be used that way - see the documentation of the driver for
    /// more information.
    ///
    /// A pod can use both types of ephemeral volumes and
    /// persistent volumes at the same time.
    ///
    /// +optional
    #[prost(message, optional, tag="29")]
    #[serde(default)]
    pub ephemeral: ::core::option::Option<EphemeralVolumeSource>,
}
/// Represents a vSphere volume resource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VsphereVirtualDiskVolumeSource {
    /// Path that identifies vSphere volume vmdk
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub volume_path: ::core::option::Option<::prost::alloc::string::String>,
    /// Filesystem type to mount.
    /// Must be a filesystem type supported by the host operating system.
    /// Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub fs_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Storage Policy Based Management (SPBM) profile name.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub storage_policy_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
    /// +optional
    #[prost(string, optional, tag="4")]
    #[serde(default)]
    pub storage_policy_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedPodAffinityTerm {
    /// weight associated with matching the corresponding podAffinityTerm,
    /// in the range 1-100.
    #[prost(int32, optional, tag="1")]
    #[serde(default)]
    pub weight: ::core::option::Option<i32>,
    /// Required. A pod affinity term, associated with the corresponding weight.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub pod_affinity_term: ::core::option::Option<PodAffinityTerm>,
}
/// WindowsSecurityContextOptions contain Windows-specific options and credentials.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsSecurityContextOptions {
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    /// +optional
    #[prost(string, optional, tag="1")]
    #[serde(default)]
    pub gmsa_credential_spec_name: ::core::option::Option<::prost::alloc::string::String>,
    /// GMSACredentialSpec is where the GMSA admission webhook
    /// (<https://github.com/kubernetes-sigs/windows-gmsa>) inlines the contents of the
    /// GMSA credential spec named by the GMSACredentialSpecName field.
    /// +optional
    #[prost(string, optional, tag="2")]
    #[serde(default)]
    pub gmsa_credential_spec: ::core::option::Option<::prost::alloc::string::String>,
    /// The UserName in Windows to run the entrypoint of the container process.
    /// Defaults to the user specified in image metadata if unspecified.
    /// May also be set in PodSecurityContext. If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// +optional
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub run_as_user_name: ::core::option::Option<::prost::alloc::string::String>,
}
