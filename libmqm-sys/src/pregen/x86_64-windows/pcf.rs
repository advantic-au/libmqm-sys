/* Generated with MQ client version 9.4.4.0 */

pub type PMQCFH = *mut MQCFH;
pub type PMQCFBF = *mut MQCFBF;
pub type PMQCFBS = *mut MQCFBS;
pub type PMQCFGR = *mut MQCFGR;
pub type PMQCFIF = *mut MQCFIF;
pub type PMQCFIL = *mut MQCFIL;
pub type PMQCFIL64 = *mut MQCFIL64;
pub type PMQCFIN = *mut MQCFIN;
pub type PMQCFIN64 = *mut MQCFIN64;
pub type PMQCFSF = *mut MQCFSF;
pub type PMQCFSL = *mut MQCFSL;
pub type PMQCFST = *mut MQCFST;
pub type PMQEPH = *mut MQEPH;
/// PCF Header
///
/// # References
/// * [IBM `MQCFH` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049400_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFH {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Command identifier
    pub Command: crate::MQLONG,
    /// Message sequence number
    pub MsgSeqNumber: crate::MQLONG,
    /// Control options
    pub Control: crate::MQLONG,
    /// Completion code
    pub CompCode: crate::MQLONG,
    /// Reason code qualifying completion code
    pub Reason: crate::MQLONG,
    /// Count of parameter structures
    pub ParameterCount: crate::MQLONG,
}
/// PCF Byte String Filter Parameter
///
/// # References
/// * [IBM `MQCFBF` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q088610_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFBF {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Operator identifier
    pub Operator: crate::MQLONG,
    /// Filter value length
    pub FilterValueLength: crate::MQLONG,
    /// Filter value -- first byte
    pub FilterValue: [crate::MQBYTE; 1usize],
}
/// PCF Byte String Parameter
///
/// # References
/// * [IBM `MQCFBS` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049380_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFBS {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Length of string
    pub StringLength: crate::MQLONG,
    /// String value -- first byte
    pub String: [crate::MQBYTE; 1usize],
}
/// PCF Group Parameter
///
/// # References
/// * [IBM `MQCFGR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049390_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFGR {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Count of group parameter structures
    pub ParameterCount: crate::MQLONG,
}
/// PCF Integer Filter Parameter
///
/// # References
/// * [IBM `MQCFIF` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q088630_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFIF {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Operator identifier
    pub Operator: crate::MQLONG,
    /// Filter value
    pub FilterValue: crate::MQLONG,
}
/// PCF Integer-List Parameter
///
/// # References
/// * [IBM `MQCFIL` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049410_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFIL {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Count of parameter values
    pub Count: crate::MQLONG,
    /// Parameter values -- first element
    pub Values: [crate::MQLONG; 1usize],
}
/// PCF 64-bit Integer-List Parameter
///
/// # References
/// * [IBM `MQCFIL64` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049420_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFIL64 {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Count of parameter values
    pub Count: crate::MQLONG,
    /// Parameter values -- first element
    pub Values: [crate::MQINT64; 1usize],
}
/// PCF Integer Parameter
///
/// # References
/// * [IBM `MQCFIN` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049430_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFIN {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Parameter value
    pub Value: crate::MQLONG,
}
/// PCF 64-bit Integer Parameter
///
/// # References
/// * [IBM `MQCFIN64` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049440_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFIN64 {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Reserved
    pub Reserved: crate::MQLONG,
    /// Parameter value
    pub Value: crate::MQINT64,
}
/// PCF String Filter Parameter
///
/// # References
/// * [IBM `MQCFSF` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q088660_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFSF {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Operator identifier
    pub Operator: crate::MQLONG,
    /// Coded character set identifier
    pub CodedCharSetId: crate::MQLONG,
    /// Filter value length
    pub FilterValueLength: crate::MQLONG,
    /// Filter value -- first character
    pub FilterValue: [crate::MQCHAR; 1usize],
}
/// PCF String-List Parameter
///
/// # References
/// * [IBM `MQCFSL` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049450_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFSL {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Coded character set identifier
    pub CodedCharSetId: crate::MQLONG,
    /// Count of parameter values
    pub Count: crate::MQLONG,
    /// Length of one string
    pub StringLength: crate::MQLONG,
    /// String values -- first character
    pub Strings: [crate::MQCHAR; 1usize],
}
/// PCF String Parameter
///
/// # References
/// * [IBM `MQCFST` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049460_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCFST {
    /// Structure type
    pub Type: crate::MQLONG,
    /// Structure length
    pub StrucLength: crate::MQLONG,
    /// Parameter identifier
    pub Parameter: crate::MQLONG,
    /// Coded character set identifier
    pub CodedCharSetId: crate::MQLONG,
    /// Length of string
    pub StringLength: crate::MQLONG,
    /// String value -- first character
    pub String: [crate::MQCHAR; 1usize],
}
/// Embedded PCF header
///
/// # References
/// * [IBM `MQEPH` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049470_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQEPH {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Total length of [`MQEPH`] including [`MQCFH`] and parameter structures that follow
    pub StrucLength: crate::MQLONG,
    /// Numeric encoding of data that follows last PCF parameter structure
    pub Encoding: crate::MQLONG,
    /// Character set identifier of data that follows last PCF parameter structure
    pub CodedCharSetId: crate::MQLONG,
    /// Format name of data that follows last PCF parameter structure
    pub Format: crate::MQCHAR8,
    /// Flags
    pub Flags: crate::MQLONG,
    /// Programmable Command Format Header
    pub PCFHeader: MQCFH,
}
pub const MQCFH_STRUC_LENGTH: usize = 36;
pub const MQCFH_VERSION_1: crate::MQLONG = 1;
pub const MQCFH_VERSION_2: crate::MQLONG = 2;
pub const MQCFH_VERSION_3: crate::MQLONG = 3;
pub const MQCFH_CURRENT_VERSION: crate::MQLONG = 3;
pub const MQCMD_NONE: crate::MQLONG = 0;
pub const MQCMD_CHANGE_Q_MGR: crate::MQLONG = 1;
pub const MQCMD_INQUIRE_Q_MGR: crate::MQLONG = 2;
pub const MQCMD_CHANGE_PROCESS: crate::MQLONG = 3;
pub const MQCMD_COPY_PROCESS: crate::MQLONG = 4;
pub const MQCMD_CREATE_PROCESS: crate::MQLONG = 5;
pub const MQCMD_DELETE_PROCESS: crate::MQLONG = 6;
pub const MQCMD_INQUIRE_PROCESS: crate::MQLONG = 7;
pub const MQCMD_CHANGE_Q: crate::MQLONG = 8;
pub const MQCMD_CLEAR_Q: crate::MQLONG = 9;
pub const MQCMD_COPY_Q: crate::MQLONG = 10;
pub const MQCMD_CREATE_Q: crate::MQLONG = 11;
pub const MQCMD_DELETE_Q: crate::MQLONG = 12;
pub const MQCMD_INQUIRE_Q: crate::MQLONG = 13;
pub const MQCMD_REFRESH_Q_MGR: crate::MQLONG = 16;
pub const MQCMD_RESET_Q_STATS: crate::MQLONG = 17;
pub const MQCMD_INQUIRE_Q_NAMES: crate::MQLONG = 18;
pub const MQCMD_INQUIRE_PROCESS_NAMES: crate::MQLONG = 19;
pub const MQCMD_INQUIRE_CHANNEL_NAMES: crate::MQLONG = 20;
pub const MQCMD_CHANGE_CHANNEL: crate::MQLONG = 21;
pub const MQCMD_COPY_CHANNEL: crate::MQLONG = 22;
pub const MQCMD_CREATE_CHANNEL: crate::MQLONG = 23;
pub const MQCMD_DELETE_CHANNEL: crate::MQLONG = 24;
pub const MQCMD_INQUIRE_CHANNEL: crate::MQLONG = 25;
pub const MQCMD_PING_CHANNEL: crate::MQLONG = 26;
pub const MQCMD_RESET_CHANNEL: crate::MQLONG = 27;
pub const MQCMD_START_CHANNEL: crate::MQLONG = 28;
pub const MQCMD_STOP_CHANNEL: crate::MQLONG = 29;
pub const MQCMD_START_CHANNEL_INIT: crate::MQLONG = 30;
pub const MQCMD_START_CHANNEL_LISTENER: crate::MQLONG = 31;
pub const MQCMD_CHANGE_NAMELIST: crate::MQLONG = 32;
pub const MQCMD_COPY_NAMELIST: crate::MQLONG = 33;
pub const MQCMD_CREATE_NAMELIST: crate::MQLONG = 34;
pub const MQCMD_DELETE_NAMELIST: crate::MQLONG = 35;
pub const MQCMD_INQUIRE_NAMELIST: crate::MQLONG = 36;
pub const MQCMD_INQUIRE_NAMELIST_NAMES: crate::MQLONG = 37;
pub const MQCMD_ESCAPE: crate::MQLONG = 38;
pub const MQCMD_RESOLVE_CHANNEL: crate::MQLONG = 39;
pub const MQCMD_PING_Q_MGR: crate::MQLONG = 40;
pub const MQCMD_INQUIRE_Q_STATUS: crate::MQLONG = 41;
pub const MQCMD_INQUIRE_CHANNEL_STATUS: crate::MQLONG = 42;
pub const MQCMD_CONFIG_EVENT: crate::MQLONG = 43;
pub const MQCMD_Q_MGR_EVENT: crate::MQLONG = 44;
pub const MQCMD_PERFM_EVENT: crate::MQLONG = 45;
pub const MQCMD_CHANNEL_EVENT: crate::MQLONG = 46;
pub const MQCMD_DELETE_PUBLICATION: crate::MQLONG = 60;
pub const MQCMD_DEREGISTER_PUBLISHER: crate::MQLONG = 61;
pub const MQCMD_DEREGISTER_SUBSCRIBER: crate::MQLONG = 62;
pub const MQCMD_PUBLISH: crate::MQLONG = 63;
pub const MQCMD_REGISTER_PUBLISHER: crate::MQLONG = 64;
pub const MQCMD_REGISTER_SUBSCRIBER: crate::MQLONG = 65;
pub const MQCMD_REQUEST_UPDATE: crate::MQLONG = 66;
pub const MQCMD_BROKER_INTERNAL: crate::MQLONG = 67;
pub const MQCMD_ACTIVITY_MSG: crate::MQLONG = 69;
pub const MQCMD_INQUIRE_CLUSTER_Q_MGR: crate::MQLONG = 70;
pub const MQCMD_RESUME_Q_MGR_CLUSTER: crate::MQLONG = 71;
pub const MQCMD_SUSPEND_Q_MGR_CLUSTER: crate::MQLONG = 72;
pub const MQCMD_REFRESH_CLUSTER: crate::MQLONG = 73;
pub const MQCMD_RESET_CLUSTER: crate::MQLONG = 74;
pub const MQCMD_TRACE_ROUTE: crate::MQLONG = 75;
pub const MQCMD_REFRESH_SECURITY: crate::MQLONG = 78;
pub const MQCMD_CHANGE_AUTH_INFO: crate::MQLONG = 79;
pub const MQCMD_COPY_AUTH_INFO: crate::MQLONG = 80;
pub const MQCMD_CREATE_AUTH_INFO: crate::MQLONG = 81;
pub const MQCMD_DELETE_AUTH_INFO: crate::MQLONG = 82;
pub const MQCMD_INQUIRE_AUTH_INFO: crate::MQLONG = 83;
pub const MQCMD_INQUIRE_AUTH_INFO_NAMES: crate::MQLONG = 84;
pub const MQCMD_INQUIRE_CONNECTION: crate::MQLONG = 85;
pub const MQCMD_STOP_CONNECTION: crate::MQLONG = 86;
pub const MQCMD_INQUIRE_AUTH_RECS: crate::MQLONG = 87;
pub const MQCMD_INQUIRE_ENTITY_AUTH: crate::MQLONG = 88;
pub const MQCMD_DELETE_AUTH_REC: crate::MQLONG = 89;
pub const MQCMD_SET_AUTH_REC: crate::MQLONG = 90;
pub const MQCMD_LOGGER_EVENT: crate::MQLONG = 91;
pub const MQCMD_RESET_Q_MGR: crate::MQLONG = 92;
pub const MQCMD_CHANGE_LISTENER: crate::MQLONG = 93;
pub const MQCMD_COPY_LISTENER: crate::MQLONG = 94;
pub const MQCMD_CREATE_LISTENER: crate::MQLONG = 95;
pub const MQCMD_DELETE_LISTENER: crate::MQLONG = 96;
pub const MQCMD_INQUIRE_LISTENER: crate::MQLONG = 97;
pub const MQCMD_INQUIRE_LISTENER_STATUS: crate::MQLONG = 98;
pub const MQCMD_COMMAND_EVENT: crate::MQLONG = 99;
pub const MQCMD_CHANGE_SECURITY: crate::MQLONG = 100;
pub const MQCMD_CHANGE_CF_STRUC: crate::MQLONG = 101;
pub const MQCMD_CHANGE_STG_CLASS: crate::MQLONG = 102;
pub const MQCMD_CHANGE_TRACE: crate::MQLONG = 103;
pub const MQCMD_ARCHIVE_LOG: crate::MQLONG = 104;
pub const MQCMD_BACKUP_CF_STRUC: crate::MQLONG = 105;
pub const MQCMD_CREATE_BUFFER_POOL: crate::MQLONG = 106;
pub const MQCMD_CREATE_PAGE_SET: crate::MQLONG = 107;
pub const MQCMD_CREATE_CF_STRUC: crate::MQLONG = 108;
pub const MQCMD_CREATE_STG_CLASS: crate::MQLONG = 109;
pub const MQCMD_COPY_CF_STRUC: crate::MQLONG = 110;
pub const MQCMD_COPY_STG_CLASS: crate::MQLONG = 111;
pub const MQCMD_DELETE_CF_STRUC: crate::MQLONG = 112;
pub const MQCMD_DELETE_STG_CLASS: crate::MQLONG = 113;
pub const MQCMD_INQUIRE_ARCHIVE: crate::MQLONG = 114;
pub const MQCMD_INQUIRE_CF_STRUC: crate::MQLONG = 115;
pub const MQCMD_INQUIRE_CF_STRUC_STATUS: crate::MQLONG = 116;
pub const MQCMD_INQUIRE_CMD_SERVER: crate::MQLONG = 117;
pub const MQCMD_INQUIRE_CHANNEL_INIT: crate::MQLONG = 118;
pub const MQCMD_INQUIRE_QSG: crate::MQLONG = 119;
pub const MQCMD_INQUIRE_LOG: crate::MQLONG = 120;
pub const MQCMD_INQUIRE_SECURITY: crate::MQLONG = 121;
pub const MQCMD_INQUIRE_STG_CLASS: crate::MQLONG = 122;
pub const MQCMD_INQUIRE_SYSTEM: crate::MQLONG = 123;
pub const MQCMD_INQUIRE_THREAD: crate::MQLONG = 124;
pub const MQCMD_INQUIRE_TRACE: crate::MQLONG = 125;
pub const MQCMD_INQUIRE_USAGE: crate::MQLONG = 126;
pub const MQCMD_MOVE_Q: crate::MQLONG = 127;
pub const MQCMD_RECOVER_BSDS: crate::MQLONG = 128;
pub const MQCMD_RECOVER_CF_STRUC: crate::MQLONG = 129;
pub const MQCMD_RESET_TPIPE: crate::MQLONG = 130;
pub const MQCMD_RESOLVE_INDOUBT: crate::MQLONG = 131;
pub const MQCMD_RESUME_Q_MGR: crate::MQLONG = 132;
pub const MQCMD_REVERIFY_SECURITY: crate::MQLONG = 133;
pub const MQCMD_SET_ARCHIVE: crate::MQLONG = 134;
pub const MQCMD_SET_LOG: crate::MQLONG = 136;
pub const MQCMD_SET_SYSTEM: crate::MQLONG = 137;
pub const MQCMD_START_CMD_SERVER: crate::MQLONG = 138;
pub const MQCMD_START_Q_MGR: crate::MQLONG = 139;
pub const MQCMD_START_TRACE: crate::MQLONG = 140;
pub const MQCMD_STOP_CHANNEL_INIT: crate::MQLONG = 141;
pub const MQCMD_STOP_CHANNEL_LISTENER: crate::MQLONG = 142;
pub const MQCMD_STOP_CMD_SERVER: crate::MQLONG = 143;
pub const MQCMD_STOP_Q_MGR: crate::MQLONG = 144;
pub const MQCMD_STOP_TRACE: crate::MQLONG = 145;
pub const MQCMD_SUSPEND_Q_MGR: crate::MQLONG = 146;
pub const MQCMD_INQUIRE_CF_STRUC_NAMES: crate::MQLONG = 147;
pub const MQCMD_INQUIRE_STG_CLASS_NAMES: crate::MQLONG = 148;
pub const MQCMD_CHANGE_SERVICE: crate::MQLONG = 149;
pub const MQCMD_COPY_SERVICE: crate::MQLONG = 150;
pub const MQCMD_CREATE_SERVICE: crate::MQLONG = 151;
pub const MQCMD_DELETE_SERVICE: crate::MQLONG = 152;
pub const MQCMD_INQUIRE_SERVICE: crate::MQLONG = 153;
pub const MQCMD_INQUIRE_SERVICE_STATUS: crate::MQLONG = 154;
pub const MQCMD_START_SERVICE: crate::MQLONG = 155;
pub const MQCMD_STOP_SERVICE: crate::MQLONG = 156;
pub const MQCMD_DELETE_BUFFER_POOL: crate::MQLONG = 157;
pub const MQCMD_DELETE_PAGE_SET: crate::MQLONG = 158;
pub const MQCMD_CHANGE_BUFFER_POOL: crate::MQLONG = 159;
pub const MQCMD_CHANGE_PAGE_SET: crate::MQLONG = 160;
pub const MQCMD_INQUIRE_Q_MGR_STATUS: crate::MQLONG = 161;
pub const MQCMD_CREATE_LOG: crate::MQLONG = 162;
pub const MQCMD_STATISTICS_MQI: crate::MQLONG = 164;
pub const MQCMD_STATISTICS_Q: crate::MQLONG = 165;
pub const MQCMD_STATISTICS_CHANNEL: crate::MQLONG = 166;
pub const MQCMD_ACCOUNTING_MQI: crate::MQLONG = 167;
pub const MQCMD_ACCOUNTING_Q: crate::MQLONG = 168;
pub const MQCMD_INQUIRE_AUTH_SERVICE: crate::MQLONG = 169;
pub const MQCMD_CHANGE_TOPIC: crate::MQLONG = 170;
pub const MQCMD_COPY_TOPIC: crate::MQLONG = 171;
pub const MQCMD_CREATE_TOPIC: crate::MQLONG = 172;
pub const MQCMD_DELETE_TOPIC: crate::MQLONG = 173;
pub const MQCMD_INQUIRE_TOPIC: crate::MQLONG = 174;
pub const MQCMD_INQUIRE_TOPIC_NAMES: crate::MQLONG = 175;
pub const MQCMD_INQUIRE_SUBSCRIPTION: crate::MQLONG = 176;
pub const MQCMD_CREATE_SUBSCRIPTION: crate::MQLONG = 177;
pub const MQCMD_CHANGE_SUBSCRIPTION: crate::MQLONG = 178;
pub const MQCMD_DELETE_SUBSCRIPTION: crate::MQLONG = 179;
pub const MQCMD_COPY_SUBSCRIPTION: crate::MQLONG = 181;
pub const MQCMD_INQUIRE_SUB_STATUS: crate::MQLONG = 182;
pub const MQCMD_INQUIRE_TOPIC_STATUS: crate::MQLONG = 183;
pub const MQCMD_CLEAR_TOPIC_STRING: crate::MQLONG = 184;
pub const MQCMD_INQUIRE_PUBSUB_STATUS: crate::MQLONG = 185;
pub const MQCMD_INQUIRE_SMDS: crate::MQLONG = 186;
pub const MQCMD_CHANGE_SMDS: crate::MQLONG = 187;
pub const MQCMD_RESET_SMDS: crate::MQLONG = 188;
pub const MQCMD_CREATE_COMM_INFO: crate::MQLONG = 190;
pub const MQCMD_INQUIRE_COMM_INFO: crate::MQLONG = 191;
pub const MQCMD_CHANGE_COMM_INFO: crate::MQLONG = 192;
pub const MQCMD_COPY_COMM_INFO: crate::MQLONG = 193;
pub const MQCMD_DELETE_COMM_INFO: crate::MQLONG = 194;
pub const MQCMD_PURGE_CHANNEL: crate::MQLONG = 195;
pub const MQCMD_MQXR_DIAGNOSTICS: crate::MQLONG = 196;
pub const MQCMD_START_SMDSCONN: crate::MQLONG = 197;
pub const MQCMD_STOP_SMDSCONN: crate::MQLONG = 198;
pub const MQCMD_INQUIRE_SMDSCONN: crate::MQLONG = 199;
pub const MQCMD_INQUIRE_MQXR_STATUS: crate::MQLONG = 200;
pub const MQCMD_START_CLIENT_TRACE: crate::MQLONG = 201;
pub const MQCMD_STOP_CLIENT_TRACE: crate::MQLONG = 202;
pub const MQCMD_SET_CHLAUTH_REC: crate::MQLONG = 203;
pub const MQCMD_INQUIRE_CHLAUTH_RECS: crate::MQLONG = 204;
pub const MQCMD_INQUIRE_PROT_POLICY: crate::MQLONG = 205;
pub const MQCMD_CREATE_PROT_POLICY: crate::MQLONG = 206;
pub const MQCMD_DELETE_PROT_POLICY: crate::MQLONG = 207;
pub const MQCMD_CHANGE_PROT_POLICY: crate::MQLONG = 208;
pub const MQCMD_SET_PROT_POLICY: crate::MQLONG = 208;
pub const MQCMD_ACTIVITY_TRACE: crate::MQLONG = 209;
pub const MQCMD_RESET_CF_STRUC: crate::MQLONG = 213;
pub const MQCMD_INQUIRE_XR_CAPABILITY: crate::MQLONG = 214;
pub const MQCMD_INQUIRE_AMQP_CAPABILITY: crate::MQLONG = 216;
pub const MQCMD_AMQP_DIAGNOSTICS: crate::MQLONG = 217;
pub const MQCMD_INTER_Q_MGR_STATUS: crate::MQLONG = 218;
pub const MQCMD_INTER_Q_MGR_BALANCE: crate::MQLONG = 219;
pub const MQCMD_INQUIRE_APPL_STATUS: crate::MQLONG = 220;
pub const MQCFC_LAST: crate::MQLONG = 1;
pub const MQCFC_NOT_LAST: crate::MQLONG = 0;
/// [IBM `MQRCCF_CFH_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046050_.html)
pub const MQRCCF_CFH_TYPE_ERROR: crate::MQLONG = 3001;
/// [IBM `MQRCCF_CFH_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046060_.html)
pub const MQRCCF_CFH_LENGTH_ERROR: crate::MQLONG = 3002;
/// [IBM `MQRCCF_CFH_VERSION_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046070_.html)
pub const MQRCCF_CFH_VERSION_ERROR: crate::MQLONG = 3003;
/// [IBM `MQRCCF_CFH_MSG_SEQ_NUMBER_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046080_.html)
pub const MQRCCF_CFH_MSG_SEQ_NUMBER_ERR: crate::MQLONG = 3004;
/// [IBM `MQRCCF_CFH_CONTROL_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046090_.html)
pub const MQRCCF_CFH_CONTROL_ERROR: crate::MQLONG = 3005;
/// [IBM `MQRCCF_CFH_PARM_COUNT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046100_.html)
pub const MQRCCF_CFH_PARM_COUNT_ERROR: crate::MQLONG = 3006;
/// [IBM `MQRCCF_CFH_COMMAND_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046110_.html)
pub const MQRCCF_CFH_COMMAND_ERROR: crate::MQLONG = 3007;
/// [IBM `MQRCCF_COMMAND_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046120_.html)
pub const MQRCCF_COMMAND_FAILED: crate::MQLONG = 3008;
/// [IBM `MQRCCF_CFIN_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046130_.html)
pub const MQRCCF_CFIN_LENGTH_ERROR: crate::MQLONG = 3009;
/// [IBM `MQRCCF_CFST_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046140_.html)
pub const MQRCCF_CFST_LENGTH_ERROR: crate::MQLONG = 3010;
/// [IBM `MQRCCF_CFST_STRING_LENGTH_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046150_.html)
pub const MQRCCF_CFST_STRING_LENGTH_ERR: crate::MQLONG = 3011;
/// [IBM `MQRCCF_FORCE_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046160_.html)
pub const MQRCCF_FORCE_VALUE_ERROR: crate::MQLONG = 3012;
/// [IBM `MQRCCF_STRUCTURE_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046170_.html)
pub const MQRCCF_STRUCTURE_TYPE_ERROR: crate::MQLONG = 3013;
/// [IBM `MQRCCF_CFIN_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046180_.html)
pub const MQRCCF_CFIN_PARM_ID_ERROR: crate::MQLONG = 3014;
/// [IBM `MQRCCF_CFST_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046190_.html)
pub const MQRCCF_CFST_PARM_ID_ERROR: crate::MQLONG = 3015;
/// [IBM `MQRCCF_MSG_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046200_.html)
pub const MQRCCF_MSG_LENGTH_ERROR: crate::MQLONG = 3016;
/// [IBM `MQRCCF_CFIN_DUPLICATE_PARM` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046210_.html)
pub const MQRCCF_CFIN_DUPLICATE_PARM: crate::MQLONG = 3017;
/// [IBM `MQRCCF_CFST_DUPLICATE_PARM` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046220_.html)
pub const MQRCCF_CFST_DUPLICATE_PARM: crate::MQLONG = 3018;
/// [IBM `MQRCCF_PARM_COUNT_TOO_SMALL` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046230_.html)
pub const MQRCCF_PARM_COUNT_TOO_SMALL: crate::MQLONG = 3019;
/// [IBM `MQRCCF_PARM_COUNT_TOO_BIG` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046240_.html)
pub const MQRCCF_PARM_COUNT_TOO_BIG: crate::MQLONG = 3020;
/// [IBM `MQRCCF_Q_ALREADY_IN_CELL` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046250_.html)
pub const MQRCCF_Q_ALREADY_IN_CELL: crate::MQLONG = 3021;
/// [IBM `MQRCCF_Q_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046260_.html)
pub const MQRCCF_Q_TYPE_ERROR: crate::MQLONG = 3022;
/// [IBM `MQRCCF_MD_FORMAT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046270_.html)
pub const MQRCCF_MD_FORMAT_ERROR: crate::MQLONG = 3023;
/// [IBM `MQRCCF_CFSL_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046280_.html)
pub const MQRCCF_CFSL_LENGTH_ERROR: crate::MQLONG = 3024;
/// [IBM `MQRCCF_REPLACE_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046290_.html)
pub const MQRCCF_REPLACE_VALUE_ERROR: crate::MQLONG = 3025;
/// [IBM `MQRCCF_CFIL_DUPLICATE_VALUE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046300_.html)
pub const MQRCCF_CFIL_DUPLICATE_VALUE: crate::MQLONG = 3026;
/// [IBM `MQRCCF_CFIL_COUNT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046310_.html)
pub const MQRCCF_CFIL_COUNT_ERROR: crate::MQLONG = 3027;
/// [IBM `MQRCCF_CFIL_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046320_.html)
pub const MQRCCF_CFIL_LENGTH_ERROR: crate::MQLONG = 3028;
/// [IBM `MQRCCF_QUIESCE_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046340_.html)
pub const MQRCCF_QUIESCE_VALUE_ERROR: crate::MQLONG = 3029;
/// [IBM `MQRCCF_MODE_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046330_.html)
pub const MQRCCF_MODE_VALUE_ERROR: crate::MQLONG = 3029;
/// [IBM `MQRCCF_MSG_SEQ_NUMBER_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046350_.html)
pub const MQRCCF_MSG_SEQ_NUMBER_ERROR: crate::MQLONG = 3030;
/// [IBM `MQRCCF_PING_DATA_COUNT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046360_.html)
pub const MQRCCF_PING_DATA_COUNT_ERROR: crate::MQLONG = 3031;
/// [IBM `MQRCCF_PING_DATA_COMPARE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046370_.html)
pub const MQRCCF_PING_DATA_COMPARE_ERROR: crate::MQLONG = 3032;
/// [IBM `MQRCCF_CFSL_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046380_.html)
pub const MQRCCF_CFSL_PARM_ID_ERROR: crate::MQLONG = 3033;
/// [IBM `MQRCCF_CHANNEL_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046390_.html)
pub const MQRCCF_CHANNEL_TYPE_ERROR: crate::MQLONG = 3034;
/// [IBM `MQRCCF_PARM_SEQUENCE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046400_.html)
pub const MQRCCF_PARM_SEQUENCE_ERROR: crate::MQLONG = 3035;
/// [IBM `MQRCCF_XMIT_PROTOCOL_TYPE_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046410_.html)
pub const MQRCCF_XMIT_PROTOCOL_TYPE_ERR: crate::MQLONG = 3036;
/// [IBM `MQRCCF_BATCH_SIZE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046420_.html)
pub const MQRCCF_BATCH_SIZE_ERROR: crate::MQLONG = 3037;
/// [IBM `MQRCCF_DISC_INT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046430_.html)
pub const MQRCCF_DISC_INT_ERROR: crate::MQLONG = 3038;
/// [IBM `MQRCCF_SHORT_RETRY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046440_.html)
pub const MQRCCF_SHORT_RETRY_ERROR: crate::MQLONG = 3039;
/// [IBM `MQRCCF_SHORT_TIMER_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046450_.html)
pub const MQRCCF_SHORT_TIMER_ERROR: crate::MQLONG = 3040;
/// [IBM `MQRCCF_LONG_RETRY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046460_.html)
pub const MQRCCF_LONG_RETRY_ERROR: crate::MQLONG = 3041;
/// [IBM `MQRCCF_LONG_TIMER_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046470_.html)
pub const MQRCCF_LONG_TIMER_ERROR: crate::MQLONG = 3042;
/// [IBM `MQRCCF_SEQ_NUMBER_WRAP_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046480_.html)
pub const MQRCCF_SEQ_NUMBER_WRAP_ERROR: crate::MQLONG = 3043;
/// [IBM `MQRCCF_MAX_MSG_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046490_.html)
pub const MQRCCF_MAX_MSG_LENGTH_ERROR: crate::MQLONG = 3044;
/// [IBM `MQRCCF_PUT_AUTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046500_.html)
pub const MQRCCF_PUT_AUTH_ERROR: crate::MQLONG = 3045;
/// [IBM `MQRCCF_PURGE_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046510_.html)
pub const MQRCCF_PURGE_VALUE_ERROR: crate::MQLONG = 3046;
/// [IBM `MQRCCF_CFIL_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046520_.html)
pub const MQRCCF_CFIL_PARM_ID_ERROR: crate::MQLONG = 3047;
/// [IBM `MQRCCF_MSG_TRUNCATED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046530_.html)
pub const MQRCCF_MSG_TRUNCATED: crate::MQLONG = 3048;
/// [IBM `MQRCCF_CCSID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046540_.html)
pub const MQRCCF_CCSID_ERROR: crate::MQLONG = 3049;
/// [IBM `MQRCCF_ENCODING_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046550_.html)
pub const MQRCCF_ENCODING_ERROR: crate::MQLONG = 3050;
pub const MQRCCF_QUEUES_VALUE_ERROR: crate::MQLONG = 3051;
/// [IBM `MQRCCF_DATA_CONV_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046560_.html)
pub const MQRCCF_DATA_CONV_VALUE_ERROR: crate::MQLONG = 3052;
/// [IBM `MQRCCF_INDOUBT_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046570_.html)
pub const MQRCCF_INDOUBT_VALUE_ERROR: crate::MQLONG = 3053;
/// [IBM `MQRCCF_ESCAPE_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046580_.html)
pub const MQRCCF_ESCAPE_TYPE_ERROR: crate::MQLONG = 3054;
pub const MQRCCF_REPOS_VALUE_ERROR: crate::MQLONG = 3055;
/// [IBM `MQRCCF_CHANNEL_TABLE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046590_.html)
pub const MQRCCF_CHANNEL_TABLE_ERROR: crate::MQLONG = 3062;
/// [IBM `MQRCCF_MCA_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046600_.html)
pub const MQRCCF_MCA_TYPE_ERROR: crate::MQLONG = 3063;
/// [IBM `MQRCCF_CHL_INST_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046610_.html)
pub const MQRCCF_CHL_INST_TYPE_ERROR: crate::MQLONG = 3064;
/// [IBM `MQRCCF_CHL_STATUS_NOT_FOUND` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046620_.html)
pub const MQRCCF_CHL_STATUS_NOT_FOUND: crate::MQLONG = 3065;
/// [IBM `MQRCCF_CFSL_DUPLICATE_PARM` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046630_.html)
pub const MQRCCF_CFSL_DUPLICATE_PARM: crate::MQLONG = 3066;
/// [IBM `MQRCCF_CFSL_TOTAL_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046640_.html)
pub const MQRCCF_CFSL_TOTAL_LENGTH_ERROR: crate::MQLONG = 3067;
/// [IBM `MQRCCF_CFSL_COUNT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046650_.html)
pub const MQRCCF_CFSL_COUNT_ERROR: crate::MQLONG = 3068;
/// [IBM `MQRCCF_CFSL_STRING_LENGTH_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046660_.html)
pub const MQRCCF_CFSL_STRING_LENGTH_ERR: crate::MQLONG = 3069;
/// [IBM `MQRCCF_BROKER_DELETED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046670_.html)
pub const MQRCCF_BROKER_DELETED: crate::MQLONG = 3070;
/// [IBM `MQRCCF_STREAM_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046680_.html)
pub const MQRCCF_STREAM_ERROR: crate::MQLONG = 3071;
/// [IBM `MQRCCF_TOPIC_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046690_.html)
pub const MQRCCF_TOPIC_ERROR: crate::MQLONG = 3072;
/// [IBM `MQRCCF_NOT_REGISTERED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046700_.html)
pub const MQRCCF_NOT_REGISTERED: crate::MQLONG = 3073;
/// [IBM `MQRCCF_Q_MGR_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046710_.html)
pub const MQRCCF_Q_MGR_NAME_ERROR: crate::MQLONG = 3074;
/// [IBM `MQRCCF_INCORRECT_STREAM` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046720_.html)
pub const MQRCCF_INCORRECT_STREAM: crate::MQLONG = 3075;
/// [IBM `MQRCCF_Q_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046730_.html)
pub const MQRCCF_Q_NAME_ERROR: crate::MQLONG = 3076;
/// [IBM `MQRCCF_NO_RETAINED_MSG` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046740_.html)
pub const MQRCCF_NO_RETAINED_MSG: crate::MQLONG = 3077;
/// [IBM `MQRCCF_DUPLICATE_IDENTITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046750_.html)
pub const MQRCCF_DUPLICATE_IDENTITY: crate::MQLONG = 3078;
/// [IBM `MQRCCF_INCORRECT_Q` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046760_.html)
pub const MQRCCF_INCORRECT_Q: crate::MQLONG = 3079;
/// [IBM `MQRCCF_CORREL_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046770_.html)
pub const MQRCCF_CORREL_ID_ERROR: crate::MQLONG = 3080;
/// [IBM `MQRCCF_NOT_AUTHORIZED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046780_.html)
pub const MQRCCF_NOT_AUTHORIZED: crate::MQLONG = 3081;
/// [IBM `MQRCCF_UNKNOWN_STREAM` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046790_.html)
pub const MQRCCF_UNKNOWN_STREAM: crate::MQLONG = 3082;
/// [IBM `MQRCCF_REG_OPTIONS_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046800_.html)
pub const MQRCCF_REG_OPTIONS_ERROR: crate::MQLONG = 3083;
/// [IBM `MQRCCF_PUB_OPTIONS_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046810_.html)
pub const MQRCCF_PUB_OPTIONS_ERROR: crate::MQLONG = 3084;
/// [IBM `MQRCCF_UNKNOWN_BROKER` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046820_.html)
pub const MQRCCF_UNKNOWN_BROKER: crate::MQLONG = 3085;
/// [IBM `MQRCCF_Q_MGR_CCSID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046830_.html)
pub const MQRCCF_Q_MGR_CCSID_ERROR: crate::MQLONG = 3086;
/// [IBM `MQRCCF_DEL_OPTIONS_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046840_.html)
pub const MQRCCF_DEL_OPTIONS_ERROR: crate::MQLONG = 3087;
/// [IBM `MQRCCF_CLUSTER_NAME_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046850_.html)
pub const MQRCCF_CLUSTER_NAME_CONFLICT: crate::MQLONG = 3088;
/// [IBM `MQRCCF_REPOS_NAME_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046860_.html)
pub const MQRCCF_REPOS_NAME_CONFLICT: crate::MQLONG = 3089;
/// [IBM `MQRCCF_CLUSTER_Q_USAGE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046870_.html)
pub const MQRCCF_CLUSTER_Q_USAGE_ERROR: crate::MQLONG = 3090;
/// [IBM `MQRCCF_ACTION_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046880_.html)
pub const MQRCCF_ACTION_VALUE_ERROR: crate::MQLONG = 3091;
/// [IBM `MQRCCF_COMMS_LIBRARY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046890_.html)
pub const MQRCCF_COMMS_LIBRARY_ERROR: crate::MQLONG = 3092;
/// [IBM `MQRCCF_NETBIOS_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046900_.html)
pub const MQRCCF_NETBIOS_NAME_ERROR: crate::MQLONG = 3093;
/// [IBM `MQRCCF_BROKER_COMMAND_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046910_.html)
pub const MQRCCF_BROKER_COMMAND_FAILED: crate::MQLONG = 3094;
/// [IBM `MQRCCF_CFST_CONFLICTING_PARM` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046920_.html)
pub const MQRCCF_CFST_CONFLICTING_PARM: crate::MQLONG = 3095;
/// [IBM `MQRCCF_PATH_NOT_VALID` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046930_.html)
pub const MQRCCF_PATH_NOT_VALID: crate::MQLONG = 3096;
/// [IBM `MQRCCF_PARM_SYNTAX_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046940_.html)
pub const MQRCCF_PARM_SYNTAX_ERROR: crate::MQLONG = 3097;
/// [IBM `MQRCCF_PWD_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046950_.html)
pub const MQRCCF_PWD_LENGTH_ERROR: crate::MQLONG = 3098;
/// [IBM `MQRCCF_FILTER_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046960_.html)
pub const MQRCCF_FILTER_ERROR: crate::MQLONG = 3150;
/// [IBM `MQRCCF_WRONG_USER` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046970_.html)
pub const MQRCCF_WRONG_USER: crate::MQLONG = 3151;
/// [IBM `MQRCCF_DUPLICATE_SUBSCRIPTION` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046980_.html)
pub const MQRCCF_DUPLICATE_SUBSCRIPTION: crate::MQLONG = 3152;
/// [IBM `MQRCCF_SUB_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q046990_.html)
pub const MQRCCF_SUB_NAME_ERROR: crate::MQLONG = 3153;
/// [IBM `MQRCCF_SUB_IDENTITY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047000_.html)
pub const MQRCCF_SUB_IDENTITY_ERROR: crate::MQLONG = 3154;
/// [IBM `MQRCCF_SUBSCRIPTION_IN_USE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047010_.html)
pub const MQRCCF_SUBSCRIPTION_IN_USE: crate::MQLONG = 3155;
/// [IBM `MQRCCF_SUBSCRIPTION_LOCKED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047020_.html)
pub const MQRCCF_SUBSCRIPTION_LOCKED: crate::MQLONG = 3156;
/// [IBM `MQRCCF_ALREADY_JOINED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047030_.html)
pub const MQRCCF_ALREADY_JOINED: crate::MQLONG = 3157;
/// [IBM `MQRCCF_OBJECT_IN_USE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047040_.html)
pub const MQRCCF_OBJECT_IN_USE: crate::MQLONG = 3160;
/// [IBM `MQRCCF_UNKNOWN_FILE_NAME` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047050_.html)
pub const MQRCCF_UNKNOWN_FILE_NAME: crate::MQLONG = 3161;
/// [IBM `MQRCCF_FILE_NOT_AVAILABLE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047060_.html)
pub const MQRCCF_FILE_NOT_AVAILABLE: crate::MQLONG = 3162;
/// [IBM `MQRCCF_DISC_RETRY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047070_.html)
pub const MQRCCF_DISC_RETRY_ERROR: crate::MQLONG = 3163;
/// [IBM `MQRCCF_ALLOC_RETRY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047080_.html)
pub const MQRCCF_ALLOC_RETRY_ERROR: crate::MQLONG = 3164;
/// [IBM `MQRCCF_ALLOC_SLOW_TIMER_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047090_.html)
pub const MQRCCF_ALLOC_SLOW_TIMER_ERROR: crate::MQLONG = 3165;
/// [IBM `MQRCCF_ALLOC_FAST_TIMER_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047100_.html)
pub const MQRCCF_ALLOC_FAST_TIMER_ERROR: crate::MQLONG = 3166;
/// [IBM `MQRCCF_PORT_NUMBER_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047110_.html)
pub const MQRCCF_PORT_NUMBER_ERROR: crate::MQLONG = 3167;
/// [IBM `MQRCCF_CHL_SYSTEM_NOT_ACTIVE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047120_.html)
pub const MQRCCF_CHL_SYSTEM_NOT_ACTIVE: crate::MQLONG = 3168;
/// [IBM `MQRCCF_ENTITY_NAME_MISSING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047130_.html)
pub const MQRCCF_ENTITY_NAME_MISSING: crate::MQLONG = 3169;
/// [IBM `MQRCCF_PROFILE_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047140_.html)
pub const MQRCCF_PROFILE_NAME_ERROR: crate::MQLONG = 3170;
/// [IBM `MQRCCF_AUTH_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047150_.html)
pub const MQRCCF_AUTH_VALUE_ERROR: crate::MQLONG = 3171;
/// [IBM `MQRCCF_AUTH_VALUE_MISSING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047160_.html)
pub const MQRCCF_AUTH_VALUE_MISSING: crate::MQLONG = 3172;
/// [IBM `MQRCCF_OBJECT_TYPE_MISSING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047170_.html)
pub const MQRCCF_OBJECT_TYPE_MISSING: crate::MQLONG = 3173;
/// [IBM `MQRCCF_CONNECTION_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047180_.html)
pub const MQRCCF_CONNECTION_ID_ERROR: crate::MQLONG = 3174;
/// [IBM `MQRCCF_LOG_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047190_.html)
pub const MQRCCF_LOG_TYPE_ERROR: crate::MQLONG = 3175;
/// [IBM `MQRCCF_PROGRAM_NOT_AVAILABLE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047200_.html)
pub const MQRCCF_PROGRAM_NOT_AVAILABLE: crate::MQLONG = 3176;
/// [IBM `MQRCCF_PROGRAM_AUTH_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047210_.html)
pub const MQRCCF_PROGRAM_AUTH_FAILED: crate::MQLONG = 3177;
/// [IBM `MQRCCF_NONE_FOUND` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047220_.html)
pub const MQRCCF_NONE_FOUND: crate::MQLONG = 3200;
/// [IBM `MQRCCF_SECURITY_SWITCH_OFF` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047230_.html)
pub const MQRCCF_SECURITY_SWITCH_OFF: crate::MQLONG = 3201;
/// [IBM `MQRCCF_SECURITY_REFRESH_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047240_.html)
pub const MQRCCF_SECURITY_REFRESH_FAILED: crate::MQLONG = 3202;
/// [IBM `MQRCCF_PARM_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047250_.html)
pub const MQRCCF_PARM_CONFLICT: crate::MQLONG = 3203;
/// [IBM `MQRCCF_COMMAND_INHIBITED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047260_.html)
pub const MQRCCF_COMMAND_INHIBITED: crate::MQLONG = 3204;
/// [IBM `MQRCCF_OBJECT_BEING_DELETED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047270_.html)
pub const MQRCCF_OBJECT_BEING_DELETED: crate::MQLONG = 3205;
/// [IBM `MQRCCF_STORAGE_CLASS_IN_USE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047280_.html)
pub const MQRCCF_STORAGE_CLASS_IN_USE: crate::MQLONG = 3207;
/// [IBM `MQRCCF_OBJECT_NAME_RESTRICTED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047290_.html)
pub const MQRCCF_OBJECT_NAME_RESTRICTED: crate::MQLONG = 3208;
/// [IBM `MQRCCF_OBJECT_LIMIT_EXCEEDED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047300_.html)
pub const MQRCCF_OBJECT_LIMIT_EXCEEDED: crate::MQLONG = 3209;
/// [IBM `MQRCCF_OBJECT_OPEN_FORCE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047310_.html)
pub const MQRCCF_OBJECT_OPEN_FORCE: crate::MQLONG = 3210;
/// [IBM `MQRCCF_DISPOSITION_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047320_.html)
pub const MQRCCF_DISPOSITION_CONFLICT: crate::MQLONG = 3211;
/// [IBM `MQRCCF_Q_MGR_NOT_IN_QSG` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047330_.html)
pub const MQRCCF_Q_MGR_NOT_IN_QSG: crate::MQLONG = 3212;
/// [IBM `MQRCCF_ATTR_VALUE_FIXED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047340_.html)
pub const MQRCCF_ATTR_VALUE_FIXED: crate::MQLONG = 3213;
/// [IBM `MQRCCF_NAMELIST_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047350_.html)
pub const MQRCCF_NAMELIST_ERROR: crate::MQLONG = 3215;
/// [IBM `MQRCCF_NO_CHANNEL_INITIATOR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047360_.html)
pub const MQRCCF_NO_CHANNEL_INITIATOR: crate::MQLONG = 3217;
/// [IBM `MQRCCF_CHANNEL_INITIATOR_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047370_.html)
pub const MQRCCF_CHANNEL_INITIATOR_ERROR: crate::MQLONG = 3218;
/// [IBM `MQRCCF_COMMAND_LEVEL_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047380_.html)
pub const MQRCCF_COMMAND_LEVEL_CONFLICT: crate::MQLONG = 3222;
/// [IBM `MQRCCF_Q_ATTR_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047390_.html)
pub const MQRCCF_Q_ATTR_CONFLICT: crate::MQLONG = 3223;
/// [IBM `MQRCCF_EVENTS_DISABLED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047400_.html)
pub const MQRCCF_EVENTS_DISABLED: crate::MQLONG = 3224;
/// [IBM `MQRCCF_COMMAND_SCOPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047410_.html)
pub const MQRCCF_COMMAND_SCOPE_ERROR: crate::MQLONG = 3225;
/// [IBM `MQRCCF_COMMAND_REPLY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047420_.html)
pub const MQRCCF_COMMAND_REPLY_ERROR: crate::MQLONG = 3226;
/// [IBM `MQRCCF_FUNCTION_RESTRICTED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047430_.html)
pub const MQRCCF_FUNCTION_RESTRICTED: crate::MQLONG = 3227;
/// [IBM `MQRCCF_PARM_MISSING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047440_.html)
pub const MQRCCF_PARM_MISSING: crate::MQLONG = 3228;
/// [IBM `MQRCCF_PARM_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047450_.html)
pub const MQRCCF_PARM_VALUE_ERROR: crate::MQLONG = 3229;
/// [IBM `MQRCCF_COMMAND_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047460_.html)
pub const MQRCCF_COMMAND_LENGTH_ERROR: crate::MQLONG = 3230;
/// [IBM `MQRCCF_COMMAND_ORIGIN_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047470_.html)
pub const MQRCCF_COMMAND_ORIGIN_ERROR: crate::MQLONG = 3231;
/// [IBM `MQRCCF_LISTENER_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047480_.html)
pub const MQRCCF_LISTENER_CONFLICT: crate::MQLONG = 3232;
/// [IBM `MQRCCF_LISTENER_STARTED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047490_.html)
pub const MQRCCF_LISTENER_STARTED: crate::MQLONG = 3233;
/// [IBM `MQRCCF_LISTENER_STOPPED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047500_.html)
pub const MQRCCF_LISTENER_STOPPED: crate::MQLONG = 3234;
/// [IBM `MQRCCF_CHANNEL_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047510_.html)
pub const MQRCCF_CHANNEL_ERROR: crate::MQLONG = 3235;
/// [IBM `MQRCCF_CF_STRUC_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047520_.html)
pub const MQRCCF_CF_STRUC_ERROR: crate::MQLONG = 3236;
/// [IBM `MQRCCF_UNKNOWN_USER_ID` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047530_.html)
pub const MQRCCF_UNKNOWN_USER_ID: crate::MQLONG = 3237;
/// [IBM `MQRCCF_UNEXPECTED_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047540_.html)
pub const MQRCCF_UNEXPECTED_ERROR: crate::MQLONG = 3238;
/// [IBM `MQRCCF_NO_XCF_PARTNER` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047550_.html)
pub const MQRCCF_NO_XCF_PARTNER: crate::MQLONG = 3239;
/// [IBM `MQRCCF_CFGR_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047560_.html)
pub const MQRCCF_CFGR_PARM_ID_ERROR: crate::MQLONG = 3240;
/// [IBM `MQRCCF_CFIF_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047570_.html)
pub const MQRCCF_CFIF_LENGTH_ERROR: crate::MQLONG = 3241;
/// [IBM `MQRCCF_CFIF_OPERATOR_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047580_.html)
pub const MQRCCF_CFIF_OPERATOR_ERROR: crate::MQLONG = 3242;
/// [IBM `MQRCCF_CFIF_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047590_.html)
pub const MQRCCF_CFIF_PARM_ID_ERROR: crate::MQLONG = 3243;
/// [IBM `MQRCCF_CFSF_FILTER_VAL_LEN_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047600_.html)
pub const MQRCCF_CFSF_FILTER_VAL_LEN_ERR: crate::MQLONG = 3244;
/// [IBM `MQRCCF_CFSF_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047610_.html)
pub const MQRCCF_CFSF_LENGTH_ERROR: crate::MQLONG = 3245;
/// [IBM `MQRCCF_CFSF_OPERATOR_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047620_.html)
pub const MQRCCF_CFSF_OPERATOR_ERROR: crate::MQLONG = 3246;
/// [IBM `MQRCCF_CFSF_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047630_.html)
pub const MQRCCF_CFSF_PARM_ID_ERROR: crate::MQLONG = 3247;
/// [IBM `MQRCCF_TOO_MANY_FILTERS` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047640_.html)
pub const MQRCCF_TOO_MANY_FILTERS: crate::MQLONG = 3248;
/// [IBM `MQRCCF_LISTENER_RUNNING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047650_.html)
pub const MQRCCF_LISTENER_RUNNING: crate::MQLONG = 3249;
/// [IBM `MQRCCF_LSTR_STATUS_NOT_FOUND` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047660_.html)
pub const MQRCCF_LSTR_STATUS_NOT_FOUND: crate::MQLONG = 3250;
/// [IBM `MQRCCF_SERVICE_RUNNING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047670_.html)
pub const MQRCCF_SERVICE_RUNNING: crate::MQLONG = 3251;
/// [IBM `MQRCCF_SERV_STATUS_NOT_FOUND` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047680_.html)
pub const MQRCCF_SERV_STATUS_NOT_FOUND: crate::MQLONG = 3252;
/// [IBM `MQRCCF_SERVICE_STOPPED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047690_.html)
pub const MQRCCF_SERVICE_STOPPED: crate::MQLONG = 3253;
/// [IBM `MQRCCF_CFBS_DUPLICATE_PARM` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047700_.html)
pub const MQRCCF_CFBS_DUPLICATE_PARM: crate::MQLONG = 3254;
/// [IBM `MQRCCF_CFBS_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047710_.html)
pub const MQRCCF_CFBS_LENGTH_ERROR: crate::MQLONG = 3255;
/// [IBM `MQRCCF_CFBS_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047720_.html)
pub const MQRCCF_CFBS_PARM_ID_ERROR: crate::MQLONG = 3256;
/// [IBM `MQRCCF_CFBS_STRING_LENGTH_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047730_.html)
pub const MQRCCF_CFBS_STRING_LENGTH_ERR: crate::MQLONG = 3257;
/// [IBM `MQRCCF_CFGR_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047740_.html)
pub const MQRCCF_CFGR_LENGTH_ERROR: crate::MQLONG = 3258;
/// [IBM `MQRCCF_CFGR_PARM_COUNT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047750_.html)
pub const MQRCCF_CFGR_PARM_COUNT_ERROR: crate::MQLONG = 3259;
/// [IBM `MQRCCF_CONN_NOT_STOPPED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047760_.html)
pub const MQRCCF_CONN_NOT_STOPPED: crate::MQLONG = 3260;
/// [IBM `MQRCCF_SERVICE_REQUEST_PENDING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047770_.html)
pub const MQRCCF_SERVICE_REQUEST_PENDING: crate::MQLONG = 3261;
/// [IBM `MQRCCF_NO_START_CMD` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047780_.html)
pub const MQRCCF_NO_START_CMD: crate::MQLONG = 3262;
/// [IBM `MQRCCF_NO_STOP_CMD` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047790_.html)
pub const MQRCCF_NO_STOP_CMD: crate::MQLONG = 3263;
/// [IBM `MQRCCF_CFBF_LENGTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047800_.html)
pub const MQRCCF_CFBF_LENGTH_ERROR: crate::MQLONG = 3264;
/// [IBM `MQRCCF_CFBF_PARM_ID_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047810_.html)
pub const MQRCCF_CFBF_PARM_ID_ERROR: crate::MQLONG = 3265;
/// [IBM `MQRCCF_CFBF_OPERATOR_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047830_.html)
pub const MQRCCF_CFBF_OPERATOR_ERROR: crate::MQLONG = 3266;
/// [IBM `MQRCCF_CFBF_FILTER_VAL_LEN_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047820_.html)
pub const MQRCCF_CFBF_FILTER_VAL_LEN_ERR: crate::MQLONG = 3267;
/// [IBM `MQRCCF_LISTENER_STILL_ACTIVE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047840_.html)
pub const MQRCCF_LISTENER_STILL_ACTIVE: crate::MQLONG = 3268;
/// [IBM `MQRCCF_DEF_XMIT_Q_CLUS_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047850_.html)
pub const MQRCCF_DEF_XMIT_Q_CLUS_ERROR: crate::MQLONG = 3269;
/// [IBM `MQRCCF_TOPICSTR_ALREADY_EXISTS` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047860_.html)
pub const MQRCCF_TOPICSTR_ALREADY_EXISTS: crate::MQLONG = 3300;
/// [IBM `MQRCCF_SHARING_CONVS_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047870_.html)
pub const MQRCCF_SHARING_CONVS_ERROR: crate::MQLONG = 3301;
/// [IBM `MQRCCF_SHARING_CONVS_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047880_.html)
pub const MQRCCF_SHARING_CONVS_TYPE: crate::MQLONG = 3302;
/// [IBM `MQRCCF_SECURITY_CASE_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047890_.html)
pub const MQRCCF_SECURITY_CASE_CONFLICT: crate::MQLONG = 3303;
/// [IBM `MQRCCF_TOPIC_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047900_.html)
pub const MQRCCF_TOPIC_TYPE_ERROR: crate::MQLONG = 3305;
/// [IBM `MQRCCF_MAX_INSTANCES_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047910_.html)
pub const MQRCCF_MAX_INSTANCES_ERROR: crate::MQLONG = 3306;
/// [IBM `MQRCCF_MAX_INSTS_PER_CLNT_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047920_.html)
pub const MQRCCF_MAX_INSTS_PER_CLNT_ERR: crate::MQLONG = 3307;
/// [IBM `MQRCCF_TOPIC_STRING_NOT_FOUND` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047930_.html)
pub const MQRCCF_TOPIC_STRING_NOT_FOUND: crate::MQLONG = 3308;
/// [IBM `MQRCCF_SUBSCRIPTION_POINT_ERR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047940_.html)
pub const MQRCCF_SUBSCRIPTION_POINT_ERR: crate::MQLONG = 3309;
/// [IBM `MQRCCF_SUB_ALREADY_EXISTS` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047950_.html)
pub const MQRCCF_SUB_ALREADY_EXISTS: crate::MQLONG = 3311;
pub const MQRCCF_UNKNOWN_OBJECT_NAME: crate::MQLONG = 3312;
pub const MQRCCF_REMOTE_Q_NAME_ERROR: crate::MQLONG = 3313;
/// [IBM `MQRCCF_DURABILITY_NOT_ALLOWED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047960_.html)
pub const MQRCCF_DURABILITY_NOT_ALLOWED: crate::MQLONG = 3314;
pub const MQRCCF_HOBJ_ERROR: crate::MQLONG = 3315;
pub const MQRCCF_DEST_NAME_ERROR: crate::MQLONG = 3316;
/// [IBM `MQRCCF_INVALID_DESTINATION` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047970_.html)
pub const MQRCCF_INVALID_DESTINATION: crate::MQLONG = 3317;
/// [IBM `MQRCCF_PUBSUB_INHIBITED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047980_.html)
pub const MQRCCF_PUBSUB_INHIBITED: crate::MQLONG = 3318;
pub const MQRCCF_GROUPUR_CHECKS_FAILED: crate::MQLONG = 3319;
pub const MQRCCF_COMM_INFO_TYPE_ERROR: crate::MQLONG = 3320;
pub const MQRCCF_USE_CLIENT_ID_ERROR: crate::MQLONG = 3321;
pub const MQRCCF_CLIENT_ID_NOT_FOUND: crate::MQLONG = 3322;
pub const MQRCCF_CLIENT_ID_ERROR: crate::MQLONG = 3323;
pub const MQRCCF_PORT_IN_USE: crate::MQLONG = 3324;
pub const MQRCCF_SSL_ALT_PROVIDER_REQD: crate::MQLONG = 3325;
/// [IBM `MQRCCF_CHLAUTH_TYPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q047990_.html)
pub const MQRCCF_CHLAUTH_TYPE_ERROR: crate::MQLONG = 3326;
/// [IBM `MQRCCF_CHLAUTH_ACTION_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048000_.html)
pub const MQRCCF_CHLAUTH_ACTION_ERROR: crate::MQLONG = 3327;
pub const MQRCCF_POLICY_NOT_FOUND: crate::MQLONG = 3328;
pub const MQRCCF_ENCRYPTION_ALG_ERROR: crate::MQLONG = 3329;
pub const MQRCCF_SIGNATURE_ALG_ERROR: crate::MQLONG = 3330;
pub const MQRCCF_TOLERATION_POL_ERROR: crate::MQLONG = 3331;
pub const MQRCCF_POLICY_VERSION_ERROR: crate::MQLONG = 3332;
pub const MQRCCF_RECIPIENT_DN_MISSING: crate::MQLONG = 3333;
pub const MQRCCF_POLICY_NAME_MISSING: crate::MQLONG = 3334;
pub const MQRCCF_CHLAUTH_USERSRC_ERROR: crate::MQLONG = 3335;
/// [IBM `MQRCCF_WRONG_CHLAUTH_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048020_.html)
pub const MQRCCF_WRONG_CHLAUTH_TYPE: crate::MQLONG = 3336;
/// [IBM `MQRCCF_CHLAUTH_ALREADY_EXISTS` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048030_.html)
pub const MQRCCF_CHLAUTH_ALREADY_EXISTS: crate::MQLONG = 3337;
/// [IBM `MQRCCF_CHLAUTH_NOT_FOUND` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048040_.html)
pub const MQRCCF_CHLAUTH_NOT_FOUND: crate::MQLONG = 3338;
/// [IBM `MQRCCF_WRONG_CHLAUTH_ACTION` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048050_.html)
pub const MQRCCF_WRONG_CHLAUTH_ACTION: crate::MQLONG = 3339;
/// [IBM `MQRCCF_WRONG_CHLAUTH_USERSRC` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048060_.html)
pub const MQRCCF_WRONG_CHLAUTH_USERSRC: crate::MQLONG = 3340;
/// [IBM `MQRCCF_CHLAUTH_WARN_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048070_.html)
pub const MQRCCF_CHLAUTH_WARN_ERROR: crate::MQLONG = 3341;
/// [IBM `MQRCCF_WRONG_CHLAUTH_MATCH` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048080_.html)
pub const MQRCCF_WRONG_CHLAUTH_MATCH: crate::MQLONG = 3342;
/// [IBM `MQRCCF_IPADDR_RANGE_CONFLICT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048090_.html)
pub const MQRCCF_IPADDR_RANGE_CONFLICT: crate::MQLONG = 3343;
/// [IBM `MQRCCF_CHLAUTH_MAX_EXCEEDED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048100_.html)
pub const MQRCCF_CHLAUTH_MAX_EXCEEDED: crate::MQLONG = 3344;
/// [IBM `MQRCCF_IPADDR_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048110_.html)
pub const MQRCCF_IPADDR_ERROR: crate::MQLONG = 3345;
pub const MQRCCF_ADDRESS_ERROR: crate::MQLONG = 3345;
/// [IBM `MQRCCF_IPADDR_RANGE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048120_.html)
pub const MQRCCF_IPADDR_RANGE_ERROR: crate::MQLONG = 3346;
/// [IBM `MQRCCF_PROFILE_NAME_MISSING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048130_.html)
pub const MQRCCF_PROFILE_NAME_MISSING: crate::MQLONG = 3347;
/// [IBM `MQRCCF_CHLAUTH_CLNTUSER_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048140_.html)
pub const MQRCCF_CHLAUTH_CLNTUSER_ERROR: crate::MQLONG = 3348;
/// [IBM `MQRCCF_CHLAUTH_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048150_.html)
pub const MQRCCF_CHLAUTH_NAME_ERROR: crate::MQLONG = 3349;
/// [IBM `MQRCCF_CHLAUTH_RUNCHECK_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048160_.html)
pub const MQRCCF_CHLAUTH_RUNCHECK_ERROR: crate::MQLONG = 3350;
pub const MQRCCF_CF_STRUC_ALREADY_FAILED: crate::MQLONG = 3351;
pub const MQRCCF_CFCONLOS_CHECKS_FAILED: crate::MQLONG = 3352;
/// [IBM `MQRCCF_SUITE_B_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048170_.html)
pub const MQRCCF_SUITE_B_ERROR: crate::MQLONG = 3353;
pub const MQRCCF_CHANNEL_NOT_STARTED: crate::MQLONG = 3354;
pub const MQRCCF_CUSTOM_ERROR: crate::MQLONG = 3355;
pub const MQRCCF_BACKLOG_OUT_OF_RANGE: crate::MQLONG = 3356;
pub const MQRCCF_CHLAUTH_DISABLED: crate::MQLONG = 3357;
pub const MQRCCF_SMDS_REQUIRES_DSGROUP: crate::MQLONG = 3358;
pub const MQRCCF_PSCLUS_DISABLED_TOPDEF: crate::MQLONG = 3359;
pub const MQRCCF_PSCLUS_TOPIC_EXISTS: crate::MQLONG = 3360;
pub const MQRCCF_SSL_CIPHER_SUITE_ERROR: crate::MQLONG = 3361;
pub const MQRCCF_SOCKET_ERROR: crate::MQLONG = 3362;
/// [IBM `MQRCCF_CLUS_XMIT_Q_USAGE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048180_.html)
pub const MQRCCF_CLUS_XMIT_Q_USAGE_ERROR: crate::MQLONG = 3363;
/// [IBM `MQRCCF_CERT_VAL_POLICY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048190_.html)
pub const MQRCCF_CERT_VAL_POLICY_ERROR: crate::MQLONG = 3364;
pub const MQRCCF_INVALID_PROTOCOL: crate::MQLONG = 3365;
/// [IBM `MQRCCF_REVDNS_DISABLED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q113230_.html)
pub const MQRCCF_REVDNS_DISABLED: crate::MQLONG = 3366;
pub const MQRCCF_CLROUTE_NOT_ALTERABLE: crate::MQLONG = 3367;
pub const MQRCCF_CLUSTER_TOPIC_CONFLICT: crate::MQLONG = 3368;
pub const MQRCCF_DEFCLXQ_MODEL_Q_ERROR: crate::MQLONG = 3369;
/// [IBM `MQRCCF_CHLAUTH_CHKCLI_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q120540_.html)
pub const MQRCCF_CHLAUTH_CHKCLI_ERROR: crate::MQLONG = 3370;
pub const MQRCCF_CERT_LABEL_NOT_ALLOWED: crate::MQLONG = 3371;
pub const MQRCCF_Q_MGR_ATTR_CONFLICT: crate::MQLONG = 3372;
pub const MQRCCF_ENTITY_TYPE_MISSING: crate::MQLONG = 3373;
pub const MQRCCF_CLWL_EXIT_NAME_ERROR: crate::MQLONG = 3374;
pub const MQRCCF_SERVICE_NAME_ERROR: crate::MQLONG = 3375;
pub const MQRCCF_REMOTE_CHL_TYPE_ERROR: crate::MQLONG = 3376;
/// [IBM `MQRCCF_TOPIC_RESTRICTED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q120545_.html)
pub const MQRCCF_TOPIC_RESTRICTED: crate::MQLONG = 3377;
pub const MQRCCF_CURRENT_LOG_EXTENT: crate::MQLONG = 3378;
pub const MQRCCF_LOG_EXTENT_NOT_FOUND: crate::MQLONG = 3379;
pub const MQRCCF_LOG_NOT_REDUCED: crate::MQLONG = 3380;
pub const MQRCCF_LOG_EXTENT_ERROR: crate::MQLONG = 3381;
pub const MQRCCF_ACCESS_BLOCKED: crate::MQLONG = 3382;
pub const MQRCCF_PS_REQUIRED_MQUC: crate::MQLONG = 3383;
pub const MQRCCF_STREAMQ_DEST_NOT_SUPP: crate::MQLONG = 3384;
pub const MQRCCF_STREAMQ_DEST_CONFLICT: crate::MQLONG = 3385;
pub const MQRCCF_STREAMQ_NOT_SUPPORTED: crate::MQLONG = 3386;
pub const MQRCCF_STREAMQ_CONFLICT: crate::MQLONG = 3387;
/// [IBM `MQRCCF_INCOMPATIBLE_QM_IN_QSG` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/rc3389.html)
pub const MQRCCF_INCOMPATIBLE_QM_IN_QSG: crate::MQLONG = 3389;
pub const MQRCCF_ATTR_VALUE_ERROR_QSG_QM: crate::MQLONG = 3390;
/// [IBM `MQRCCF_AUTHORIZED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/rc3391.html)
pub const MQRCCF_AUTHORIZED: crate::MQLONG = 3391;
/// [IBM `MQRCCF_OBJECT_ALREADY_EXISTS` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048200_.html)
pub const MQRCCF_OBJECT_ALREADY_EXISTS: crate::MQLONG = 4001;
/// [IBM `MQRCCF_OBJECT_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048210_.html)
pub const MQRCCF_OBJECT_WRONG_TYPE: crate::MQLONG = 4002;
/// [IBM `MQRCCF_LIKE_OBJECT_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048220_.html)
pub const MQRCCF_LIKE_OBJECT_WRONG_TYPE: crate::MQLONG = 4003;
/// [IBM `MQRCCF_OBJECT_OPEN` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048230_.html)
pub const MQRCCF_OBJECT_OPEN: crate::MQLONG = 4004;
/// [IBM `MQRCCF_ATTR_VALUE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048240_.html)
pub const MQRCCF_ATTR_VALUE_ERROR: crate::MQLONG = 4005;
/// [IBM `MQRCCF_UNKNOWN_Q_MGR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048250_.html)
pub const MQRCCF_UNKNOWN_Q_MGR: crate::MQLONG = 4006;
/// [IBM `MQRCCF_Q_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048260_.html)
pub const MQRCCF_Q_WRONG_TYPE: crate::MQLONG = 4007;
/// [IBM `MQRCCF_OBJECT_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048270_.html)
pub const MQRCCF_OBJECT_NAME_ERROR: crate::MQLONG = 4008;
/// [IBM `MQRCCF_ALLOCATE_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048280_.html)
pub const MQRCCF_ALLOCATE_FAILED: crate::MQLONG = 4009;
/// [IBM `MQRCCF_HOST_NOT_AVAILABLE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048290_.html)
pub const MQRCCF_HOST_NOT_AVAILABLE: crate::MQLONG = 4010;
/// [IBM `MQRCCF_CONFIGURATION_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048300_.html)
pub const MQRCCF_CONFIGURATION_ERROR: crate::MQLONG = 4011;
/// [IBM `MQRCCF_CONNECTION_REFUSED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048310_.html)
pub const MQRCCF_CONNECTION_REFUSED: crate::MQLONG = 4012;
/// [IBM `MQRCCF_ENTRY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048320_.html)
pub const MQRCCF_ENTRY_ERROR: crate::MQLONG = 4013;
/// [IBM `MQRCCF_SEND_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048330_.html)
pub const MQRCCF_SEND_FAILED: crate::MQLONG = 4014;
/// [IBM `MQRCCF_RECEIVED_DATA_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048340_.html)
pub const MQRCCF_RECEIVED_DATA_ERROR: crate::MQLONG = 4015;
/// [IBM `MQRCCF_RECEIVE_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048350_.html)
pub const MQRCCF_RECEIVE_FAILED: crate::MQLONG = 4016;
/// [IBM `MQRCCF_CONNECTION_CLOSED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048360_.html)
pub const MQRCCF_CONNECTION_CLOSED: crate::MQLONG = 4017;
/// [IBM `MQRCCF_NO_STORAGE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048370_.html)
pub const MQRCCF_NO_STORAGE: crate::MQLONG = 4018;
/// [IBM `MQRCCF_NO_COMMS_MANAGER` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048380_.html)
pub const MQRCCF_NO_COMMS_MANAGER: crate::MQLONG = 4019;
/// [IBM `MQRCCF_LISTENER_NOT_STARTED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048390_.html)
pub const MQRCCF_LISTENER_NOT_STARTED: crate::MQLONG = 4020;
/// [IBM `MQRCCF_BIND_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048400_.html)
pub const MQRCCF_BIND_FAILED: crate::MQLONG = 4024;
/// [IBM `MQRCCF_CHANNEL_INDOUBT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048410_.html)
pub const MQRCCF_CHANNEL_INDOUBT: crate::MQLONG = 4025;
/// [IBM `MQRCCF_MQCONN_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048420_.html)
pub const MQRCCF_MQCONN_FAILED: crate::MQLONG = 4026;
/// [IBM `MQRCCF_MQOPEN_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048430_.html)
pub const MQRCCF_MQOPEN_FAILED: crate::MQLONG = 4027;
/// [IBM `MQRCCF_MQGET_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048440_.html)
pub const MQRCCF_MQGET_FAILED: crate::MQLONG = 4028;
/// [IBM `MQRCCF_MQPUT_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048450_.html)
pub const MQRCCF_MQPUT_FAILED: crate::MQLONG = 4029;
/// [IBM `MQRCCF_PING_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048460_.html)
pub const MQRCCF_PING_ERROR: crate::MQLONG = 4030;
/// [IBM `MQRCCF_CHANNEL_IN_USE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048470_.html)
pub const MQRCCF_CHANNEL_IN_USE: crate::MQLONG = 4031;
/// [IBM `MQRCCF_CHANNEL_NOT_FOUND` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048480_.html)
pub const MQRCCF_CHANNEL_NOT_FOUND: crate::MQLONG = 4032;
/// [IBM `MQRCCF_UNKNOWN_REMOTE_CHANNEL` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048490_.html)
pub const MQRCCF_UNKNOWN_REMOTE_CHANNEL: crate::MQLONG = 4033;
/// [IBM `MQRCCF_REMOTE_QM_UNAVAILABLE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048500_.html)
pub const MQRCCF_REMOTE_QM_UNAVAILABLE: crate::MQLONG = 4034;
/// [IBM `MQRCCF_REMOTE_QM_TERMINATING` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048510_.html)
pub const MQRCCF_REMOTE_QM_TERMINATING: crate::MQLONG = 4035;
/// [IBM `MQRCCF_MQINQ_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048520_.html)
pub const MQRCCF_MQINQ_FAILED: crate::MQLONG = 4036;
/// [IBM `MQRCCF_NOT_XMIT_Q` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048530_.html)
pub const MQRCCF_NOT_XMIT_Q: crate::MQLONG = 4037;
/// [IBM `MQRCCF_CHANNEL_DISABLED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048540_.html)
pub const MQRCCF_CHANNEL_DISABLED: crate::MQLONG = 4038;
/// [IBM `MQRCCF_USER_EXIT_NOT_AVAILABLE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048550_.html)
pub const MQRCCF_USER_EXIT_NOT_AVAILABLE: crate::MQLONG = 4039;
/// [IBM `MQRCCF_COMMIT_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048560_.html)
pub const MQRCCF_COMMIT_FAILED: crate::MQLONG = 4040;
/// [IBM `MQRCCF_WRONG_CHANNEL_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048570_.html)
pub const MQRCCF_WRONG_CHANNEL_TYPE: crate::MQLONG = 4041;
/// [IBM `MQRCCF_CHANNEL_ALREADY_EXISTS` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048580_.html)
pub const MQRCCF_CHANNEL_ALREADY_EXISTS: crate::MQLONG = 4042;
/// [IBM `MQRCCF_DATA_TOO_LARGE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048590_.html)
pub const MQRCCF_DATA_TOO_LARGE: crate::MQLONG = 4043;
/// [IBM `MQRCCF_CHANNEL_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048600_.html)
pub const MQRCCF_CHANNEL_NAME_ERROR: crate::MQLONG = 4044;
/// [IBM `MQRCCF_XMIT_Q_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048610_.html)
pub const MQRCCF_XMIT_Q_NAME_ERROR: crate::MQLONG = 4045;
/// [IBM `MQRCCF_MCA_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048620_.html)
pub const MQRCCF_MCA_NAME_ERROR: crate::MQLONG = 4047;
/// [IBM `MQRCCF_SEND_EXIT_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048630_.html)
pub const MQRCCF_SEND_EXIT_NAME_ERROR: crate::MQLONG = 4048;
/// [IBM `MQRCCF_SEC_EXIT_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048640_.html)
pub const MQRCCF_SEC_EXIT_NAME_ERROR: crate::MQLONG = 4049;
/// [IBM `MQRCCF_MSG_EXIT_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048650_.html)
pub const MQRCCF_MSG_EXIT_NAME_ERROR: crate::MQLONG = 4050;
/// [IBM `MQRCCF_RCV_EXIT_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048660_.html)
pub const MQRCCF_RCV_EXIT_NAME_ERROR: crate::MQLONG = 4051;
/// [IBM `MQRCCF_XMIT_Q_NAME_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048670_.html)
pub const MQRCCF_XMIT_Q_NAME_WRONG_TYPE: crate::MQLONG = 4052;
/// [IBM `MQRCCF_MCA_NAME_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048680_.html)
pub const MQRCCF_MCA_NAME_WRONG_TYPE: crate::MQLONG = 4053;
/// [IBM `MQRCCF_DISC_INT_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048690_.html)
pub const MQRCCF_DISC_INT_WRONG_TYPE: crate::MQLONG = 4054;
/// [IBM `MQRCCF_SHORT_RETRY_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048700_.html)
pub const MQRCCF_SHORT_RETRY_WRONG_TYPE: crate::MQLONG = 4055;
/// [IBM `MQRCCF_SHORT_TIMER_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048710_.html)
pub const MQRCCF_SHORT_TIMER_WRONG_TYPE: crate::MQLONG = 4056;
/// [IBM `MQRCCF_LONG_RETRY_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048720_.html)
pub const MQRCCF_LONG_RETRY_WRONG_TYPE: crate::MQLONG = 4057;
/// [IBM `MQRCCF_LONG_TIMER_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048730_.html)
pub const MQRCCF_LONG_TIMER_WRONG_TYPE: crate::MQLONG = 4058;
/// [IBM `MQRCCF_PUT_AUTH_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048740_.html)
pub const MQRCCF_PUT_AUTH_WRONG_TYPE: crate::MQLONG = 4059;
pub const MQRCCF_KEEP_ALIVE_INT_ERROR: crate::MQLONG = 4060;
/// [IBM `MQRCCF_MISSING_CONN_NAME` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048750_.html)
pub const MQRCCF_MISSING_CONN_NAME: crate::MQLONG = 4061;
/// [IBM `MQRCCF_CONN_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048760_.html)
pub const MQRCCF_CONN_NAME_ERROR: crate::MQLONG = 4062;
/// [IBM `MQRCCF_MQSET_FAILED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048770_.html)
pub const MQRCCF_MQSET_FAILED: crate::MQLONG = 4063;
/// [IBM `MQRCCF_CHANNEL_NOT_ACTIVE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048780_.html)
pub const MQRCCF_CHANNEL_NOT_ACTIVE: crate::MQLONG = 4064;
/// [IBM `MQRCCF_TERMINATED_BY_SEC_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048790_.html)
pub const MQRCCF_TERMINATED_BY_SEC_EXIT: crate::MQLONG = 4065;
/// [IBM `MQRCCF_DYNAMIC_Q_SCOPE_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048800_.html)
pub const MQRCCF_DYNAMIC_Q_SCOPE_ERROR: crate::MQLONG = 4067;
/// [IBM `MQRCCF_CELL_DIR_NOT_AVAILABLE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048810_.html)
pub const MQRCCF_CELL_DIR_NOT_AVAILABLE: crate::MQLONG = 4068;
/// [IBM `MQRCCF_MR_COUNT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048820_.html)
pub const MQRCCF_MR_COUNT_ERROR: crate::MQLONG = 4069;
/// [IBM `MQRCCF_MR_COUNT_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048830_.html)
pub const MQRCCF_MR_COUNT_WRONG_TYPE: crate::MQLONG = 4070;
/// [IBM `MQRCCF_MR_EXIT_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048840_.html)
pub const MQRCCF_MR_EXIT_NAME_ERROR: crate::MQLONG = 4071;
/// [IBM `MQRCCF_MR_EXIT_NAME_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048850_.html)
pub const MQRCCF_MR_EXIT_NAME_WRONG_TYPE: crate::MQLONG = 4072;
/// [IBM `MQRCCF_MR_INTERVAL_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048860_.html)
pub const MQRCCF_MR_INTERVAL_ERROR: crate::MQLONG = 4073;
/// [IBM `MQRCCF_MR_INTERVAL_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048870_.html)
pub const MQRCCF_MR_INTERVAL_WRONG_TYPE: crate::MQLONG = 4074;
/// [IBM `MQRCCF_NPM_SPEED_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048880_.html)
pub const MQRCCF_NPM_SPEED_ERROR: crate::MQLONG = 4075;
/// [IBM `MQRCCF_NPM_SPEED_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048890_.html)
pub const MQRCCF_NPM_SPEED_WRONG_TYPE: crate::MQLONG = 4076;
/// [IBM `MQRCCF_HB_INTERVAL_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048900_.html)
pub const MQRCCF_HB_INTERVAL_ERROR: crate::MQLONG = 4077;
/// [IBM `MQRCCF_HB_INTERVAL_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048910_.html)
pub const MQRCCF_HB_INTERVAL_WRONG_TYPE: crate::MQLONG = 4078;
/// [IBM `MQRCCF_CHAD_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048920_.html)
pub const MQRCCF_CHAD_ERROR: crate::MQLONG = 4079;
/// [IBM `MQRCCF_CHAD_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048930_.html)
pub const MQRCCF_CHAD_WRONG_TYPE: crate::MQLONG = 4080;
/// [IBM `MQRCCF_CHAD_EVENT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048940_.html)
pub const MQRCCF_CHAD_EVENT_ERROR: crate::MQLONG = 4081;
/// [IBM `MQRCCF_CHAD_EVENT_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048950_.html)
pub const MQRCCF_CHAD_EVENT_WRONG_TYPE: crate::MQLONG = 4082;
/// [IBM `MQRCCF_CHAD_EXIT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048960_.html)
pub const MQRCCF_CHAD_EXIT_ERROR: crate::MQLONG = 4083;
/// [IBM `MQRCCF_CHAD_EXIT_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048970_.html)
pub const MQRCCF_CHAD_EXIT_WRONG_TYPE: crate::MQLONG = 4084;
/// [IBM `MQRCCF_SUPPRESSED_BY_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048980_.html)
pub const MQRCCF_SUPPRESSED_BY_EXIT: crate::MQLONG = 4085;
/// [IBM `MQRCCF_BATCH_INT_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q048990_.html)
pub const MQRCCF_BATCH_INT_ERROR: crate::MQLONG = 4086;
/// [IBM `MQRCCF_BATCH_INT_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049000_.html)
pub const MQRCCF_BATCH_INT_WRONG_TYPE: crate::MQLONG = 4087;
/// [IBM `MQRCCF_NET_PRIORITY_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049010_.html)
pub const MQRCCF_NET_PRIORITY_ERROR: crate::MQLONG = 4088;
/// [IBM `MQRCCF_NET_PRIORITY_WRONG_TYPE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049020_.html)
pub const MQRCCF_NET_PRIORITY_WRONG_TYPE: crate::MQLONG = 4089;
/// [IBM `MQRCCF_CHANNEL_CLOSED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049030_.html)
pub const MQRCCF_CHANNEL_CLOSED: crate::MQLONG = 4090;
pub const MQRCCF_Q_STATUS_NOT_FOUND: crate::MQLONG = 4091;
/// [IBM `MQRCCF_SSL_CIPHER_SPEC_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049040_.html)
pub const MQRCCF_SSL_CIPHER_SPEC_ERROR: crate::MQLONG = 4092;
/// [IBM `MQRCCF_SSL_PEER_NAME_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049050_.html)
pub const MQRCCF_SSL_PEER_NAME_ERROR: crate::MQLONG = 4093;
/// [IBM `MQRCCF_SSL_CLIENT_AUTH_ERROR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049060_.html)
pub const MQRCCF_SSL_CLIENT_AUTH_ERROR: crate::MQLONG = 4094;
/// [IBM `MQRCCF_RETAINED_NOT_SUPPORTED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/reference/q049070_.html)
pub const MQRCCF_RETAINED_NOT_SUPPORTED: crate::MQLONG = 4095;
pub const MQRCCF_KWD_VALUE_WRONG_TYPE: crate::MQLONG = 4096;
pub const MQRCCF_APPL_STATUS_NOT_FOUND: crate::MQLONG = 4097;
pub const MQRCCF_NHA_NOT_AVAILABLE: crate::MQLONG = 4098;
pub const MQRCCF_Q_MGR_STATUS_NOT_FOUND: crate::MQLONG = 4099;
pub const MQCFBF_STRUC_LENGTH_FIXED: usize = 20;
pub const MQCFBS_STRUC_LENGTH_FIXED: usize = 16;
pub const MQCFGR_STRUC_LENGTH: usize = 16;
pub const MQCFIF_STRUC_LENGTH: usize = 20;
pub const MQCFIL_STRUC_LENGTH_FIXED: usize = 16;
pub const MQCFIL64_STRUC_LENGTH_FIXED: usize = 16;
pub const MQCFIN_STRUC_LENGTH: usize = 16;
pub const MQCFIN64_STRUC_LENGTH: usize = 24;
pub const MQCFSF_STRUC_LENGTH_FIXED: usize = 24;
pub const MQCFSL_STRUC_LENGTH_FIXED: usize = 24;
pub const MQCFST_STRUC_LENGTH_FIXED: usize = 20;
pub const MQEPH_STRUC_ID: &::std::ffi::CStr = c"EPH ";
pub const MQEPH_STRUC_LENGTH_FIXED: usize = 68;
pub const MQEPH_VERSION_1: crate::MQLONG = 1;
pub const MQEPH_CURRENT_VERSION: crate::MQLONG = 1;
pub const MQEPH_LENGTH_1: usize = 68;
pub const MQEPH_CURRENT_LENGTH: usize = 68;
pub const MQEPH_NONE: crate::MQLONG = 0;
pub const MQEPH_CCSID_EMBEDDED: crate::MQLONG = 1;
pub const MQ_ARCHIVE_PFX_LENGTH: usize = 36;
pub const MQ_ARCHIVE_UNIT_LENGTH: usize = 8;
pub const MQ_ASID_LENGTH: usize = 4;
pub const MQ_AUTH_PROFILE_NAME_LENGTH: usize = 48;
pub const MQ_CF_LEID_LENGTH: usize = 12;
pub const MQ_COMMAND_MQSC_LENGTH: usize = 32768;
pub const MQ_DATA_SET_NAME_LENGTH: usize = 44;
pub const MQ_DB2_NAME_LENGTH: usize = 4;
pub const MQ_DSG_NAME_LENGTH: usize = 8;
pub const MQ_ENTITY_NAME_LENGTH: usize = 1024;
pub const MQ_ENV_INFO_LENGTH: usize = 96;
pub const MQ_GROUP_ADDRESS_LENGTH: usize = 264;
pub const MQ_HOST_NAME_LENGTH: usize = 256;
pub const MQ_IP_ADDRESS_LENGTH: usize = 48;
pub const MQ_LOG_CORREL_ID_LENGTH: usize = 8;
pub const MQ_LOG_EXTENT_NAME_LENGTH: usize = 24;
pub const MQ_LOG_PATH_LENGTH: usize = 1024;
pub const MQ_LRSN_LENGTH: usize = 12;
pub const MQ_LSN_LENGTH: usize = 64;
pub const MQ_NHA_REPL_ADDRESS_LENGTH: usize = 264;
pub const MQ_ORIGIN_NAME_LENGTH: usize = 8;
pub const MQ_PSB_NAME_LENGTH: usize = 8;
pub const MQ_PST_ID_LENGTH: usize = 8;
pub const MQ_Q_MGR_CPF_LENGTH: usize = 4;
pub const MQ_Q_MGR_DATA_PATH_LENGTH: usize = 1024;
pub const MQ_RESPONSE_ID_LENGTH: usize = 24;
pub const MQ_RBA_LENGTH: usize = 16;
pub const MQ_REMOTE_PRODUCT_LENGTH: usize = 4;
pub const MQ_REMOTE_VERSION_LENGTH: usize = 8;
pub const MQ_SECURITY_PROFILE_LENGTH: usize = 40;
pub const MQ_SERVICE_COMPONENT_LENGTH: usize = 48;
pub const MQ_SUB_NAME_LENGTH: usize = 10240;
pub const MQ_SYSP_SERVICE_LENGTH: usize = 32;
pub const MQ_SYSTEM_NAME_LENGTH: usize = 8;
pub const MQ_TASK_NUMBER_LENGTH: usize = 8;
pub const MQ_TPIPE_PFX_LENGTH: usize = 4;
pub const MQ_UOW_ID_LENGTH: usize = 256;
pub const MQ_USER_DATA_LENGTH: usize = 10240;
pub const MQ_VOLSER_LENGTH: usize = 6;
pub const MQCFOP_LESS: crate::MQLONG = 1;
pub const MQCFOP_EQUAL: crate::MQLONG = 2;
pub const MQCFOP_GREATER: crate::MQLONG = 4;
pub const MQCFOP_NOT_LESS: crate::MQLONG = 6;
pub const MQCFOP_NOT_EQUAL: crate::MQLONG = 5;
pub const MQCFOP_NOT_GREATER: crate::MQLONG = 3;
pub const MQCFOP_LIKE: crate::MQLONG = 18;
pub const MQCFOP_NOT_LIKE: crate::MQLONG = 21;
pub const MQCFOP_CONTAINS: crate::MQLONG = 10;
pub const MQCFOP_EXCLUDES: crate::MQLONG = 13;
pub const MQCFOP_CONTAINS_GEN: crate::MQLONG = 26;
pub const MQCFOP_EXCLUDES_GEN: crate::MQLONG = 29;
pub const MQCFT_NONE: crate::MQLONG = 0;
pub const MQCFT_COMMAND: crate::MQLONG = 1;
pub const MQCFT_RESPONSE: crate::MQLONG = 2;
pub const MQCFT_INTEGER: crate::MQLONG = 3;
pub const MQCFT_STRING: crate::MQLONG = 4;
pub const MQCFT_INTEGER_LIST: crate::MQLONG = 5;
pub const MQCFT_STRING_LIST: crate::MQLONG = 6;
pub const MQCFT_EVENT: crate::MQLONG = 7;
pub const MQCFT_USER: crate::MQLONG = 8;
pub const MQCFT_BYTE_STRING: crate::MQLONG = 9;
pub const MQCFT_TRACE_ROUTE: crate::MQLONG = 10;
pub const MQCFT_REPORT: crate::MQLONG = 12;
pub const MQCFT_INTEGER_FILTER: crate::MQLONG = 13;
pub const MQCFT_STRING_FILTER: crate::MQLONG = 14;
pub const MQCFT_BYTE_STRING_FILTER: crate::MQLONG = 15;
pub const MQCFT_COMMAND_XR: crate::MQLONG = 16;
pub const MQCFT_XR_MSG: crate::MQLONG = 17;
pub const MQCFT_XR_ITEM: crate::MQLONG = 18;
pub const MQCFT_XR_SUMMARY: crate::MQLONG = 19;
pub const MQCFT_GROUP: crate::MQLONG = 20;
pub const MQCFT_STATISTICS: crate::MQLONG = 21;
pub const MQCFT_ACCOUNTING: crate::MQLONG = 22;
pub const MQCFT_INTEGER64: crate::MQLONG = 23;
pub const MQCFT_INTEGER64_LIST: crate::MQLONG = 25;
pub const MQCFT_APP_ACTIVITY: crate::MQLONG = 26;
pub const MQCFT_STATUS: crate::MQLONG = 27;
pub const MQOPMODE_COMPAT: crate::MQLONG = 0;
pub const MQOPMODE_NEW_FUNCTION: crate::MQLONG = 1;
pub const MQBACF_FIRST: crate::MQLONG = 7001;
pub const MQBACF_EVENT_ACCOUNTING_TOKEN: crate::MQLONG = 7001;
pub const MQBACF_EVENT_SECURITY_ID: crate::MQLONG = 7002;
pub const MQBACF_RESPONSE_SET: crate::MQLONG = 7003;
pub const MQBACF_RESPONSE_ID: crate::MQLONG = 7004;
pub const MQBACF_EXTERNAL_UOW_ID: crate::MQLONG = 7005;
pub const MQBACF_CONNECTION_ID: crate::MQLONG = 7006;
pub const MQBACF_GENERIC_CONNECTION_ID: crate::MQLONG = 7007;
pub const MQBACF_ORIGIN_UOW_ID: crate::MQLONG = 7008;
pub const MQBACF_Q_MGR_UOW_ID: crate::MQLONG = 7009;
pub const MQBACF_ACCOUNTING_TOKEN: crate::MQLONG = 7010;
pub const MQBACF_CORREL_ID: crate::MQLONG = 7011;
pub const MQBACF_GROUP_ID: crate::MQLONG = 7012;
pub const MQBACF_MSG_ID: crate::MQLONG = 7013;
pub const MQBACF_CF_LEID: crate::MQLONG = 7014;
pub const MQBACF_DESTINATION_CORREL_ID: crate::MQLONG = 7015;
pub const MQBACF_SUB_ID: crate::MQLONG = 7016;
pub const MQBACF_ALTERNATE_SECURITYID: crate::MQLONG = 7019;
pub const MQBACF_MESSAGE_DATA: crate::MQLONG = 7020;
pub const MQBACF_MQBO_STRUCT: crate::MQLONG = 7021;
pub const MQBACF_MQCB_FUNCTION: crate::MQLONG = 7022;
pub const MQBACF_MQCBC_STRUCT: crate::MQLONG = 7023;
pub const MQBACF_MQCBD_STRUCT: crate::MQLONG = 7024;
pub const MQBACF_MQCD_STRUCT: crate::MQLONG = 7025;
pub const MQBACF_MQCNO_STRUCT: crate::MQLONG = 7026;
pub const MQBACF_MQGMO_STRUCT: crate::MQLONG = 7027;
pub const MQBACF_MQMD_STRUCT: crate::MQLONG = 7028;
pub const MQBACF_MQPMO_STRUCT: crate::MQLONG = 7029;
pub const MQBACF_MQSD_STRUCT: crate::MQLONG = 7030;
pub const MQBACF_MQSTS_STRUCT: crate::MQLONG = 7031;
pub const MQBACF_SUB_CORREL_ID: crate::MQLONG = 7032;
pub const MQBACF_XA_XID: crate::MQLONG = 7033;
pub const MQBACF_XQH_CORREL_ID: crate::MQLONG = 7034;
pub const MQBACF_XQH_MSG_ID: crate::MQLONG = 7035;
pub const MQBACF_REQUEST_ID: crate::MQLONG = 7036;
pub const MQBACF_PROPERTIES_DATA: crate::MQLONG = 7037;
pub const MQBACF_CONN_TAG: crate::MQLONG = 7038;
pub const MQBACF_MQBNO_STRUCT: crate::MQLONG = 7039;
pub const MQBACF_LAST_USED: crate::MQLONG = 7039;
pub const MQIAMO_FIRST: crate::MQLONG = 701;
pub const MQIAMO_AVG_BATCH_SIZE: crate::MQLONG = 702;
pub const MQIAMO_AVG_Q_TIME: crate::MQLONG = 703;
pub const MQIAMO64_AVG_Q_TIME: crate::MQLONG = 703;
pub const MQIAMO_BACKOUTS: crate::MQLONG = 704;
pub const MQIAMO_BROWSES: crate::MQLONG = 705;
pub const MQIAMO_BROWSE_MAX_BYTES: crate::MQLONG = 706;
pub const MQIAMO_BROWSE_MIN_BYTES: crate::MQLONG = 707;
pub const MQIAMO_BROWSES_FAILED: crate::MQLONG = 708;
pub const MQIAMO_CLOSES: crate::MQLONG = 709;
pub const MQIAMO_COMMITS: crate::MQLONG = 710;
pub const MQIAMO_COMMITS_FAILED: crate::MQLONG = 711;
pub const MQIAMO_CONNS: crate::MQLONG = 712;
pub const MQIAMO_CONNS_MAX: crate::MQLONG = 713;
pub const MQIAMO_DISCS: crate::MQLONG = 714;
pub const MQIAMO_DISCS_IMPLICIT: crate::MQLONG = 715;
pub const MQIAMO_DISC_TYPE: crate::MQLONG = 716;
pub const MQIAMO_EXIT_TIME_AVG: crate::MQLONG = 717;
pub const MQIAMO_EXIT_TIME_MAX: crate::MQLONG = 718;
pub const MQIAMO_EXIT_TIME_MIN: crate::MQLONG = 719;
pub const MQIAMO_FULL_BATCHES: crate::MQLONG = 720;
pub const MQIAMO_GENERATED_MSGS: crate::MQLONG = 721;
pub const MQIAMO_GETS: crate::MQLONG = 722;
pub const MQIAMO_GET_MAX_BYTES: crate::MQLONG = 723;
pub const MQIAMO_GET_MIN_BYTES: crate::MQLONG = 724;
pub const MQIAMO_GETS_FAILED: crate::MQLONG = 725;
pub const MQIAMO_INCOMPLETE_BATCHES: crate::MQLONG = 726;
pub const MQIAMO_INQS: crate::MQLONG = 727;
pub const MQIAMO_MSGS: crate::MQLONG = 728;
pub const MQIAMO_NET_TIME_AVG: crate::MQLONG = 729;
pub const MQIAMO_NET_TIME_MAX: crate::MQLONG = 730;
pub const MQIAMO_NET_TIME_MIN: crate::MQLONG = 731;
pub const MQIAMO_OBJECT_COUNT: crate::MQLONG = 732;
pub const MQIAMO_OPENS: crate::MQLONG = 733;
pub const MQIAMO_PUT1S: crate::MQLONG = 734;
pub const MQIAMO_PUTS: crate::MQLONG = 735;
pub const MQIAMO_PUT_MAX_BYTES: crate::MQLONG = 736;
pub const MQIAMO_PUT_MIN_BYTES: crate::MQLONG = 737;
pub const MQIAMO_PUT_RETRIES: crate::MQLONG = 738;
pub const MQIAMO_Q_MAX_DEPTH: crate::MQLONG = 739;
pub const MQIAMO_Q_MIN_DEPTH: crate::MQLONG = 740;
pub const MQIAMO_Q_TIME_AVG: crate::MQLONG = 741;
pub const MQIAMO64_Q_TIME_AVG: crate::MQLONG = 741;
pub const MQIAMO_Q_TIME_MAX: crate::MQLONG = 742;
pub const MQIAMO64_Q_TIME_MAX: crate::MQLONG = 742;
pub const MQIAMO_Q_TIME_MIN: crate::MQLONG = 743;
pub const MQIAMO64_Q_TIME_MIN: crate::MQLONG = 743;
pub const MQIAMO_SETS: crate::MQLONG = 744;
pub const MQIAMO64_BROWSE_BYTES: crate::MQLONG = 745;
pub const MQIAMO64_BYTES: crate::MQLONG = 746;
pub const MQIAMO64_GET_BYTES: crate::MQLONG = 747;
pub const MQIAMO64_PUT_BYTES: crate::MQLONG = 748;
pub const MQIAMO_CONNS_FAILED: crate::MQLONG = 749;
pub const MQIAMO_OPENS_FAILED: crate::MQLONG = 751;
pub const MQIAMO_INQS_FAILED: crate::MQLONG = 752;
pub const MQIAMO_SETS_FAILED: crate::MQLONG = 753;
pub const MQIAMO_PUTS_FAILED: crate::MQLONG = 754;
pub const MQIAMO_PUT1S_FAILED: crate::MQLONG = 755;
pub const MQIAMO_CLOSES_FAILED: crate::MQLONG = 757;
pub const MQIAMO_MSGS_EXPIRED: crate::MQLONG = 758;
pub const MQIAMO_MSGS_NOT_QUEUED: crate::MQLONG = 759;
pub const MQIAMO_MSGS_PURGED: crate::MQLONG = 760;
pub const MQIAMO_SUBS_DUR: crate::MQLONG = 764;
pub const MQIAMO_SUBS_NDUR: crate::MQLONG = 765;
pub const MQIAMO_SUBS_FAILED: crate::MQLONG = 766;
pub const MQIAMO_SUBRQS: crate::MQLONG = 767;
pub const MQIAMO_SUBRQS_FAILED: crate::MQLONG = 768;
pub const MQIAMO_CBS: crate::MQLONG = 769;
pub const MQIAMO_CBS_FAILED: crate::MQLONG = 770;
pub const MQIAMO_CTLS: crate::MQLONG = 771;
pub const MQIAMO_CTLS_FAILED: crate::MQLONG = 772;
pub const MQIAMO_STATS: crate::MQLONG = 773;
pub const MQIAMO_STATS_FAILED: crate::MQLONG = 774;
pub const MQIAMO_SUB_DUR_HIGHWATER: crate::MQLONG = 775;
pub const MQIAMO_SUB_DUR_LOWWATER: crate::MQLONG = 776;
pub const MQIAMO_SUB_NDUR_HIGHWATER: crate::MQLONG = 777;
pub const MQIAMO_SUB_NDUR_LOWWATER: crate::MQLONG = 778;
pub const MQIAMO_TOPIC_PUTS: crate::MQLONG = 779;
pub const MQIAMO_TOPIC_PUTS_FAILED: crate::MQLONG = 780;
pub const MQIAMO_TOPIC_PUT1S: crate::MQLONG = 781;
pub const MQIAMO_TOPIC_PUT1S_FAILED: crate::MQLONG = 782;
pub const MQIAMO64_TOPIC_PUT_BYTES: crate::MQLONG = 783;
pub const MQIAMO_PUBLISH_MSG_COUNT: crate::MQLONG = 784;
pub const MQIAMO64_PUBLISH_MSG_BYTES: crate::MQLONG = 785;
pub const MQIAMO_UNSUBS_DUR: crate::MQLONG = 786;
pub const MQIAMO_UNSUBS_NDUR: crate::MQLONG = 787;
pub const MQIAMO_UNSUBS_FAILED: crate::MQLONG = 788;
pub const MQIAMO_INTERVAL: crate::MQLONG = 789;
pub const MQIAMO_MSGS_SENT: crate::MQLONG = 790;
pub const MQIAMO_BYTES_SENT: crate::MQLONG = 791;
pub const MQIAMO_REPAIR_BYTES: crate::MQLONG = 792;
pub const MQIAMO_FEEDBACK_MODE: crate::MQLONG = 793;
pub const MQIAMO_RELIABILITY_TYPE: crate::MQLONG = 794;
pub const MQIAMO_LATE_JOIN_MARK: crate::MQLONG = 795;
pub const MQIAMO_NACKS_RCVD: crate::MQLONG = 796;
pub const MQIAMO_REPAIR_PKTS: crate::MQLONG = 797;
pub const MQIAMO_HISTORY_PKTS: crate::MQLONG = 798;
pub const MQIAMO_PENDING_PKTS: crate::MQLONG = 799;
pub const MQIAMO_PKT_RATE: crate::MQLONG = 800;
pub const MQIAMO_MCAST_XMIT_RATE: crate::MQLONG = 801;
pub const MQIAMO_MCAST_BATCH_TIME: crate::MQLONG = 802;
pub const MQIAMO_MCAST_HEARTBEAT: crate::MQLONG = 803;
pub const MQIAMO_DEST_DATA_PORT: crate::MQLONG = 804;
pub const MQIAMO_DEST_REPAIR_PORT: crate::MQLONG = 805;
pub const MQIAMO_ACKS_RCVD: crate::MQLONG = 806;
pub const MQIAMO_ACTIVE_ACKERS: crate::MQLONG = 807;
pub const MQIAMO_PKTS_SENT: crate::MQLONG = 808;
pub const MQIAMO_TOTAL_REPAIR_PKTS: crate::MQLONG = 809;
pub const MQIAMO_TOTAL_PKTS_SENT: crate::MQLONG = 810;
pub const MQIAMO_TOTAL_MSGS_SENT: crate::MQLONG = 811;
pub const MQIAMO_TOTAL_BYTES_SENT: crate::MQLONG = 812;
pub const MQIAMO_NUM_STREAMS: crate::MQLONG = 813;
pub const MQIAMO_ACK_FEEDBACK: crate::MQLONG = 814;
pub const MQIAMO_NACK_FEEDBACK: crate::MQLONG = 815;
pub const MQIAMO_PKTS_LOST: crate::MQLONG = 816;
pub const MQIAMO_MSGS_RCVD: crate::MQLONG = 817;
pub const MQIAMO_MSG_BYTES_RCVD: crate::MQLONG = 818;
pub const MQIAMO_MSGS_DELIVERED: crate::MQLONG = 819;
pub const MQIAMO_PKTS_PROCESSED: crate::MQLONG = 820;
pub const MQIAMO_PKTS_DELIVERED: crate::MQLONG = 821;
pub const MQIAMO_PKTS_DROPPED: crate::MQLONG = 822;
pub const MQIAMO_PKTS_DUPLICATED: crate::MQLONG = 823;
pub const MQIAMO_NACKS_CREATED: crate::MQLONG = 824;
pub const MQIAMO_NACK_PKTS_SENT: crate::MQLONG = 825;
pub const MQIAMO_REPAIR_PKTS_RQSTD: crate::MQLONG = 826;
pub const MQIAMO_REPAIR_PKTS_RCVD: crate::MQLONG = 827;
pub const MQIAMO_PKTS_REPAIRED: crate::MQLONG = 828;
pub const MQIAMO_TOTAL_MSGS_RCVD: crate::MQLONG = 829;
pub const MQIAMO_TOTAL_MSG_BYTES_RCVD: crate::MQLONG = 830;
pub const MQIAMO_TOTAL_REPAIR_PKTS_RCVD: crate::MQLONG = 831;
pub const MQIAMO_TOTAL_REPAIR_PKTS_RQSTD: crate::MQLONG = 832;
pub const MQIAMO_TOTAL_MSGS_PROCESSED: crate::MQLONG = 833;
pub const MQIAMO_TOTAL_MSGS_SELECTED: crate::MQLONG = 834;
pub const MQIAMO_TOTAL_MSGS_EXPIRED: crate::MQLONG = 835;
pub const MQIAMO_TOTAL_MSGS_DELIVERED: crate::MQLONG = 836;
pub const MQIAMO_TOTAL_MSGS_RETURNED: crate::MQLONG = 837;
pub const MQIAMO64_HIGHRES_TIME: crate::MQLONG = 838;
pub const MQIAMO_MONITOR_CLASS: crate::MQLONG = 839;
pub const MQIAMO_MONITOR_TYPE: crate::MQLONG = 840;
pub const MQIAMO_MONITOR_ELEMENT: crate::MQLONG = 841;
pub const MQIAMO_MONITOR_DATATYPE: crate::MQLONG = 842;
pub const MQIAMO_MONITOR_FLAGS: crate::MQLONG = 843;
pub const MQIAMO64_QMGR_OP_DURATION: crate::MQLONG = 844;
pub const MQIAMO64_MONITOR_INTERVAL: crate::MQLONG = 845;
pub const MQIAMO_LAST_USED: crate::MQLONG = 845;
pub const MQIAMO_MONITOR_FLAGS_NONE: crate::MQLONG = 0;
pub const MQIAMO_MONITOR_FLAGS_OBJNAME: crate::MQLONG = 1;
pub const MQIAMO_MONITOR_UNIT: crate::MQLONG = 1;
pub const MQIAMO_MONITOR_DELTA: crate::MQLONG = 2;
pub const MQIAMO_MONITOR_LSN: crate::MQLONG = 3;
pub const MQIAMO_MONITOR_HUNDREDTHS: crate::MQLONG = 100;
pub const MQIAMO_MONITOR_KB: crate::MQLONG = 1024;
pub const MQIAMO_MONITOR_PERCENT: crate::MQLONG = 10000;
pub const MQIAMO_MONITOR_MICROSEC: crate::MQLONG = 1000000;
pub const MQIAMO_MONITOR_MB: crate::MQLONG = 1048576;
pub const MQIAMO_MONITOR_GB: crate::MQLONG = 100000000;
pub const MQIACF_FIRST: crate::MQLONG = 1001;
pub const MQIACF_Q_MGR_ATTRS: crate::MQLONG = 1001;
pub const MQIACF_Q_ATTRS: crate::MQLONG = 1002;
pub const MQIACF_PROCESS_ATTRS: crate::MQLONG = 1003;
pub const MQIACF_NAMELIST_ATTRS: crate::MQLONG = 1004;
pub const MQIACF_FORCE: crate::MQLONG = 1005;
pub const MQIACF_REPLACE: crate::MQLONG = 1006;
pub const MQIACF_PURGE: crate::MQLONG = 1007;
pub const MQIACF_QUIESCE: crate::MQLONG = 1008;
pub const MQIACF_MODE: crate::MQLONG = 1008;
pub const MQIACF_ALL: crate::MQLONG = 1009;
pub const MQIACF_EVENT_APPL_TYPE: crate::MQLONG = 1010;
pub const MQIACF_EVENT_ORIGIN: crate::MQLONG = 1011;
pub const MQIACF_PARAMETER_ID: crate::MQLONG = 1012;
pub const MQIACF_ERROR_ID: crate::MQLONG = 1013;
pub const MQIACF_ERROR_IDENTIFIER: crate::MQLONG = 1013;
pub const MQIACF_SELECTOR: crate::MQLONG = 1014;
pub const MQIACF_CHANNEL_ATTRS: crate::MQLONG = 1015;
pub const MQIACF_OBJECT_TYPE: crate::MQLONG = 1016;
pub const MQIACF_ESCAPE_TYPE: crate::MQLONG = 1017;
pub const MQIACF_ERROR_OFFSET: crate::MQLONG = 1018;
pub const MQIACF_AUTH_INFO_ATTRS: crate::MQLONG = 1019;
pub const MQIACF_REASON_QUALIFIER: crate::MQLONG = 1020;
pub const MQIACF_COMMAND: crate::MQLONG = 1021;
pub const MQIACF_OPEN_OPTIONS: crate::MQLONG = 1022;
pub const MQIACF_OPEN_TYPE: crate::MQLONG = 1023;
pub const MQIACF_PROCESS_ID: crate::MQLONG = 1024;
pub const MQIACF_THREAD_ID: crate::MQLONG = 1025;
pub const MQIACF_Q_STATUS_ATTRS: crate::MQLONG = 1026;
pub const MQIACF_UNCOMMITTED_MSGS: crate::MQLONG = 1027;
pub const MQIACF_HANDLE_STATE: crate::MQLONG = 1028;
pub const MQIACF_AUX_ERROR_DATA_INT_1: crate::MQLONG = 1070;
pub const MQIACF_AUX_ERROR_DATA_INT_2: crate::MQLONG = 1071;
pub const MQIACF_CONV_REASON_CODE: crate::MQLONG = 1072;
pub const MQIACF_BRIDGE_TYPE: crate::MQLONG = 1073;
pub const MQIACF_INQUIRY: crate::MQLONG = 1074;
pub const MQIACF_WAIT_INTERVAL: crate::MQLONG = 1075;
pub const MQIACF_OPTIONS: crate::MQLONG = 1076;
pub const MQIACF_BROKER_OPTIONS: crate::MQLONG = 1077;
pub const MQIACF_REFRESH_TYPE: crate::MQLONG = 1078;
pub const MQIACF_SEQUENCE_NUMBER: crate::MQLONG = 1079;
pub const MQIACF_INTEGER_DATA: crate::MQLONG = 1080;
pub const MQIACF_REGISTRATION_OPTIONS: crate::MQLONG = 1081;
pub const MQIACF_PUBLICATION_OPTIONS: crate::MQLONG = 1082;
pub const MQIACF_CLUSTER_INFO: crate::MQLONG = 1083;
pub const MQIACF_Q_MGR_DEFINITION_TYPE: crate::MQLONG = 1084;
pub const MQIACF_Q_MGR_TYPE: crate::MQLONG = 1085;
pub const MQIACF_ACTION: crate::MQLONG = 1086;
pub const MQIACF_SUSPEND: crate::MQLONG = 1087;
pub const MQIACF_BROKER_COUNT: crate::MQLONG = 1088;
pub const MQIACF_APPL_COUNT: crate::MQLONG = 1089;
pub const MQIACF_ANONYMOUS_COUNT: crate::MQLONG = 1090;
pub const MQIACF_REG_REG_OPTIONS: crate::MQLONG = 1091;
pub const MQIACF_DELETE_OPTIONS: crate::MQLONG = 1092;
pub const MQIACF_CLUSTER_Q_MGR_ATTRS: crate::MQLONG = 1093;
pub const MQIACF_REFRESH_INTERVAL: crate::MQLONG = 1094;
pub const MQIACF_REFRESH_REPOSITORY: crate::MQLONG = 1095;
pub const MQIACF_REMOVE_QUEUES: crate::MQLONG = 1096;
pub const MQIACF_OPEN_INPUT_TYPE: crate::MQLONG = 1098;
pub const MQIACF_OPEN_OUTPUT: crate::MQLONG = 1099;
pub const MQIACF_OPEN_SET: crate::MQLONG = 1100;
pub const MQIACF_OPEN_INQUIRE: crate::MQLONG = 1101;
pub const MQIACF_OPEN_BROWSE: crate::MQLONG = 1102;
pub const MQIACF_Q_STATUS_TYPE: crate::MQLONG = 1103;
pub const MQIACF_Q_HANDLE: crate::MQLONG = 1104;
pub const MQIACF_Q_STATUS: crate::MQLONG = 1105;
pub const MQIACF_SECURITY_TYPE: crate::MQLONG = 1106;
pub const MQIACF_CONNECTION_ATTRS: crate::MQLONG = 1107;
pub const MQIACF_CONNECT_OPTIONS: crate::MQLONG = 1108;
pub const MQIACF_CONN_INFO_TYPE: crate::MQLONG = 1110;
pub const MQIACF_CONN_INFO_CONN: crate::MQLONG = 1111;
pub const MQIACF_CONN_INFO_HANDLE: crate::MQLONG = 1112;
pub const MQIACF_CONN_INFO_ALL: crate::MQLONG = 1113;
pub const MQIACF_AUTH_PROFILE_ATTRS: crate::MQLONG = 1114;
pub const MQIACF_AUTHORIZATION_LIST: crate::MQLONG = 1115;
pub const MQIACF_AUTH_ADD_AUTHS: crate::MQLONG = 1116;
pub const MQIACF_AUTH_REMOVE_AUTHS: crate::MQLONG = 1117;
pub const MQIACF_ENTITY_TYPE: crate::MQLONG = 1118;
pub const MQIACF_COMMAND_INFO: crate::MQLONG = 1120;
pub const MQIACF_CMDSCOPE_Q_MGR_COUNT: crate::MQLONG = 1121;
pub const MQIACF_Q_MGR_SYSTEM: crate::MQLONG = 1122;
pub const MQIACF_Q_MGR_EVENT: crate::MQLONG = 1123;
pub const MQIACF_Q_MGR_DQM: crate::MQLONG = 1124;
pub const MQIACF_Q_MGR_CLUSTER: crate::MQLONG = 1125;
pub const MQIACF_QSG_DISPS: crate::MQLONG = 1126;
pub const MQIACF_UOW_STATE: crate::MQLONG = 1128;
pub const MQIACF_SECURITY_ITEM: crate::MQLONG = 1129;
pub const MQIACF_CF_STRUC_STATUS: crate::MQLONG = 1130;
pub const MQIACF_UOW_TYPE: crate::MQLONG = 1132;
pub const MQIACF_CF_STRUC_ATTRS: crate::MQLONG = 1133;
pub const MQIACF_EXCLUDE_INTERVAL: crate::MQLONG = 1134;
pub const MQIACF_CF_STATUS_TYPE: crate::MQLONG = 1135;
pub const MQIACF_CF_STATUS_SUMMARY: crate::MQLONG = 1136;
pub const MQIACF_CF_STATUS_CONNECT: crate::MQLONG = 1137;
pub const MQIACF_CF_STATUS_BACKUP: crate::MQLONG = 1138;
pub const MQIACF_CF_STRUC_TYPE: crate::MQLONG = 1139;
pub const MQIACF_CF_STRUC_SIZE_MAX: crate::MQLONG = 1140;
pub const MQIACF_CF_STRUC_SIZE_USED: crate::MQLONG = 1141;
pub const MQIACF_CF_STRUC_ENTRIES_MAX: crate::MQLONG = 1142;
pub const MQIACF_CF_STRUC_ENTRIES_USED: crate::MQLONG = 1143;
pub const MQIACF_CF_STRUC_BACKUP_SIZE: crate::MQLONG = 1144;
pub const MQIACF_MOVE_TYPE: crate::MQLONG = 1145;
pub const MQIACF_MOVE_TYPE_MOVE: crate::MQLONG = 1146;
pub const MQIACF_MOVE_TYPE_ADD: crate::MQLONG = 1147;
pub const MQIACF_Q_MGR_NUMBER: crate::MQLONG = 1148;
pub const MQIACF_Q_MGR_STATUS: crate::MQLONG = 1149;
pub const MQIACF_DB2_CONN_STATUS: crate::MQLONG = 1150;
pub const MQIACF_SECURITY_ATTRS: crate::MQLONG = 1151;
pub const MQIACF_SECURITY_TIMEOUT: crate::MQLONG = 1152;
pub const MQIACF_SECURITY_INTERVAL: crate::MQLONG = 1153;
pub const MQIACF_SECURITY_SWITCH: crate::MQLONG = 1154;
pub const MQIACF_SECURITY_SETTING: crate::MQLONG = 1155;
pub const MQIACF_STORAGE_CLASS_ATTRS: crate::MQLONG = 1156;
pub const MQIACF_USAGE_TYPE: crate::MQLONG = 1157;
pub const MQIACF_BUFFER_POOL_ID: crate::MQLONG = 1158;
pub const MQIACF_USAGE_TOTAL_PAGES: crate::MQLONG = 1159;
pub const MQIACF_USAGE_UNUSED_PAGES: crate::MQLONG = 1160;
pub const MQIACF_USAGE_PERSIST_PAGES: crate::MQLONG = 1161;
pub const MQIACF_USAGE_NONPERSIST_PAGES: crate::MQLONG = 1162;
pub const MQIACF_USAGE_RESTART_EXTENTS: crate::MQLONG = 1163;
pub const MQIACF_USAGE_EXPAND_COUNT: crate::MQLONG = 1164;
pub const MQIACF_PAGESET_STATUS: crate::MQLONG = 1165;
pub const MQIACF_USAGE_TOTAL_BUFFERS: crate::MQLONG = 1166;
pub const MQIACF_USAGE_DATA_SET_TYPE: crate::MQLONG = 1167;
pub const MQIACF_USAGE_PAGESET: crate::MQLONG = 1168;
pub const MQIACF_USAGE_DATA_SET: crate::MQLONG = 1169;
pub const MQIACF_USAGE_BUFFER_POOL: crate::MQLONG = 1170;
pub const MQIACF_MOVE_COUNT: crate::MQLONG = 1171;
pub const MQIACF_EXPIRY_Q_COUNT: crate::MQLONG = 1172;
pub const MQIACF_CONFIGURATION_OBJECTS: crate::MQLONG = 1173;
pub const MQIACF_CONFIGURATION_EVENTS: crate::MQLONG = 1174;
pub const MQIACF_SYSP_TYPE: crate::MQLONG = 1175;
pub const MQIACF_SYSP_DEALLOC_INTERVAL: crate::MQLONG = 1176;
pub const MQIACF_SYSP_MAX_ARCHIVE: crate::MQLONG = 1177;
pub const MQIACF_SYSP_MAX_READ_TAPES: crate::MQLONG = 1178;
pub const MQIACF_SYSP_IN_BUFFER_SIZE: crate::MQLONG = 1179;
pub const MQIACF_SYSP_OUT_BUFFER_SIZE: crate::MQLONG = 1180;
pub const MQIACF_SYSP_OUT_BUFFER_COUNT: crate::MQLONG = 1181;
pub const MQIACF_SYSP_ARCHIVE: crate::MQLONG = 1182;
pub const MQIACF_SYSP_DUAL_ACTIVE: crate::MQLONG = 1183;
pub const MQIACF_SYSP_DUAL_ARCHIVE: crate::MQLONG = 1184;
pub const MQIACF_SYSP_DUAL_BSDS: crate::MQLONG = 1185;
pub const MQIACF_SYSP_MAX_CONNS: crate::MQLONG = 1186;
pub const MQIACF_SYSP_MAX_CONNS_FORE: crate::MQLONG = 1187;
pub const MQIACF_SYSP_MAX_CONNS_BACK: crate::MQLONG = 1188;
pub const MQIACF_SYSP_EXIT_INTERVAL: crate::MQLONG = 1189;
pub const MQIACF_SYSP_EXIT_TASKS: crate::MQLONG = 1190;
pub const MQIACF_SYSP_CHKPOINT_COUNT: crate::MQLONG = 1191;
pub const MQIACF_SYSP_OTMA_INTERVAL: crate::MQLONG = 1192;
pub const MQIACF_SYSP_Q_INDEX_DEFER: crate::MQLONG = 1193;
pub const MQIACF_SYSP_DB2_TASKS: crate::MQLONG = 1194;
pub const MQIACF_SYSP_RESLEVEL_AUDIT: crate::MQLONG = 1195;
pub const MQIACF_SYSP_ROUTING_CODE: crate::MQLONG = 1196;
pub const MQIACF_SYSP_SMF_ACCOUNTING: crate::MQLONG = 1197;
pub const MQIACF_SYSP_SMF_STATS: crate::MQLONG = 1198;
pub const MQIACF_SYSP_SMF_INTERVAL: crate::MQLONG = 1199;
pub const MQIACF_SYSP_SMF_STAT_TIME_MINS: crate::MQLONG = 1199;
pub const MQIACF_SYSP_TRACE_CLASS: crate::MQLONG = 1200;
pub const MQIACF_SYSP_TRACE_SIZE: crate::MQLONG = 1201;
pub const MQIACF_SYSP_WLM_INTERVAL: crate::MQLONG = 1202;
pub const MQIACF_SYSP_ALLOC_UNIT: crate::MQLONG = 1203;
pub const MQIACF_SYSP_ARCHIVE_RETAIN: crate::MQLONG = 1204;
pub const MQIACF_SYSP_ARCHIVE_WTOR: crate::MQLONG = 1205;
pub const MQIACF_SYSP_BLOCK_SIZE: crate::MQLONG = 1206;
pub const MQIACF_SYSP_CATALOG: crate::MQLONG = 1207;
pub const MQIACF_SYSP_COMPACT: crate::MQLONG = 1208;
pub const MQIACF_SYSP_ALLOC_PRIMARY: crate::MQLONG = 1209;
pub const MQIACF_SYSP_ALLOC_SECONDARY: crate::MQLONG = 1210;
pub const MQIACF_SYSP_PROTECT: crate::MQLONG = 1211;
pub const MQIACF_SYSP_QUIESCE_INTERVAL: crate::MQLONG = 1212;
pub const MQIACF_SYSP_TIMESTAMP: crate::MQLONG = 1213;
pub const MQIACF_SYSP_UNIT_ADDRESS: crate::MQLONG = 1214;
pub const MQIACF_SYSP_UNIT_STATUS: crate::MQLONG = 1215;
pub const MQIACF_SYSP_LOG_COPY: crate::MQLONG = 1216;
pub const MQIACF_SYSP_LOG_USED: crate::MQLONG = 1217;
pub const MQIACF_SYSP_LOG_SUSPEND: crate::MQLONG = 1218;
pub const MQIACF_SYSP_OFFLOAD_STATUS: crate::MQLONG = 1219;
pub const MQIACF_SYSP_TOTAL_LOGS: crate::MQLONG = 1220;
pub const MQIACF_SYSP_FULL_LOGS: crate::MQLONG = 1221;
pub const MQIACF_LISTENER_ATTRS: crate::MQLONG = 1222;
pub const MQIACF_LISTENER_STATUS_ATTRS: crate::MQLONG = 1223;
pub const MQIACF_SERVICE_ATTRS: crate::MQLONG = 1224;
pub const MQIACF_SERVICE_STATUS_ATTRS: crate::MQLONG = 1225;
pub const MQIACF_Q_TIME_INDICATOR: crate::MQLONG = 1226;
pub const MQIACF_OLDEST_MSG_AGE: crate::MQLONG = 1227;
pub const MQIACF_AUTH_OPTIONS: crate::MQLONG = 1228;
pub const MQIACF_Q_MGR_STATUS_ATTRS: crate::MQLONG = 1229;
pub const MQIACF_CONNECTION_COUNT: crate::MQLONG = 1230;
pub const MQIACF_Q_MGR_FACILITY: crate::MQLONG = 1231;
pub const MQIACF_CHINIT_STATUS: crate::MQLONG = 1232;
pub const MQIACF_CMD_SERVER_STATUS: crate::MQLONG = 1233;
pub const MQIACF_ROUTE_DETAIL: crate::MQLONG = 1234;
pub const MQIACF_RECORDED_ACTIVITIES: crate::MQLONG = 1235;
pub const MQIACF_MAX_ACTIVITIES: crate::MQLONG = 1236;
pub const MQIACF_DISCONTINUITY_COUNT: crate::MQLONG = 1237;
pub const MQIACF_ROUTE_ACCUMULATION: crate::MQLONG = 1238;
pub const MQIACF_ROUTE_DELIVERY: crate::MQLONG = 1239;
pub const MQIACF_OPERATION_TYPE: crate::MQLONG = 1240;
pub const MQIACF_BACKOUT_COUNT: crate::MQLONG = 1241;
pub const MQIACF_COMP_CODE: crate::MQLONG = 1242;
pub const MQIACF_ENCODING: crate::MQLONG = 1243;
pub const MQIACF_EXPIRY: crate::MQLONG = 1244;
pub const MQIACF_FEEDBACK: crate::MQLONG = 1245;
pub const MQIACF_MSG_FLAGS: crate::MQLONG = 1247;
pub const MQIACF_MSG_LENGTH: crate::MQLONG = 1248;
pub const MQIACF_MSG_TYPE: crate::MQLONG = 1249;
pub const MQIACF_OFFSET: crate::MQLONG = 1250;
pub const MQIACF_ORIGINAL_LENGTH: crate::MQLONG = 1251;
pub const MQIACF_PERSISTENCE: crate::MQLONG = 1252;
pub const MQIACF_PRIORITY: crate::MQLONG = 1253;
pub const MQIACF_REASON_CODE: crate::MQLONG = 1254;
pub const MQIACF_REPORT: crate::MQLONG = 1255;
pub const MQIACF_VERSION: crate::MQLONG = 1256;
pub const MQIACF_UNRECORDED_ACTIVITIES: crate::MQLONG = 1257;
pub const MQIACF_MONITORING: crate::MQLONG = 1258;
pub const MQIACF_ROUTE_FORWARDING: crate::MQLONG = 1259;
pub const MQIACF_SERVICE_STATUS: crate::MQLONG = 1260;
pub const MQIACF_Q_TYPES: crate::MQLONG = 1261;
pub const MQIACF_USER_ID_SUPPORT: crate::MQLONG = 1262;
pub const MQIACF_INTERFACE_VERSION: crate::MQLONG = 1263;
pub const MQIACF_AUTH_SERVICE_ATTRS: crate::MQLONG = 1264;
pub const MQIACF_USAGE_EXPAND_TYPE: crate::MQLONG = 1265;
pub const MQIACF_SYSP_CLUSTER_CACHE: crate::MQLONG = 1266;
pub const MQIACF_SYSP_DB2_BLOB_TASKS: crate::MQLONG = 1267;
pub const MQIACF_SYSP_WLM_INT_UNITS: crate::MQLONG = 1268;
pub const MQIACF_TOPIC_ATTRS: crate::MQLONG = 1269;
pub const MQIACF_PUBSUB_PROPERTIES: crate::MQLONG = 1271;
pub const MQIACF_DESTINATION_CLASS: crate::MQLONG = 1273;
pub const MQIACF_DURABLE_SUBSCRIPTION: crate::MQLONG = 1274;
pub const MQIACF_SUBSCRIPTION_SCOPE: crate::MQLONG = 1275;
pub const MQIACF_VARIABLE_USER_ID: crate::MQLONG = 1277;
pub const MQIACF_REQUEST_ONLY: crate::MQLONG = 1280;
pub const MQIACF_PUB_PRIORITY: crate::MQLONG = 1283;
pub const MQIACF_SUB_ATTRS: crate::MQLONG = 1287;
pub const MQIACF_WILDCARD_SCHEMA: crate::MQLONG = 1288;
pub const MQIACF_SUB_TYPE: crate::MQLONG = 1289;
pub const MQIACF_MESSAGE_COUNT: crate::MQLONG = 1290;
pub const MQIACF_Q_MGR_PUBSUB: crate::MQLONG = 1291;
pub const MQIACF_Q_MGR_VERSION: crate::MQLONG = 1292;
pub const MQIACF_SUB_STATUS_ATTRS: crate::MQLONG = 1294;
pub const MQIACF_TOPIC_STATUS: crate::MQLONG = 1295;
pub const MQIACF_TOPIC_SUB: crate::MQLONG = 1296;
pub const MQIACF_TOPIC_PUB: crate::MQLONG = 1297;
pub const MQIACF_RETAINED_PUBLICATION: crate::MQLONG = 1300;
pub const MQIACF_TOPIC_STATUS_ATTRS: crate::MQLONG = 1301;
pub const MQIACF_TOPIC_STATUS_TYPE: crate::MQLONG = 1302;
pub const MQIACF_SUB_OPTIONS: crate::MQLONG = 1303;
pub const MQIACF_PUBLISH_COUNT: crate::MQLONG = 1304;
pub const MQIACF_CLEAR_TYPE: crate::MQLONG = 1305;
pub const MQIACF_CLEAR_SCOPE: crate::MQLONG = 1306;
pub const MQIACF_SUB_LEVEL: crate::MQLONG = 1307;
pub const MQIACF_ASYNC_STATE: crate::MQLONG = 1308;
pub const MQIACF_SUB_SUMMARY: crate::MQLONG = 1309;
pub const MQIACF_OBSOLETE_MSGS: crate::MQLONG = 1310;
pub const MQIACF_PUBSUB_STATUS: crate::MQLONG = 1311;
pub const MQIACF_PS_STATUS_TYPE: crate::MQLONG = 1314;
pub const MQIACF_PUBSUB_STATUS_ATTRS: crate::MQLONG = 1318;
pub const MQIACF_SELECTOR_TYPE: crate::MQLONG = 1321;
pub const MQIACF_LOG_COMPRESSION: crate::MQLONG = 1322;
pub const MQIACF_GROUPUR_CHECK_ID: crate::MQLONG = 1323;
pub const MQIACF_MULC_CAPTURE: crate::MQLONG = 1324;
pub const MQIACF_PERMIT_STANDBY: crate::MQLONG = 1325;
pub const MQIACF_OPERATION_MODE: crate::MQLONG = 1326;
pub const MQIACF_COMM_INFO_ATTRS: crate::MQLONG = 1327;
pub const MQIACF_CF_SMDS_BLOCK_SIZE: crate::MQLONG = 1328;
pub const MQIACF_CF_SMDS_EXPAND: crate::MQLONG = 1329;
pub const MQIACF_USAGE_FREE_BUFF: crate::MQLONG = 1330;
pub const MQIACF_USAGE_FREE_BUFF_PERC: crate::MQLONG = 1331;
pub const MQIACF_CF_STRUC_ACCESS: crate::MQLONG = 1332;
pub const MQIACF_CF_STATUS_SMDS: crate::MQLONG = 1333;
pub const MQIACF_SMDS_ATTRS: crate::MQLONG = 1334;
pub const MQIACF_USAGE_SMDS: crate::MQLONG = 1335;
pub const MQIACF_USAGE_BLOCK_SIZE: crate::MQLONG = 1336;
pub const MQIACF_USAGE_DATA_BLOCKS: crate::MQLONG = 1337;
pub const MQIACF_USAGE_EMPTY_BUFFERS: crate::MQLONG = 1338;
pub const MQIACF_USAGE_INUSE_BUFFERS: crate::MQLONG = 1339;
pub const MQIACF_USAGE_LOWEST_FREE: crate::MQLONG = 1340;
pub const MQIACF_USAGE_OFFLOAD_MSGS: crate::MQLONG = 1341;
pub const MQIACF_USAGE_READS_SAVED: crate::MQLONG = 1342;
pub const MQIACF_USAGE_SAVED_BUFFERS: crate::MQLONG = 1343;
pub const MQIACF_USAGE_TOTAL_BLOCKS: crate::MQLONG = 1344;
pub const MQIACF_USAGE_USED_BLOCKS: crate::MQLONG = 1345;
pub const MQIACF_USAGE_USED_RATE: crate::MQLONG = 1346;
pub const MQIACF_USAGE_WAIT_RATE: crate::MQLONG = 1347;
pub const MQIACF_SMDS_OPENMODE: crate::MQLONG = 1348;
pub const MQIACF_SMDS_STATUS: crate::MQLONG = 1349;
pub const MQIACF_SMDS_AVAIL: crate::MQLONG = 1350;
pub const MQIACF_MCAST_REL_INDICATOR: crate::MQLONG = 1351;
pub const MQIACF_CHLAUTH_TYPE: crate::MQLONG = 1352;
pub const MQIACF_MQXR_DIAGNOSTICS_TYPE: crate::MQLONG = 1354;
pub const MQIACF_CHLAUTH_ATTRS: crate::MQLONG = 1355;
pub const MQIACF_OPERATION_ID: crate::MQLONG = 1356;
pub const MQIACF_API_CALLER_TYPE: crate::MQLONG = 1357;
pub const MQIACF_API_ENVIRONMENT: crate::MQLONG = 1358;
pub const MQIACF_TRACE_DETAIL: crate::MQLONG = 1359;
pub const MQIACF_HOBJ: crate::MQLONG = 1360;
pub const MQIACF_CALL_TYPE: crate::MQLONG = 1361;
pub const MQIACF_MQCB_OPERATION: crate::MQLONG = 1362;
pub const MQIACF_MQCB_TYPE: crate::MQLONG = 1363;
pub const MQIACF_MQCB_OPTIONS: crate::MQLONG = 1364;
pub const MQIACF_CLOSE_OPTIONS: crate::MQLONG = 1365;
pub const MQIACF_CTL_OPERATION: crate::MQLONG = 1366;
pub const MQIACF_GET_OPTIONS: crate::MQLONG = 1367;
pub const MQIACF_RECS_PRESENT: crate::MQLONG = 1368;
pub const MQIACF_KNOWN_DEST_COUNT: crate::MQLONG = 1369;
pub const MQIACF_UNKNOWN_DEST_COUNT: crate::MQLONG = 1370;
pub const MQIACF_INVALID_DEST_COUNT: crate::MQLONG = 1371;
pub const MQIACF_RESOLVED_TYPE: crate::MQLONG = 1372;
pub const MQIACF_PUT_OPTIONS: crate::MQLONG = 1373;
pub const MQIACF_BUFFER_LENGTH: crate::MQLONG = 1374;
pub const MQIACF_TRACE_DATA_LENGTH: crate::MQLONG = 1375;
pub const MQIACF_SMDS_EXPANDST: crate::MQLONG = 1376;
pub const MQIACF_STRUC_LENGTH: usize = 1377;
pub const MQIACF_ITEM_COUNT: crate::MQLONG = 1378;
pub const MQIACF_EXPIRY_TIME: crate::MQLONG = 1379;
pub const MQIACF_CONNECT_TIME: crate::MQLONG = 1380;
pub const MQIACF_DISCONNECT_TIME: crate::MQLONG = 1381;
pub const MQIACF_HSUB: crate::MQLONG = 1382;
pub const MQIACF_SUBRQ_OPTIONS: crate::MQLONG = 1383;
pub const MQIACF_XA_RMID: crate::MQLONG = 1384;
pub const MQIACF_XA_FLAGS: crate::MQLONG = 1385;
pub const MQIACF_XA_RETCODE: crate::MQLONG = 1386;
pub const MQIACF_XA_HANDLE: crate::MQLONG = 1387;
pub const MQIACF_XA_RETVAL: crate::MQLONG = 1388;
pub const MQIACF_STATUS_TYPE: crate::MQLONG = 1389;
pub const MQIACF_XA_COUNT: crate::MQLONG = 1390;
pub const MQIACF_SELECTOR_COUNT: crate::MQLONG = 1391;
pub const MQIACF_SELECTORS: crate::MQLONG = 1392;
pub const MQIACF_INTATTR_COUNT: crate::MQLONG = 1393;
pub const MQIACF_INT_ATTRS: crate::MQLONG = 1394;
pub const MQIACF_SUBRQ_ACTION: crate::MQLONG = 1395;
pub const MQIACF_NUM_PUBS: crate::MQLONG = 1396;
pub const MQIACF_POINTER_SIZE: crate::MQLONG = 1397;
pub const MQIACF_REMOVE_AUTHREC: crate::MQLONG = 1398;
pub const MQIACF_XR_ATTRS: crate::MQLONG = 1399;
pub const MQIACF_APPL_FUNCTION_TYPE: crate::MQLONG = 1400;
pub const MQIACF_AMQP_ATTRS: crate::MQLONG = 1401;
pub const MQIACF_EXPORT_TYPE: crate::MQLONG = 1402;
pub const MQIACF_EXPORT_ATTRS: crate::MQLONG = 1403;
pub const MQIACF_SYSTEM_OBJECTS: crate::MQLONG = 1404;
pub const MQIACF_CONNECTION_SWAP: crate::MQLONG = 1405;
pub const MQIACF_AMQP_DIAGNOSTICS_TYPE: crate::MQLONG = 1406;
pub const MQIACF_BUFFER_POOL_LOCATION: crate::MQLONG = 1408;
pub const MQIACF_LDAP_CONNECTION_STATUS: crate::MQLONG = 1409;
pub const MQIACF_SYSP_MAX_ACE_POOL: crate::MQLONG = 1410;
pub const MQIACF_PAGECLAS: crate::MQLONG = 1411;
pub const MQIACF_AUTH_REC_TYPE: crate::MQLONG = 1412;
pub const MQIACF_SYSP_MAX_CONC_OFFLOADS: crate::MQLONG = 1413;
pub const MQIACF_SYSP_ZHYPERWRITE: crate::MQLONG = 1414;
pub const MQIACF_Q_MGR_STATUS_LOG: crate::MQLONG = 1415;
pub const MQIACF_ARCHIVE_LOG_SIZE: crate::MQLONG = 1416;
pub const MQIACF_MEDIA_LOG_SIZE: crate::MQLONG = 1417;
pub const MQIACF_RESTART_LOG_SIZE: crate::MQLONG = 1418;
pub const MQIACF_REUSABLE_LOG_SIZE: crate::MQLONG = 1419;
pub const MQIACF_LOG_IN_USE: crate::MQLONG = 1420;
pub const MQIACF_LOG_UTILIZATION: crate::MQLONG = 1421;
pub const MQIACF_LOG_REDUCTION: crate::MQLONG = 1422;
pub const MQIACF_IGNORE_STATE: crate::MQLONG = 1423;
pub const MQIACF_MOVABLE_APPL_COUNT: crate::MQLONG = 1424;
pub const MQIACF_APPL_INFO_ATTRS: crate::MQLONG = 1425;
pub const MQIACF_APPL_MOVABLE: crate::MQLONG = 1426;
pub const MQIACF_REMOTE_QMGR_ACTIVE: crate::MQLONG = 1427;
pub const MQIACF_APPL_INFO_TYPE: crate::MQLONG = 1428;
pub const MQIACF_APPL_INFO_APPL: crate::MQLONG = 1429;
pub const MQIACF_APPL_INFO_QMGR: crate::MQLONG = 1430;
pub const MQIACF_APPL_INFO_LOCAL: crate::MQLONG = 1431;
pub const MQIACF_APPL_IMMOVABLE_COUNT: crate::MQLONG = 1432;
pub const MQIACF_BALANCED: crate::MQLONG = 1433;
pub const MQIACF_BALSTATE: crate::MQLONG = 1434;
pub const MQIACF_APPL_IMMOVABLE_REASON: crate::MQLONG = 1435;
pub const MQIACF_DS_ENCRYPTED: crate::MQLONG = 1436;
pub const MQIACF_CUR_Q_FILE_SIZE: crate::MQLONG = 1437;
pub const MQIACF_CUR_MAX_FILE_SIZE: crate::MQLONG = 1438;
pub const MQIACF_BALANCING_TYPE: crate::MQLONG = 1439;
pub const MQIACF_BALANCING_OPTIONS: crate::MQLONG = 1440;
pub const MQIACF_BALANCING_TIMEOUT: crate::MQLONG = 1441;
pub const MQIACF_SYSP_SMF_STAT_TIME_SECS: crate::MQLONG = 1442;
pub const MQIACF_SYSP_SMF_ACCT_TIME_MINS: crate::MQLONG = 1443;
pub const MQIACF_SYSP_SMF_ACCT_TIME_SECS: crate::MQLONG = 1444;
pub const MQIACF_Q_MGR_STATUS_INFO_TYPE: crate::MQLONG = 1445;
pub const MQIACF_Q_MGR_STATUS_INFO_Q_MGR: crate::MQLONG = 1446;
pub const MQIACF_Q_MGR_STATUS_INFO_NHA: crate::MQLONG = 1447;
pub const MQIACF_AUTO_CLUSTER_TYPE: crate::MQLONG = 1448;
pub const MQIACF_DATA_FS_IN_USE: crate::MQLONG = 1449;
pub const MQIACF_DATA_FS_SIZE: crate::MQLONG = 1450;
pub const MQIACF_LOG_EXTENT_SIZE: crate::MQLONG = 1451;
pub const MQIACF_LOG_FS_IN_USE: crate::MQLONG = 1452;
pub const MQIACF_LOG_FS_SIZE: crate::MQLONG = 1453;
pub const MQIACF_LOG_PRIMARIES: crate::MQLONG = 1454;
pub const MQIACF_LOG_SECONDARIES: crate::MQLONG = 1455;
pub const MQIACF_LOG_TYPE: crate::MQLONG = 1456;
pub const MQIACF_NHA_INSTANCE_ACTV_CONNS: crate::MQLONG = 1457;
pub const MQIACF_NHA_INSTANCE_BACKLOG: crate::MQLONG = 1458;
pub const MQIACF_NHA_INSTANCE_IN_SYNC: crate::MQLONG = 1459;
pub const MQIACF_NHA_INSTANCE_ROLE: crate::MQLONG = 1460;
pub const MQIACF_NHA_IN_SYNC_INSTANCES: crate::MQLONG = 1461;
pub const MQIACF_NHA_TOTAL_INSTANCES: crate::MQLONG = 1462;
pub const MQIACF_Q_MGR_FS_ENCRYPTED: crate::MQLONG = 1463;
pub const MQIACF_Q_MGR_FS_IN_USE: crate::MQLONG = 1464;
pub const MQIACF_Q_MGR_FS_SIZE: crate::MQLONG = 1465;
pub const MQIACF_SYSP_ZHYPERLINK: crate::MQLONG = 1466;
pub const MQIACF_CHECKPOINT_COUNT: crate::MQLONG = 1468;
pub const MQIACF_CHECKPOINT_OPERATIONS: crate::MQLONG = 1469;
pub const MQIACF_CHECKPOINT_SIZE: crate::MQLONG = 1470;
pub const MQIACF_NHA_GROUP_BACKLOG: crate::MQLONG = 1471;
pub const MQIACF_NHA_GROUP_CONNECTED: crate::MQLONG = 1472;
pub const MQIACF_NHA_GROUP_IN_SYNC: crate::MQLONG = 1473;
pub const MQIACF_NHA_GROUP_ROLE: crate::MQLONG = 1474;
pub const MQIACF_NHA_GROUP_STATUS: crate::MQLONG = 1475;
pub const MQIACF_NHA_INSTANCE_STATUS: crate::MQLONG = 1476;
pub const MQIACF_NHA_TYPE: crate::MQLONG = 1477;
pub const MQIACF_EVENT_DUPLICATE_COUNT: crate::MQLONG = 1478;
pub const MQIACF_LAST_USED: crate::MQLONG = 1478;
pub const MQCFACCESS_ENABLED: crate::MQLONG = 0;
pub const MQCFACCESS_SUSPENDED: crate::MQLONG = 1;
pub const MQCFACCESS_DISABLED: crate::MQLONG = 2;
pub const MQS_OPENMODE_NONE: crate::MQLONG = 0;
pub const MQS_OPENMODE_READONLY: crate::MQLONG = 1;
pub const MQS_OPENMODE_UPDATE: crate::MQLONG = 2;
pub const MQS_OPENMODE_RECOVERY: crate::MQLONG = 3;
pub const MQS_STATUS_CLOSED: crate::MQLONG = 0;
pub const MQS_STATUS_CLOSING: crate::MQLONG = 1;
pub const MQS_STATUS_OPENING: crate::MQLONG = 2;
pub const MQS_STATUS_OPEN: crate::MQLONG = 3;
pub const MQS_STATUS_NOTENABLED: crate::MQLONG = 4;
pub const MQS_STATUS_ALLOCFAIL: crate::MQLONG = 5;
pub const MQS_STATUS_OPENFAIL: crate::MQLONG = 6;
pub const MQS_STATUS_STGFAIL: crate::MQLONG = 7;
pub const MQS_STATUS_DATAFAIL: crate::MQLONG = 8;
pub const MQS_AVAIL_NORMAL: crate::MQLONG = 0;
pub const MQS_AVAIL_ERROR: crate::MQLONG = 1;
pub const MQS_AVAIL_STOPPED: crate::MQLONG = 2;
pub const MQBPLOCATION_BELOW: crate::MQLONG = 0;
pub const MQBPLOCATION_ABOVE: crate::MQLONG = 1;
pub const MQBPLOCATION_SWITCHING_ABOVE: crate::MQLONG = 2;
pub const MQBPLOCATION_SWITCHING_BELOW: crate::MQLONG = 3;
pub const MQPAGECLAS_4KB: crate::MQLONG = 0;
pub const MQPAGECLAS_FIXED4KB: crate::MQLONG = 1;
pub const MQS_EXPANDST_NORMAL: crate::MQLONG = 0;
pub const MQS_EXPANDST_FAILED: crate::MQLONG = 1;
pub const MQS_EXPANDST_MAXIMUM: crate::MQLONG = 2;
pub const MQUSAGE_SMDS_AVAILABLE: crate::MQLONG = 0;
pub const MQUSAGE_SMDS_NO_DATA: crate::MQLONG = 1;
pub const MQIACH_FIRST: crate::MQLONG = 1501;
pub const MQIACH_XMIT_PROTOCOL_TYPE: crate::MQLONG = 1501;
pub const MQIACH_BATCH_SIZE: crate::MQLONG = 1502;
pub const MQIACH_DISC_INTERVAL: crate::MQLONG = 1503;
pub const MQIACH_SHORT_TIMER: crate::MQLONG = 1504;
pub const MQIACH_SHORT_RETRY: crate::MQLONG = 1505;
pub const MQIACH_LONG_TIMER: crate::MQLONG = 1506;
pub const MQIACH_LONG_RETRY: crate::MQLONG = 1507;
pub const MQIACH_PUT_AUTHORITY: crate::MQLONG = 1508;
pub const MQIACH_SEQUENCE_NUMBER_WRAP: crate::MQLONG = 1509;
pub const MQIACH_MAX_MSG_LENGTH: crate::MQLONG = 1510;
pub const MQIACH_CHANNEL_TYPE: crate::MQLONG = 1511;
pub const MQIACH_DATA_COUNT: crate::MQLONG = 1512;
pub const MQIACH_NAME_COUNT: crate::MQLONG = 1513;
pub const MQIACH_MSG_SEQUENCE_NUMBER: crate::MQLONG = 1514;
pub const MQIACH_DATA_CONVERSION: crate::MQLONG = 1515;
pub const MQIACH_IN_DOUBT: crate::MQLONG = 1516;
pub const MQIACH_MCA_TYPE: crate::MQLONG = 1517;
pub const MQIACH_SESSION_COUNT: crate::MQLONG = 1518;
pub const MQIACH_ADAPTER: crate::MQLONG = 1519;
pub const MQIACH_COMMAND_COUNT: crate::MQLONG = 1520;
pub const MQIACH_SOCKET: crate::MQLONG = 1521;
pub const MQIACH_PORT: crate::MQLONG = 1522;
pub const MQIACH_CHANNEL_INSTANCE_TYPE: crate::MQLONG = 1523;
pub const MQIACH_CHANNEL_INSTANCE_ATTRS: crate::MQLONG = 1524;
pub const MQIACH_CHANNEL_ERROR_DATA: crate::MQLONG = 1525;
pub const MQIACH_CHANNEL_TABLE: crate::MQLONG = 1526;
pub const MQIACH_CHANNEL_STATUS: crate::MQLONG = 1527;
pub const MQIACH_INDOUBT_STATUS: crate::MQLONG = 1528;
pub const MQIACH_LAST_SEQ_NUMBER: crate::MQLONG = 1529;
pub const MQIACH_LAST_SEQUENCE_NUMBER: crate::MQLONG = 1529;
pub const MQIACH_CURRENT_MSGS: crate::MQLONG = 1531;
pub const MQIACH_CURRENT_SEQ_NUMBER: crate::MQLONG = 1532;
pub const MQIACH_CURRENT_SEQUENCE_NUMBER: crate::MQLONG = 1532;
pub const MQIACH_SSL_RETURN_CODE: crate::MQLONG = 1533;
pub const MQIACH_MSGS: crate::MQLONG = 1534;
pub const MQIACH_BYTES_SENT: crate::MQLONG = 1535;
pub const MQIACH_BYTES_RCVD: crate::MQLONG = 1536;
pub const MQIACH_BYTES_RECEIVED: crate::MQLONG = 1536;
pub const MQIACH_BATCHES: crate::MQLONG = 1537;
pub const MQIACH_BUFFERS_SENT: crate::MQLONG = 1538;
pub const MQIACH_BUFFERS_RCVD: crate::MQLONG = 1539;
pub const MQIACH_BUFFERS_RECEIVED: crate::MQLONG = 1539;
pub const MQIACH_LONG_RETRIES_LEFT: crate::MQLONG = 1540;
pub const MQIACH_SHORT_RETRIES_LEFT: crate::MQLONG = 1541;
pub const MQIACH_MCA_STATUS: crate::MQLONG = 1542;
pub const MQIACH_STOP_REQUESTED: crate::MQLONG = 1543;
pub const MQIACH_MR_COUNT: crate::MQLONG = 1544;
pub const MQIACH_MR_INTERVAL: crate::MQLONG = 1545;
pub const MQIACH_NPM_SPEED: crate::MQLONG = 1562;
pub const MQIACH_HB_INTERVAL: crate::MQLONG = 1563;
pub const MQIACH_BATCH_INTERVAL: crate::MQLONG = 1564;
pub const MQIACH_NETWORK_PRIORITY: crate::MQLONG = 1565;
pub const MQIACH_KEEP_ALIVE_INTERVAL: crate::MQLONG = 1566;
pub const MQIACH_BATCH_HB: crate::MQLONG = 1567;
pub const MQIACH_SSL_CLIENT_AUTH: crate::MQLONG = 1568;
pub const MQIACH_ALLOC_RETRY: crate::MQLONG = 1570;
pub const MQIACH_ALLOC_FAST_TIMER: crate::MQLONG = 1571;
pub const MQIACH_ALLOC_SLOW_TIMER: crate::MQLONG = 1572;
pub const MQIACH_DISC_RETRY: crate::MQLONG = 1573;
pub const MQIACH_PORT_NUMBER: crate::MQLONG = 1574;
pub const MQIACH_HDR_COMPRESSION: crate::MQLONG = 1575;
pub const MQIACH_MSG_COMPRESSION: crate::MQLONG = 1576;
pub const MQIACH_CLWL_CHANNEL_RANK: crate::MQLONG = 1577;
pub const MQIACH_CLWL_CHANNEL_PRIORITY: crate::MQLONG = 1578;
pub const MQIACH_CLWL_CHANNEL_WEIGHT: crate::MQLONG = 1579;
pub const MQIACH_CHANNEL_DISP: crate::MQLONG = 1580;
pub const MQIACH_INBOUND_DISP: crate::MQLONG = 1581;
pub const MQIACH_CHANNEL_TYPES: crate::MQLONG = 1582;
pub const MQIACH_ADAPS_STARTED: crate::MQLONG = 1583;
pub const MQIACH_ADAPS_MAX: crate::MQLONG = 1584;
pub const MQIACH_DISPS_STARTED: crate::MQLONG = 1585;
pub const MQIACH_DISPS_MAX: crate::MQLONG = 1586;
pub const MQIACH_SSLTASKS_STARTED: crate::MQLONG = 1587;
pub const MQIACH_SSLTASKS_MAX: crate::MQLONG = 1588;
pub const MQIACH_CURRENT_CHL: crate::MQLONG = 1589;
pub const MQIACH_CURRENT_CHL_MAX: crate::MQLONG = 1590;
pub const MQIACH_CURRENT_CHL_TCP: crate::MQLONG = 1591;
pub const MQIACH_CURRENT_CHL_LU62: crate::MQLONG = 1592;
pub const MQIACH_ACTIVE_CHL: crate::MQLONG = 1593;
pub const MQIACH_ACTIVE_CHL_MAX: crate::MQLONG = 1594;
pub const MQIACH_ACTIVE_CHL_PAUSED: crate::MQLONG = 1595;
pub const MQIACH_ACTIVE_CHL_STARTED: crate::MQLONG = 1596;
pub const MQIACH_ACTIVE_CHL_STOPPED: crate::MQLONG = 1597;
pub const MQIACH_ACTIVE_CHL_RETRY: crate::MQLONG = 1598;
pub const MQIACH_LISTENER_STATUS: crate::MQLONG = 1599;
pub const MQIACH_SHARED_CHL_RESTART: crate::MQLONG = 1600;
pub const MQIACH_LISTENER_CONTROL: crate::MQLONG = 1601;
pub const MQIACH_BACKLOG: crate::MQLONG = 1602;
pub const MQIACH_XMITQ_TIME_INDICATOR: crate::MQLONG = 1604;
pub const MQIACH_NETWORK_TIME_INDICATOR: crate::MQLONG = 1605;
pub const MQIACH_EXIT_TIME_INDICATOR: crate::MQLONG = 1606;
pub const MQIACH_BATCH_SIZE_INDICATOR: crate::MQLONG = 1607;
pub const MQIACH_XMITQ_MSGS_AVAILABLE: crate::MQLONG = 1608;
pub const MQIACH_CHANNEL_SUBSTATE: crate::MQLONG = 1609;
pub const MQIACH_SSL_KEY_RESETS: crate::MQLONG = 1610;
pub const MQIACH_COMPRESSION_RATE: crate::MQLONG = 1611;
pub const MQIACH_COMPRESSION_TIME: crate::MQLONG = 1612;
pub const MQIACH_MAX_XMIT_SIZE: crate::MQLONG = 1613;
pub const MQIACH_DEF_CHANNEL_DISP: crate::MQLONG = 1614;
pub const MQIACH_SHARING_CONVERSATIONS: crate::MQLONG = 1615;
pub const MQIACH_MAX_SHARING_CONVS: crate::MQLONG = 1616;
pub const MQIACH_CURRENT_SHARING_CONVS: crate::MQLONG = 1617;
pub const MQIACH_MAX_INSTANCES: crate::MQLONG = 1618;
pub const MQIACH_MAX_INSTS_PER_CLIENT: crate::MQLONG = 1619;
pub const MQIACH_CLIENT_CHANNEL_WEIGHT: crate::MQLONG = 1620;
pub const MQIACH_CONNECTION_AFFINITY: crate::MQLONG = 1621;
pub const MQIACH_AUTH_INFO_TYPES: crate::MQLONG = 1622;
pub const MQIACH_RESET_REQUESTED: crate::MQLONG = 1623;
pub const MQIACH_BATCH_DATA_LIMIT: crate::MQLONG = 1624;
pub const MQIACH_MSG_HISTORY: crate::MQLONG = 1625;
pub const MQIACH_MULTICAST_PROPERTIES: crate::MQLONG = 1626;
pub const MQIACH_NEW_SUBSCRIBER_HISTORY: crate::MQLONG = 1627;
pub const MQIACH_MC_HB_INTERVAL: crate::MQLONG = 1628;
pub const MQIACH_USE_CLIENT_ID: crate::MQLONG = 1629;
pub const MQIACH_MQTT_KEEP_ALIVE: crate::MQLONG = 1630;
pub const MQIACH_IN_DOUBT_IN: crate::MQLONG = 1631;
pub const MQIACH_IN_DOUBT_OUT: crate::MQLONG = 1632;
pub const MQIACH_MSGS_SENT: crate::MQLONG = 1633;
pub const MQIACH_MSGS_RECEIVED: crate::MQLONG = 1634;
pub const MQIACH_MSGS_RCVD: crate::MQLONG = 1634;
pub const MQIACH_PENDING_OUT: crate::MQLONG = 1635;
pub const MQIACH_AVAILABLE_CIPHERSPECS: crate::MQLONG = 1636;
pub const MQIACH_MATCH: crate::MQLONG = 1637;
pub const MQIACH_USER_SOURCE: crate::MQLONG = 1638;
pub const MQIACH_WARNING: crate::MQLONG = 1639;
pub const MQIACH_DEF_RECONNECT: crate::MQLONG = 1640;
pub const MQIACH_CHANNEL_SUMMARY_ATTRS: crate::MQLONG = 1642;
pub const MQIACH_PROTOCOL: crate::MQLONG = 1643;
pub const MQIACH_AMQP_KEEP_ALIVE: crate::MQLONG = 1644;
pub const MQIACH_SECURITY_PROTOCOL: crate::MQLONG = 1645;
pub const MQIACH_SPL_PROTECTION: crate::MQLONG = 1646;
pub const MQIACH_LAST_USED: crate::MQLONG = 1646;
pub const MQCAMO_FIRST: crate::MQLONG = 2701;
pub const MQCAMO_CLOSE_DATE: crate::MQLONG = 2701;
pub const MQCAMO_CLOSE_TIME: crate::MQLONG = 2702;
pub const MQCAMO_CONN_DATE: crate::MQLONG = 2703;
pub const MQCAMO_CONN_TIME: crate::MQLONG = 2704;
pub const MQCAMO_DISC_DATE: crate::MQLONG = 2705;
pub const MQCAMO_DISC_TIME: crate::MQLONG = 2706;
pub const MQCAMO_END_DATE: crate::MQLONG = 2707;
pub const MQCAMO_END_TIME: crate::MQLONG = 2708;
pub const MQCAMO_OPEN_DATE: crate::MQLONG = 2709;
pub const MQCAMO_OPEN_TIME: crate::MQLONG = 2710;
pub const MQCAMO_START_DATE: crate::MQLONG = 2711;
pub const MQCAMO_START_TIME: crate::MQLONG = 2712;
pub const MQCAMO_MONITOR_CLASS: crate::MQLONG = 2713;
pub const MQCAMO_MONITOR_TYPE: crate::MQLONG = 2714;
pub const MQCAMO_MONITOR_DESC: crate::MQLONG = 2715;
pub const MQCAMO_LAST_USED: crate::MQLONG = 2715;
pub const MQCACF_FIRST: crate::MQLONG = 3001;
pub const MQCACF_FROM_Q_NAME: crate::MQLONG = 3001;
pub const MQCACF_TO_Q_NAME: crate::MQLONG = 3002;
pub const MQCACF_FROM_PROCESS_NAME: crate::MQLONG = 3003;
pub const MQCACF_TO_PROCESS_NAME: crate::MQLONG = 3004;
pub const MQCACF_FROM_NAMELIST_NAME: crate::MQLONG = 3005;
pub const MQCACF_TO_NAMELIST_NAME: crate::MQLONG = 3006;
pub const MQCACF_FROM_CHANNEL_NAME: crate::MQLONG = 3007;
pub const MQCACF_TO_CHANNEL_NAME: crate::MQLONG = 3008;
pub const MQCACF_FROM_AUTH_INFO_NAME: crate::MQLONG = 3009;
pub const MQCACF_TO_AUTH_INFO_NAME: crate::MQLONG = 3010;
pub const MQCACF_Q_NAMES: crate::MQLONG = 3011;
pub const MQCACF_PROCESS_NAMES: crate::MQLONG = 3012;
pub const MQCACF_NAMELIST_NAMES: crate::MQLONG = 3013;
pub const MQCACF_ESCAPE_TEXT: crate::MQLONG = 3014;
pub const MQCACF_LOCAL_Q_NAMES: crate::MQLONG = 3015;
pub const MQCACF_MODEL_Q_NAMES: crate::MQLONG = 3016;
pub const MQCACF_ALIAS_Q_NAMES: crate::MQLONG = 3017;
pub const MQCACF_REMOTE_Q_NAMES: crate::MQLONG = 3018;
pub const MQCACF_SENDER_CHANNEL_NAMES: crate::MQLONG = 3019;
pub const MQCACF_SERVER_CHANNEL_NAMES: crate::MQLONG = 3020;
pub const MQCACF_REQUESTER_CHANNEL_NAMES: crate::MQLONG = 3021;
pub const MQCACF_RECEIVER_CHANNEL_NAMES: crate::MQLONG = 3022;
pub const MQCACF_OBJECT_Q_MGR_NAME: crate::MQLONG = 3023;
pub const MQCACF_APPL_NAME: crate::MQLONG = 3024;
pub const MQCACF_USER_IDENTIFIER: crate::MQLONG = 3025;
pub const MQCACF_AUX_ERROR_DATA_STR_1: crate::MQLONG = 3026;
pub const MQCACF_AUX_ERROR_DATA_STR_2: crate::MQLONG = 3027;
pub const MQCACF_AUX_ERROR_DATA_STR_3: crate::MQLONG = 3028;
pub const MQCACF_BRIDGE_NAME: crate::MQLONG = 3029;
pub const MQCACF_STREAM_NAME: crate::MQLONG = 3030;
pub const MQCACF_TOPIC: crate::MQLONG = 3031;
pub const MQCACF_PARENT_Q_MGR_NAME: crate::MQLONG = 3032;
pub const MQCACF_CORREL_ID: crate::MQLONG = 3033;
pub const MQCACF_PUBLISH_TIMESTAMP: crate::MQLONG = 3034;
pub const MQCACF_STRING_DATA: crate::MQLONG = 3035;
pub const MQCACF_SUPPORTED_STREAM_NAME: crate::MQLONG = 3036;
pub const MQCACF_REG_TOPIC: crate::MQLONG = 3037;
pub const MQCACF_REG_TIME: crate::MQLONG = 3038;
pub const MQCACF_REG_USER_ID: crate::MQLONG = 3039;
pub const MQCACF_CHILD_Q_MGR_NAME: crate::MQLONG = 3040;
pub const MQCACF_REG_STREAM_NAME: crate::MQLONG = 3041;
pub const MQCACF_REG_Q_MGR_NAME: crate::MQLONG = 3042;
pub const MQCACF_REG_Q_NAME: crate::MQLONG = 3043;
pub const MQCACF_REG_CORREL_ID: crate::MQLONG = 3044;
pub const MQCACF_EVENT_USER_ID: crate::MQLONG = 3045;
pub const MQCACF_OBJECT_NAME: crate::MQLONG = 3046;
pub const MQCACF_EVENT_Q_MGR: crate::MQLONG = 3047;
pub const MQCACF_AUTH_INFO_NAMES: crate::MQLONG = 3048;
pub const MQCACF_EVENT_APPL_IDENTITY: crate::MQLONG = 3049;
pub const MQCACF_EVENT_APPL_NAME: crate::MQLONG = 3050;
pub const MQCACF_EVENT_APPL_ORIGIN: crate::MQLONG = 3051;
pub const MQCACF_SUBSCRIPTION_NAME: crate::MQLONG = 3052;
pub const MQCACF_REG_SUB_NAME: crate::MQLONG = 3053;
pub const MQCACF_SUBSCRIPTION_IDENTITY: crate::MQLONG = 3054;
pub const MQCACF_REG_SUB_IDENTITY: crate::MQLONG = 3055;
pub const MQCACF_SUBSCRIPTION_USER_DATA: crate::MQLONG = 3056;
pub const MQCACF_REG_SUB_USER_DATA: crate::MQLONG = 3057;
pub const MQCACF_APPL_TAG: crate::MQLONG = 3058;
pub const MQCACF_DATA_SET_NAME: crate::MQLONG = 3059;
pub const MQCACF_UOW_START_DATE: crate::MQLONG = 3060;
pub const MQCACF_UOW_START_TIME: crate::MQLONG = 3061;
pub const MQCACF_UOW_LOG_START_DATE: crate::MQLONG = 3062;
pub const MQCACF_UOW_LOG_START_TIME: crate::MQLONG = 3063;
pub const MQCACF_UOW_LOG_EXTENT_NAME: crate::MQLONG = 3064;
pub const MQCACF_PRINCIPAL_ENTITY_NAMES: crate::MQLONG = 3065;
pub const MQCACF_GROUP_ENTITY_NAMES: crate::MQLONG = 3066;
pub const MQCACF_AUTH_PROFILE_NAME: crate::MQLONG = 3067;
pub const MQCACF_ENTITY_NAME: crate::MQLONG = 3068;
pub const MQCACF_SERVICE_COMPONENT: crate::MQLONG = 3069;
pub const MQCACF_RESPONSE_Q_MGR_NAME: crate::MQLONG = 3070;
pub const MQCACF_CURRENT_LOG_EXTENT_NAME: crate::MQLONG = 3071;
pub const MQCACF_RESTART_LOG_EXTENT_NAME: crate::MQLONG = 3072;
pub const MQCACF_MEDIA_LOG_EXTENT_NAME: crate::MQLONG = 3073;
pub const MQCACF_LOG_PATH: crate::MQLONG = 3074;
pub const MQCACF_COMMAND_MQSC: crate::MQLONG = 3075;
pub const MQCACF_Q_MGR_CPF: crate::MQLONG = 3076;
pub const MQCACF_USAGE_LOG_RBA: crate::MQLONG = 3078;
pub const MQCACF_USAGE_LOG_LRSN: crate::MQLONG = 3079;
pub const MQCACF_COMMAND_SCOPE: crate::MQLONG = 3080;
pub const MQCACF_ASID: crate::MQLONG = 3081;
pub const MQCACF_PSB_NAME: crate::MQLONG = 3082;
pub const MQCACF_PST_ID: crate::MQLONG = 3083;
pub const MQCACF_TASK_NUMBER: crate::MQLONG = 3084;
pub const MQCACF_TRANSACTION_ID: crate::MQLONG = 3085;
pub const MQCACF_Q_MGR_UOW_ID: crate::MQLONG = 3086;
pub const MQCACF_ORIGIN_NAME: crate::MQLONG = 3088;
pub const MQCACF_ENV_INFO: crate::MQLONG = 3089;
pub const MQCACF_SECURITY_PROFILE: crate::MQLONG = 3090;
pub const MQCACF_CONFIGURATION_DATE: crate::MQLONG = 3091;
pub const MQCACF_CONFIGURATION_TIME: crate::MQLONG = 3092;
pub const MQCACF_FROM_CF_STRUC_NAME: crate::MQLONG = 3093;
pub const MQCACF_TO_CF_STRUC_NAME: crate::MQLONG = 3094;
pub const MQCACF_CF_STRUC_NAMES: crate::MQLONG = 3095;
pub const MQCACF_FAIL_DATE: crate::MQLONG = 3096;
pub const MQCACF_FAIL_TIME: crate::MQLONG = 3097;
pub const MQCACF_BACKUP_DATE: crate::MQLONG = 3098;
pub const MQCACF_BACKUP_TIME: crate::MQLONG = 3099;
pub const MQCACF_SYSTEM_NAME: crate::MQLONG = 3100;
pub const MQCACF_CF_STRUC_BACKUP_START: crate::MQLONG = 3101;
pub const MQCACF_CF_STRUC_BACKUP_END: crate::MQLONG = 3102;
pub const MQCACF_CF_STRUC_LOG_Q_MGRS: crate::MQLONG = 3103;
pub const MQCACF_FROM_STORAGE_CLASS: crate::MQLONG = 3104;
pub const MQCACF_TO_STORAGE_CLASS: crate::MQLONG = 3105;
pub const MQCACF_STORAGE_CLASS_NAMES: crate::MQLONG = 3106;
pub const MQCACF_DSG_NAME: crate::MQLONG = 3108;
pub const MQCACF_DB2_NAME: crate::MQLONG = 3109;
pub const MQCACF_SYSP_CMD_USER_ID: crate::MQLONG = 3110;
pub const MQCACF_SYSP_OTMA_GROUP: crate::MQLONG = 3111;
pub const MQCACF_SYSP_OTMA_MEMBER: crate::MQLONG = 3112;
pub const MQCACF_SYSP_OTMA_DRU_EXIT: crate::MQLONG = 3113;
pub const MQCACF_SYSP_OTMA_TPIPE_PFX: crate::MQLONG = 3114;
pub const MQCACF_SYSP_ARCHIVE_PFX1: crate::MQLONG = 3115;
pub const MQCACF_SYSP_ARCHIVE_UNIT1: crate::MQLONG = 3116;
pub const MQCACF_SYSP_LOG_CORREL_ID: crate::MQLONG = 3117;
pub const MQCACF_SYSP_UNIT_VOLSER: crate::MQLONG = 3118;
pub const MQCACF_SYSP_Q_MGR_TIME: crate::MQLONG = 3119;
pub const MQCACF_SYSP_Q_MGR_DATE: crate::MQLONG = 3120;
pub const MQCACF_SYSP_Q_MGR_RBA: crate::MQLONG = 3121;
pub const MQCACF_SYSP_LOG_RBA: crate::MQLONG = 3122;
pub const MQCACF_SYSP_SERVICE: crate::MQLONG = 3123;
pub const MQCACF_FROM_LISTENER_NAME: crate::MQLONG = 3124;
pub const MQCACF_TO_LISTENER_NAME: crate::MQLONG = 3125;
pub const MQCACF_FROM_SERVICE_NAME: crate::MQLONG = 3126;
pub const MQCACF_TO_SERVICE_NAME: crate::MQLONG = 3127;
pub const MQCACF_LAST_PUT_DATE: crate::MQLONG = 3128;
pub const MQCACF_LAST_PUT_TIME: crate::MQLONG = 3129;
pub const MQCACF_LAST_GET_DATE: crate::MQLONG = 3130;
pub const MQCACF_LAST_GET_TIME: crate::MQLONG = 3131;
pub const MQCACF_OPERATION_DATE: crate::MQLONG = 3132;
pub const MQCACF_OPERATION_TIME: crate::MQLONG = 3133;
pub const MQCACF_ACTIVITY_DESC: crate::MQLONG = 3134;
pub const MQCACF_APPL_IDENTITY_DATA: crate::MQLONG = 3135;
pub const MQCACF_APPL_ORIGIN_DATA: crate::MQLONG = 3136;
pub const MQCACF_PUT_DATE: crate::MQLONG = 3137;
pub const MQCACF_PUT_TIME: crate::MQLONG = 3138;
pub const MQCACF_REPLY_TO_Q: crate::MQLONG = 3139;
pub const MQCACF_REPLY_TO_Q_MGR: crate::MQLONG = 3140;
pub const MQCACF_RESOLVED_Q_NAME: crate::MQLONG = 3141;
pub const MQCACF_STRUC_ID: crate::MQLONG = 3142;
pub const MQCACF_VALUE_NAME: crate::MQLONG = 3143;
pub const MQCACF_SERVICE_START_DATE: crate::MQLONG = 3144;
pub const MQCACF_SERVICE_START_TIME: crate::MQLONG = 3145;
pub const MQCACF_SYSP_OFFLINE_RBA: crate::MQLONG = 3146;
pub const MQCACF_SYSP_ARCHIVE_PFX2: crate::MQLONG = 3147;
pub const MQCACF_SYSP_ARCHIVE_UNIT2: crate::MQLONG = 3148;
pub const MQCACF_TO_TOPIC_NAME: crate::MQLONG = 3149;
pub const MQCACF_FROM_TOPIC_NAME: crate::MQLONG = 3150;
pub const MQCACF_TOPIC_NAMES: crate::MQLONG = 3151;
pub const MQCACF_SUB_NAME: crate::MQLONG = 3152;
pub const MQCACF_DESTINATION_Q_MGR: crate::MQLONG = 3153;
pub const MQCACF_DESTINATION: crate::MQLONG = 3154;
pub const MQCACF_SUB_USER_ID: crate::MQLONG = 3156;
pub const MQCACF_SUB_USER_DATA: crate::MQLONG = 3159;
pub const MQCACF_SUB_SELECTOR: crate::MQLONG = 3160;
pub const MQCACF_LAST_PUB_DATE: crate::MQLONG = 3161;
pub const MQCACF_LAST_PUB_TIME: crate::MQLONG = 3162;
pub const MQCACF_FROM_SUB_NAME: crate::MQLONG = 3163;
pub const MQCACF_TO_SUB_NAME: crate::MQLONG = 3164;
pub const MQCACF_LAST_MSG_TIME: crate::MQLONG = 3167;
pub const MQCACF_LAST_MSG_DATE: crate::MQLONG = 3168;
pub const MQCACF_SUBSCRIPTION_POINT: crate::MQLONG = 3169;
pub const MQCACF_FILTER: crate::MQLONG = 3170;
pub const MQCACF_NONE: crate::MQLONG = 3171;
pub const MQCACF_ADMIN_TOPIC_NAMES: crate::MQLONG = 3172;
pub const MQCACF_ROUTING_FINGER_PRINT: crate::MQLONG = 3173;
pub const MQCACF_APPL_DESC: crate::MQLONG = 3174;
pub const MQCACF_Q_MGR_START_DATE: crate::MQLONG = 3175;
pub const MQCACF_Q_MGR_START_TIME: crate::MQLONG = 3176;
pub const MQCACF_FROM_COMM_INFO_NAME: crate::MQLONG = 3177;
pub const MQCACF_TO_COMM_INFO_NAME: crate::MQLONG = 3178;
pub const MQCACF_CF_OFFLOAD_SIZE1: crate::MQLONG = 3179;
pub const MQCACF_CF_OFFLOAD_SIZE2: crate::MQLONG = 3180;
pub const MQCACF_CF_OFFLOAD_SIZE3: crate::MQLONG = 3181;
pub const MQCACF_CF_SMDS_GENERIC_NAME: crate::MQLONG = 3182;
pub const MQCACF_CF_SMDS: crate::MQLONG = 3183;
pub const MQCACF_RECOVERY_DATE: crate::MQLONG = 3184;
pub const MQCACF_RECOVERY_TIME: crate::MQLONG = 3185;
pub const MQCACF_CF_SMDSCONN: crate::MQLONG = 3186;
pub const MQCACF_CF_STRUC_NAME: crate::MQLONG = 3187;
pub const MQCACF_ALTERNATE_USERID: crate::MQLONG = 3188;
pub const MQCACF_CHAR_ATTRS: crate::MQLONG = 3189;
pub const MQCACF_DYNAMIC_Q_NAME: crate::MQLONG = 3190;
pub const MQCACF_HOST_NAME: crate::MQLONG = 3191;
pub const MQCACF_MQCB_NAME: crate::MQLONG = 3192;
pub const MQCACF_OBJECT_STRING: crate::MQLONG = 3193;
pub const MQCACF_RESOLVED_LOCAL_Q_MGR: crate::MQLONG = 3194;
pub const MQCACF_RESOLVED_LOCAL_Q_NAME: crate::MQLONG = 3195;
pub const MQCACF_RESOLVED_OBJECT_STRING: crate::MQLONG = 3196;
pub const MQCACF_RESOLVED_Q_MGR: crate::MQLONG = 3197;
pub const MQCACF_SELECTION_STRING: crate::MQLONG = 3198;
pub const MQCACF_XA_INFO: crate::MQLONG = 3199;
pub const MQCACF_APPL_FUNCTION: crate::MQLONG = 3200;
pub const MQCACF_XQH_REMOTE_Q_NAME: crate::MQLONG = 3201;
pub const MQCACF_XQH_REMOTE_Q_MGR: crate::MQLONG = 3202;
pub const MQCACF_XQH_PUT_TIME: crate::MQLONG = 3203;
pub const MQCACF_XQH_PUT_DATE: crate::MQLONG = 3204;
pub const MQCACF_EXCL_OPERATOR_MESSAGES: crate::MQLONG = 3205;
pub const MQCACF_CSP_USER_IDENTIFIER: crate::MQLONG = 3206;
pub const MQCACF_AMQP_CLIENT_ID: crate::MQLONG = 3207;
pub const MQCACF_ARCHIVE_LOG_EXTENT_NAME: crate::MQLONG = 3208;
pub const MQCACF_APPL_IMMOVABLE_DATE: crate::MQLONG = 3209;
pub const MQCACF_APPL_IMMOVABLE_TIME: crate::MQLONG = 3210;
pub const MQCACF_NHA_INSTANCE_NAME: crate::MQLONG = 3211;
pub const MQCACF_Q_MGR_DATA_PATH: crate::MQLONG = 3212;
pub const MQCACF_UNIFORM_CLUSTER_NAME: crate::MQLONG = 3213;
pub const MQCACF_LOG_START_DATE: crate::MQLONG = 3214;
pub const MQCACF_LOG_START_LSN: crate::MQLONG = 3215;
pub const MQCACF_LOG_START_TIME: crate::MQLONG = 3216;
pub const MQCACF_NHA_GROUP_INITIAL_DATE: crate::MQLONG = 3217;
pub const MQCACF_NHA_GROUP_INITIAL_LSN: crate::MQLONG = 3218;
pub const MQCACF_NHA_GROUP_INITIAL_TIME: crate::MQLONG = 3219;
pub const MQCACF_NHA_REPL_ADDRESS: crate::MQLONG = 3220;
pub const MQCACF_DISK_WRITTEN_LSN: crate::MQLONG = 3221;
pub const MQCACF_NHA_ACKNOWLEDGED_LSN: crate::MQLONG = 3222;
pub const MQCACF_NHA_GROUP_ADDRESS: crate::MQLONG = 3223;
pub const MQCACF_NHA_GROUP_SYNC_ISOTIME: crate::MQLONG = 3224;
pub const MQCACF_NHA_GROUP_INIT_ISOTIME: crate::MQLONG = 3225;
pub const MQCACF_NHA_GROUP_LIVE_ISOTIME: crate::MQLONG = 3226;
pub const MQCACF_NHA_GROUP_LSN: crate::MQLONG = 3227;
pub const MQCACF_NHA_GROUP_NAME: crate::MQLONG = 3228;
pub const MQCACF_NHA_GROUP_RECOV_LSN: crate::MQLONG = 3229;
pub const MQCACF_NHA_GROUP_RECOV_ISOTIME: crate::MQLONG = 3230;
pub const MQCACF_NHA_SYNC_ISOTIME: crate::MQLONG = 3231;
pub const MQCACF_EVENT_DUPLICATE_FROM: crate::MQLONG = 3232;
pub const MQCACF_LAST_USED: crate::MQLONG = 3232;
pub const MQCACH_FIRST: crate::MQLONG = 3501;
pub const MQCACH_CHANNEL_NAME: crate::MQLONG = 3501;
pub const MQCACH_DESC: crate::MQLONG = 3502;
pub const MQCACH_MODE_NAME: crate::MQLONG = 3503;
pub const MQCACH_TP_NAME: crate::MQLONG = 3504;
pub const MQCACH_XMIT_Q_NAME: crate::MQLONG = 3505;
pub const MQCACH_CONNECTION_NAME: crate::MQLONG = 3506;
pub const MQCACH_MCA_NAME: crate::MQLONG = 3507;
pub const MQCACH_SEC_EXIT_NAME: crate::MQLONG = 3508;
pub const MQCACH_MSG_EXIT_NAME: crate::MQLONG = 3509;
pub const MQCACH_SEND_EXIT_NAME: crate::MQLONG = 3510;
pub const MQCACH_RCV_EXIT_NAME: crate::MQLONG = 3511;
pub const MQCACH_CHANNEL_NAMES: crate::MQLONG = 3512;
pub const MQCACH_SEC_EXIT_USER_DATA: crate::MQLONG = 3513;
pub const MQCACH_MSG_EXIT_USER_DATA: crate::MQLONG = 3514;
pub const MQCACH_SEND_EXIT_USER_DATA: crate::MQLONG = 3515;
pub const MQCACH_RCV_EXIT_USER_DATA: crate::MQLONG = 3516;
pub const MQCACH_USER_ID: crate::MQLONG = 3517;
pub const MQCACH_PASSWORD: crate::MQLONG = 3518;
pub const MQCACH_LOCAL_ADDRESS: crate::MQLONG = 3520;
pub const MQCACH_LOCAL_NAME: crate::MQLONG = 3521;
pub const MQCACH_LAST_MSG_TIME: crate::MQLONG = 3524;
pub const MQCACH_LAST_MSG_DATE: crate::MQLONG = 3525;
pub const MQCACH_MCA_USER_ID: crate::MQLONG = 3527;
pub const MQCACH_CHANNEL_START_TIME: crate::MQLONG = 3528;
pub const MQCACH_CHANNEL_START_DATE: crate::MQLONG = 3529;
pub const MQCACH_MCA_JOB_NAME: crate::MQLONG = 3530;
pub const MQCACH_LAST_LUWID: crate::MQLONG = 3531;
pub const MQCACH_CURRENT_LUWID: crate::MQLONG = 3532;
pub const MQCACH_FORMAT_NAME: crate::MQLONG = 3533;
pub const MQCACH_MR_EXIT_NAME: crate::MQLONG = 3534;
pub const MQCACH_MR_EXIT_USER_DATA: crate::MQLONG = 3535;
pub const MQCACH_SSL_CIPHER_SPEC: crate::MQLONG = 3544;
pub const MQCACH_SSL_PEER_NAME: crate::MQLONG = 3545;
pub const MQCACH_SSL_HANDSHAKE_STAGE: crate::MQLONG = 3546;
pub const MQCACH_SSL_SHORT_PEER_NAME: crate::MQLONG = 3547;
pub const MQCACH_REMOTE_APPL_TAG: crate::MQLONG = 3548;
pub const MQCACH_SSL_CERT_USER_ID: crate::MQLONG = 3549;
pub const MQCACH_SSL_CERT_ISSUER_NAME: crate::MQLONG = 3550;
pub const MQCACH_LU_NAME: crate::MQLONG = 3551;
pub const MQCACH_IP_ADDRESS: crate::MQLONG = 3552;
pub const MQCACH_TCP_NAME: crate::MQLONG = 3553;
pub const MQCACH_LISTENER_NAME: crate::MQLONG = 3554;
pub const MQCACH_LISTENER_DESC: crate::MQLONG = 3555;
pub const MQCACH_LISTENER_START_DATE: crate::MQLONG = 3556;
pub const MQCACH_LISTENER_START_TIME: crate::MQLONG = 3557;
pub const MQCACH_SSL_KEY_RESET_DATE: crate::MQLONG = 3558;
pub const MQCACH_SSL_KEY_RESET_TIME: crate::MQLONG = 3559;
pub const MQCACH_REMOTE_VERSION: crate::MQLONG = 3560;
pub const MQCACH_REMOTE_PRODUCT: crate::MQLONG = 3561;
pub const MQCACH_GROUP_ADDRESS: crate::MQLONG = 3562;
pub const MQCACH_JAAS_CONFIG: crate::MQLONG = 3563;
pub const MQCACH_CLIENT_ID: crate::MQLONG = 3564;
pub const MQCACH_SSL_KEY_PASSPHRASE: crate::MQLONG = 3565;
pub const MQCACH_CONNECTION_NAME_LIST: crate::MQLONG = 3566;
pub const MQCACH_CLIENT_USER_ID: crate::MQLONG = 3567;
pub const MQCACH_MCA_USER_ID_LIST: crate::MQLONG = 3568;
pub const MQCACH_SSL_CIPHER_SUITE: crate::MQLONG = 3569;
pub const MQCACH_WEBCONTENT_PATH: crate::MQLONG = 3570;
pub const MQCACH_TOPIC_ROOT: crate::MQLONG = 3571;
pub const MQCACH_TEMPORARY_MODEL_Q: crate::MQLONG = 3572;
pub const MQCACH_TEMPORARY_Q_PREFIX: crate::MQLONG = 3573;
pub const MQCACH_LAST_USED: crate::MQLONG = 3573;
pub const MQGACF_FIRST: crate::MQLONG = 8001;
pub const MQGACF_COMMAND_CONTEXT: crate::MQLONG = 8001;
pub const MQGACF_COMMAND_DATA: crate::MQLONG = 8002;
pub const MQGACF_TRACE_ROUTE: crate::MQLONG = 8003;
pub const MQGACF_OPERATION: crate::MQLONG = 8004;
pub const MQGACF_ACTIVITY: crate::MQLONG = 8005;
pub const MQGACF_EMBEDDED_MQMD: crate::MQLONG = 8006;
pub const MQGACF_MESSAGE: crate::MQLONG = 8007;
pub const MQGACF_MQMD: crate::MQLONG = 8008;
pub const MQGACF_VALUE_NAMING: crate::MQLONG = 8009;
pub const MQGACF_Q_ACCOUNTING_DATA: crate::MQLONG = 8010;
pub const MQGACF_Q_STATISTICS_DATA: crate::MQLONG = 8011;
pub const MQGACF_CHL_STATISTICS_DATA: crate::MQLONG = 8012;
pub const MQGACF_ACTIVITY_TRACE: crate::MQLONG = 8013;
pub const MQGACF_APP_DIST_LIST: crate::MQLONG = 8014;
pub const MQGACF_MONITOR_CLASS: crate::MQLONG = 8015;
pub const MQGACF_MONITOR_TYPE: crate::MQLONG = 8016;
pub const MQGACF_MONITOR_ELEMENT: crate::MQLONG = 8017;
pub const MQGACF_APPL_STATUS: crate::MQLONG = 8018;
pub const MQGACF_CHANGED_APPLS: crate::MQLONG = 8019;
pub const MQGACF_ALL_APPLS: crate::MQLONG = 8020;
pub const MQGACF_APPL_BALANCE: crate::MQLONG = 8021;
pub const MQGACF_LAST_USED: crate::MQLONG = 8021;
pub const MQACT_FORCE_REMOVE: crate::MQLONG = 1;
pub const MQACT_ADVANCE_LOG: crate::MQLONG = 2;
pub const MQACT_COLLECT_STATISTICS: crate::MQLONG = 3;
pub const MQACT_PUBSUB: crate::MQLONG = 4;
pub const MQACT_ADD: crate::MQLONG = 5;
pub const MQACT_REPLACE: crate::MQLONG = 6;
pub const MQACT_REMOVE: crate::MQLONG = 7;
pub const MQACT_REMOVEALL: crate::MQLONG = 8;
pub const MQACT_FAIL: crate::MQLONG = 9;
pub const MQACT_REDUCE_LOG: crate::MQLONG = 10;
pub const MQACT_ARCHIVE_LOG: crate::MQLONG = 11;
pub const MQIS_NO: crate::MQLONG = 0;
pub const MQIS_YES: crate::MQLONG = 1;
pub const MQAPPL_IMMOVABLE: crate::MQLONG = 0;
pub const MQAPPL_MOVABLE: crate::MQLONG = 1;
pub const MQACTIVE_NO: crate::MQLONG = 0;
pub const MQACTIVE_YES: crate::MQLONG = 1;
pub const MQBALANCED_NO: crate::MQLONG = 0;
pub const MQBALANCED_YES: crate::MQLONG = 1;
pub const MQBALANCED_NOT_APPLICABLE: crate::MQLONG = 2;
pub const MQBALANCED_UNKNOWN: crate::MQLONG = 3;
pub const MQBALSTATE_NOT_APPLICABLE: crate::MQLONG = 0;
pub const MQBALSTATE_LOW: crate::MQLONG = 1;
pub const MQBALSTATE_OK: crate::MQLONG = 2;
pub const MQBALSTATE_HIGH: crate::MQLONG = 3;
pub const MQBALSTATE_UNKNOWN: crate::MQLONG = 4;
pub const MQIMMREASON_NONE: crate::MQLONG = 0;
pub const MQIMMREASON_NOT_CLIENT: crate::MQLONG = 1;
pub const MQIMMREASON_NOT_RECONNECTABLE: crate::MQLONG = 2;
pub const MQIMMREASON_MOVING: crate::MQLONG = 3;
pub const MQIMMREASON_APPLNAME_CHANGED: crate::MQLONG = 4;
pub const MQIMMREASON_IN_TRANSACTION: crate::MQLONG = 5;
pub const MQIMMREASON_AWAITS_REPLY: crate::MQLONG = 6;
pub const MQIMMREASON_NO_REDIRECT: crate::MQLONG = 7;
pub const MQAS_NONE: crate::MQLONG = 0;
pub const MQAS_STARTED: crate::MQLONG = 1;
pub const MQAS_START_WAIT: crate::MQLONG = 2;
pub const MQAS_STOPPED: crate::MQLONG = 3;
pub const MQAS_SUSPENDED: crate::MQLONG = 4;
pub const MQAS_SUSPENDED_TEMPORARY: crate::MQLONG = 5;
pub const MQAS_ACTIVE: crate::MQLONG = 6;
pub const MQAS_INACTIVE: crate::MQLONG = 7;
pub const MQAUTH_NONE: crate::MQLONG = 0;
pub const MQAUTH_ALT_USER_AUTHORITY: crate::MQLONG = 1;
pub const MQAUTH_BROWSE: crate::MQLONG = 2;
pub const MQAUTH_CHANGE: crate::MQLONG = 3;
pub const MQAUTH_CLEAR: crate::MQLONG = 4;
pub const MQAUTH_CONNECT: crate::MQLONG = 5;
pub const MQAUTH_CREATE: crate::MQLONG = 6;
pub const MQAUTH_DELETE: crate::MQLONG = 7;
pub const MQAUTH_DISPLAY: crate::MQLONG = 8;
pub const MQAUTH_INPUT: crate::MQLONG = 9;
pub const MQAUTH_INQUIRE: crate::MQLONG = 10;
pub const MQAUTH_OUTPUT: crate::MQLONG = 11;
pub const MQAUTH_PASS_ALL_CONTEXT: crate::MQLONG = 12;
pub const MQAUTH_PASS_IDENTITY_CONTEXT: crate::MQLONG = 13;
pub const MQAUTH_SET: crate::MQLONG = 14;
pub const MQAUTH_SET_ALL_CONTEXT: crate::MQLONG = 15;
pub const MQAUTH_SET_IDENTITY_CONTEXT: crate::MQLONG = 16;
pub const MQAUTH_CONTROL: crate::MQLONG = 17;
pub const MQAUTH_CONTROL_EXTENDED: crate::MQLONG = 18;
pub const MQAUTH_PUBLISH: crate::MQLONG = 19;
pub const MQAUTH_SUBSCRIBE: crate::MQLONG = 20;
pub const MQAUTH_RESUME: crate::MQLONG = 21;
pub const MQAUTH_SYSTEM: crate::MQLONG = 22;
pub const MQAUTH_ALL: crate::MQLONG = -1;
pub const MQAUTH_ALL_ADMIN: crate::MQLONG = -2;
pub const MQAUTH_ALL_MQI: crate::MQLONG = -3;
pub const MQAUTHOPT_ENTITY_EXPLICIT: crate::MQLONG = 1;
pub const MQAUTHOPT_ENTITY_SET: crate::MQLONG = 2;
pub const MQAUTHOPT_NAME_EXPLICIT: crate::MQLONG = 16;
pub const MQAUTHOPT_NAME_ALL_MATCHING: crate::MQLONG = 32;
pub const MQAUTHOPT_NAME_AS_WILDCARD: crate::MQLONG = 64;
pub const MQAUTHOPT_CUMULATIVE: crate::MQLONG = 256;
pub const MQAUTHOPT_EXCLUDE_TEMP: crate::MQLONG = 512;
pub const MQBT_OTMA: crate::MQLONG = 1;
pub const MQCFO_REFRESH_REPOSITORY_YES: crate::MQLONG = 1;
pub const MQCFO_REFRESH_REPOSITORY_NO: crate::MQLONG = 0;
pub const MQCFO_REMOVE_QUEUES_YES: crate::MQLONG = 1;
pub const MQCFO_REMOVE_QUEUES_NO: crate::MQLONG = 0;
pub const MQCAUT_ALL: crate::MQLONG = 0;
pub const MQCAUT_BLOCKUSER: crate::MQLONG = 1;
pub const MQCAUT_BLOCKADDR: crate::MQLONG = 2;
pub const MQCAUT_SSLPEERMAP: crate::MQLONG = 3;
pub const MQCAUT_ADDRESSMAP: crate::MQLONG = 4;
pub const MQCAUT_USERMAP: crate::MQLONG = 5;
pub const MQCAUT_QMGRMAP: crate::MQLONG = 6;
pub const MQCFSTATUS_NOT_FOUND: crate::MQLONG = 0;
pub const MQCFSTATUS_ACTIVE: crate::MQLONG = 1;
pub const MQCFSTATUS_IN_RECOVER: crate::MQLONG = 2;
pub const MQCFSTATUS_IN_BACKUP: crate::MQLONG = 3;
pub const MQCFSTATUS_FAILED: crate::MQLONG = 4;
pub const MQCFSTATUS_NONE: crate::MQLONG = 5;
pub const MQCFSTATUS_UNKNOWN: crate::MQLONG = 6;
pub const MQCFSTATUS_RECOVERED: crate::MQLONG = 7;
pub const MQCFSTATUS_EMPTY: crate::MQLONG = 8;
pub const MQCFSTATUS_NEW: crate::MQLONG = 9;
pub const MQCFSTATUS_ADMIN_INCOMPLETE: crate::MQLONG = 20;
pub const MQCFSTATUS_NEVER_USED: crate::MQLONG = 21;
pub const MQCFSTATUS_NO_BACKUP: crate::MQLONG = 22;
pub const MQCFSTATUS_NOT_FAILED: crate::MQLONG = 23;
pub const MQCFSTATUS_NOT_RECOVERABLE: crate::MQLONG = 24;
pub const MQCFSTATUS_XES_ERROR: crate::MQLONG = 25;
pub const MQCFTYPE_APPL: crate::MQLONG = 0;
pub const MQCFTYPE_ADMIN: crate::MQLONG = 1;
pub const MQCHIDS_NOT_INDOUBT: crate::MQLONG = 0;
pub const MQCHIDS_INDOUBT: crate::MQLONG = 1;
pub const MQCHLD_ALL: crate::MQLONG = -1;
pub const MQCHLD_DEFAULT: crate::MQLONG = 1;
pub const MQCHLD_SHARED: crate::MQLONG = 2;
pub const MQCHLD_PRIVATE: crate::MQLONG = 4;
pub const MQCHLD_FIXSHARED: crate::MQLONG = 5;
pub const MQUCI_YES: crate::MQLONG = 1;
pub const MQUCI_NO: crate::MQLONG = 0;
pub const MQCHS_INACTIVE: crate::MQLONG = 0;
pub const MQCHS_BINDING: crate::MQLONG = 1;
pub const MQCHS_STARTING: crate::MQLONG = 2;
pub const MQCHS_RUNNING: crate::MQLONG = 3;
pub const MQCHS_STOPPING: crate::MQLONG = 4;
pub const MQCHS_RETRYING: crate::MQLONG = 5;
pub const MQCHS_STOPPED: crate::MQLONG = 6;
pub const MQCHS_REQUESTING: crate::MQLONG = 7;
pub const MQCHS_PAUSED: crate::MQLONG = 8;
pub const MQCHS_DISCONNECTED: crate::MQLONG = 9;
pub const MQCHS_INITIALIZING: crate::MQLONG = 13;
pub const MQCHS_SWITCHING: crate::MQLONG = 14;
pub const MQCHSSTATE_OTHER: crate::MQLONG = 0;
pub const MQCHSSTATE_END_OF_BATCH: crate::MQLONG = 100;
pub const MQCHSSTATE_SENDING: crate::MQLONG = 200;
pub const MQCHSSTATE_RECEIVING: crate::MQLONG = 300;
pub const MQCHSSTATE_SERIALIZING: crate::MQLONG = 400;
pub const MQCHSSTATE_RESYNCHING: crate::MQLONG = 500;
pub const MQCHSSTATE_HEARTBEATING: crate::MQLONG = 600;
pub const MQCHSSTATE_IN_SCYEXIT: crate::MQLONG = 700;
pub const MQCHSSTATE_IN_RCVEXIT: crate::MQLONG = 800;
pub const MQCHSSTATE_IN_SENDEXIT: crate::MQLONG = 900;
pub const MQCHSSTATE_IN_MSGEXIT: crate::MQLONG = 1000;
pub const MQCHSSTATE_IN_MREXIT: crate::MQLONG = 1100;
pub const MQCHSSTATE_IN_CHADEXIT: crate::MQLONG = 1200;
pub const MQCHSSTATE_NET_CONNECTING: crate::MQLONG = 1250;
pub const MQCHSSTATE_SSL_HANDSHAKING: crate::MQLONG = 1300;
pub const MQCHSSTATE_NAME_SERVER: crate::MQLONG = 1400;
pub const MQCHSSTATE_IN_MQPUT: crate::MQLONG = 1500;
pub const MQCHSSTATE_IN_MQGET: crate::MQLONG = 1600;
pub const MQCHSSTATE_IN_MQI_CALL: crate::MQLONG = 1700;
pub const MQCHSSTATE_COMPRESSING: crate::MQLONG = 1800;
pub const MQCHSH_RESTART_NO: crate::MQLONG = 0;
pub const MQCHSH_RESTART_YES: crate::MQLONG = 1;
pub const MQCHSR_STOP_NOT_REQUESTED: crate::MQLONG = 0;
pub const MQCHSR_STOP_REQUESTED: crate::MQLONG = 1;
pub const MQCHRR_RESET_NOT_REQUESTED: crate::MQLONG = 0;
pub const MQCHTAB_Q_MGR: crate::MQLONG = 1;
pub const MQCHTAB_CLNTCONN: crate::MQLONG = 2;
pub const MQCLRS_LOCAL: crate::MQLONG = 1;
pub const MQCLRS_GLOBAL: crate::MQLONG = 2;
pub const MQCLRT_RETAINED: crate::MQLONG = 1;
pub const MQCMDI_CMDSCOPE_ACCEPTED: crate::MQLONG = 1;
pub const MQCMDI_CMDSCOPE_GENERATED: crate::MQLONG = 2;
pub const MQCMDI_CMDSCOPE_COMPLETED: crate::MQLONG = 3;
pub const MQCMDI_QSG_DISP_COMPLETED: crate::MQLONG = 4;
pub const MQCMDI_COMMAND_ACCEPTED: crate::MQLONG = 5;
pub const MQCMDI_CLUSTER_REQUEST_QUEUED: crate::MQLONG = 6;
pub const MQCMDI_CHANNEL_INIT_STARTED: crate::MQLONG = 7;
pub const MQCMDI_RECOVER_STARTED: crate::MQLONG = 11;
pub const MQCMDI_BACKUP_STARTED: crate::MQLONG = 12;
pub const MQCMDI_RECOVER_COMPLETED: crate::MQLONG = 13;
pub const MQCMDI_SEC_TIMER_ZERO: crate::MQLONG = 14;
pub const MQCMDI_REFRESH_CONFIGURATION: crate::MQLONG = 16;
pub const MQCMDI_SEC_SIGNOFF_ERROR: crate::MQLONG = 17;
pub const MQCMDI_IMS_BRIDGE_SUSPENDED: crate::MQLONG = 18;
pub const MQCMDI_DB2_SUSPENDED: crate::MQLONG = 19;
pub const MQCMDI_DB2_OBSOLETE_MSGS: crate::MQLONG = 20;
pub const MQCMDI_SEC_UPPERCASE: crate::MQLONG = 21;
pub const MQCMDI_SEC_MIXEDCASE: crate::MQLONG = 22;
pub const MQDISCONNECT_NORMAL: crate::MQLONG = 0;
pub const MQDISCONNECT_IMPLICIT: crate::MQLONG = 1;
pub const MQDISCONNECT_Q_MGR: crate::MQLONG = 2;
pub const MQET_MQSC: crate::MQLONG = 1;
pub const MQEVO_OTHER: crate::MQLONG = 0;
pub const MQEVO_CONSOLE: crate::MQLONG = 1;
pub const MQEVO_INIT: crate::MQLONG = 2;
pub const MQEVO_MSG: crate::MQLONG = 3;
pub const MQEVO_MQSET: crate::MQLONG = 4;
pub const MQEVO_INTERNAL: crate::MQLONG = 5;
pub const MQEVO_MQSUB: crate::MQLONG = 6;
pub const MQEVO_CTLMSG: crate::MQLONG = 7;
pub const MQEVO_REST: crate::MQLONG = 8;
pub const MQEVR_DISABLED: crate::MQLONG = 0;
pub const MQEVR_ENABLED: crate::MQLONG = 1;
pub const MQEVR_EXCEPTION: crate::MQLONG = 2;
pub const MQEVR_NO_DISPLAY: crate::MQLONG = 3;
pub const MQEVR_API_ONLY: crate::MQLONG = 4;
pub const MQEVR_ADMIN_ONLY: crate::MQLONG = 5;
pub const MQEVR_USER_ONLY: crate::MQLONG = 6;
pub const MQAUSC_FAILURES: crate::MQLONG = 0;
pub const MQAUSC_ALLCONNS: crate::MQLONG = 1;
pub const MQAUSC_ALLCHECKS: crate::MQLONG = 2;
pub const MQFC_YES: crate::MQLONG = 1;
pub const MQFC_NO: crate::MQLONG = 0;
pub const MQHSTATE_INACTIVE: crate::MQLONG = 0;
pub const MQHSTATE_ACTIVE: crate::MQLONG = 1;
pub const MQINBD_Q_MGR: crate::MQLONG = 0;
pub const MQINBD_GROUP: crate::MQLONG = 3;
pub const MQIDO_COMMIT: crate::MQLONG = 1;
pub const MQIDO_BACKOUT: crate::MQLONG = 2;
pub const MQMATCH_GENERIC: crate::MQLONG = 0;
pub const MQMATCH_RUNCHECK: crate::MQLONG = 1;
pub const MQMATCH_EXACT: crate::MQLONG = 2;
pub const MQMATCH_ALL: crate::MQLONG = 3;
pub const MQMCAS_STOPPED: crate::MQLONG = 0;
pub const MQMCAS_RUNNING: crate::MQLONG = 3;
pub const MQMODE_FORCE: crate::MQLONG = 0;
pub const MQMODE_QUIESCE: crate::MQLONG = 1;
pub const MQMODE_TERMINATE: crate::MQLONG = 2;
pub const MQMLP_TOLERATE_UNPROTECTED_NO: crate::MQLONG = 0;
pub const MQMLP_TOLERATE_UNPROTECTED_YES: crate::MQLONG = 1;
pub const MQMLP_ENCRYPTION_ALG_NONE: crate::MQLONG = 0;
pub const MQMLP_ENCRYPTION_ALG_RC2: crate::MQLONG = 1;
pub const MQMLP_ENCRYPTION_ALG_DES: crate::MQLONG = 2;
pub const MQMLP_ENCRYPTION_ALG_3DES: crate::MQLONG = 3;
pub const MQMLP_ENCRYPTION_ALG_AES128: crate::MQLONG = 4;
pub const MQMLP_ENCRYPTION_ALG_AES256: crate::MQLONG = 5;
pub const MQMLP_SIGN_ALG_NONE: crate::MQLONG = 0;
pub const MQMLP_SIGN_ALG_MD5: crate::MQLONG = 1;
pub const MQMLP_SIGN_ALG_SHA1: crate::MQLONG = 2;
pub const MQMLP_SIGN_ALG_SHA224: crate::MQLONG = 3;
pub const MQMLP_SIGN_ALG_SHA256: crate::MQLONG = 4;
pub const MQMLP_SIGN_ALG_SHA384: crate::MQLONG = 5;
pub const MQMLP_SIGN_ALG_SHA512: crate::MQLONG = 6;
pub const MQPO_YES: crate::MQLONG = 1;
pub const MQPO_NO: crate::MQLONG = 0;
pub const MQPSCT_NONE: crate::MQLONG = -1;
pub const MQPSST_ALL: crate::MQLONG = 0;
pub const MQPSST_LOCAL: crate::MQLONG = 1;
pub const MQPSST_PARENT: crate::MQLONG = 2;
pub const MQPSST_CHILD: crate::MQLONG = 3;
pub const MQPS_STATUS_INACTIVE: crate::MQLONG = 0;
pub const MQPS_STATUS_STARTING: crate::MQLONG = 1;
pub const MQPS_STATUS_STOPPING: crate::MQLONG = 2;
pub const MQPS_STATUS_ACTIVE: crate::MQLONG = 3;
pub const MQPS_STATUS_COMPAT: crate::MQLONG = 4;
pub const MQPS_STATUS_ERROR: crate::MQLONG = 5;
pub const MQPS_STATUS_REFUSED: crate::MQLONG = 6;
pub const MQQMDT_EXPLICIT_CLUSTER_SENDER: crate::MQLONG = 1;
pub const MQQMDT_AUTO_CLUSTER_SENDER: crate::MQLONG = 2;
pub const MQQMDT_AUTO_EXP_CLUSTER_SENDER: crate::MQLONG = 4;
pub const MQQMDT_CLUSTER_RECEIVER: crate::MQLONG = 3;
pub const MQQMFAC_IMS_BRIDGE: crate::MQLONG = 1;
pub const MQQMFAC_DB2: crate::MQLONG = 2;
pub const MQQMSTA_STARTING: crate::MQLONG = 1;
pub const MQQMSTA_RUNNING: crate::MQLONG = 2;
pub const MQQMSTA_QUIESCING: crate::MQLONG = 3;
pub const MQQMSTA_STANDBY: crate::MQLONG = 4;
pub const MQQMT_NORMAL: crate::MQLONG = 0;
pub const MQQMT_REPOSITORY: crate::MQLONG = 1;
pub const MQQO_YES: crate::MQLONG = 1;
pub const MQQO_NO: crate::MQLONG = 0;
pub const MQQSIE_NONE: crate::MQLONG = 0;
pub const MQQSIE_HIGH: crate::MQLONG = 1;
pub const MQQSIE_OK: crate::MQLONG = 2;
pub const MQQSOT_ALL: crate::MQLONG = 1;
pub const MQQSOT_INPUT: crate::MQLONG = 2;
pub const MQQSOT_OUTPUT: crate::MQLONG = 3;
pub const MQQSGS_UNKNOWN: crate::MQLONG = 0;
pub const MQQSGS_CREATED: crate::MQLONG = 1;
pub const MQQSGS_ACTIVE: crate::MQLONG = 2;
pub const MQQSGS_INACTIVE: crate::MQLONG = 3;
pub const MQQSGS_FAILED: crate::MQLONG = 4;
pub const MQQSGS_PENDING: crate::MQLONG = 5;
pub const MQQSO_NO: crate::MQLONG = 0;
pub const MQQSO_YES: crate::MQLONG = 1;
pub const MQQSO_SHARED: crate::MQLONG = 1;
pub const MQQSO_EXCLUSIVE: crate::MQLONG = 2;
pub const MQQSUM_YES: crate::MQLONG = 1;
pub const MQQSUM_NO: crate::MQLONG = 0;
pub const MQRAR_YES: crate::MQLONG = 1;
pub const MQRAR_NO: crate::MQLONG = 0;
pub const MQRP_YES: crate::MQLONG = 1;
pub const MQRP_NO: crate::MQLONG = 0;
pub const MQRQ_CONN_NOT_AUTHORIZED: crate::MQLONG = 1;
pub const MQRQ_OPEN_NOT_AUTHORIZED: crate::MQLONG = 2;
pub const MQRQ_CLOSE_NOT_AUTHORIZED: crate::MQLONG = 3;
pub const MQRQ_CMD_NOT_AUTHORIZED: crate::MQLONG = 4;
pub const MQRQ_Q_MGR_STOPPING: crate::MQLONG = 5;
pub const MQRQ_Q_MGR_QUIESCING: crate::MQLONG = 6;
pub const MQRQ_CHANNEL_STOPPED_OK: crate::MQLONG = 7;
pub const MQRQ_CHANNEL_STOPPED_ERROR: crate::MQLONG = 8;
pub const MQRQ_CHANNEL_STOPPED_RETRY: crate::MQLONG = 9;
pub const MQRQ_CHANNEL_STOPPED_DISABLED: crate::MQLONG = 10;
pub const MQRQ_BRIDGE_STOPPED_OK: crate::MQLONG = 11;
pub const MQRQ_BRIDGE_STOPPED_ERROR: crate::MQLONG = 12;
pub const MQRQ_SSL_HANDSHAKE_ERROR: crate::MQLONG = 13;
pub const MQRQ_SSL_CIPHER_SPEC_ERROR: crate::MQLONG = 14;
pub const MQRQ_SSL_CLIENT_AUTH_ERROR: crate::MQLONG = 15;
pub const MQRQ_SSL_PEER_NAME_ERROR: crate::MQLONG = 16;
pub const MQRQ_SUB_NOT_AUTHORIZED: crate::MQLONG = 17;
pub const MQRQ_SUB_DEST_NOT_AUTHORIZED: crate::MQLONG = 18;
pub const MQRQ_SSL_UNKNOWN_REVOCATION: crate::MQLONG = 19;
pub const MQRQ_SYS_CONN_NOT_AUTHORIZED: crate::MQLONG = 20;
pub const MQRQ_CHANNEL_BLOCKED_ADDRESS: crate::MQLONG = 21;
pub const MQRQ_CHANNEL_BLOCKED_USERID: crate::MQLONG = 22;
pub const MQRQ_CHANNEL_BLOCKED_NOACCESS: crate::MQLONG = 23;
pub const MQRQ_MAX_ACTIVE_CHANNELS: crate::MQLONG = 24;
pub const MQRQ_MAX_CHANNELS: crate::MQLONG = 25;
pub const MQRQ_SVRCONN_INST_LIMIT: crate::MQLONG = 26;
pub const MQRQ_CLIENT_INST_LIMIT: crate::MQLONG = 27;
pub const MQRQ_CAF_NOT_INSTALLED: crate::MQLONG = 28;
pub const MQRQ_CSP_NOT_AUTHORIZED: crate::MQLONG = 29;
pub const MQRQ_FAILOVER_PERMITTED: crate::MQLONG = 30;
pub const MQRQ_FAILOVER_NOT_PERMITTED: crate::MQLONG = 31;
pub const MQRQ_STANDBY_ACTIVATED: crate::MQLONG = 32;
pub const MQRQ_REPLICA_ACTIVATED: crate::MQLONG = 33;
pub const MQRQ_CONN_AUTHORIZED: crate::MQLONG = 65;
pub const MQRQ_OPEN_AUTHORIZED: crate::MQLONG = 66;
pub const MQRQ_SUB_AUTHORIZED: crate::MQLONG = 67;
pub const MQRQ_SUB_DEST_AUTHORIZED: crate::MQLONG = 68;
pub const MQRT_CONFIGURATION: crate::MQLONG = 1;
pub const MQRT_EXPIRY: crate::MQLONG = 2;
pub const MQRT_NSPROC: crate::MQLONG = 3;
pub const MQRT_PROXYSUB: crate::MQLONG = 4;
pub const MQRT_SUB_CONFIGURATION: crate::MQLONG = 5;
pub const MQSCO_Q_MGR: crate::MQLONG = 1;
pub const MQSCO_CELL: crate::MQLONG = 2;
pub const MQSECITEM_ALL: crate::MQLONG = 0;
pub const MQSECITEM_MQADMIN: crate::MQLONG = 1;
pub const MQSECITEM_MQNLIST: crate::MQLONG = 2;
pub const MQSECITEM_MQPROC: crate::MQLONG = 3;
pub const MQSECITEM_MQQUEUE: crate::MQLONG = 4;
pub const MQSECITEM_MQCONN: crate::MQLONG = 5;
pub const MQSECITEM_MQCMDS: crate::MQLONG = 6;
pub const MQSECITEM_MXADMIN: crate::MQLONG = 7;
pub const MQSECITEM_MXNLIST: crate::MQLONG = 8;
pub const MQSECITEM_MXPROC: crate::MQLONG = 9;
pub const MQSECITEM_MXQUEUE: crate::MQLONG = 10;
pub const MQSECITEM_MXTOPIC: crate::MQLONG = 11;
pub const MQSECSW_PROCESS: crate::MQLONG = 1;
pub const MQSECSW_NAMELIST: crate::MQLONG = 2;
pub const MQSECSW_Q: crate::MQLONG = 3;
pub const MQSECSW_TOPIC: crate::MQLONG = 4;
pub const MQSECSW_CONTEXT: crate::MQLONG = 6;
pub const MQSECSW_ALTERNATE_USER: crate::MQLONG = 7;
pub const MQSECSW_COMMAND: crate::MQLONG = 8;
pub const MQSECSW_CONNECTION: crate::MQLONG = 9;
pub const MQSECSW_SUBSYSTEM: crate::MQLONG = 10;
pub const MQSECSW_COMMAND_RESOURCES: crate::MQLONG = 11;
pub const MQSECSW_Q_MGR: crate::MQLONG = 15;
pub const MQSECSW_QSG: crate::MQLONG = 16;
pub const MQSECSW_OFF_FOUND: crate::MQLONG = 21;
pub const MQSECSW_ON_FOUND: crate::MQLONG = 22;
pub const MQSECSW_OFF_NOT_FOUND: crate::MQLONG = 23;
pub const MQSECSW_ON_NOT_FOUND: crate::MQLONG = 24;
pub const MQSECSW_OFF_ERROR: crate::MQLONG = 25;
pub const MQSECSW_ON_OVERRIDDEN: crate::MQLONG = 26;
pub const MQSECTYPE_AUTHSERV: crate::MQLONG = 1;
pub const MQSECTYPE_SSL: crate::MQLONG = 2;
pub const MQSECTYPE_CLASSES: crate::MQLONG = 3;
pub const MQSECTYPE_CONNAUTH: crate::MQLONG = 4;
pub const MQCHK_OPTIONAL: crate::MQLONG = 0;
pub const MQCHK_NONE: crate::MQLONG = 1;
pub const MQCHK_REQUIRED_ADMIN: crate::MQLONG = 2;
pub const MQCHK_REQUIRED: crate::MQLONG = 3;
pub const MQCHK_AS_Q_MGR: crate::MQLONG = 4;
pub const MQADPCTX_NO: crate::MQLONG = 0;
pub const MQADPCTX_YES: crate::MQLONG = 1;
pub const MQSECCOMM_NO: crate::MQLONG = 0;
pub const MQSECCOMM_YES: crate::MQLONG = 1;
pub const MQSECCOMM_ANON: crate::MQLONG = 2;
pub const MQLDAP_AUTHORMD_OS: crate::MQLONG = 0;
pub const MQLDAP_AUTHORMD_SEARCHGRP: crate::MQLONG = 1;
pub const MQLDAP_AUTHORMD_SEARCHUSR: crate::MQLONG = 2;
pub const MQLDAP_AUTHORMD_SRCHGRPSN: crate::MQLONG = 3;
pub const MQLDAP_NESTGRP_NO: crate::MQLONG = 0;
pub const MQLDAP_NESTGRP_YES: crate::MQLONG = 1;
pub const MQAUTHENTICATE_OS: crate::MQLONG = 0;
pub const MQAUTHENTICATE_PAM: crate::MQLONG = 1;
pub const MQLDAPC_INACTIVE: crate::MQLONG = 0;
pub const MQLDAPC_CONNECTED: crate::MQLONG = 1;
pub const MQLDAPC_ERROR: crate::MQLONG = 2;
pub const MQSELTYPE_NONE: crate::MQLONG = 0;
pub const MQSELTYPE_STANDARD: crate::MQLONG = 1;
pub const MQSELTYPE_EXTENDED: crate::MQLONG = 2;
pub const MQCHLA_DISABLED: crate::MQLONG = 0;
pub const MQCHLA_ENABLED: crate::MQLONG = 1;
pub const MQRDNS_ENABLED: crate::MQLONG = 0;
pub const MQRDNS_DISABLED: crate::MQLONG = 1;
pub const MQCLROUTE_DIRECT: crate::MQLONG = 0;
pub const MQCLROUTE_TOPIC_HOST: crate::MQLONG = 1;
pub const MQCLROUTE_NONE: crate::MQLONG = 2;
pub const MQCLST_ACTIVE: crate::MQLONG = 0;
pub const MQCLST_PENDING: crate::MQLONG = 1;
pub const MQCLST_INVALID: crate::MQLONG = 2;
pub const MQCLST_ERROR: crate::MQLONG = 3;
pub const MQCLXQ_SCTQ: crate::MQLONG = 0;
pub const MQCLXQ_CHANNEL: crate::MQLONG = 1;
pub const MQSUS_YES: crate::MQLONG = 1;
pub const MQSUS_NO: crate::MQLONG = 0;
pub const MQSYNCPOINT_YES: crate::MQLONG = 0;
pub const MQSYNCPOINT_IFPER: crate::MQLONG = 1;
pub const MQSYSP_NO: crate::MQLONG = 0;
pub const MQSYSP_YES: crate::MQLONG = 1;
pub const MQSYSP_EXTENDED: crate::MQLONG = 2;
pub const MQSYSP_TYPE_INITIAL: crate::MQLONG = 10;
pub const MQSYSP_TYPE_SET: crate::MQLONG = 11;
pub const MQSYSP_TYPE_LOG_COPY: crate::MQLONG = 12;
pub const MQSYSP_TYPE_LOG_STATUS: crate::MQLONG = 13;
pub const MQSYSP_TYPE_ARCHIVE_TAPE: crate::MQLONG = 14;
pub const MQSYSP_ALLOC_BLK: crate::MQLONG = 20;
pub const MQSYSP_ALLOC_TRK: crate::MQLONG = 21;
pub const MQSYSP_ALLOC_CYL: crate::MQLONG = 22;
pub const MQSYSP_STATUS_BUSY: crate::MQLONG = 30;
pub const MQSYSP_STATUS_PREMOUNT: crate::MQLONG = 31;
pub const MQSYSP_STATUS_AVAILABLE: crate::MQLONG = 32;
pub const MQSYSP_STATUS_UNKNOWN: crate::MQLONG = 33;
pub const MQSYSP_STATUS_ALLOC_ARCHIVE: crate::MQLONG = 34;
pub const MQSYSP_STATUS_COPYING_BSDS: crate::MQLONG = 35;
pub const MQSYSP_STATUS_COPYING_LOG: crate::MQLONG = 36;
pub const MQEXT_ALL: crate::MQLONG = 0;
pub const MQEXT_OBJECT: crate::MQLONG = 1;
pub const MQEXT_AUTHORITY: crate::MQLONG = 2;
pub const MQEXTATTRS_ALL: crate::MQLONG = 0;
pub const MQEXTATTRS_NONDEF: crate::MQLONG = 1;
pub const MQSYSOBJ_YES: crate::MQLONG = 0;
pub const MQSYSOBJ_NO: crate::MQLONG = 1;
pub const MQSUBTYPE_API: crate::MQLONG = 1;
pub const MQSUBTYPE_ADMIN: crate::MQLONG = 2;
pub const MQSUBTYPE_PROXY: crate::MQLONG = 3;
pub const MQSUBTYPE_ALL: crate::MQLONG = -1;
pub const MQSUBTYPE_USER: crate::MQLONG = -2;
pub const MQDOPT_RESOLVED: crate::MQLONG = 0;
pub const MQDOPT_DEFINED: crate::MQLONG = 1;
pub const MQTIME_UNIT_MINS: crate::MQLONG = 0;
pub const MQTIME_UNIT_SECS: crate::MQLONG = 1;
pub const MQUIDSUPP_NO: crate::MQLONG = 0;
pub const MQUIDSUPP_YES: crate::MQLONG = 1;
pub const MQUNDELIVERED_NORMAL: crate::MQLONG = 0;
pub const MQUNDELIVERED_SAFE: crate::MQLONG = 1;
pub const MQUNDELIVERED_DISCARD: crate::MQLONG = 2;
pub const MQUNDELIVERED_KEEP: crate::MQLONG = 3;
pub const MQUOWST_NONE: crate::MQLONG = 0;
pub const MQUOWST_ACTIVE: crate::MQLONG = 1;
pub const MQUOWST_PREPARED: crate::MQLONG = 2;
pub const MQUOWST_UNRESOLVED: crate::MQLONG = 3;
pub const MQUOWT_Q_MGR: crate::MQLONG = 0;
pub const MQUOWT_CICS: crate::MQLONG = 1;
pub const MQUOWT_RRS: crate::MQLONG = 2;
pub const MQUOWT_IMS: crate::MQLONG = 3;
pub const MQUOWT_XA: crate::MQLONG = 4;
pub const MQUSAGE_PS_AVAILABLE: crate::MQLONG = 0;
pub const MQUSAGE_PS_DEFINED: crate::MQLONG = 1;
pub const MQUSAGE_PS_OFFLINE: crate::MQLONG = 2;
pub const MQUSAGE_PS_NOT_DEFINED: crate::MQLONG = 3;
pub const MQUSAGE_PS_SUSPENDED: crate::MQLONG = 4;
pub const MQUSAGE_EXPAND_USER: crate::MQLONG = 1;
pub const MQUSAGE_EXPAND_SYSTEM: crate::MQLONG = 2;
pub const MQUSAGE_EXPAND_NONE: crate::MQLONG = 3;
pub const MQUSAGE_DS_OLDEST_ACTIVE_UOW: crate::MQLONG = 10;
pub const MQUSAGE_DS_OLDEST_PS_RECOVERY: crate::MQLONG = 11;
pub const MQUSAGE_DS_OLDEST_CF_RECOVERY: crate::MQLONG = 12;
pub const MQMCP_REPLY: crate::MQLONG = 2;
pub const MQMCP_USER: crate::MQLONG = 1;
pub const MQMCP_NONE: crate::MQLONG = 0;
pub const MQMCP_ALL: crate::MQLONG = -1;
pub const MQMCP_COMPAT: crate::MQLONG = -2;
pub const MQNSH_NONE: crate::MQLONG = 0;
pub const MQNSH_ALL: crate::MQLONG = -1;
pub const MQLR_ONE: crate::MQLONG = 1;
pub const MQLR_AUTO: crate::MQLONG = -1;
pub const MQLR_MAX: crate::MQLONG = -2;
pub const MQAUTOCLUS_TYPE_NONE: crate::MQLONG = 0;
pub const MQAUTOCLUS_TYPE_UNIFORM: crate::MQLONG = 1;
pub const MQFS_SHARED: crate::MQLONG = -1;
pub const MQFSENC_NO: crate::MQLONG = 0;
pub const MQFSENC_YES: crate::MQLONG = 1;
pub const MQFSENC_UNKNOWN: crate::MQLONG = 2;
pub const MQLOGTYPE_CIRCULAR: crate::MQLONG = 0;
pub const MQLOGTYPE_LINEAR: crate::MQLONG = 1;
pub const MQLOGTYPE_REPLICATED: crate::MQLONG = 2;
pub const MQNHACONNACTV_NO: crate::MQLONG = 0;
pub const MQNHACONNACTV_YES: crate::MQLONG = 1;
pub const MQNHABACKLOG_UNKNOWN: crate::MQLONG = -1;
pub const MQNHACONNGRP_NO: crate::MQLONG = 0;
pub const MQNHACONNGRP_YES: crate::MQLONG = 1;
pub const MQNHACONNGRP_SUSPENDED: crate::MQLONG = 2;
pub const MQNHAGRPROLE_UNKNOWN: crate::MQLONG = 0;
pub const MQNHAGRPROLE_NOT_CONFIGURED: crate::MQLONG = 1;
pub const MQNHAGRPROLE_LIVE: crate::MQLONG = 2;
pub const MQNHAGRPROLE_RECOVERY: crate::MQLONG = 3;
pub const MQNHAGRPROLE_PENDING_LIVE: crate::MQLONG = 4;
pub const MQNHAGRPROLE_PENDING_RECOVERY: crate::MQLONG = 5;
pub const MQNHAROLE_UNKNOWN: crate::MQLONG = 0;
pub const MQNHAROLE_ACTIVE: crate::MQLONG = 1;
pub const MQNHAROLE_REPLICA: crate::MQLONG = 2;
pub const MQNHAROLE_LEADER: crate::MQLONG = 3;
pub const MQNHAINSYNC_NO: crate::MQLONG = 0;
pub const MQNHAINSYNC_YES: crate::MQLONG = 1;
pub const MQNHASTATUS_UNKNOWN: crate::MQLONG = 0;
pub const MQNHASTATUS_NORMAL: crate::MQLONG = 1;
pub const MQNHASTATUS_CHECKING: crate::MQLONG = 2;
pub const MQNHASTATUS_SYNCHRONIZING: crate::MQLONG = 3;
pub const MQNHASTATUS_REBASING: crate::MQLONG = 4;
pub const MQNHASTATUS_DISK_FULL: crate::MQLONG = 5;
pub const MQNHASTATUS_DISCONNECTED: crate::MQLONG = 6;
pub const MQNHASTATUS_PARTITIONED: crate::MQLONG = 7;
pub const MQNHATYPE_INSTANCE: crate::MQLONG = 0;
pub const MQNHATYPE_GROUP: crate::MQLONG = 1;
pub const MQNHATYPE_ALL: crate::MQLONG = -1;
pub const MQOPER_SYSTEM_FIRST: crate::MQLONG = 0;
pub const MQOPER_UNKNOWN: crate::MQLONG = 0;
pub const MQOPER_BROWSE: crate::MQLONG = 1;
pub const MQOPER_DISCARD: crate::MQLONG = 2;
pub const MQOPER_GET: crate::MQLONG = 3;
pub const MQOPER_PUT: crate::MQLONG = 4;
pub const MQOPER_PUT_REPLY: crate::MQLONG = 5;
pub const MQOPER_PUT_REPORT: crate::MQLONG = 6;
pub const MQOPER_RECEIVE: crate::MQLONG = 7;
pub const MQOPER_SEND: crate::MQLONG = 8;
pub const MQOPER_TRANSFORM: crate::MQLONG = 9;
pub const MQOPER_PUBLISH: crate::MQLONG = 10;
pub const MQOPER_EXCLUDED_PUBLISH: crate::MQLONG = 11;
pub const MQOPER_DISCARDED_PUBLISH: crate::MQLONG = 12;
pub const MQOPER_SYSTEM_LAST: crate::MQLONG = 65535;
pub const MQOPER_APPL_FIRST: crate::MQLONG = 65536;
pub const MQOPER_APPL_LAST: crate::MQLONG = 999999999;
pub const MQROUTE_UNLIMITED_ACTIVITIES: crate::MQLONG = 0;
pub const MQROUTE_DETAIL_LOW: crate::MQLONG = 2;
pub const MQROUTE_DETAIL_MEDIUM: crate::MQLONG = 8;
pub const MQROUTE_DETAIL_HIGH: crate::MQLONG = 32;
pub const MQROUTE_FORWARD_ALL: crate::MQLONG = 256;
pub const MQROUTE_FORWARD_IF_SUPPORTED: crate::MQLONG = 512;
pub const MQROUTE_FORWARD_REJ_UNSUP_MASK: crate::MQLONG = -65536;
pub const MQROUTE_DELIVER_YES: crate::MQLONG = 4096;
pub const MQROUTE_DELIVER_NO: crate::MQLONG = 8192;
pub const MQROUTE_DELIVER_REJ_UNSUP_MASK: crate::MQLONG = -65536;
pub const MQROUTE_ACCUMULATE_NONE: crate::MQLONG = 65539;
pub const MQROUTE_ACCUMULATE_IN_MSG: crate::MQLONG = 65540;
pub const MQROUTE_ACCUMULATE_AND_REPLY: crate::MQLONG = 65541;
pub const MQDELO_NONE: crate::MQLONG = 0;
pub const MQDELO_LOCAL: crate::MQLONG = 4;
pub const MQPUBO_NONE: crate::MQLONG = 0;
pub const MQPUBO_CORREL_ID_AS_IDENTITY: crate::MQLONG = 1;
pub const MQPUBO_RETAIN_PUBLICATION: crate::MQLONG = 2;
pub const MQPUBO_OTHER_SUBSCRIBERS_ONLY: crate::MQLONG = 4;
pub const MQPUBO_NO_REGISTRATION: crate::MQLONG = 8;
pub const MQPUBO_IS_RETAINED_PUBLICATION: crate::MQLONG = 16;
pub const MQREGO_NONE: crate::MQLONG = 0;
pub const MQREGO_CORREL_ID_AS_IDENTITY: crate::MQLONG = 1;
pub const MQREGO_ANONYMOUS: crate::MQLONG = 2;
pub const MQREGO_LOCAL: crate::MQLONG = 4;
pub const MQREGO_DIRECT_REQUESTS: crate::MQLONG = 8;
pub const MQREGO_NEW_PUBLICATIONS_ONLY: crate::MQLONG = 16;
pub const MQREGO_PUBLISH_ON_REQUEST_ONLY: crate::MQLONG = 32;
pub const MQREGO_DEREGISTER_ALL: crate::MQLONG = 64;
pub const MQREGO_INCLUDE_STREAM_NAME: crate::MQLONG = 128;
pub const MQREGO_INFORM_IF_RETAINED: crate::MQLONG = 256;
pub const MQREGO_DUPLICATES_OK: crate::MQLONG = 512;
pub const MQREGO_NON_PERSISTENT: crate::MQLONG = 1024;
pub const MQREGO_PERSISTENT: crate::MQLONG = 2048;
pub const MQREGO_PERSISTENT_AS_PUBLISH: crate::MQLONG = 4096;
pub const MQREGO_PERSISTENT_AS_Q: crate::MQLONG = 8192;
pub const MQREGO_ADD_NAME: crate::MQLONG = 16384;
pub const MQREGO_NO_ALTERATION: crate::MQLONG = 32768;
pub const MQREGO_FULL_RESPONSE: crate::MQLONG = 65536;
pub const MQREGO_JOIN_SHARED: crate::MQLONG = 131072;
pub const MQREGO_JOIN_EXCLUSIVE: crate::MQLONG = 262144;
pub const MQREGO_LEAVE_ONLY: crate::MQLONG = 524288;
pub const MQREGO_VARIABLE_USER_ID: crate::MQLONG = 1048576;
pub const MQREGO_LOCKED: crate::MQLONG = 2097152;
pub const MQUA_FIRST: crate::MQLONG = 65536;
pub const MQUA_LAST: crate::MQLONG = 999999999;
pub const MQGUR_DISABLED: crate::MQLONG = 0;
pub const MQGUR_ENABLED: crate::MQLONG = 1;
pub const MQMULC_STANDARD: crate::MQLONG = 0;
pub const MQMULC_REFINED: crate::MQLONG = 1;
pub const MQSTDBY_NOT_PERMITTED: crate::MQLONG = 0;
pub const MQSTDBY_PERMITTED: crate::MQLONG = 1;
