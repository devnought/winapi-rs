// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! VSS header file
use shared::guiddef::{CLSID, GUID};
use shared::minwindef::{DWORD, INT, ULONG};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LONG, LONGLONG, WCHAR};
ENUM!{enum VSS_OBJECT_TYPE {
    VSS_OBJECT_UNKNOWN = 0,
    VSS_OBJECT_NONE = 1,
    VSS_OBJECT_SNAPSHOT_SET = 2,
    VSS_OBJECT_SNAPSHOT = 3,
    VSS_OBJECT_PROVIDER = 4,
    VSS_OBJECT_TYPE_COUNT = 5,
}}
pub type PVSS_OBJECT_TYPE = *mut VSS_OBJECT_TYPE;
ENUM!{enum VSS_SNAPSHOT_STATE {
    VSS_SS_UNKNOWN = 0x00,
    VSS_SS_PREPARING = 0x01,
    VSS_SS_PROCESSING_PREPARE = 0x02,
    VSS_SS_PREPARED = 0x03,
    VSS_SS_PROCESSING_PRECOMMIT = 0x04,
    VSS_SS_PRECOMMITTED = 0x05,
    VSS_SS_PROCESSING_COMMIT = 0x06,
    VSS_SS_COMMITTED = 0x07,
    VSS_SS_PROCESSING_POSTCOMMIT = 0x08,
    VSS_SS_PROCESSING_PREFINALCOMMIT = 0x09,
    VSS_SS_PREFINALCOMMITTED = 0x0a,
    VSS_SS_PROCESSING_POSTFINALCOMMIT = 0x0b,
    VSS_SS_CREATED = 0x0c,
    VSS_SS_ABORTED = 0x0d,
    VSS_SS_DELETED = 0x0e,
    VSS_SS_POSTCOMMITTED = 0x0f,
    VSS_SS_COUNT = 0x10,
}}
pub type PVSS_SNAPSHOT_STATE = *mut VSS_SNAPSHOT_STATE;
ENUM!{enum VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    VSS_VOLSNAP_ATTR_PERSISTENT = 0x00000001,
    VSS_VOLSNAP_ATTR_NO_AUTORECOVERY = 0x00000002,
    VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE = 0x00000004,
    VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE = 0x00000008,
    VSS_VOLSNAP_ATTR_NO_WRITERS = 0x00000010,
    VSS_VOLSNAP_ATTR_TRANSPORTABLE = 0x00000020,
    VSS_VOLSNAP_ATTR_NOT_SURFACED = 0x00000040,
    VSS_VOLSNAP_ATTR_NOT_TRANSACTED = 0x00000080,
    VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED = 0x00010000,
    VSS_VOLSNAP_ATTR_DIFFERENTIAL = 0x00020000,
    VSS_VOLSNAP_ATTR_PLEX = 0x00040000,
    VSS_VOLSNAP_ATTR_IMPORTED = 0x00080000,
    VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY = 0x00100000,
    VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY = 0x00200000,
    VSS_VOLSNAP_ATTR_AUTORECOVER = 0x00400000,
    VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY = 0x00800000,
    VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT = 0x01000000,
    VSS_VOLSNAP_ATTR_TXF_RECOVERY = 0x02000000,
    VSS_VOLSNAP_ATTR_FILE_SHARE = 0x04000000,
}}
pub type PVSS_VOLUME_SNAPSHOT_ATTRIBUTES = *mut VSS_VOLUME_SNAPSHOT_ATTRIBUTES;
ENUM!{enum VSS_SNAPSHOT_CONTEXT {
    VSS_CTX_BACKUP = 0,
    VSS_CTX_FILE_SHARE_BACKUP = VSS_VOLSNAP_ATTR_NO_WRITERS,
    VSS_CTX_NAS_ROLLBACK = VSS_VOLSNAP_ATTR_PERSISTENT
        | VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE | VSS_VOLSNAP_ATTR_NO_WRITERS,
    VSS_CTX_APP_ROLLBACK = VSS_VOLSNAP_ATTR_PERSISTENT
        | VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE,
    VSS_CTX_CLIENT_ACCESSIBLE = VSS_VOLSNAP_ATTR_PERSISTENT
        | VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE | VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE
        | VSS_VOLSNAP_ATTR_NO_WRITERS,
    VSS_CTX_CLIENT_ACCESSIBLE_WRITERS = VSS_VOLSNAP_ATTR_PERSISTENT
        | VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE | VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE,
    VSS_CTX_ALL = 0xffffffff,
}}
pub type PVSS_SNAPSHOT_CONTEXT = *mut VSS_SNAPSHOT_CONTEXT;
ENUM!{enum VSS_PROVIDER_CAPABILITIES {
    VSS_PRV_CAPABILITY_LEGACY = 0x1,
    VSS_PRV_CAPABILITY_COMPLIANT = 0x2,
    VSS_PRV_CAPABILITY_LUN_REPOINT = 0x4,
    VSS_PRV_CAPABILITY_LUN_RESYNC = 0x8,
    VSS_PRV_CAPABILITY_OFFLINE_CREATION = 0x10,
    VSS_PRV_CAPABILITY_MULTIPLE_IMPORT = 0x20,
    VSS_PRV_CAPABILITY_RECYCLING = 0x40,
    VSS_PRV_CAPABILITY_PLEX = 0x80,
    VSS_PRV_CAPABILITY_DIFFERENTIAL = 0x100,
    VSS_PRV_CAPABILITY_CLUSTERED = 0x200,
}}
pub type PVSS_PROVIDER_CAPABILITIES = *mut VSS_PROVIDER_CAPABILITIES;
ENUM!{enum VSS_HARDWARE_OPTIONS {
    VSS_BREAKEX_FLAG_MASK_LUNS = 0x1,
    VSS_BREAKEX_FLAG_MAKE_READ_WRITE = 0x2,
    VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL = 0x4,
    VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE = 0x8,
    VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE = 0x100,
    VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY = 0x200,
    VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY = 0x400,
    VSS_ONLUNSTATECHANGE_DO_MASK_LUNS = 0x800,
}}
pub type PVSS_HARDWARE_OPTIONS = *mut VSS_HARDWARE_OPTIONS;
ENUM!{enum VSS_RECOVERY_OPTIONS {
    VSS_RECOVERY_REVERT_IDENTITY_ALL = 0x00000100,
    VSS_RECOVERY_NO_VOLUME_CHECK = 0x00000200,
}}
pub type PVSS_RECOVERY_OPTIONS = *mut VSS_RECOVERY_OPTIONS;
ENUM!{enum VSS_WRITER_STATE {
    VSS_WS_UNKNOWN = 0,
    VSS_WS_STABLE = 1,
    VSS_WS_WAITING_FOR_FREEZE = 2,
    VSS_WS_WAITING_FOR_THAW = 3,
    VSS_WS_WAITING_FOR_POST_SNAPSHOT = 4,
    VSS_WS_WAITING_FOR_BACKUP_COMPLETE = 5,
    VSS_WS_FAILED_AT_IDENTIFY = 6,
    VSS_WS_FAILED_AT_PREPARE_BACKUP = 7,
    VSS_WS_FAILED_AT_PREPARE_SNAPSHOT = 8,
    VSS_WS_FAILED_AT_FREEZE = 9,
    VSS_WS_FAILED_AT_THAW = 10,
    VSS_WS_FAILED_AT_POST_SNAPSHOT = 11,
    VSS_WS_FAILED_AT_BACKUP_COMPLETE = 12,
    VSS_WS_FAILED_AT_PRE_RESTORE = 13,
    VSS_WS_FAILED_AT_POST_RESTORE = 14,
    VSS_WS_FAILED_AT_BACKUPSHUTDOWN = 15,
    VSS_WS_COUNT = 16,
}}
pub type PVSS_WRITER_STATE = *mut VSS_WRITER_STATE;
ENUM!{enum VSS_BACKUP_TYPE {
    VSS_BT_UNDEFINED = 0,
    VSS_BT_FULL = 1,
    VSS_BT_INCREMENTAL = 2,
    VSS_BT_DIFFERENTIAL = 3,
    VSS_BT_LOG = 4,
    VSS_BT_COPY = 5,
    VSS_BT_OTHER = 6,
}}
pub type PVSS_BACKUP_TYPE = *mut VSS_BACKUP_TYPE;
ENUM!{enum VSS_RESTORE_TYPE {
    VSS_RTYPE_UNDEFINED = 0,
    VSS_RTYPE_BY_COPY = 1,
    VSS_RTYPE_IMPORT = 2,
    VSS_RTYPE_OTHER = 3,
}}
pub type PVSS_RESTORE_TYPE = *mut VSS_RESTORE_TYPE;
ENUM!{enum VSS_ROLLFORWARD_TYPE {
    VSS_RF_UNDEFINED = 0,
    VSS_RF_NONE = 1,
    VSS_RF_ALL = 2,
    VSS_RF_PARTIAL = 3,
}}
pub type PVSS_ROLLFORWARD_TYPE = *mut VSS_ROLLFORWARD_TYPE;
ENUM!{enum VSS_PROVIDER_TYPE {
    VSS_PROV_UNKNOWN = 0,
    VSS_PROV_SYSTEM = 1,
    VSS_PROV_SOFTWARE = 2,
    VSS_PROV_HARDWARE = 3,
    VSS_PROV_FILESHARE = 4,
}}
pub type PVSS_PROVIDER_TYPE = *mut VSS_PROVIDER_TYPE;
ENUM!{enum VSS_APPLICATION_LEVEL {
    VSS_APP_UNKNOWN = 0,
    VSS_APP_SYSTEM = 1,
    VSS_APP_BACK_END = 2,
    VSS_APP_FRONT_END = 3,
    VSS_APP_SYSTEM_RM = 4,
    VSS_APP_AUTO = -1i32 as u32,
}}
pub type PVSS_APPLICATION_LEVEL = *mut VSS_APPLICATION_LEVEL;
ENUM!{enum _VSS_SNAPSHOT_COMPATIBILITY {
    VSS_SC_DISABLE_DEFRAG = 0x1,
    VSS_SC_DISABLE_CONTENTINDEX = 0x2,
}}
ENUM!{enum VSS_SNAPSHOT_PROPERTY_ID {
    VSS_SPROPID_UNKNOWN = 0,
    VSS_SPROPID_SNAPSHOT_ID = 0x1,
    VSS_SPROPID_SNAPSHOT_SET_ID = 0x2,
    VSS_SPROPID_SNAPSHOTS_COUNT = 0x3,
    VSS_SPROPID_SNAPSHOT_DEVICE = 0x4,
    VSS_SPROPID_ORIGINAL_VOLUME = 0x5,
    VSS_SPROPID_ORIGINATING_MACHINE = 0x6,
    VSS_SPROPID_SERVICE_MACHINE = 0x7,
    VSS_SPROPID_EXPOSED_NAME = 0x8,
    VSS_SPROPID_EXPOSED_PATH = 0x9,
    VSS_SPROPID_PROVIDER_ID = 0xa,
    VSS_SPROPID_SNAPSHOT_ATTRIBUTES = 0xb,
    VSS_SPROPID_CREATION_TIMESTAMP = 0xc,
    VSS_SPROPID_STATUS = 0xd,
}}
pub type PVSS_SNAPSHOT_PROPERTY_ID = *mut VSS_SNAPSHOT_PROPERTY_ID;
ENUM!{enum VSS_FILE_SPEC_BACKUP_TYPE {
    VSS_FSBT_FULL_BACKUP_REQUIRED = 0x1,
    VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED = 0x2,
    VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED = 0x4,
    VSS_FSBT_LOG_BACKUP_REQUIRED = 0x8,
    VSS_FSBT_FULL_SNAPSHOT_REQUIRED = 0x100,
    VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED = 0x200,
    VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED = 0x400,
    VSS_FSBT_LOG_SNAPSHOT_REQUIRED = 0x800,
    VSS_FSBT_CREATED_DURING_BACKUP = 0x10000,
    VSS_FSBT_ALL_BACKUP_REQUIRED = 0xf,
    VSS_FSBT_ALL_SNAPSHOT_REQUIRED = 0xf00,
}}
pub type PVSS_FILE_SPEC_BACKUP_TYPE = *mut VSS_FILE_SPEC_BACKUP_TYPE;
ENUM!{enum VSS_BACKUP_SCHEMA {
    VSS_BS_UNDEFINED = 0,
    VSS_BS_DIFFERENTIAL = 0x1,
    VSS_BS_INCREMENTAL = 0x2,
    VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL = 0x4,
    VSS_BS_LOG = 0x8,
    VSS_BS_COPY = 0x10,
    VSS_BS_TIMESTAMPED = 0x20,
    VSS_BS_LAST_MODIFY = 0x40,
    VSS_BS_LSN = 0x80,
    VSS_BS_WRITER_SUPPORTS_NEW_TARGET = 0x100,
    VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE = 0x200,
    VSS_BS_INDEPENDENT_SYSTEM_STATE = 0x400,
    VSS_BS_ROLLFORWARD_RESTORE = 0x1000,
    VSS_BS_RESTORE_RENAME = 0x2000,
    VSS_BS_AUTHORITATIVE_RESTORE = 0x4000,
    VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES = 0x8000,
}}
pub type PVSS_BACKUP_SCHEMA = *mut VSS_BACKUP_SCHEMA;
pub type VSS_ID = GUID;
pub type VSS_PWSZ = *mut WCHAR;
pub type VSS_TIMESTAMP = LONGLONG;
STRUCT!{struct VSS_SNAPSHOT_PROP {
    m_SnapshotId: VSS_ID,
    m_SnapshotSetId: VSS_ID,
    m_lSnapshotsCount: LONG,
    m_pwszSnapshotDeviceObject: VSS_PWSZ,
    m_pwszOriginalVolumeName: VSS_PWSZ,
    m_pwszOriginatingMachine: VSS_PWSZ,
    m_pwszServiceMachine: VSS_PWSZ,
    m_pwszExposedName: VSS_PWSZ,
    m_pwszExposedPath: VSS_PWSZ,
    m_ProviderId: VSS_ID,
    m_lSnapshotAttributes: LONG,
    m_tsCreationTimestamp: VSS_TIMESTAMP,
    m_eStatus: VSS_SNAPSHOT_STATE,
}}
pub type PVSS_SNAPSHOT_PROP = *mut VSS_SNAPSHOT_PROP;
STRUCT!{struct VSS_PROVIDER_PROP {
    m_ProviderId: VSS_ID,
    m_pwszProviderName: VSS_PWSZ,
    m_eProviderType: VSS_PROVIDER_TYPE,
    m_pwszProviderVersion: VSS_PWSZ,
    m_ProviderVersionId: VSS_ID,
    m_ClassId: CLSID,
}}
pub type PVSS_PROVIDER_PROP = *mut VSS_PROVIDER_PROP;
UNION!{union VSS_OBJECT_UNION {
    [u64; 12] [u64; 16],
    Snap Snap_mut: VSS_SNAPSHOT_PROP,
    Prov Prov_mut: VSS_PROVIDER_PROP,
}}
STRUCT!{struct VSS_OBJECT_PROP {
    Type: VSS_OBJECT_TYPE,
    Obj: VSS_OBJECT_UNION,
}}
pub type PVSS_OBJECT_PROP = *mut VSS_OBJECT_PROP;
RIDL!(
#[uuid(0xae1c7110, 0x2f60, 0x11d3, 0x8a, 0x39, 0x00, 0xc0, 0x4f, 0x72, 0xd8, 0xe3)]
interface IVssEnumObject(IVssEnumObjectVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: *mut VSS_OBJECT_PROP,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IVssEnumObject,
    ) -> HRESULT,
}
);
RIDL!(
#[uuid(0x507c37b4, 0xcf5b, 0x4e95, 0xb0, 0xaf, 0x14, 0xeb, 0x97, 0x67, 0x46, 0x7e)]
interface IVssAsync(IVssAsyncVtbl): IUnknown(IUnknownVtbl) {
    fn Cancel() -> HRESULT,
    fn Wait(
        dwMilliseconds: DWORD,
    ) -> HRESULT,
    fn QueryStatus(
        pHrResult: *mut HRESULT,
        pReserved: *mut INT,
    ) -> HRESULT,
}
);
