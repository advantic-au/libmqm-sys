/* Generated with MQ client version 9.4.3.0 */

pub type MQBYTE = ::std::os::raw::c_uchar;
pub type PMQBYTE = *mut MQBYTE;
pub type MQBYTE4 = [MQBYTE; 4usize];
pub type PMQBYTE4 = *mut MQBYTE4;
pub type MQBYTE8 = [MQBYTE; 8usize];
pub type PMQBYTE8 = *mut MQBYTE8;
pub type MQBYTE16 = [MQBYTE; 16usize];
pub type PMQBYTE16 = *mut MQBYTE16;
pub type MQBYTE24 = [MQBYTE; 24usize];
pub type PMQBYTE24 = *mut MQBYTE24;
pub type MQBYTE32 = [MQBYTE; 32usize];
pub type PMQBYTE32 = *mut MQBYTE32;
pub type MQBYTE40 = [MQBYTE; 40usize];
pub type PMQBYTE40 = *mut MQBYTE40;
pub type MQBYTE48 = [MQBYTE; 48usize];
pub type PMQBYTE48 = *mut MQBYTE48;
pub type MQBYTE128 = [MQBYTE; 128usize];
pub type PMQBYTE128 = *mut MQBYTE128;
pub type MQCHAR = ::std::os::raw::c_char;
pub type PMQCHAR = *mut MQCHAR;
pub type MQCHAR4 = [MQCHAR; 4usize];
pub type PMQCHAR4 = *mut MQCHAR4;
pub type MQCHAR8 = [MQCHAR; 8usize];
pub type PMQCHAR8 = *mut MQCHAR8;
pub type MQCHAR12 = [MQCHAR; 12usize];
pub type PMQCHAR12 = *mut MQCHAR12;
pub type MQCHAR16 = [MQCHAR; 16usize];
pub type PMQCHAR16 = *mut MQCHAR16;
pub type MQCHAR20 = [MQCHAR; 20usize];
pub type PMQCHAR20 = *mut MQCHAR20;
pub type MQCHAR28 = [MQCHAR; 28usize];
pub type PMQCHAR28 = *mut MQCHAR28;
pub type MQCHAR32 = [MQCHAR; 32usize];
pub type PMQCHAR32 = *mut MQCHAR32;
pub type MQCHAR48 = [MQCHAR; 48usize];
pub type PMQCHAR48 = *mut MQCHAR48;
pub type MQCHAR64 = [MQCHAR; 64usize];
pub type PMQCHAR64 = *mut MQCHAR64;
pub type MQCHAR128 = [MQCHAR; 128usize];
pub type PMQCHAR128 = *mut MQCHAR128;
pub type MQCHAR256 = [MQCHAR; 256usize];
pub type PMQCHAR256 = *mut MQCHAR256;
pub type MQCHAR264 = [MQCHAR; 264usize];
pub type PMQCHAR264 = *mut MQCHAR264;
pub type MQCHAR1024 = [MQCHAR; 1024usize];
pub type PMQCHAR1024 = *mut MQCHAR1024;
pub type MQLONG = ::std::os::raw::c_int;
pub type MQULONG = ::std::os::raw::c_uint;
pub type MQINT64 = ::std::os::raw::c_long;
pub type MQUINT64 = ::std::os::raw::c_ulong;
pub type PMQLONG = *mut MQLONG;
pub type MQINT8 = ::std::os::raw::c_schar;
pub type PMQINT8 = *mut MQINT8;
pub type MQUINT8 = ::std::os::raw::c_uchar;
pub type PMQUINT8 = *mut MQUINT8;
pub type MQINT16 = ::std::os::raw::c_short;
pub type PMQINT16 = *mut MQINT16;
pub type MQUINT16 = ::std::os::raw::c_ushort;
pub type PMQUINT16 = *mut MQUINT16;
pub type MQINT32 = MQLONG;
pub type PMQINT32 = PMQLONG;
pub type PMQINT64 = *mut MQINT64;
pub type PMQULONG = *mut MQULONG;
pub type MQUINT32 = MQULONG;
pub type PMQUINT32 = PMQULONG;
pub type PMQUINT64 = *mut MQUINT64;
pub type MQFLOAT32 = f32;
pub type PMQFLOAT32 = *mut MQFLOAT32;
pub type MQFLOAT64 = f64;
pub type PMQFLOAT64 = *mut MQFLOAT64;
pub type MQHCONN = MQLONG;
pub type PMQHCONN = *mut MQHCONN;
pub type MQHOBJ = MQLONG;
pub type PMQHOBJ = *mut MQHOBJ;
pub type MQPTR = *mut ::std::os::raw::c_void;
pub type PMQPTR = *mut MQPTR;
pub type PMQFUNC = *mut ::std::os::raw::c_void;
pub type PMQVOID = *mut ::std::os::raw::c_void;
pub type MQBOOL = MQLONG;
pub type PMQBOOL = *mut MQBOOL;
pub type MQHMSG = MQINT64;
pub type PMQHMSG = *mut MQHMSG;
pub type MQPID = MQLONG;
pub type PMQPID = *mut MQPID;
pub type MQTID = MQLONG;
pub type PMQTID = *mut MQTID;
/// Authentication Information Record
pub type MQAIR = tagMQAIR;
pub type PMQAIR = *mut MQAIR;
/// MQ Balancing Options
pub type MQBNO = tagMQBNO;
pub type PMQBNO = *mut MQBNO;
/// Buffer To Message Handle Options
pub type MQBMHO = tagMQBMHO;
pub type PMQBMHO = *mut MQBMHO;
/// Begin Options
pub type MQBO = tagMQBO;
pub type PMQBO = *mut MQBO;
/// Callback Context
pub type MQCBC = tagMQCBC;
pub type PMQCBC = *mut MQCBC;
/// Callback Data Descriptor
pub type MQCBD = tagMQCBD;
pub type PMQCBD = *mut MQCBD;
/// Variable-length string
pub type MQCHARV = tagMQCHARV;
pub type PMQCHARV = *mut MQCHARV;
/// CICS Information Header
pub type MQCIH = tagMQCIH;
pub type PMQCIH = *mut MQCIH;
/// Create Message Handle Options
pub type MQCMHO = tagMQCMHO;
pub type PMQCMHO = *mut MQCMHO;
/// MQCTL function options
pub type MQCTLO = tagMQCTLO;
pub type PMQCTLO = *mut MQCTLO;
/// SSL Configuration Options
pub type MQSCO = tagMQSCO;
pub type PMQSCO = *mut MQSCO;
/// Security Parameters
pub type MQCSP = tagMQCSP;
pub type PMQCSP = *mut MQCSP;
/// Connect Options
pub type MQCNO = tagMQCNO;
pub type PMQCNO = *mut MQCNO;
/// Distribution Header
pub type MQDH = tagMQDH;
pub type PMQDH = *mut MQDH;
/// Dead Letter Header
pub type MQDLH = tagMQDLH;
pub type PMQDLH = *mut MQDLH;
/// Delete Message Handle Options
pub type MQDMHO = tagMQDMHO;
pub type PMQDMHO = *mut MQDMHO;
/// Delete Message Property Options
pub type MQDMPO = tagMQDMPO;
pub type PMQDMPO = *mut MQDMPO;
/// Get Message Options
pub type MQGMO = tagMQGMO;
pub type PMQGMO = *mut MQGMO;
/// IMS Information Header
pub type MQIIH = tagMQIIH;
pub type PMQIIH = *mut MQIIH;
/// Inquire Message Property Options
pub type MQIMPO = tagMQIMPO;
pub type PMQIMPO = *mut MQIMPO;
/// Message Descriptor
pub type MQMD = tagMQMD;
pub type PMQMD = *mut MQMD;
/// Message Descriptor Extension
pub type MQMDE = tagMQMDE;
pub type PMQMDE = *mut MQMDE;
/// Version-1 Message Descriptor
pub type MQMD1 = tagMQMD1;
pub type PMQMD1 = *mut MQMD1;
/// Version-2 Message Descriptor
pub type MQMD2 = tagMQMD2;
pub type PMQMD2 = *mut MQMD2;
/// Message Handle To Buffer Options
pub type MQMHBO = tagMQMHBO;
pub type PMQMHBO = *mut MQMHBO;
/// Object descriptor
pub type MQOD = tagMQOD;
pub type PMQOD = *mut MQOD;
/// Object Record
pub type MQOR = tagMQOR;
pub type PMQOR = *mut MQOR;
/// Property descriptor
pub type MQPD = tagMQPD;
pub type PMQPD = *mut MQPD;
/// Put Message Options
pub type MQPMO = tagMQPMO;
pub type PMQPMO = *mut MQPMO;
/// Rules and Formatting Header
pub type MQRFH = tagMQRFH;
pub type PMQRFH = *mut MQRFH;
/// Rules and Formatting Header 2
pub type MQRFH2 = tagMQRFH2;
pub type PMQRFH2 = *mut MQRFH2;
/// Reference Message Header
pub type MQRMH = tagMQRMH;
pub type PMQRMH = *mut MQRMH;
/// Response Record
pub type MQRR = tagMQRR;
pub type PMQRR = *mut MQRR;
/// Subscription Descriptor
pub type MQSD = tagMQSD;
pub type PMQSD = *mut MQSD;
/// Set Message Property Options
pub type MQSMPO = tagMQSMPO;
pub type PMQSMPO = *mut MQSMPO;
/// Subscription Request Options
pub type MQSRO = tagMQSRO;
pub type PMQSRO = *mut MQSRO;
/// Status Information Record
pub type MQSTS = tagMQSTS;
pub type PMQSTS = *mut MQSTS;
/// Trigger Message
pub type MQTM = tagMQTM;
pub type PMQTM = *mut MQTM;
/// Trigger Message 2 (Character)
pub type MQTMC2 = tagMQTMC2;
pub type PMQTMC2 = *mut MQTMC2;
/// Work Information Header
pub type MQWIH = tagMQWIH;
pub type PMQWIH = *mut MQWIH;
/// Transmission Queue Header
pub type MQXQH = tagMQXQH;
pub type PMQXQH = *mut MQXQH;
/// Message Consumer routine (Called by MQ)
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `pMsgDesc`: Message descriptor
/// * `pGetMsgOpts`: Area containing the MQGMO
/// * `pBuffer`: Area containing the message data
/// * `pContext`: Area containing the Consumer context
pub type MQCB_FUNCTION = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: &mut MQGMO,
        pBuffer: PMQVOID,
        pContext: PMQCBC,
    ),
>;
pub type PMQCB_FUNCTION = MQCB_FUNCTION;
/// Authentication Information Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQAIR {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of authentication information
    pub AuthInfoType: MQLONG,
    /// Connection name of CRL LDAP server
    pub AuthInfoConnName: [MQCHAR; 264usize],
    /// Address of LDAP user name
    pub LDAPUserNamePtr: PMQCHAR,
    /// Offset of LDAP user name from start of MQAIR structure
    pub LDAPUserNameOffset: MQLONG,
    /// Length of LDAP user name
    pub LDAPUserNameLength: MQLONG,
    /// Password to access LDAP server
    pub LDAPPassword: MQCHAR32,
    /// URL of the OCSP responder
    pub OCSPResponderURL: MQCHAR256,
}
/// MQ Balancing Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQBNO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Application Balancing Type
    pub ApplType: MQLONG,
    /// Timeout value in seconds
    pub Timeout: MQLONG,
    /// Additional Balancing Options
    pub Options: MQLONG,
}
/// Buffer To Message Handle Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQBMHO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQBUFMH
    pub Options: MQLONG,
}
/// Begin Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQBO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQBEGIN
    pub Options: MQLONG,
}
/// Callback Context
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCBC {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Why Function was called
    pub CallType: MQLONG,
    /// Object Handle
    pub Hobj: MQHOBJ,
    /// Callback data passed to the function
    pub CallbackArea: MQPTR,
    /// MQCTL Data area passed to the function
    pub ConnectionArea: MQPTR,
    /// Completion Code
    pub CompCode: MQLONG,
    /// Reason Code
    pub Reason: MQLONG,
    /// Consumer State
    pub State: MQLONG,
    /// Message Data Length
    pub DataLength: MQLONG,
    /// Buffer Length
    pub BufferLength: MQLONG,
    /// Flags containing information about this consumer
    pub Flags: MQLONG,
    /// Number of milliseconds before reconnect attempt
    pub ReconnectDelay: MQLONG,
}
/// Callback Data Descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCBD {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Callback function type
    pub CallbackType: MQLONG,
    /// Options controlling message consumption
    pub Options: MQLONG,
    /// User data passed to the function
    pub CallbackArea: MQPTR,
    /// Callback function pointer
    pub CallbackFunction: MQPTR,
    /// Callback name
    pub CallbackName: MQCHAR128,
    /// Maximum message length
    pub MaxMsgLength: MQLONG,
}
/// Variable-length string
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCHARV {
    /// Address of variable length string
    pub VSPtr: MQPTR,
    /// Offset of variable length string
    pub VSOffset: MQLONG,
    /// Size of buffer
    pub VSBufSize: MQLONG,
    /// Length of variable length string
    pub VSLength: MQLONG,
    /// CCSID of variable length string
    pub VSCCSID: MQLONG,
}
/// CICS Information Header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCIH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQCIH structure
    pub StrucLength: MQLONG,
    /// Reserved
    pub Encoding: MQLONG,
    /// Reserved
    pub CodedCharSetId: MQLONG,
    /// MQ format name of data that follows MQCIH
    pub Format: MQCHAR8,
    /// Flags
    pub Flags: MQLONG,
    /// Return code from bridge
    pub ReturnCode: MQLONG,
    /// MQ completion code or CICS EIBRESP
    pub CompCode: MQLONG,
    /// MQ reason or feedback code, or CICS EIBRESP2
    pub Reason: MQLONG,
    /// Unit-of-work control
    pub UOWControl: MQLONG,
    /// Wait interval for MQGET call issued by bridge task
    pub GetWaitInterval: MQLONG,
    /// Link type
    pub LinkType: MQLONG,
    /// Output COMMAREA data length
    pub OutputDataLength: MQLONG,
    /// Bridge facility release time
    pub FacilityKeepTime: MQLONG,
    /// Send/receive ADS descriptor
    pub ADSDescriptor: MQLONG,
    /// Whether task can be conversational
    pub ConversationalTask: MQLONG,
    /// Status at end of task
    pub TaskEndStatus: MQLONG,
    /// Bridge facility token
    pub Facility: MQBYTE8,
    /// MQ call name or CICS EIBFN function
    pub Function: MQCHAR4,
    /// Abend code
    pub AbendCode: MQCHAR4,
    /// Password or passticket
    pub Authenticator: MQCHAR8,
    /// Reserved
    pub Reserved1: MQCHAR8,
    /// MQ format name of reply message
    pub ReplyToFormat: MQCHAR8,
    /// Remote CICS system id to use
    pub RemoteSysId: MQCHAR4,
    /// CICS RTRANSID to use
    pub RemoteTransId: MQCHAR4,
    /// Transaction to attach
    pub TransactionId: MQCHAR4,
    /// Terminal emulated attributes
    pub FacilityLike: MQCHAR4,
    /// AID key
    pub AttentionId: MQCHAR4,
    /// Transaction start code
    pub StartCode: MQCHAR4,
    /// Abend transaction code
    pub CancelCode: MQCHAR4,
    /// Next transaction to attach
    pub NextTransactionId: MQCHAR4,
    /// Reserved
    pub Reserved2: MQCHAR8,
    /// Reserved
    pub Reserved3: MQCHAR8,
    /// Cursor position
    pub CursorPosition: MQLONG,
    /// Offset of error in message
    pub ErrorOffset: MQLONG,
    /// Reserved
    pub InputItem: MQLONG,
    /// Reserved
    pub Reserved4: MQLONG,
}
/// Create Message Handle Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCMHO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQCRTMH
    pub Options: MQLONG,
}
/// MQCTL function options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCTLO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQCTL
    pub Options: MQLONG,
    /// Reserved
    pub Reserved: MQLONG,
    /// MQCTL Data area passed to the function
    pub ConnectionArea: MQPTR,
}
/// SSL Configuration Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQSCO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Location of SSL key repository
    pub KeyRepository: MQCHAR256,
    /// Cryptographic hardware configuration string
    pub CryptoHardware: MQCHAR256,
    /// Number of MQAIR records present
    pub AuthInfoRecCount: MQLONG,
    /// Offset of first MQAIR record from start of MQSCO structure
    pub AuthInfoRecOffset: MQLONG,
    /// Address of first MQAIR record
    pub AuthInfoRecPtr: PMQAIR,
    /// Number of unencrypted bytes sent/received before secret key is reset
    pub KeyResetCount: MQLONG,
    /// Using FIPS-certified algorithms
    pub FipsRequired: MQLONG,
    /// Use only Suite B cryptographic algorithms
    pub EncryptionPolicySuiteB: [MQLONG; 4usize],
    /// Certificate validation policy
    pub CertificateValPolicy: MQLONG,
    /// SSL/TLS certificate label
    pub CertificateLabel: MQCHAR64,
    /// Address of key repository password
    pub KeyRepoPasswordPtr: MQPTR,
    /// Offset of key repository password
    pub KeyRepoPasswordOffset: MQLONG,
    /// Length of key repository password
    pub KeyRepoPasswordLength: MQLONG,
    /// HTTPS certificate validation level
    pub HTTPSCertValidation: MQLONG,
    /// HTTPS certificate revocation level
    pub HTTPSCertRevocation: MQLONG,
    /// Address of HTTPS Keystore
    pub HTTPSKeyStorePtr: MQPTR,
    /// Offset of HTTPS Keystore
    pub HTTPSKeyStoreOffset: MQLONG,
    /// Length of HTTPS keystore
    pub HTTPSKeyStoreLength: MQLONG,
}
/// Security Parameters
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCSP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of authentication
    pub AuthenticationType: MQLONG,
    /// Reserved
    pub Reserved1: MQBYTE4,
    /// Address of user ID
    pub CSPUserIdPtr: MQPTR,
    /// Offset of user ID
    pub CSPUserIdOffset: MQLONG,
    /// Length of user ID
    pub CSPUserIdLength: MQLONG,
    /// Reserved
    pub Reserved2: MQBYTE8,
    /// Address of password
    pub CSPPasswordPtr: MQPTR,
    /// Offset of password
    pub CSPPasswordOffset: MQLONG,
    /// Length of password
    pub CSPPasswordLength: MQLONG,
    /// Reserved
    pub Reserved3: MQBYTE8,
    /// Address of initial key
    pub InitialKeyPtr: MQPTR,
    /// Offset of initial key
    pub InitialKeyOffset: MQLONG,
    /// Length of initial key
    pub InitialKeyLength: MQLONG,
    /// Reserved
    pub Reserved4: MQBYTE8,
    /// Address of Token
    pub TokenPtr: MQPTR,
    /// Offset of Token
    pub TokenOffset: MQLONG,
    /// Length of Token
    pub TokenLength: MQLONG,
}
/// Connect Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCNO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQCONNX
    pub Options: MQLONG,
    /// Offset of MQCD structure for client connection
    pub ClientConnOffset: MQLONG,
    /// Address of MQCD structure for client connection
    pub ClientConnPtr: MQPTR,
    /// Queue-manager connection tag
    pub ConnTag: MQBYTE128,
    /// Address of MQSCO structure for client connection
    pub SSLConfigPtr: PMQSCO,
    /// Offset of MQSCO structure for client connection
    pub SSLConfigOffset: MQLONG,
    /// Unique Connection Identifier
    pub ConnectionId: MQBYTE24,
    /// Offset of MQCSP structure
    pub SecurityParmsOffset: MQLONG,
    /// Address of MQCSP structure
    pub SecurityParmsPtr: PMQCSP,
    /// Address of CCDT URL string
    pub CCDTUrlPtr: PMQCHAR,
    /// Offset of CCDT URL string
    pub CCDTUrlOffset: MQLONG,
    /// Length of CCDT URL
    pub CCDTUrlLength: MQLONG,
    /// Reserved
    pub Reserved: MQBYTE8,
    /// Application name
    pub ApplName: MQCHAR28,
    /// Reserved
    pub Reserved2: MQBYTE4,
    /// Balance Parameter Pointer
    pub BalanceParmsPtr: PMQBNO,
    /// Balance Parameter Offset
    pub BalanceParmsOffset: MQLONG,
    /// Reserved
    pub Reserved3: MQBYTE4,
}
/// Distribution Header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQDH structure plus following MQOR and MQPMR records
    pub StrucLength: MQLONG,
    /// Numeric encoding of data that follows the MQOR and MQPMR records
    pub Encoding: MQLONG,
    /// Character set identifier of data that follows the MQOR and MQPMR records
    pub CodedCharSetId: MQLONG,
    /// Format name of data that follows the MQOR and MQPMR records
    pub Format: MQCHAR8,
    /// General flags
    pub Flags: MQLONG,
    /// Flags indicating which MQPMR fields are present
    pub PutMsgRecFields: MQLONG,
    /// Number of MQOR records present
    pub RecsPresent: MQLONG,
    /// Offset of first MQOR record from start of MQDH
    pub ObjectRecOffset: MQLONG,
    /// Offset of first MQPMR record from start of MQDH
    pub PutMsgRecOffset: MQLONG,
}
/// Dead Letter Header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDLH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Reason message arrived on dead-letter (undelivered-message) queue
    pub Reason: MQLONG,
    /// Name of original destination queue
    pub DestQName: MQCHAR48,
    /// Name of original destination queue manager
    pub DestQMgrName: MQCHAR48,
    /// Numeric encoding of data that follows MQDLH
    pub Encoding: MQLONG,
    /// Character set identifier of data that follows MQDLH
    pub CodedCharSetId: MQLONG,
    /// Format name of data that follows MQDLH
    pub Format: MQCHAR8,
    /// Type of application that put message on dead-letter (undelivered-message) queue
    pub PutApplType: MQLONG,
    /// Name of application that put message on dead-letter (undelivered-message) queue
    pub PutApplName: MQCHAR28,
    /// Date when message was put on dead-letter (undelivered-message) queue
    pub PutDate: MQCHAR8,
    /// Time when message was put on dead-letter (undelivered-message) queue
    pub PutTime: MQCHAR8,
}
/// Delete Message Handle Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDMHO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQDLTMH
    pub Options: MQLONG,
}
/// Delete Message Property Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDMPO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQDLTMP
    pub Options: MQLONG,
}
/// Get Message Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQGMO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQGET
    pub Options: MQLONG,
    /// Wait interval
    pub WaitInterval: MQLONG,
    /// Signal
    pub Signal1: MQLONG,
    /// Signal identifier
    pub Signal2: MQLONG,
    /// Resolved name of destination queue
    pub ResolvedQName: MQCHAR48,
    /// Options controlling selection criteria used for MQGET
    pub MatchOptions: MQLONG,
    /// Flag indicating whether message retrieved is in a group
    pub GroupStatus: MQCHAR,
    /// Flag indicating whether message retrieved is a segment of a logical message
    pub SegmentStatus: MQCHAR,
    /// Flag indicating whether further segmentation is allowed for the message retrieved
    pub Segmentation: MQCHAR,
    /// Reserved
    pub Reserved1: MQCHAR,
    /// Message token
    pub MsgToken: MQBYTE16,
    /// Length of message data returned (bytes)
    pub ReturnedLength: MQLONG,
    /// Reserved
    pub Reserved2: MQLONG,
    /// Message handle
    pub MsgHandle: MQHMSG,
}
/// IMS Information Header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQIIH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQIIH structure
    pub StrucLength: MQLONG,
    /// Reserved
    pub Encoding: MQLONG,
    /// Reserved
    pub CodedCharSetId: MQLONG,
    /// MQ format name of data that follows MQIIH
    pub Format: MQCHAR8,
    /// Flags
    pub Flags: MQLONG,
    /// Logical terminal override
    pub LTermOverride: MQCHAR8,
    /// Message format services map name
    pub MFSMapName: MQCHAR8,
    /// MQ format name of reply message
    pub ReplyToFormat: MQCHAR8,
    /// RACF password or passticket
    pub Authenticator: MQCHAR8,
    /// Transaction instance identifier
    pub TranInstanceId: MQBYTE16,
    /// Transaction state
    pub TranState: MQCHAR,
    /// Commit mode
    pub CommitMode: MQCHAR,
    /// Security scope
    pub SecurityScope: MQCHAR,
    /// Reserved
    pub Reserved: MQCHAR,
}
/// Inquire Message Property Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQIMPO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQINQMP
    pub Options: MQLONG,
    /// Requested encoding of Value
    pub RequestedEncoding: MQLONG,
    /// Requested character set identifier of Value
    pub RequestedCCSID: MQLONG,
    /// Returned encoding of Value
    pub ReturnedEncoding: MQLONG,
    /// Returned character set identifier of Value
    pub ReturnedCCSID: MQLONG,
    /// Reserved
    pub Reserved1: MQLONG,
    /// Returned property name
    pub ReturnedName: MQCHARV,
    /// Property data type as a string
    pub TypeString: MQCHAR8,
}
/// Message Descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQMD {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options for report messages
    pub Report: MQLONG,
    /// Message type
    pub MsgType: MQLONG,
    /// Message lifetime
    pub Expiry: MQLONG,
    /// Feedback or reason code
    pub Feedback: MQLONG,
    /// Numeric encoding of message data
    pub Encoding: MQLONG,
    /// Character set identifier of message data
    pub CodedCharSetId: MQLONG,
    /// Format name of message data
    pub Format: MQCHAR8,
    /// Message priority
    pub Priority: MQLONG,
    /// Message persistence
    pub Persistence: MQLONG,
    /// Message identifier
    pub MsgId: MQBYTE24,
    /// Correlation identifier
    pub CorrelId: MQBYTE24,
    /// Backout counter
    pub BackoutCount: MQLONG,
    /// Name of reply queue
    pub ReplyToQ: MQCHAR48,
    /// Name of reply queue manager
    pub ReplyToQMgr: MQCHAR48,
    /// User identifier
    pub UserIdentifier: MQCHAR12,
    /// Accounting token
    pub AccountingToken: MQBYTE32,
    /// Application data relating to identity
    pub ApplIdentityData: MQCHAR32,
    /// Type of application that put the message
    pub PutApplType: MQLONG,
    /// Name of application that put the message
    pub PutApplName: MQCHAR28,
    /// Date when message was put
    pub PutDate: MQCHAR8,
    /// Time when message was put
    pub PutTime: MQCHAR8,
    /// Application data relating to origin
    pub ApplOriginData: MQCHAR4,
    /// Group identifier
    pub GroupId: MQBYTE24,
    /// Sequence number of logical message within group
    pub MsgSeqNumber: MQLONG,
    /// Offset of data in physical message from start of logical message
    pub Offset: MQLONG,
    /// Message flags
    pub MsgFlags: MQLONG,
    /// Length of original message
    pub OriginalLength: MQLONG,
}
/// Message Descriptor Extension
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQMDE {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQMDE structure
    pub StrucLength: MQLONG,
    /// Numeric encoding of data that follows MQMDE
    pub Encoding: MQLONG,
    /// Character-set identifier of data that follows MQMDE
    pub CodedCharSetId: MQLONG,
    /// Format name of data that follows MQMDE
    pub Format: MQCHAR8,
    /// General flags
    pub Flags: MQLONG,
    /// Group identifier
    pub GroupId: MQBYTE24,
    /// Sequence number of logical message within group
    pub MsgSeqNumber: MQLONG,
    /// Offset of data in physical message from start of logical message
    pub Offset: MQLONG,
    /// Message flags
    pub MsgFlags: MQLONG,
    /// Length of original message
    pub OriginalLength: MQLONG,
}
/// Version-1 Message Descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQMD1 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options for report messages
    pub Report: MQLONG,
    /// Message type
    pub MsgType: MQLONG,
    /// Message lifetime
    pub Expiry: MQLONG,
    /// Feedback or reason code
    pub Feedback: MQLONG,
    /// Numeric encoding of message data
    pub Encoding: MQLONG,
    /// Character set identifier of message data
    pub CodedCharSetId: MQLONG,
    /// Format name of message data
    pub Format: MQCHAR8,
    /// Message priority
    pub Priority: MQLONG,
    /// Message persistence
    pub Persistence: MQLONG,
    /// Message identifier
    pub MsgId: MQBYTE24,
    /// Correlation identifier
    pub CorrelId: MQBYTE24,
    /// Backout counter
    pub BackoutCount: MQLONG,
    /// Name of reply queue
    pub ReplyToQ: MQCHAR48,
    /// Name of reply queue manager
    pub ReplyToQMgr: MQCHAR48,
    /// User identifier
    pub UserIdentifier: MQCHAR12,
    /// Accounting token
    pub AccountingToken: MQBYTE32,
    /// Application data relating to identity
    pub ApplIdentityData: MQCHAR32,
    /// Type of application that put the message
    pub PutApplType: MQLONG,
    /// Name of application that put the message
    pub PutApplName: MQCHAR28,
    /// Date when message was put
    pub PutDate: MQCHAR8,
    /// Time when message was put
    pub PutTime: MQCHAR8,
    /// Application data relating to origin
    pub ApplOriginData: MQCHAR4,
}
/// Version-2 Message Descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQMD2 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options for report messages
    pub Report: MQLONG,
    /// Message type
    pub MsgType: MQLONG,
    /// Message lifetime
    pub Expiry: MQLONG,
    /// Feedback or reason code
    pub Feedback: MQLONG,
    /// Numeric encoding of message data
    pub Encoding: MQLONG,
    /// Character set identifier of message data
    pub CodedCharSetId: MQLONG,
    /// Format name of message data
    pub Format: MQCHAR8,
    /// Message priority
    pub Priority: MQLONG,
    /// Message persistence
    pub Persistence: MQLONG,
    /// Message identifier
    pub MsgId: MQBYTE24,
    /// Correlation identifier
    pub CorrelId: MQBYTE24,
    /// Backout counter
    pub BackoutCount: MQLONG,
    /// Name of reply queue
    pub ReplyToQ: MQCHAR48,
    /// Name of reply queue manager
    pub ReplyToQMgr: MQCHAR48,
    /// User identifier
    pub UserIdentifier: MQCHAR12,
    /// Accounting token
    pub AccountingToken: MQBYTE32,
    /// Application data relating to identity
    pub ApplIdentityData: MQCHAR32,
    /// Type of application that put the message
    pub PutApplType: MQLONG,
    /// Name of application that put the message
    pub PutApplName: MQCHAR28,
    /// Date when message was put
    pub PutDate: MQCHAR8,
    /// Time when message was put
    pub PutTime: MQCHAR8,
    /// Application data relating to origin
    pub ApplOriginData: MQCHAR4,
    /// Group identifier
    pub GroupId: MQBYTE24,
    /// Sequence number of logical message within group
    pub MsgSeqNumber: MQLONG,
    /// Offset of data in physical message from start of logical message
    pub Offset: MQLONG,
    /// Message flags
    pub MsgFlags: MQLONG,
    /// Length of original message
    pub OriginalLength: MQLONG,
}
/// Message Handle To Buffer Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQMHBO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQMHBUF
    pub Options: MQLONG,
}
/// Object descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQOD {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Object type
    pub ObjectType: MQLONG,
    /// Object name
    pub ObjectName: MQCHAR48,
    /// Object queue manager name
    pub ObjectQMgrName: MQCHAR48,
    /// Dynamic queue name
    pub DynamicQName: MQCHAR48,
    /// Alternate user identifier
    pub AlternateUserId: MQCHAR12,
    /// Number of object records present
    pub RecsPresent: MQLONG,
    /// Number of local queues opened successfully
    pub KnownDestCount: MQLONG,
    /// Number of remote queues opened
    pub UnknownDestCount: MQLONG,
    /// Number of queues that failed to open
    pub InvalidDestCount: MQLONG,
    /// Offset of first object record from start of MQOD
    pub ObjectRecOffset: MQLONG,
    /// Offset of first response record from start of MQOD
    pub ResponseRecOffset: MQLONG,
    /// Address of first object record
    pub ObjectRecPtr: MQPTR,
    /// Address of first response record
    pub ResponseRecPtr: MQPTR,
    /// Alternate security identifier
    pub AlternateSecurityId: MQBYTE40,
    /// Resolved queue name
    pub ResolvedQName: MQCHAR48,
    /// Resolved queue manager name
    pub ResolvedQMgrName: MQCHAR48,
    /// Object long name
    pub ObjectString: MQCHARV,
    /// Message Selector
    pub SelectionString: MQCHARV,
    /// Resolved long object name
    pub ResObjectString: MQCHARV,
    /// Alias queue resolved object type
    pub ResolvedType: MQLONG,
}
/// Object Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQOR {
    /// Object name
    pub ObjectName: MQCHAR48,
    /// Object queue manager name
    pub ObjectQMgrName: MQCHAR48,
}
/// Property descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQPD {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQSETMP and MQINQMP
    pub Options: MQLONG,
    /// Property support option
    pub Support: MQLONG,
    /// Property context
    pub Context: MQLONG,
    /// Property copy options
    pub CopyOptions: MQLONG,
}
/// Put Message Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQPMO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQPUT and MQPUT1
    pub Options: MQLONG,
    /// Reserved
    pub Timeout: MQLONG,
    /// Object handle of input queue
    pub Context: MQHOBJ,
    /// Number of messages sent successfully to local queues
    pub KnownDestCount: MQLONG,
    /// Number of messages sent successfully to remote queues
    pub UnknownDestCount: MQLONG,
    /// Number of messages that could not be sent
    pub InvalidDestCount: MQLONG,
    /// Resolved name of destination queue
    pub ResolvedQName: MQCHAR48,
    /// Resolved name of destination queue manager
    pub ResolvedQMgrName: MQCHAR48,
    /// Number of put message records or response records present
    pub RecsPresent: MQLONG,
    /// Flags indicating which MQPMR fields are present
    pub PutMsgRecFields: MQLONG,
    /// Offset of first put message record from start of MQPMO
    pub PutMsgRecOffset: MQLONG,
    /// Offset of first response record from start of MQPMO
    pub ResponseRecOffset: MQLONG,
    /// Address of first put message record
    pub PutMsgRecPtr: MQPTR,
    /// Address of first response record
    pub ResponseRecPtr: MQPTR,
    /// Original message handle
    pub OriginalMsgHandle: MQHMSG,
    /// New message handle
    pub NewMsgHandle: MQHMSG,
    /// The action being performed
    pub Action: MQLONG,
    /// Publication level
    pub PubLevel: MQLONG,
}
/// Rules and Formatting Header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQRFH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Total length of MQRFH including NameValueString
    pub StrucLength: MQLONG,
    /// Numeric encoding of data that follows NameValueString
    pub Encoding: MQLONG,
    /// Character set identifier of data that follows NameValueString
    pub CodedCharSetId: MQLONG,
    /// Format name of data that follows NameValueString
    pub Format: MQCHAR8,
    /// Flags
    pub Flags: MQLONG,
}
/// Rules and Formatting Header 2
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQRFH2 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Total length of MQRFH2 including all NameValueLength and NameValueData fields
    pub StrucLength: MQLONG,
    /// Numeric encoding of data that follows last NameValueData field
    pub Encoding: MQLONG,
    /// Character set identifier of data that follows last NameValueData field
    pub CodedCharSetId: MQLONG,
    /// Format name of data that follows last NameValueData field
    pub Format: MQCHAR8,
    /// Flags
    pub Flags: MQLONG,
    /// Character set identifier of NameValueData
    pub NameValueCCSID: MQLONG,
}
/// Reference Message Header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQRMH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Total length of MQRMH, including strings at end of fixed fields, but not the bulk data
    pub StrucLength: MQLONG,
    /// Numeric encoding of bulk data
    pub Encoding: MQLONG,
    /// Character set identifier of bulk data
    pub CodedCharSetId: MQLONG,
    /// Format name of bulk data
    pub Format: MQCHAR8,
    /// Reference message flags
    pub Flags: MQLONG,
    /// Object type
    pub ObjectType: MQCHAR8,
    /// Object instance identifier
    pub ObjectInstanceId: MQBYTE24,
    /// Length of source environment data
    pub SrcEnvLength: MQLONG,
    /// Offset of source environment data
    pub SrcEnvOffset: MQLONG,
    /// Length of source object name
    pub SrcNameLength: MQLONG,
    /// Offset of source object name
    pub SrcNameOffset: MQLONG,
    /// Length of destination environment data
    pub DestEnvLength: MQLONG,
    /// Offset of destination environment
    pub DestEnvOffset: MQLONG,
    /// Length of destination object name
    pub DestNameLength: MQLONG,
    /// Offset of destination object name
    pub DestNameOffset: MQLONG,
    /// Length of bulk data
    pub DataLogicalLength: MQLONG,
    /// Low offset of bulk data
    pub DataLogicalOffset: MQLONG,
    /// High offset of bulk data
    pub DataLogicalOffset2: MQLONG,
}
/// Response Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQRR {
    /// Completion code for queue
    pub CompCode: MQLONG,
    /// Reason code for queue
    pub Reason: MQLONG,
}
/// Subscription Descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQSD {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options associated with subscribing
    pub Options: MQLONG,
    /// Object name
    pub ObjectName: MQCHAR48,
    /// Alternate user identifier
    pub AlternateUserId: MQCHAR12,
    /// Alternate security identifier
    pub AlternateSecurityId: MQBYTE40,
    /// Expiry of Subscription
    pub SubExpiry: MQLONG,
    /// Object long name
    pub ObjectString: MQCHARV,
    /// Subscription name
    pub SubName: MQCHARV,
    /// Subscription user data
    pub SubUserData: MQCHARV,
    /// Correlation Id related to this subscription
    pub SubCorrelId: MQBYTE24,
    /// Priority set in publications
    pub PubPriority: MQLONG,
    /// Accounting Token set in publications
    pub PubAccountingToken: MQBYTE32,
    /// Appl Identity Data set in publications
    pub PubApplIdentityData: MQCHAR32,
    /// Message selector structure
    pub SelectionString: MQCHARV,
    /// Subscription level
    pub SubLevel: MQLONG,
    /// Resolved long object name
    pub ResObjectString: MQCHARV,
}
/// Set Message Property Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQSMPO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQSETMP
    pub Options: MQLONG,
    /// Encoding of Value
    pub ValueEncoding: MQLONG,
    /// Character set identifier of Value
    pub ValueCCSID: MQLONG,
}
/// Subscription Request Options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQSRO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQSUBRQ
    pub Options: MQLONG,
    /// Number of publications sent
    pub NumPubs: MQLONG,
}
/// Status Information Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQSTS {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Completion Code of first error
    pub CompCode: MQLONG,
    /// Reason Code of first error
    pub Reason: MQLONG,
    /// Number of Async put calls succeeded
    pub PutSuccessCount: MQLONG,
    /// Number of Async put calls had warnings
    pub PutWarningCount: MQLONG,
    /// Number of Async put calls had failures
    pub PutFailureCount: MQLONG,
    /// Failing object type
    pub ObjectType: MQLONG,
    /// Failing object name
    pub ObjectName: MQCHAR48,
    /// Failing object queue manager
    pub ObjectQMgrName: MQCHAR48,
    /// Resolved name of destination queue
    pub ResolvedObjectName: MQCHAR48,
    /// Resolved name of destination qmgr
    pub ResolvedQMgrName: MQCHAR48,
    /// Failing object long name
    pub ObjectString: MQCHARV,
    /// Failing subscription name
    pub SubName: MQCHARV,
    /// Failing open options
    pub OpenOptions: MQLONG,
    /// Failing subscription options
    pub SubOptions: MQLONG,
}
/// Trigger Message
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQTM {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Name of triggered queue
    pub QName: MQCHAR48,
    /// Name of process object
    pub ProcessName: MQCHAR48,
    /// Trigger data
    pub TriggerData: MQCHAR64,
    /// Application type
    pub ApplType: MQLONG,
    /// Application identifier
    pub ApplId: MQCHAR256,
    /// Environment data
    pub EnvData: MQCHAR128,
    /// User data
    pub UserData: MQCHAR128,
}
/// Trigger Message 2 (Character)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQTMC2 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQCHAR4,
    /// Name of triggered queue
    pub QName: MQCHAR48,
    /// Name of process object
    pub ProcessName: MQCHAR48,
    /// Trigger data
    pub TriggerData: MQCHAR64,
    /// Application type
    pub ApplType: MQCHAR4,
    /// Application identifier
    pub ApplId: MQCHAR256,
    /// Environment data
    pub EnvData: MQCHAR128,
    /// User data
    pub UserData: MQCHAR128,
    /// Queue manager name
    pub QMgrName: MQCHAR48,
}
/// Work Information Header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWIH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWIH structure
    pub StrucLength: MQLONG,
    /// Numeric encoding of data that follows MQWIH
    pub Encoding: MQLONG,
    /// Character-set identifier of data that follows MQWIH
    pub CodedCharSetId: MQLONG,
    /// Format name of data that follows MQWIH
    pub Format: MQCHAR8,
    /// Flags
    pub Flags: MQLONG,
    /// Service name
    pub ServiceName: MQCHAR32,
    /// Service step name
    pub ServiceStep: MQCHAR8,
    /// Message token
    pub MsgToken: MQBYTE16,
    /// Reserved
    pub Reserved: MQCHAR32,
}
/// Transmission Queue Header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQXQH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Name of destination queue
    pub RemoteQName: MQCHAR48,
    /// Name of destination queue manager
    pub RemoteQMgrName: MQCHAR48,
    /// Original message descriptor
    pub MsgDesc: MQMD1,
}
pub const MQAIR_STRUC_ID: &::std::ffi::CStr = c"AIR ";
pub const MQAIR_VERSION_1: MQLONG = 1;
pub const MQAIR_VERSION_2: MQLONG = 2;
pub const MQAIR_CURRENT_VERSION: MQLONG = 2;
pub const MQAIR_LENGTH_1: usize = 328;
pub const MQAIR_LENGTH_2: usize = 584;
pub const MQAIR_CURRENT_LENGTH: usize = 584;
pub const MQAIT_ALL: MQLONG = 0;
pub const MQAIT_CRL_LDAP: MQLONG = 1;
pub const MQAIT_OCSP: MQLONG = 2;
pub const MQAIT_IDPW_OS: MQLONG = 3;
pub const MQAIT_IDPW_LDAP: MQLONG = 4;
pub const MQBNO_STRUC_ID: &::std::ffi::CStr = c"BNO ";
pub const MQBNO_VERSION_1: MQLONG = 1;
pub const MQBNO_CURRENT_VERSION: MQLONG = 1;
pub const MQBNO_LENGTH_1: usize = 20;
pub const MQBNO_CURRENT_LENGTH: usize = 20;
pub const MQBNO_OPTIONS_NONE: MQLONG = 0;
pub const MQBNO_OPTIONS_IGNORE_TRANS: MQLONG = 1;
pub const MQBNO_BALTYPE_SIMPLE: MQLONG = 0;
pub const MQBNO_BALTYPE_REQREP: MQLONG = 1;
pub const MQBNO_BALTYPE_RA_MANAGED: MQLONG = 65536;
pub const MQBNO_TIMEOUT_AS_DEFAULT: MQLONG = -1;
pub const MQBNO_TIMEOUT_IMMEDIATE: MQLONG = 0;
pub const MQBNO_TIMEOUT_NEVER: MQLONG = -2;
pub const MQBMHO_STRUC_ID: &::std::ffi::CStr = c"BMHO";
pub const MQBMHO_VERSION_1: MQLONG = 1;
pub const MQBMHO_CURRENT_VERSION: MQLONG = 1;
pub const MQBMHO_LENGTH_1: usize = 12;
pub const MQBMHO_CURRENT_LENGTH: usize = 12;
pub const MQBMHO_NONE: MQLONG = 0;
pub const MQBMHO_DELETE_PROPERTIES: MQLONG = 1;
pub const MQBO_STRUC_ID: &::std::ffi::CStr = c"BO  ";
pub const MQBO_VERSION_1: MQLONG = 1;
pub const MQBO_CURRENT_VERSION: MQLONG = 1;
pub const MQBO_LENGTH_1: usize = 12;
pub const MQBO_CURRENT_LENGTH: usize = 12;
pub const MQBO_NONE: MQLONG = 0;
pub const MQCBC_STRUC_ID: &::std::ffi::CStr = c"CBC ";
pub const MQCBC_VERSION_1: MQLONG = 1;
pub const MQCBC_VERSION_2: MQLONG = 2;
pub const MQCBC_CURRENT_VERSION: MQLONG = 2;
pub const MQCBC_LENGTH_1: usize = 56;
pub const MQCBC_LENGTH_2: usize = 64;
pub const MQCBC_CURRENT_LENGTH: usize = 64;
pub const MQCBCF_NONE: MQLONG = 0;
pub const MQCBCF_READA_BUFFER_EMPTY: MQLONG = 1;
pub const MQCBCT_START_CALL: MQLONG = 1;
pub const MQCBCT_STOP_CALL: MQLONG = 2;
pub const MQCBCT_REGISTER_CALL: MQLONG = 3;
pub const MQCBCT_DEREGISTER_CALL: MQLONG = 4;
pub const MQCBCT_EVENT_CALL: MQLONG = 5;
pub const MQCBCT_MSG_REMOVED: MQLONG = 6;
pub const MQCBCT_MSG_NOT_REMOVED: MQLONG = 7;
pub const MQCBCT_MC_EVENT_CALL: MQLONG = 8;
pub const MQCS_NONE: MQLONG = 0;
pub const MQCS_SUSPENDED_TEMPORARY: MQLONG = 1;
pub const MQCS_SUSPENDED_USER_ACTION: MQLONG = 2;
pub const MQCS_SUSPENDED: MQLONG = 3;
pub const MQCS_STOPPED: MQLONG = 4;
pub const MQRD_NO_RECONNECT: MQLONG = -1;
pub const MQRD_NO_DELAY: MQLONG = 0;
pub const MQCBD_STRUC_ID: &::std::ffi::CStr = c"CBD ";
pub const MQCBD_VERSION_1: MQLONG = 1;
pub const MQCBD_CURRENT_VERSION: MQLONG = 1;
pub const MQCBD_LENGTH_1: usize = 168;
pub const MQCBD_CURRENT_LENGTH: usize = 168;
pub const MQCBDO_NONE: MQLONG = 0;
pub const MQCBDO_START_CALL: MQLONG = 1;
pub const MQCBDO_STOP_CALL: MQLONG = 4;
pub const MQCBDO_REGISTER_CALL: MQLONG = 256;
pub const MQCBDO_DEREGISTER_CALL: MQLONG = 512;
pub const MQCBDO_FAIL_IF_QUIESCING: MQLONG = 8192;
pub const MQCBDO_EVENT_CALL: MQLONG = 16384;
pub const MQCBDO_MC_EVENT_CALL: MQLONG = 32768;
pub const MQCBT_MESSAGE_CONSUMER: MQLONG = 1;
pub const MQCBT_EVENT_HANDLER: MQLONG = 2;
pub const MQCBD_FULL_MSG_LENGTH: MQLONG = -1;
pub const MQVS_NULL_TERMINATED: MQLONG = -1;
pub const MQCIH_STRUC_ID: &::std::ffi::CStr = c"CIH ";
pub const MQCIH_VERSION_1: MQLONG = 1;
pub const MQCIH_VERSION_2: MQLONG = 2;
pub const MQCIH_CURRENT_VERSION: MQLONG = 2;
pub const MQCIH_LENGTH_1: usize = 164;
pub const MQCIH_LENGTH_2: usize = 180;
pub const MQCIH_CURRENT_LENGTH: usize = 180;
pub const MQCIH_NONE: MQLONG = 0;
pub const MQCIH_PASS_EXPIRATION: MQLONG = 1;
pub const MQCIH_UNLIMITED_EXPIRATION: MQLONG = 0;
pub const MQCIH_REPLY_WITHOUT_NULLS: MQLONG = 2;
pub const MQCIH_REPLY_WITH_NULLS: MQLONG = 0;
pub const MQCIH_SYNC_ON_RETURN: MQLONG = 4;
pub const MQCIH_NO_SYNC_ON_RETURN: MQLONG = 0;
pub const MQCRC_OK: MQLONG = 0;
pub const MQCRC_CICS_EXEC_ERROR: MQLONG = 1;
pub const MQCRC_MQ_API_ERROR: MQLONG = 2;
pub const MQCRC_BRIDGE_ERROR: MQLONG = 3;
pub const MQCRC_BRIDGE_ABEND: MQLONG = 4;
pub const MQCRC_APPLICATION_ABEND: MQLONG = 5;
pub const MQCRC_SECURITY_ERROR: MQLONG = 6;
pub const MQCRC_PROGRAM_NOT_AVAILABLE: MQLONG = 7;
pub const MQCRC_BRIDGE_TIMEOUT: MQLONG = 8;
pub const MQCRC_TRANSID_NOT_AVAILABLE: MQLONG = 9;
pub const MQCUOWC_ONLY: MQLONG = 273;
pub const MQCUOWC_CONTINUE: MQLONG = 65536;
pub const MQCUOWC_FIRST: MQLONG = 17;
pub const MQCUOWC_MIDDLE: MQLONG = 16;
pub const MQCUOWC_LAST: MQLONG = 272;
pub const MQCUOWC_COMMIT: MQLONG = 256;
pub const MQCUOWC_BACKOUT: MQLONG = 4352;
pub const MQCGWI_DEFAULT: MQLONG = -2;
pub const MQCLT_PROGRAM: MQLONG = 1;
pub const MQCLT_TRANSACTION: MQLONG = 2;
pub const MQCODL_AS_INPUT: MQLONG = -1;
pub const MQCADSD_NONE: MQLONG = 0;
pub const MQCADSD_SEND: MQLONG = 1;
pub const MQCADSD_RECV: MQLONG = 16;
pub const MQCADSD_MSGFORMAT: MQLONG = 256;
pub const MQCCT_YES: MQLONG = 1;
pub const MQCCT_NO: MQLONG = 0;
pub const MQCTES_NOSYNC: MQLONG = 0;
pub const MQCTES_COMMIT: MQLONG = 256;
pub const MQCTES_BACKOUT: MQLONG = 4352;
pub const MQCTES_ENDTASK: MQLONG = 65536;
pub const MQCFAC_NONE: &[u8; 9] = b"\0\0\0\0\0\0\0\0\0";
pub const MQCFUNC_MQCONN: &::std::ffi::CStr = c"CONN";
pub const MQCFUNC_MQGET: &::std::ffi::CStr = c"GET ";
pub const MQCFUNC_MQINQ: &::std::ffi::CStr = c"INQ ";
pub const MQCFUNC_MQOPEN: &::std::ffi::CStr = c"OPEN";
pub const MQCFUNC_MQPUT: &::std::ffi::CStr = c"PUT ";
pub const MQCFUNC_MQPUT1: &::std::ffi::CStr = c"PUT1";
pub const MQCFUNC_NONE: &::std::ffi::CStr = c"    ";
pub const MQCSC_START: &::std::ffi::CStr = c"S   ";
pub const MQCSC_STARTDATA: &::std::ffi::CStr = c"SD  ";
pub const MQCSC_TERMINPUT: &::std::ffi::CStr = c"TD  ";
pub const MQCSC_NONE: &::std::ffi::CStr = c"    ";
pub const MQCMHO_STRUC_ID: &::std::ffi::CStr = c"CMHO";
pub const MQCMHO_VERSION_1: MQLONG = 1;
pub const MQCMHO_CURRENT_VERSION: MQLONG = 1;
pub const MQCMHO_LENGTH_1: usize = 12;
pub const MQCMHO_CURRENT_LENGTH: usize = 12;
pub const MQCMHO_DEFAULT_VALIDATION: MQLONG = 0;
pub const MQCMHO_NO_VALIDATION: MQLONG = 1;
pub const MQCMHO_VALIDATE: MQLONG = 2;
pub const MQCMHO_NONE: MQLONG = 0;
pub const MQCTLO_STRUC_ID: &::std::ffi::CStr = c"CTLO";
pub const MQCTLO_VERSION_1: MQLONG = 1;
pub const MQCTLO_CURRENT_VERSION: MQLONG = 1;
pub const MQCTLO_LENGTH_1: usize = 24;
pub const MQCTLO_CURRENT_LENGTH: usize = 24;
pub const MQCTLO_NONE: MQLONG = 0;
pub const MQCTLO_THREAD_AFFINITY: MQLONG = 1;
pub const MQCTLO_FAIL_IF_QUIESCING: MQLONG = 8192;
pub const MQSCO_STRUC_ID: &::std::ffi::CStr = c"SCO ";
pub const MQSCO_VERSION_1: MQLONG = 1;
pub const MQSCO_VERSION_2: MQLONG = 2;
pub const MQSCO_VERSION_3: MQLONG = 3;
pub const MQSCO_VERSION_4: MQLONG = 4;
pub const MQSCO_VERSION_5: MQLONG = 5;
pub const MQSCO_VERSION_6: MQLONG = 6;
pub const MQSCO_VERSION_7: MQLONG = 7;
pub const MQSCO_CURRENT_VERSION: MQLONG = 7;
pub const MQSCO_LENGTH_1: usize = 536;
pub const MQSCO_LENGTH_2: usize = 544;
pub const MQSCO_LENGTH_3: usize = 560;
pub const MQSCO_LENGTH_4: usize = 568;
pub const MQSCO_LENGTH_5: usize = 632;
pub const MQSCO_LENGTH_6: usize = 648;
pub const MQSCO_LENGTH_7: usize = 672;
pub const MQSCO_CURRENT_LENGTH: usize = 672;
pub const MQ_SUITE_B_NOT_AVAILABLE: MQLONG = 0;
pub const MQ_SUITE_B_NONE: MQLONG = 1;
pub const MQ_SUITE_B_128_BIT: MQLONG = 2;
pub const MQ_SUITE_B_192_BIT: MQLONG = 4;
pub const MQSCO_RESET_COUNT_DEFAULT: MQLONG = 0;
pub const MQ_CERT_VAL_POLICY_DEFAULT: MQLONG = 0;
pub const MQ_CERT_VAL_POLICY_ANY: MQLONG = 0;
pub const MQ_CERT_VAL_POLICY_RFC5280: MQLONG = 1;
pub const MQ_CERT_VAL_POLICY_NONE: MQLONG = 2;
pub const MQ_HTTPSCERTVAL_DEFAULT: MQLONG = 0;
pub const MQ_HTTPSCERTVAL_ANY: MQLONG = 1;
pub const MQ_HTTPSCERTVAL_NONE: MQLONG = 2;
pub const MQ_HTTPSCERTVAL_HOSTNAMECN: MQLONG = 3;
pub const MQ_HTTPSCERTREV_DEFAULT: MQLONG = 0;
pub const MQ_HTTPSCERTREV_REQUIRED: MQLONG = 1;
pub const MQ_HTTPSCERTREV_DISABLED: MQLONG = 2;
pub const MQ_HTTPSCERTREV_OPTIONAL: MQLONG = 3;
pub const MQCSP_STRUC_ID: &::std::ffi::CStr = c"CSP ";
pub const MQCSP_VERSION_1: MQLONG = 1;
pub const MQCSP_VERSION_2: MQLONG = 2;
pub const MQCSP_VERSION_3: MQLONG = 3;
pub const MQCSP_CURRENT_VERSION: MQLONG = 3;
pub const MQCSP_LENGTH_1: usize = 56;
pub const MQCSP_LENGTH_2: usize = 80;
pub const MQCSP_LENGTH_3: usize = 104;
pub const MQCSP_CURRENT_LENGTH: usize = 104;
pub const MQCSP_AUTH_NONE: MQLONG = 0;
pub const MQCSP_AUTH_USER_ID_AND_PWD: MQLONG = 1;
pub const MQCSP_AUTH_ID_TOKEN: MQLONG = 2;
pub const MQCNO_STRUC_ID: &::std::ffi::CStr = c"CNO ";
pub const MQCNO_VERSION_1: MQLONG = 1;
pub const MQCNO_VERSION_2: MQLONG = 2;
pub const MQCNO_VERSION_3: MQLONG = 3;
pub const MQCNO_VERSION_4: MQLONG = 4;
pub const MQCNO_VERSION_5: MQLONG = 5;
pub const MQCNO_VERSION_6: MQLONG = 6;
pub const MQCNO_VERSION_7: MQLONG = 7;
pub const MQCNO_VERSION_8: MQLONG = 8;
pub const MQCNO_CURRENT_VERSION: MQLONG = 8;
pub const MQCNO_LENGTH_1: usize = 12;
pub const MQCNO_LENGTH_2: usize = 24;
pub const MQCNO_LENGTH_3: usize = 152;
pub const MQCNO_LENGTH_4: usize = 168;
pub const MQCNO_LENGTH_5: usize = 200;
pub const MQCNO_LENGTH_6: usize = 224;
pub const MQCNO_LENGTH_7: usize = 256;
pub const MQCNO_LENGTH_8: usize = 272;
pub const MQCNO_CURRENT_LENGTH: usize = 272;
pub const MQCNO_STANDARD_BINDING: MQLONG = 0;
pub const MQCNO_FASTPATH_BINDING: MQLONG = 1;
pub const MQCNO_SERIALIZE_CONN_TAG_Q_MGR: MQLONG = 2;
pub const MQCNO_SERIALIZE_CONN_TAG_QSG: MQLONG = 4;
pub const MQCNO_RESTRICT_CONN_TAG_Q_MGR: MQLONG = 8;
pub const MQCNO_RESTRICT_CONN_TAG_QSG: MQLONG = 16;
pub const MQCNO_HANDLE_SHARE_NONE: MQLONG = 32;
pub const MQCNO_HANDLE_SHARE_BLOCK: MQLONG = 64;
pub const MQCNO_HANDLE_SHARE_NO_BLOCK: MQLONG = 128;
pub const MQCNO_SHARED_BINDING: MQLONG = 256;
pub const MQCNO_ISOLATED_BINDING: MQLONG = 512;
pub const MQCNO_LOCAL_BINDING: MQLONG = 1024;
pub const MQCNO_CLIENT_BINDING: MQLONG = 2048;
pub const MQCNO_ACCOUNTING_MQI_ENABLED: MQLONG = 4096;
pub const MQCNO_ACCOUNTING_MQI_DISABLED: MQLONG = 8192;
pub const MQCNO_ACCOUNTING_Q_ENABLED: MQLONG = 16384;
pub const MQCNO_ACCOUNTING_Q_DISABLED: MQLONG = 32768;
pub const MQCNO_NO_CONV_SHARING: MQLONG = 65536;
pub const MQCNO_ALL_CONVS_SHARE: MQLONG = 262144;
pub const MQCNO_CD_FOR_OUTPUT_ONLY: MQLONG = 524288;
pub const MQCNO_USE_CD_SELECTION: MQLONG = 1048576;
pub const MQCNO_GENERATE_CONN_TAG: MQLONG = 2097152;
pub const MQCNO_RECONNECT_AS_DEF: MQLONG = 0;
pub const MQCNO_RECONNECT: MQLONG = 16777216;
pub const MQCNO_RECONNECT_DISABLED: MQLONG = 33554432;
pub const MQCNO_RECONNECT_Q_MGR: MQLONG = 67108864;
pub const MQCNO_ACTIVITY_TRACE_ENABLED: MQLONG = 134217728;
pub const MQCNO_ACTIVITY_TRACE_DISABLED: MQLONG = 268435456;
pub const MQCNO_NONE: MQLONG = 0;
pub const MQCT_NONE: &[u8; 129] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQCONNID_NONE: &[u8; 25] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQAN_NONE: &::std::ffi::CStr = c"                            ";
pub const MQDH_STRUC_ID: &::std::ffi::CStr = c"DH  ";
pub const MQDH_VERSION_1: MQLONG = 1;
pub const MQDH_CURRENT_VERSION: MQLONG = 1;
pub const MQDH_LENGTH_1: usize = 48;
pub const MQDH_CURRENT_LENGTH: usize = 48;
pub const MQDHF_NEW_MSG_IDS: MQLONG = 1;
pub const MQDHF_NONE: MQLONG = 0;
pub const MQDLH_STRUC_ID: &::std::ffi::CStr = c"DLH ";
pub const MQDLH_VERSION_1: MQLONG = 1;
pub const MQDLH_CURRENT_VERSION: MQLONG = 1;
pub const MQDLH_LENGTH_1: usize = 172;
pub const MQDLH_CURRENT_LENGTH: usize = 172;
pub const MQDMHO_STRUC_ID: &::std::ffi::CStr = c"DMHO";
pub const MQDMHO_VERSION_1: MQLONG = 1;
pub const MQDMHO_CURRENT_VERSION: MQLONG = 1;
pub const MQDMHO_LENGTH_1: usize = 12;
pub const MQDMHO_CURRENT_LENGTH: usize = 12;
pub const MQDMHO_NONE: MQLONG = 0;
pub const MQDMPO_STRUC_ID: &::std::ffi::CStr = c"DMPO";
pub const MQDMPO_VERSION_1: MQLONG = 1;
pub const MQDMPO_CURRENT_VERSION: MQLONG = 1;
pub const MQDMPO_LENGTH_1: usize = 12;
pub const MQDMPO_CURRENT_LENGTH: usize = 12;
pub const MQDMPO_DEL_FIRST: MQLONG = 0;
pub const MQDMPO_DEL_PROP_UNDER_CURSOR: MQLONG = 1;
pub const MQDMPO_NONE: MQLONG = 0;
pub const MQGMO_STRUC_ID: &::std::ffi::CStr = c"GMO ";
pub const MQGMO_VERSION_1: MQLONG = 1;
pub const MQGMO_VERSION_2: MQLONG = 2;
pub const MQGMO_VERSION_3: MQLONG = 3;
pub const MQGMO_VERSION_4: MQLONG = 4;
pub const MQGMO_CURRENT_VERSION: MQLONG = 4;
pub const MQGMO_LENGTH_1: usize = 72;
pub const MQGMO_LENGTH_2: usize = 80;
pub const MQGMO_LENGTH_3: usize = 100;
pub const MQGMO_LENGTH_4: usize = 112;
pub const MQGMO_CURRENT_LENGTH: usize = 112;
pub const MQGMO_WAIT: MQLONG = 1;
pub const MQGMO_NO_WAIT: MQLONG = 0;
pub const MQGMO_SET_SIGNAL: MQLONG = 8;
pub const MQGMO_FAIL_IF_QUIESCING: MQLONG = 8192;
pub const MQGMO_SYNCPOINT: MQLONG = 2;
pub const MQGMO_SYNCPOINT_IF_PERSISTENT: MQLONG = 4096;
pub const MQGMO_NO_SYNCPOINT: MQLONG = 4;
pub const MQGMO_MARK_SKIP_BACKOUT: MQLONG = 128;
pub const MQGMO_BROWSE_FIRST: MQLONG = 16;
pub const MQGMO_BROWSE_NEXT: MQLONG = 32;
pub const MQGMO_BROWSE_MSG_UNDER_CURSOR: MQLONG = 2048;
pub const MQGMO_MSG_UNDER_CURSOR: MQLONG = 256;
pub const MQGMO_LOCK: MQLONG = 512;
pub const MQGMO_UNLOCK: MQLONG = 1024;
pub const MQGMO_ACCEPT_TRUNCATED_MSG: MQLONG = 64;
pub const MQGMO_CONVERT: MQLONG = 16384;
pub const MQGMO_LOGICAL_ORDER: MQLONG = 32768;
pub const MQGMO_COMPLETE_MSG: MQLONG = 65536;
pub const MQGMO_ALL_MSGS_AVAILABLE: MQLONG = 131072;
pub const MQGMO_ALL_SEGMENTS_AVAILABLE: MQLONG = 262144;
pub const MQGMO_MARK_BROWSE_HANDLE: MQLONG = 1048576;
pub const MQGMO_MARK_BROWSE_CO_OP: MQLONG = 2097152;
pub const MQGMO_UNMARK_BROWSE_CO_OP: MQLONG = 4194304;
pub const MQGMO_UNMARK_BROWSE_HANDLE: MQLONG = 8388608;
pub const MQGMO_UNMARKED_BROWSE_MSG: MQLONG = 16777216;
pub const MQGMO_PROPERTIES_FORCE_MQRFH2: MQLONG = 33554432;
pub const MQGMO_NO_PROPERTIES: MQLONG = 67108864;
pub const MQGMO_PROPERTIES_IN_HANDLE: MQLONG = 134217728;
pub const MQGMO_PROPERTIES_COMPATIBILITY: MQLONG = 268435456;
pub const MQGMO_PROPERTIES_AS_Q_DEF: MQLONG = 0;
pub const MQGMO_NONE: MQLONG = 0;
pub const MQGMO_BROWSE_HANDLE: MQLONG = 17825808;
pub const MQGMO_BROWSE_CO_OP: MQLONG = 18874384;
pub const MQWI_UNLIMITED: MQLONG = -1;
pub const MQEC_MSG_ARRIVED: MQLONG = 2;
pub const MQEC_WAIT_INTERVAL_EXPIRED: MQLONG = 3;
pub const MQEC_WAIT_CANCELED: MQLONG = 4;
pub const MQEC_Q_MGR_QUIESCING: MQLONG = 5;
pub const MQEC_CONNECTION_QUIESCING: MQLONG = 6;
pub const MQMO_MATCH_MSG_ID: MQLONG = 1;
pub const MQMO_MATCH_CORREL_ID: MQLONG = 2;
pub const MQMO_MATCH_GROUP_ID: MQLONG = 4;
pub const MQMO_MATCH_MSG_SEQ_NUMBER: MQLONG = 8;
pub const MQMO_MATCH_OFFSET: MQLONG = 16;
pub const MQMO_MATCH_MSG_TOKEN: MQLONG = 32;
pub const MQMO_NONE: MQLONG = 0;
pub const MQGS_NOT_IN_GROUP: u8 = 32u8;
pub const MQGS_MSG_IN_GROUP: u8 = 71u8;
pub const MQGS_LAST_MSG_IN_GROUP: u8 = 76u8;
pub const MQSS_NOT_A_SEGMENT: u8 = 32u8;
pub const MQSS_SEGMENT: u8 = 83u8;
pub const MQSS_LAST_SEGMENT: u8 = 76u8;
pub const MQSEG_INHIBITED: u8 = 32u8;
pub const MQSEG_ALLOWED: u8 = 65u8;
pub const MQMTOK_NONE: &[u8; 17] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQRL_UNDEFINED: MQLONG = -1;
pub const MQIIH_STRUC_ID: &::std::ffi::CStr = c"IIH ";
pub const MQIIH_VERSION_1: MQLONG = 1;
pub const MQIIH_CURRENT_VERSION: MQLONG = 1;
pub const MQIIH_LENGTH_1: usize = 84;
pub const MQIIH_CURRENT_LENGTH: usize = 84;
pub const MQIIH_NONE: MQLONG = 0;
pub const MQIIH_PASS_EXPIRATION: MQLONG = 1;
pub const MQIIH_UNLIMITED_EXPIRATION: MQLONG = 0;
pub const MQIIH_REPLY_FORMAT_NONE: MQLONG = 8;
pub const MQIIH_IGNORE_PURG: MQLONG = 16;
pub const MQIIH_CM0_REQUEST_RESPONSE: MQLONG = 32;
pub const MQIAUT_NONE: &::std::ffi::CStr = c"        ";
pub const MQITII_NONE: &[u8; 17] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQITS_IN_CONVERSATION: u8 = 67u8;
pub const MQITS_NOT_IN_CONVERSATION: u8 = 32u8;
pub const MQITS_ARCHITECTED: u8 = 65u8;
pub const MQICM_COMMIT_THEN_SEND: u8 = 48u8;
pub const MQICM_SEND_THEN_COMMIT: u8 = 49u8;
pub const MQISS_CHECK: u8 = 67u8;
pub const MQISS_FULL: u8 = 70u8;
pub const MQIMPO_STRUC_ID: &::std::ffi::CStr = c"IMPO";
pub const MQIMPO_VERSION_1: MQLONG = 1;
pub const MQIMPO_CURRENT_VERSION: MQLONG = 1;
pub const MQIMPO_LENGTH_1: usize = 64;
pub const MQIMPO_CURRENT_LENGTH: usize = 64;
pub const MQIMPO_CONVERT_TYPE: MQLONG = 2;
pub const MQIMPO_QUERY_LENGTH: MQLONG = 4;
pub const MQIMPO_INQ_FIRST: MQLONG = 0;
pub const MQIMPO_INQ_NEXT: MQLONG = 8;
pub const MQIMPO_INQ_PROP_UNDER_CURSOR: MQLONG = 16;
pub const MQIMPO_CONVERT_VALUE: MQLONG = 32;
pub const MQIMPO_NONE: MQLONG = 0;
pub const MQMD_STRUC_ID: &::std::ffi::CStr = c"MD  ";
pub const MQMD_VERSION_1: MQLONG = 1;
pub const MQMD_VERSION_2: MQLONG = 2;
pub const MQMD_CURRENT_VERSION: MQLONG = 2;
pub const MQMD_LENGTH_1: usize = 324;
pub const MQMD_LENGTH_2: usize = 364;
pub const MQMD_CURRENT_LENGTH: usize = 364;
pub const MQRO_EXCEPTION: MQLONG = 16777216;
pub const MQRO_EXCEPTION_WITH_DATA: MQLONG = 50331648;
pub const MQRO_EXCEPTION_WITH_FULL_DATA: MQLONG = 117440512;
pub const MQRO_EXPIRATION: MQLONG = 2097152;
pub const MQRO_EXPIRATION_WITH_DATA: MQLONG = 6291456;
pub const MQRO_EXPIRATION_WITH_FULL_DATA: MQLONG = 14680064;
pub const MQRO_COA: MQLONG = 256;
pub const MQRO_COA_WITH_DATA: MQLONG = 768;
pub const MQRO_COA_WITH_FULL_DATA: MQLONG = 1792;
pub const MQRO_COD: MQLONG = 2048;
pub const MQRO_COD_WITH_DATA: MQLONG = 6144;
pub const MQRO_COD_WITH_FULL_DATA: MQLONG = 14336;
pub const MQRO_PAN: MQLONG = 1;
pub const MQRO_NAN: MQLONG = 2;
pub const MQRO_ACTIVITY: MQLONG = 4;
pub const MQRO_NEW_MSG_ID: MQLONG = 0;
pub const MQRO_PASS_MSG_ID: MQLONG = 128;
pub const MQRO_COPY_MSG_ID_TO_CORREL_ID: MQLONG = 0;
pub const MQRO_PASS_CORREL_ID: MQLONG = 64;
pub const MQRO_DEAD_LETTER_Q: MQLONG = 0;
pub const MQRO_DISCARD_MSG: MQLONG = 134217728;
pub const MQRO_PASS_DISCARD_AND_EXPIRY: MQLONG = 16384;
pub const MQRO_NONE: MQLONG = 0;
pub const MQRO_REJECT_UNSUP_MASK: MQLONG = 270270464;
pub const MQRO_ACCEPT_UNSUP_MASK: MQLONG = -270532353;
pub const MQRO_ACCEPT_UNSUP_IF_XMIT_MASK: MQLONG = 261888;
pub const MQMT_SYSTEM_FIRST: MQLONG = 1;
pub const MQMT_REQUEST: MQLONG = 1;
pub const MQMT_REPLY: MQLONG = 2;
pub const MQMT_DATAGRAM: MQLONG = 8;
pub const MQMT_REPORT: MQLONG = 4;
pub const MQMT_MQE_FIELDS_FROM_MQE: MQLONG = 112;
pub const MQMT_MQE_FIELDS: MQLONG = 113;
pub const MQMT_SYSTEM_LAST: MQLONG = 65535;
pub const MQMT_APPL_FIRST: MQLONG = 65536;
pub const MQMT_APPL_LAST: MQLONG = 999999999;
pub const MQEI_UNLIMITED: MQLONG = -1;
pub const MQFB_NONE: MQLONG = 0;
pub const MQFB_SYSTEM_FIRST: MQLONG = 1;
pub const MQFB_QUIT: MQLONG = 256;
pub const MQFB_EXPIRATION: MQLONG = 258;
pub const MQFB_COA: MQLONG = 259;
pub const MQFB_COD: MQLONG = 260;
pub const MQFB_CHANNEL_COMPLETED: MQLONG = 262;
pub const MQFB_CHANNEL_FAIL_RETRY: MQLONG = 263;
pub const MQFB_CHANNEL_FAIL: MQLONG = 264;
pub const MQFB_APPL_CANNOT_BE_STARTED: MQLONG = 265;
pub const MQFB_TM_ERROR: MQLONG = 266;
pub const MQFB_APPL_TYPE_ERROR: MQLONG = 267;
pub const MQFB_STOPPED_BY_MSG_EXIT: MQLONG = 268;
pub const MQFB_ACTIVITY: MQLONG = 269;
pub const MQFB_XMIT_Q_MSG_ERROR: MQLONG = 271;
pub const MQFB_PAN: MQLONG = 275;
pub const MQFB_NAN: MQLONG = 276;
pub const MQFB_STOPPED_BY_CHAD_EXIT: MQLONG = 277;
pub const MQFB_STOPPED_BY_PUBSUB_EXIT: MQLONG = 279;
pub const MQFB_NOT_A_REPOSITORY_MSG: MQLONG = 280;
pub const MQFB_BIND_OPEN_CLUSRCVR_DEL: MQLONG = 281;
pub const MQFB_MAX_ACTIVITIES: MQLONG = 282;
pub const MQFB_NOT_FORWARDED: MQLONG = 283;
pub const MQFB_NOT_DELIVERED: MQLONG = 284;
pub const MQFB_UNSUPPORTED_FORWARDING: MQLONG = 285;
pub const MQFB_UNSUPPORTED_DELIVERY: MQLONG = 286;
pub const MQFB_DATA_LENGTH_ZERO: MQLONG = 291;
pub const MQFB_DATA_LENGTH_NEGATIVE: MQLONG = 292;
pub const MQFB_DATA_LENGTH_TOO_BIG: MQLONG = 293;
pub const MQFB_BUFFER_OVERFLOW: MQLONG = 294;
pub const MQFB_LENGTH_OFF_BY_ONE: MQLONG = 295;
pub const MQFB_IIH_ERROR: MQLONG = 296;
pub const MQFB_NOT_AUTHORIZED_FOR_IMS: MQLONG = 298;
pub const MQFB_DATA_LENGTH_TOO_SHORT: MQLONG = 299;
pub const MQFB_IMS_ERROR: MQLONG = 300;
pub const MQFB_IMS_FIRST: MQLONG = 301;
pub const MQFB_IMS_LAST: MQLONG = 399;
pub const MQFB_CICS_INTERNAL_ERROR: MQLONG = 401;
pub const MQFB_CICS_NOT_AUTHORIZED: MQLONG = 402;
pub const MQFB_CICS_BRIDGE_FAILURE: MQLONG = 403;
pub const MQFB_CICS_CORREL_ID_ERROR: MQLONG = 404;
pub const MQFB_CICS_CCSID_ERROR: MQLONG = 405;
pub const MQFB_CICS_ENCODING_ERROR: MQLONG = 406;
pub const MQFB_CICS_CIH_ERROR: MQLONG = 407;
pub const MQFB_CICS_UOW_ERROR: MQLONG = 408;
pub const MQFB_CICS_COMMAREA_ERROR: MQLONG = 409;
pub const MQFB_CICS_APPL_NOT_STARTED: MQLONG = 410;
pub const MQFB_CICS_APPL_ABENDED: MQLONG = 411;
pub const MQFB_CICS_DLQ_ERROR: MQLONG = 412;
pub const MQFB_CICS_UOW_BACKED_OUT: MQLONG = 413;
pub const MQFB_PUBLICATIONS_ON_REQUEST: MQLONG = 501;
pub const MQFB_SUBSCRIBER_IS_PUBLISHER: MQLONG = 502;
pub const MQFB_MSG_SCOPE_MISMATCH: MQLONG = 503;
pub const MQFB_SELECTOR_MISMATCH: MQLONG = 504;
pub const MQFB_NOT_A_GROUPUR_MSG: MQLONG = 505;
pub const MQFB_IMS_NACK_1A_REASON_FIRST: MQLONG = 600;
pub const MQFB_IMS_NACK_1A_REASON_LAST: MQLONG = 855;
pub const MQFB_SYSTEM_LAST: MQLONG = 65535;
pub const MQFB_APPL_FIRST: MQLONG = 65536;
pub const MQFB_APPL_LAST: MQLONG = 999999999;
pub const MQENC_NATIVE: MQLONG = 546;
pub const MQENC_INTEGER_MASK: MQLONG = 15;
pub const MQENC_DECIMAL_MASK: MQLONG = 240;
pub const MQENC_FLOAT_MASK: MQLONG = 3840;
pub const MQENC_RESERVED_MASK: MQLONG = -4096;
pub const MQENC_INTEGER_UNDEFINED: MQLONG = 0;
pub const MQENC_INTEGER_NORMAL: MQLONG = 1;
pub const MQENC_INTEGER_REVERSED: MQLONG = 2;
pub const MQENC_DECIMAL_UNDEFINED: MQLONG = 0;
pub const MQENC_DECIMAL_NORMAL: MQLONG = 16;
pub const MQENC_DECIMAL_REVERSED: MQLONG = 32;
pub const MQENC_FLOAT_UNDEFINED: MQLONG = 0;
pub const MQENC_FLOAT_IEEE_NORMAL: MQLONG = 256;
pub const MQENC_FLOAT_IEEE_REVERSED: MQLONG = 512;
pub const MQENC_FLOAT_S390: MQLONG = 768;
pub const MQENC_FLOAT_TNS: MQLONG = 1024;
pub const MQENC_NORMAL: MQLONG = 273;
pub const MQENC_REVERSED: MQLONG = 546;
pub const MQENC_S390: MQLONG = 785;
pub const MQENC_TNS: MQLONG = 1041;
pub const MQENC_AS_PUBLISHED: MQLONG = -1;
pub const MQCCSI_UNDEFINED: MQLONG = 0;
pub const MQCCSI_DEFAULT: MQLONG = 0;
pub const MQCCSI_Q_MGR: MQLONG = 0;
pub const MQCCSI_INHERIT: MQLONG = -2;
pub const MQCCSI_EMBEDDED: MQLONG = -1;
pub const MQCCSI_APPL: MQLONG = -3;
pub const MQCCSI_AS_PUBLISHED: MQLONG = -4;
pub const MQFMT_NONE: &::std::ffi::CStr = c"        ";
pub const MQFMT_ADMIN: &::std::ffi::CStr = c"MQADMIN ";
pub const MQFMT_AMQP: &::std::ffi::CStr = c"MQAMQP  ";
pub const MQFMT_CHANNEL_COMPLETED: &::std::ffi::CStr = c"MQCHCOM ";
pub const MQFMT_CICS: &::std::ffi::CStr = c"MQCICS  ";
pub const MQFMT_COMMAND_1: &::std::ffi::CStr = c"MQCMD1  ";
pub const MQFMT_COMMAND_2: &::std::ffi::CStr = c"MQCMD2  ";
pub const MQFMT_DEAD_LETTER_HEADER: &::std::ffi::CStr = c"MQDEAD  ";
pub const MQFMT_DIST_HEADER: &::std::ffi::CStr = c"MQHDIST ";
pub const MQFMT_EMBEDDED_PCF: &::std::ffi::CStr = c"MQHEPCF ";
pub const MQFMT_EVENT: &::std::ffi::CStr = c"MQEVENT ";
pub const MQFMT_IMS: &::std::ffi::CStr = c"MQIMS   ";
pub const MQFMT_IMS_VAR_STRING: &::std::ffi::CStr = c"MQIMSVS ";
pub const MQFMT_MD_EXTENSION: &::std::ffi::CStr = c"MQHMDE  ";
pub const MQFMT_PCF: &::std::ffi::CStr = c"MQPCF   ";
pub const MQFMT_REF_MSG_HEADER: &::std::ffi::CStr = c"MQHREF  ";
pub const MQFMT_RF_HEADER: &::std::ffi::CStr = c"MQHRF   ";
pub const MQFMT_RF_HEADER_1: &::std::ffi::CStr = c"MQHRF   ";
pub const MQFMT_RF_HEADER_2: &::std::ffi::CStr = c"MQHRF2  ";
pub const MQFMT_STRING: &::std::ffi::CStr = c"MQSTR   ";
pub const MQFMT_TRIGGER: &::std::ffi::CStr = c"MQTRIG  ";
pub const MQFMT_WORK_INFO_HEADER: &::std::ffi::CStr = c"MQHWIH  ";
pub const MQFMT_XMIT_Q_HEADER: &::std::ffi::CStr = c"MQXMIT  ";
pub const MQPRI_PRIORITY_AS_Q_DEF: MQLONG = -1;
pub const MQPRI_PRIORITY_AS_PARENT: MQLONG = -2;
pub const MQPRI_PRIORITY_AS_PUBLISHED: MQLONG = -3;
pub const MQPRI_PRIORITY_AS_TOPIC_DEF: MQLONG = -1;
pub const MQPER_PERSISTENCE_AS_PARENT: MQLONG = -1;
pub const MQPER_NOT_PERSISTENT: MQLONG = 0;
pub const MQPER_PERSISTENT: MQLONG = 1;
pub const MQPER_PERSISTENCE_AS_Q_DEF: MQLONG = 2;
pub const MQPER_PERSISTENCE_AS_TOPIC_DEF: MQLONG = 2;
pub const MQPRT_RESPONSE_AS_PARENT: MQLONG = 0;
pub const MQPRT_SYNC_RESPONSE: MQLONG = 1;
pub const MQPRT_ASYNC_RESPONSE: MQLONG = 2;
pub const MQMI_NONE: &[u8; 25] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQCI_NONE: &[u8; 25] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQCI_NEW_SESSION: &::std::ffi::CStr = c"AMQ!NEW_SESSION_CORRELID";
pub const MQACT_NONE: &[u8; 33] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQACTT_UNKNOWN: u8 = 0u8;
pub const MQACTT_CICS_LUOW_ID: u8 = 1u8;
pub const MQACTT_OS2_DEFAULT: u8 = 4u8;
pub const MQACTT_DOS_DEFAULT: u8 = 5u8;
pub const MQACTT_UNIX_NUMERIC_ID: u8 = 6u8;
pub const MQACTT_OS400_ACCOUNT_TOKEN: u8 = 8u8;
pub const MQACTT_WINDOWS_DEFAULT: u8 = 9u8;
pub const MQACTT_NT_SECURITY_ID: u8 = 11u8;
pub const MQACTT_AZUREAD_SECURITY_ID: u8 = 12u8;
pub const MQACTT_MS_ACC_AUTH_SECURITY_ID: u8 = 13u8;
pub const MQACTT_USER: u8 = 25u8;
pub const MQAT_UNKNOWN: MQLONG = -1;
pub const MQAT_NO_CONTEXT: MQLONG = 0;
pub const MQAT_CICS: MQLONG = 1;
pub const MQAT_MVS: MQLONG = 2;
pub const MQAT_OS390: MQLONG = 2;
pub const MQAT_ZOS: MQLONG = 2;
pub const MQAT_IMS: MQLONG = 3;
pub const MQAT_OS2: MQLONG = 4;
pub const MQAT_DOS: MQLONG = 5;
pub const MQAT_AIX: MQLONG = 6;
pub const MQAT_UNIX: MQLONG = 6;
pub const MQAT_QMGR: MQLONG = 7;
pub const MQAT_OS400: MQLONG = 8;
pub const MQAT_WINDOWS: MQLONG = 9;
pub const MQAT_CICS_VSE: MQLONG = 10;
pub const MQAT_WINDOWS_NT: MQLONG = 11;
pub const MQAT_VMS: MQLONG = 12;
pub const MQAT_GUARDIAN: MQLONG = 13;
pub const MQAT_NSK: MQLONG = 13;
pub const MQAT_VOS: MQLONG = 14;
pub const MQAT_OPEN_TP1: MQLONG = 15;
pub const MQAT_VM: MQLONG = 18;
pub const MQAT_IMS_BRIDGE: MQLONG = 19;
pub const MQAT_XCF: MQLONG = 20;
pub const MQAT_CICS_BRIDGE: MQLONG = 21;
pub const MQAT_NOTES_AGENT: MQLONG = 22;
pub const MQAT_TPF: MQLONG = 23;
pub const MQAT_USER: MQLONG = 25;
pub const MQAT_BROKER: MQLONG = 26;
pub const MQAT_QMGR_PUBLISH: MQLONG = 26;
pub const MQAT_JAVA: MQLONG = 28;
pub const MQAT_DQM: MQLONG = 29;
pub const MQAT_CHANNEL_INITIATOR: MQLONG = 30;
pub const MQAT_WLM: MQLONG = 31;
pub const MQAT_BATCH: MQLONG = 32;
pub const MQAT_RRS_BATCH: MQLONG = 33;
pub const MQAT_SIB: MQLONG = 34;
pub const MQAT_SYSTEM_EXTENSION: MQLONG = 35;
pub const MQAT_MCAST_PUBLISH: MQLONG = 36;
pub const MQAT_AMQP: MQLONG = 37;
pub const MQAT_DEFAULT: MQLONG = 6;
pub const MQAT_USER_FIRST: MQLONG = 65536;
pub const MQAT_USER_LAST: MQLONG = 999999999;
pub const MQGI_NONE: &[u8; 25] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQMF_SEGMENTATION_INHIBITED: MQLONG = 0;
pub const MQMF_SEGMENTATION_ALLOWED: MQLONG = 1;
pub const MQMF_MSG_IN_GROUP: MQLONG = 8;
pub const MQMF_LAST_MSG_IN_GROUP: MQLONG = 16;
pub const MQMF_SEGMENT: MQLONG = 2;
pub const MQMF_LAST_SEGMENT: MQLONG = 4;
pub const MQMF_NONE: MQLONG = 0;
pub const MQMF_REJECT_UNSUP_MASK: MQLONG = 4095;
pub const MQMF_ACCEPT_UNSUP_MASK: MQLONG = -1048576;
pub const MQMF_ACCEPT_UNSUP_IF_XMIT_MASK: MQLONG = 1044480;
pub const MQOL_UNDEFINED: MQLONG = -1;
pub const MQMDE_STRUC_ID: &::std::ffi::CStr = c"MDE ";
pub const MQMDE_VERSION_2: MQLONG = 2;
pub const MQMDE_CURRENT_VERSION: MQLONG = 2;
pub const MQMDE_LENGTH_2: usize = 72;
pub const MQMDE_CURRENT_LENGTH: usize = 72;
pub const MQMDEF_NONE: MQLONG = 0;
pub const MQMD1_LENGTH_1: usize = 324;
pub const MQMD1_CURRENT_LENGTH: usize = 324;
pub const MQMD2_LENGTH_1: usize = 324;
pub const MQMD2_LENGTH_2: usize = 364;
pub const MQMD2_CURRENT_LENGTH: usize = 364;
pub const MQMHBO_STRUC_ID: &::std::ffi::CStr = c"MHBO";
pub const MQMHBO_VERSION_1: MQLONG = 1;
pub const MQMHBO_CURRENT_VERSION: MQLONG = 1;
pub const MQMHBO_LENGTH_1: usize = 12;
pub const MQMHBO_CURRENT_LENGTH: usize = 12;
pub const MQMHBO_PROPERTIES_IN_MQRFH2: MQLONG = 1;
pub const MQMHBO_DELETE_PROPERTIES: MQLONG = 2;
pub const MQMHBO_NONE: MQLONG = 0;
pub const MQOD_STRUC_ID: &::std::ffi::CStr = c"OD  ";
pub const MQOD_VERSION_1: MQLONG = 1;
pub const MQOD_VERSION_2: MQLONG = 2;
pub const MQOD_VERSION_3: MQLONG = 3;
pub const MQOD_VERSION_4: MQLONG = 4;
pub const MQOD_CURRENT_VERSION: MQLONG = 4;
pub const MQOD_LENGTH_1: usize = 168;
pub const MQOD_LENGTH_2: usize = 208;
pub const MQOD_LENGTH_3: usize = 344;
pub const MQOD_LENGTH_4: usize = 424;
pub const MQOD_CURRENT_LENGTH: usize = 424;
pub const MQOM_NO: MQLONG = 0;
pub const MQOM_YES: MQLONG = 1;
pub const MQOT_NONE: MQLONG = 0;
pub const MQOT_Q: MQLONG = 1;
pub const MQOT_NAMELIST: MQLONG = 2;
pub const MQOT_PROCESS: MQLONG = 3;
pub const MQOT_STORAGE_CLASS: MQLONG = 4;
pub const MQOT_Q_MGR: MQLONG = 5;
pub const MQOT_CHANNEL: MQLONG = 6;
pub const MQOT_AUTH_INFO: MQLONG = 7;
pub const MQOT_TOPIC: MQLONG = 8;
pub const MQOT_COMM_INFO: MQLONG = 9;
pub const MQOT_CF_STRUC: MQLONG = 10;
pub const MQOT_LISTENER: MQLONG = 11;
pub const MQOT_SERVICE: MQLONG = 12;
pub const MQOT_RESERVED_1: MQLONG = 999;
pub const MQOT_ALL: MQLONG = 1001;
pub const MQOT_ALIAS_Q: MQLONG = 1002;
pub const MQOT_MODEL_Q: MQLONG = 1003;
pub const MQOT_LOCAL_Q: MQLONG = 1004;
pub const MQOT_REMOTE_Q: MQLONG = 1005;
pub const MQOT_SENDER_CHANNEL: MQLONG = 1007;
pub const MQOT_SERVER_CHANNEL: MQLONG = 1008;
pub const MQOT_REQUESTER_CHANNEL: MQLONG = 1009;
pub const MQOT_RECEIVER_CHANNEL: MQLONG = 1010;
pub const MQOT_CURRENT_CHANNEL: MQLONG = 1011;
pub const MQOT_SAVED_CHANNEL: MQLONG = 1012;
pub const MQOT_SVRCONN_CHANNEL: MQLONG = 1013;
pub const MQOT_CLNTCONN_CHANNEL: MQLONG = 1014;
pub const MQOT_SHORT_CHANNEL: MQLONG = 1015;
pub const MQOT_CHLAUTH: MQLONG = 1016;
pub const MQOT_REMOTE_Q_MGR_NAME: MQLONG = 1017;
pub const MQOT_PROT_POLICY: MQLONG = 1019;
pub const MQOT_TT_CHANNEL: MQLONG = 1020;
pub const MQOT_AMQP_CHANNEL: MQLONG = 1021;
pub const MQOT_AUTH_REC: MQLONG = 1022;
pub const MQPD_STRUC_ID: &::std::ffi::CStr = c"PD  ";
pub const MQPD_VERSION_1: MQLONG = 1;
pub const MQPD_CURRENT_VERSION: MQLONG = 1;
pub const MQPD_LENGTH_1: usize = 24;
pub const MQPD_CURRENT_LENGTH: usize = 24;
pub const MQPD_NONE: MQLONG = 0;
pub const MQPD_SUPPORT_OPTIONAL: MQLONG = 1;
pub const MQPD_SUPPORT_REQUIRED: MQLONG = 1048576;
pub const MQPD_SUPPORT_REQUIRED_IF_LOCAL: MQLONG = 1024;
pub const MQPD_REJECT_UNSUP_MASK: MQLONG = -1048576;
pub const MQPD_ACCEPT_UNSUP_IF_XMIT_MASK: MQLONG = 1047552;
pub const MQPD_ACCEPT_UNSUP_MASK: MQLONG = 1023;
pub const MQPD_NO_CONTEXT: MQLONG = 0;
pub const MQPD_USER_CONTEXT: MQLONG = 1;
pub const MQCOPY_NONE: MQLONG = 0;
pub const MQCOPY_ALL: MQLONG = 1;
pub const MQCOPY_FORWARD: MQLONG = 2;
pub const MQCOPY_PUBLISH: MQLONG = 4;
pub const MQCOPY_REPLY: MQLONG = 8;
pub const MQCOPY_REPORT: MQLONG = 16;
pub const MQCOPY_DEFAULT: MQLONG = 22;
pub const MQPMO_STRUC_ID: &::std::ffi::CStr = c"PMO ";
pub const MQPMO_VERSION_1: MQLONG = 1;
pub const MQPMO_VERSION_2: MQLONG = 2;
pub const MQPMO_VERSION_3: MQLONG = 3;
pub const MQPMO_CURRENT_VERSION: MQLONG = 3;
pub const MQPMO_LENGTH_1: usize = 128;
pub const MQPMO_LENGTH_2: usize = 160;
pub const MQPMO_LENGTH_3: usize = 184;
pub const MQPMO_CURRENT_LENGTH: usize = 184;
pub const MQPMO_SYNCPOINT: MQLONG = 2;
pub const MQPMO_NO_SYNCPOINT: MQLONG = 4;
pub const MQPMO_DEFAULT_CONTEXT: MQLONG = 32;
pub const MQPMO_NEW_MSG_ID: MQLONG = 64;
pub const MQPMO_NEW_CORREL_ID: MQLONG = 128;
pub const MQPMO_PASS_IDENTITY_CONTEXT: MQLONG = 256;
pub const MQPMO_PASS_ALL_CONTEXT: MQLONG = 512;
pub const MQPMO_SET_IDENTITY_CONTEXT: MQLONG = 1024;
pub const MQPMO_SET_ALL_CONTEXT: MQLONG = 2048;
pub const MQPMO_ALTERNATE_USER_AUTHORITY: MQLONG = 4096;
pub const MQPMO_FAIL_IF_QUIESCING: MQLONG = 8192;
pub const MQPMO_NO_CONTEXT: MQLONG = 16384;
pub const MQPMO_LOGICAL_ORDER: MQLONG = 32768;
pub const MQPMO_ASYNC_RESPONSE: MQLONG = 65536;
pub const MQPMO_SYNC_RESPONSE: MQLONG = 131072;
pub const MQPMO_RESOLVE_LOCAL_Q: MQLONG = 262144;
pub const MQPMO_WARN_IF_NO_SUBS_MATCHED: MQLONG = 524288;
pub const MQPMO_RETAIN: MQLONG = 2097152;
pub const MQPMO_MD_FOR_OUTPUT_ONLY: MQLONG = 8388608;
pub const MQPMO_SCOPE_QMGR: MQLONG = 67108864;
pub const MQPMO_SUPPRESS_REPLYTO: MQLONG = 134217728;
pub const MQPMO_NOT_OWN_SUBS: MQLONG = 268435456;
pub const MQPMO_RESPONSE_AS_Q_DEF: MQLONG = 0;
pub const MQPMO_RESPONSE_AS_TOPIC_DEF: MQLONG = 0;
pub const MQPMO_NONE: MQLONG = 0;
pub const MQPMO_PUB_OPTIONS_MASK: MQLONG = 2097152;
pub const MQPMRF_MSG_ID: MQLONG = 1;
pub const MQPMRF_CORREL_ID: MQLONG = 2;
pub const MQPMRF_GROUP_ID: MQLONG = 4;
pub const MQPMRF_FEEDBACK: MQLONG = 8;
pub const MQPMRF_ACCOUNTING_TOKEN: MQLONG = 16;
pub const MQPMRF_NONE: MQLONG = 0;
pub const MQACTP_NEW: MQLONG = 0;
pub const MQACTP_FORWARD: MQLONG = 1;
pub const MQACTP_REPLY: MQLONG = 2;
pub const MQACTP_REPORT: MQLONG = 3;
pub const MQRFH_STRUC_ID: &::std::ffi::CStr = c"RFH ";
pub const MQRFH_VERSION_1: MQLONG = 1;
pub const MQRFH_VERSION_2: MQLONG = 2;
pub const MQRFH_STRUC_LENGTH_FIXED: usize = 32;
pub const MQRFH_STRUC_LENGTH_FIXED_2: usize = 36;
pub const MQRFH_LENGTH_1: usize = 32;
pub const MQRFH_CURRENT_LENGTH: usize = 32;
pub const MQRFH_NONE: MQLONG = 0;
pub const MQRFH_NO_FLAGS: MQLONG = 0;
pub const MQRFH_FLAGS_RESTRICTED_MASK: MQLONG = -65536;
pub const MQNVS_APPL_TYPE: &::std::ffi::CStr = c"OPT_APP_GRP ";
pub const MQNVS_MSG_TYPE: &::std::ffi::CStr = c"OPT_MSG_TYPE ";
pub const MQRFH2_LENGTH_2: usize = 36;
pub const MQRFH2_CURRENT_LENGTH: usize = 36;
pub const MQRMH_STRUC_ID: &::std::ffi::CStr = c"RMH ";
pub const MQRMH_VERSION_1: MQLONG = 1;
pub const MQRMH_CURRENT_VERSION: MQLONG = 1;
pub const MQRMH_LENGTH_1: usize = 108;
pub const MQRMH_CURRENT_LENGTH: usize = 108;
pub const MQRMHF_LAST: MQLONG = 1;
pub const MQRMHF_NOT_LAST: MQLONG = 0;
pub const MQOII_NONE: &[u8; 25] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQSD_STRUC_ID: &::std::ffi::CStr = c"SD  ";
pub const MQSD_VERSION_1: MQLONG = 1;
pub const MQSD_CURRENT_VERSION: MQLONG = 1;
pub const MQSD_LENGTH_1: usize = 344;
pub const MQSD_CURRENT_LENGTH: usize = 344;
pub const MQSID_NONE: &[u8; 41] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQSIDT_NONE: u8 = 0u8;
pub const MQSIDT_NT_SECURITY_ID: u8 = 1u8;
pub const MQSIDT_WAS_SECURITY_ID: u8 = 2u8;
pub const MQSMPO_STRUC_ID: &::std::ffi::CStr = c"SMPO";
pub const MQSMPO_VERSION_1: MQLONG = 1;
pub const MQSMPO_CURRENT_VERSION: MQLONG = 1;
pub const MQSMPO_LENGTH_1: usize = 20;
pub const MQSMPO_CURRENT_LENGTH: usize = 20;
pub const MQSMPO_SET_FIRST: MQLONG = 0;
pub const MQSMPO_SET_PROP_UNDER_CURSOR: MQLONG = 1;
pub const MQSMPO_SET_PROP_AFTER_CURSOR: MQLONG = 2;
pub const MQSMPO_APPEND_PROPERTY: MQLONG = 4;
pub const MQSMPO_SET_PROP_BEFORE_CURSOR: MQLONG = 8;
pub const MQSMPO_NONE: MQLONG = 0;
pub const MQSRO_STRUC_ID: &::std::ffi::CStr = c"SRO ";
pub const MQSRO_VERSION_1: MQLONG = 1;
pub const MQSRO_CURRENT_VERSION: MQLONG = 1;
pub const MQSRO_LENGTH_1: usize = 16;
pub const MQSRO_CURRENT_LENGTH: usize = 16;
pub const MQSRO_NONE: MQLONG = 0;
pub const MQSRO_FAIL_IF_QUIESCING: MQLONG = 8192;
pub const MQSTS_STRUC_ID: &::std::ffi::CStr = c"STAT";
pub const MQSTS_VERSION_1: MQLONG = 1;
pub const MQSTS_VERSION_2: MQLONG = 2;
pub const MQSTS_CURRENT_VERSION: MQLONG = 2;
pub const MQSTS_LENGTH_1: usize = 224;
pub const MQSTS_LENGTH_2: usize = 280;
pub const MQSTS_CURRENT_LENGTH: usize = 280;
pub const MQTM_STRUC_ID: &::std::ffi::CStr = c"TM  ";
pub const MQTM_VERSION_1: MQLONG = 1;
pub const MQTM_CURRENT_VERSION: MQLONG = 1;
pub const MQTM_LENGTH_1: usize = 684;
pub const MQTM_CURRENT_LENGTH: usize = 684;
pub const MQTMC_STRUC_ID: &::std::ffi::CStr = c"TMC ";
pub const MQTMC2_LENGTH_1: usize = 684;
pub const MQTMC2_LENGTH_2: usize = 732;
pub const MQTMC2_CURRENT_LENGTH: usize = 732;
pub const MQTMC_VERSION_1: &::std::ffi::CStr = c"   1";
pub const MQTMC_VERSION_2: &::std::ffi::CStr = c"   2";
pub const MQTMC_CURRENT_VERSION: &::std::ffi::CStr = c"   2";
pub const MQWIH_STRUC_ID: &::std::ffi::CStr = c"WIH ";
pub const MQWIH_VERSION_1: MQLONG = 1;
pub const MQWIH_CURRENT_VERSION: MQLONG = 1;
pub const MQWIH_LENGTH_1: usize = 120;
pub const MQWIH_CURRENT_LENGTH: usize = 120;
pub const MQWIH_NONE: MQLONG = 0;
pub const MQXQH_STRUC_ID: &::std::ffi::CStr = c"XQH ";
pub const MQXQH_VERSION_1: MQLONG = 1;
pub const MQXQH_CURRENT_VERSION: MQLONG = 1;
pub const MQXQH_LENGTH_1: usize = 428;
pub const MQXQH_CURRENT_LENGTH: usize = 428;
pub const MQHC_DEF_HCONN: MQHCONN = 0;
pub const MQHC_UNUSABLE_HCONN: MQHCONN = -1;
pub const MQHC_UNASSOCIATED_HCONN: MQHCONN = -3;
pub const MQ_OPERATOR_MESSAGE_LENGTH: usize = 4;
pub const MQ_ABEND_CODE_LENGTH: usize = 4;
pub const MQ_ACCOUNTING_TOKEN_LENGTH: usize = 32;
pub const MQ_APPL_DESC_LENGTH: usize = 64;
pub const MQ_APPL_IDENTITY_DATA_LENGTH: usize = 32;
pub const MQ_APPL_NAME_LENGTH: usize = 28;
pub const MQ_APPL_ORIGIN_DATA_LENGTH: usize = 4;
pub const MQ_APPL_TAG_LENGTH: usize = 28;
pub const MQ_ARM_SUFFIX_LENGTH: usize = 2;
pub const MQ_ATTENTION_ID_LENGTH: usize = 4;
pub const MQ_AUTH_INFO_CONN_NAME_LENGTH: usize = 264;
pub const MQ_AUTH_INFO_DESC_LENGTH: usize = 64;
pub const MQ_AUTH_INFO_NAME_LENGTH: usize = 48;
pub const MQ_AUTH_INFO_OCSP_URL_LENGTH: usize = 256;
pub const MQ_AUTHENTICATOR_LENGTH: usize = 8;
pub const MQ_AUTO_REORG_CATALOG_LENGTH: usize = 44;
pub const MQ_AUTO_REORG_TIME_LENGTH: usize = 4;
pub const MQ_BATCH_INTERFACE_ID_LENGTH: usize = 8;
pub const MQ_BRIDGE_NAME_LENGTH: usize = 24;
pub const MQ_CANCEL_CODE_LENGTH: usize = 4;
pub const MQ_CF_STRUC_DESC_LENGTH: usize = 64;
pub const MQ_CF_STRUC_NAME_LENGTH: usize = 12;
pub const MQ_CHANNEL_DATE_LENGTH: usize = 12;
pub const MQ_CHANNEL_DESC_LENGTH: usize = 64;
pub const MQ_CHANNEL_NAME_LENGTH: usize = 20;
pub const MQ_CHANNEL_TIME_LENGTH: usize = 8;
pub const MQ_CHINIT_SERVICE_PARM_LENGTH: usize = 32;
pub const MQ_CICS_FILE_NAME_LENGTH: usize = 8;
pub const MQ_AMQP_CLIENT_ID_LENGTH: usize = 256;
pub const MQ_CLIENT_ID_LENGTH: usize = 23;
pub const MQ_CLIENT_USER_ID_LENGTH: usize = 1024;
pub const MQ_CLUSTER_NAME_LENGTH: usize = 48;
pub const MQ_COMM_INFO_DESC_LENGTH: usize = 64;
pub const MQ_COMM_INFO_NAME_LENGTH: usize = 48;
pub const MQ_CONN_NAME_LENGTH: usize = 264;
pub const MQ_CONN_TAG_LENGTH: usize = 128;
pub const MQ_CONNECTION_ID_LENGTH: usize = 24;
pub const MQ_CORREL_ID_LENGTH: usize = 24;
pub const MQ_CREATION_DATE_LENGTH: usize = 12;
pub const MQ_CREATION_TIME_LENGTH: usize = 8;
pub const MQ_CSP_PASSWORD_LENGTH: usize = 256;
pub const MQ_CSP_TOKEN_LENGTH: usize = 8192;
pub const MQ_DATE_LENGTH: usize = 12;
pub const MQ_DISTINGUISHED_NAME_LENGTH: usize = 1024;
pub const MQ_DNS_GROUP_NAME_LENGTH: usize = 18;
pub const MQ_EXIT_DATA_LENGTH: usize = 32;
pub const MQ_EXIT_INFO_NAME_LENGTH: usize = 48;
pub const MQ_EXIT_NAME_LENGTH: usize = 128;
pub const MQ_EXIT_PD_AREA_LENGTH: usize = 48;
pub const MQ_EXIT_USER_AREA_LENGTH: usize = 16;
pub const MQ_FACILITY_LENGTH: usize = 8;
pub const MQ_FACILITY_LIKE_LENGTH: usize = 4;
pub const MQ_FORMAT_LENGTH: usize = 8;
pub const MQ_FUNCTION_LENGTH: usize = 4;
pub const MQ_GROUP_ID_LENGTH: usize = 24;
pub const MQ_APPL_FUNCTION_NAME_LENGTH: usize = 10;
pub const MQ_INITIAL_KEY_LENGTH: usize = 256;
pub const MQ_INSTALLATION_DESC_LENGTH: usize = 64;
pub const MQ_INSTALLATION_NAME_LENGTH: usize = 16;
pub const MQ_INSTALLATION_PATH_LENGTH: usize = 256;
pub const MQ_ISOTIME_LENGTH: usize = 64;
pub const MQ_JAAS_CONFIG_LENGTH: usize = 1024;
pub const MQ_LDAP_PASSWORD_LENGTH: usize = 32;
pub const MQ_LDAP_BASE_DN_LENGTH: usize = 1024;
pub const MQ_LDAP_FIELD_LENGTH: usize = 128;
pub const MQ_LDAP_CLASS_LENGTH: usize = 128;
pub const MQ_LISTENER_NAME_LENGTH: usize = 48;
pub const MQ_LISTENER_DESC_LENGTH: usize = 64;
pub const MQ_LOCAL_ADDRESS_LENGTH: usize = 48;
pub const MQ_LTERM_OVERRIDE_LENGTH: usize = 8;
pub const MQ_LU_NAME_LENGTH: usize = 8;
pub const MQ_LUWID_LENGTH: usize = 16;
pub const MQ_MAX_EXIT_NAME_LENGTH: usize = 128;
pub const MQ_MAX_MCA_USER_ID_LENGTH: usize = 64;
pub const MQ_MAX_LDAP_MCA_USER_ID_LENGTH: usize = 1024;
pub const MQ_MAX_PROPERTY_NAME_LENGTH: usize = 4095;
pub const MQ_MAX_USER_ID_LENGTH: usize = 64;
pub const MQ_MCA_JOB_NAME_LENGTH: usize = 28;
pub const MQ_MCA_NAME_LENGTH: usize = 20;
pub const MQ_MCA_USER_DATA_LENGTH: usize = 32;
pub const MQ_MCA_USER_ID_LENGTH: usize = 12;
pub const MQ_LDAP_MCA_USER_ID_LENGTH: usize = 1024;
pub const MQ_MFS_MAP_NAME_LENGTH: usize = 8;
pub const MQ_MODE_NAME_LENGTH: usize = 8;
pub const MQ_MSG_HEADER_LENGTH: usize = 4000;
pub const MQ_MSG_ID_LENGTH: usize = 24;
pub const MQ_MSG_TOKEN_LENGTH: usize = 16;
pub const MQ_NAMELIST_DESC_LENGTH: usize = 64;
pub const MQ_NAMELIST_NAME_LENGTH: usize = 48;
pub const MQ_NHA_GROUP_NAME_LENGTH: usize = 48;
pub const MQ_NHA_INSTANCE_NAME_LENGTH: usize = 48;
pub const MQ_OBJECT_INSTANCE_ID_LENGTH: usize = 24;
pub const MQ_OBJECT_NAME_LENGTH: usize = 48;
pub const MQ_PASS_TICKET_APPL_LENGTH: usize = 8;
pub const MQ_PASSWORD_LENGTH: usize = 12;
pub const MQ_PROCESS_APPL_ID_LENGTH: usize = 256;
pub const MQ_PROCESS_DESC_LENGTH: usize = 64;
pub const MQ_PROCESS_ENV_DATA_LENGTH: usize = 128;
pub const MQ_PROCESS_NAME_LENGTH: usize = 48;
pub const MQ_PROCESS_USER_DATA_LENGTH: usize = 128;
pub const MQ_PROGRAM_NAME_LENGTH: usize = 20;
pub const MQ_PUT_APPL_NAME_LENGTH: usize = 28;
pub const MQ_PUT_DATE_LENGTH: usize = 8;
pub const MQ_PUT_TIME_LENGTH: usize = 8;
pub const MQ_Q_DESC_LENGTH: usize = 64;
pub const MQ_Q_MGR_DESC_LENGTH: usize = 64;
pub const MQ_Q_MGR_IDENTIFIER_LENGTH: usize = 48;
pub const MQ_Q_MGR_NAME_LENGTH: usize = 48;
pub const MQ_Q_NAME_LENGTH: usize = 48;
pub const MQ_QSG_NAME_LENGTH: usize = 4;
pub const MQ_REMOTE_SYS_ID_LENGTH: usize = 4;
pub const MQ_SECURITY_ID_LENGTH: usize = 40;
pub const MQ_SELECTOR_LENGTH: usize = 10240;
pub const MQ_SERVICE_ARGS_LENGTH: usize = 255;
pub const MQ_SERVICE_COMMAND_LENGTH: usize = 255;
pub const MQ_SERVICE_DESC_LENGTH: usize = 64;
pub const MQ_SERVICE_NAME_LENGTH: usize = 32;
pub const MQ_SERVICE_PATH_LENGTH: usize = 255;
pub const MQ_SERVICE_STEP_LENGTH: usize = 8;
pub const MQ_SHORT_CONN_NAME_LENGTH: usize = 20;
pub const MQ_SHORT_DNAME_LENGTH: usize = 256;
pub const MQ_SSL_CIPHER_SPEC_LENGTH: usize = 32;
pub const MQ_SSL_CIPHER_SUITE_LENGTH: usize = 32;
pub const MQ_SSL_CRYPTO_HARDWARE_LENGTH: usize = 256;
pub const MQ_SSL_ENCRYP_KEY_REPO_PWD_LEN: usize = 1536;
pub const MQ_SSL_HANDSHAKE_STAGE_LENGTH: usize = 32;
pub const MQ_SSL_KEY_LIBRARY_LENGTH: usize = 44;
pub const MQ_SSL_KEY_MEMBER_LENGTH: usize = 8;
pub const MQ_SSL_KEY_REPOSITORY_LENGTH: usize = 256;
pub const MQ_SSL_KEY_REPO_PWD_LEN: usize = 1024;
pub const MQ_SSL_PEER_NAME_LENGTH: usize = 1024;
pub const MQ_SSL_SHORT_PEER_NAME_LENGTH: usize = 256;
pub const MQ_START_CODE_LENGTH: usize = 4;
pub const MQ_STORAGE_CLASS_DESC_LENGTH: usize = 64;
pub const MQ_STORAGE_CLASS_LENGTH: usize = 8;
pub const MQ_SUB_IDENTITY_LENGTH: usize = 128;
pub const MQ_SUB_POINT_LENGTH: usize = 128;
pub const MQ_TCP_NAME_LENGTH: usize = 8;
pub const MQ_TEMPORARY_Q_PREFIX_LENGTH: usize = 32;
pub const MQ_TIME_LENGTH: usize = 8;
pub const MQ_TOPIC_DESC_LENGTH: usize = 64;
pub const MQ_TOPIC_NAME_LENGTH: usize = 48;
pub const MQ_TOPIC_STR_LENGTH: usize = 10240;
pub const MQ_TOTAL_EXIT_DATA_LENGTH: usize = 999;
pub const MQ_TOTAL_EXIT_NAME_LENGTH: usize = 999;
pub const MQ_TP_NAME_LENGTH: usize = 64;
pub const MQ_TPIPE_NAME_LENGTH: usize = 8;
pub const MQ_TRAN_INSTANCE_ID_LENGTH: usize = 16;
pub const MQ_TRANSACTION_ID_LENGTH: usize = 4;
pub const MQ_TRIGGER_DATA_LENGTH: usize = 64;
pub const MQ_TRIGGER_PROGRAM_NAME_LENGTH: usize = 8;
pub const MQ_TRIGGER_TERM_ID_LENGTH: usize = 4;
pub const MQ_TRIGGER_TRANS_ID_LENGTH: usize = 4;
pub const MQ_USER_ID_LENGTH: usize = 12;
pub const MQ_LONG_USER_ID_LENGTH: usize = 1024;
pub const MQ_VERSION_LENGTH: usize = 8;
pub const MQ_XCF_GROUP_NAME_LENGTH: usize = 8;
pub const MQ_XCF_MEMBER_NAME_LENGTH: usize = 16;
pub const MQ_SMDS_NAME_LENGTH: usize = 4;
pub const MQ_CHLAUTH_DESC_LENGTH: usize = 64;
pub const MQ_CUSTOM_LENGTH: usize = 128;
pub const MQ_SUITE_B_SIZE: MQLONG = 4;
pub const MQ_CERT_LABEL_LENGTH: usize = 64;
pub const MQCC_OK: MQLONG = 0;
pub const MQCC_WARNING: MQLONG = 1;
pub const MQCC_FAILED: MQLONG = 2;
pub const MQCC_UNKNOWN: MQLONG = -1;
pub const MQRC_NONE: MQLONG = 0;
pub const MQRC_APPL_FIRST: MQLONG = 900;
pub const MQRC_APPL_LAST: MQLONG = 999;
pub const MQRC_ALIAS_BASE_Q_TYPE_ERROR: MQLONG = 2001;
pub const MQRC_ALREADY_CONNECTED: MQLONG = 2002;
pub const MQRC_BACKED_OUT: MQLONG = 2003;
pub const MQRC_BUFFER_ERROR: MQLONG = 2004;
pub const MQRC_BUFFER_LENGTH_ERROR: MQLONG = 2005;
pub const MQRC_CHAR_ATTR_LENGTH_ERROR: MQLONG = 2006;
pub const MQRC_CHAR_ATTRS_ERROR: MQLONG = 2007;
pub const MQRC_CHAR_ATTRS_TOO_SHORT: MQLONG = 2008;
pub const MQRC_CONNECTION_BROKEN: MQLONG = 2009;
pub const MQRC_DATA_LENGTH_ERROR: MQLONG = 2010;
pub const MQRC_DYNAMIC_Q_NAME_ERROR: MQLONG = 2011;
pub const MQRC_ENVIRONMENT_ERROR: MQLONG = 2012;
pub const MQRC_EXPIRY_ERROR: MQLONG = 2013;
pub const MQRC_FEEDBACK_ERROR: MQLONG = 2014;
pub const MQRC_GET_INHIBITED: MQLONG = 2016;
pub const MQRC_HANDLE_NOT_AVAILABLE: MQLONG = 2017;
pub const MQRC_HCONN_ERROR: MQLONG = 2018;
pub const MQRC_HOBJ_ERROR: MQLONG = 2019;
pub const MQRC_INHIBIT_VALUE_ERROR: MQLONG = 2020;
pub const MQRC_INT_ATTR_COUNT_ERROR: MQLONG = 2021;
pub const MQRC_INT_ATTR_COUNT_TOO_SMALL: MQLONG = 2022;
pub const MQRC_INT_ATTRS_ARRAY_ERROR: MQLONG = 2023;
pub const MQRC_SYNCPOINT_LIMIT_REACHED: MQLONG = 2024;
pub const MQRC_MAX_CONNS_LIMIT_REACHED: MQLONG = 2025;
pub const MQRC_MD_ERROR: MQLONG = 2026;
pub const MQRC_MISSING_REPLY_TO_Q: MQLONG = 2027;
pub const MQRC_MSG_TYPE_ERROR: MQLONG = 2029;
pub const MQRC_MSG_TOO_BIG_FOR_Q: MQLONG = 2030;
pub const MQRC_MSG_TOO_BIG_FOR_Q_MGR: MQLONG = 2031;
pub const MQRC_NO_MSG_AVAILABLE: MQLONG = 2033;
pub const MQRC_NO_MSG_UNDER_CURSOR: MQLONG = 2034;
pub const MQRC_NOT_AUTHORIZED: MQLONG = 2035;
pub const MQRC_NOT_OPEN_FOR_BROWSE: MQLONG = 2036;
pub const MQRC_NOT_OPEN_FOR_INPUT: MQLONG = 2037;
pub const MQRC_NOT_OPEN_FOR_INQUIRE: MQLONG = 2038;
pub const MQRC_NOT_OPEN_FOR_OUTPUT: MQLONG = 2039;
pub const MQRC_NOT_OPEN_FOR_SET: MQLONG = 2040;
pub const MQRC_OBJECT_CHANGED: MQLONG = 2041;
pub const MQRC_OBJECT_IN_USE: MQLONG = 2042;
pub const MQRC_OBJECT_TYPE_ERROR: MQLONG = 2043;
pub const MQRC_OD_ERROR: MQLONG = 2044;
pub const MQRC_OPTION_NOT_VALID_FOR_TYPE: MQLONG = 2045;
pub const MQRC_OPTIONS_ERROR: MQLONG = 2046;
pub const MQRC_PERSISTENCE_ERROR: MQLONG = 2047;
pub const MQRC_PERSISTENT_NOT_ALLOWED: MQLONG = 2048;
pub const MQRC_PRIORITY_EXCEEDS_MAXIMUM: MQLONG = 2049;
pub const MQRC_PRIORITY_ERROR: MQLONG = 2050;
pub const MQRC_PUT_INHIBITED: MQLONG = 2051;
pub const MQRC_Q_DELETED: MQLONG = 2052;
pub const MQRC_Q_FULL: MQLONG = 2053;
pub const MQRC_Q_NOT_EMPTY: MQLONG = 2055;
pub const MQRC_Q_SPACE_NOT_AVAILABLE: MQLONG = 2056;
pub const MQRC_Q_TYPE_ERROR: MQLONG = 2057;
pub const MQRC_Q_MGR_NAME_ERROR: MQLONG = 2058;
pub const MQRC_Q_MGR_NOT_AVAILABLE: MQLONG = 2059;
pub const MQRC_REPORT_OPTIONS_ERROR: MQLONG = 2061;
pub const MQRC_SECOND_MARK_NOT_ALLOWED: MQLONG = 2062;
pub const MQRC_SECURITY_ERROR: MQLONG = 2063;
pub const MQRC_TOKEN_TIMESTAMP_NOT_VALID: MQLONG = 2064;
pub const MQRC_SELECTOR_COUNT_ERROR: MQLONG = 2065;
pub const MQRC_SELECTOR_LIMIT_EXCEEDED: MQLONG = 2066;
pub const MQRC_SELECTOR_ERROR: MQLONG = 2067;
pub const MQRC_SELECTOR_NOT_FOR_TYPE: MQLONG = 2068;
pub const MQRC_SIGNAL_OUTSTANDING: MQLONG = 2069;
pub const MQRC_SIGNAL_REQUEST_ACCEPTED: MQLONG = 2070;
pub const MQRC_STORAGE_NOT_AVAILABLE: MQLONG = 2071;
pub const MQRC_SYNCPOINT_NOT_AVAILABLE: MQLONG = 2072;
pub const MQRC_TRIGGER_CONTROL_ERROR: MQLONG = 2075;
pub const MQRC_TRIGGER_DEPTH_ERROR: MQLONG = 2076;
pub const MQRC_TRIGGER_MSG_PRIORITY_ERR: MQLONG = 2077;
pub const MQRC_TRIGGER_TYPE_ERROR: MQLONG = 2078;
pub const MQRC_TRUNCATED_MSG_ACCEPTED: MQLONG = 2079;
pub const MQRC_TRUNCATED_MSG_FAILED: MQLONG = 2080;
pub const MQRC_UNKNOWN_ALIAS_BASE_Q: MQLONG = 2082;
pub const MQRC_UNKNOWN_OBJECT_NAME: MQLONG = 2085;
pub const MQRC_UNKNOWN_OBJECT_Q_MGR: MQLONG = 2086;
pub const MQRC_UNKNOWN_REMOTE_Q_MGR: MQLONG = 2087;
pub const MQRC_WAIT_INTERVAL_ERROR: MQLONG = 2090;
pub const MQRC_XMIT_Q_TYPE_ERROR: MQLONG = 2091;
pub const MQRC_XMIT_Q_USAGE_ERROR: MQLONG = 2092;
pub const MQRC_NOT_OPEN_FOR_PASS_ALL: MQLONG = 2093;
pub const MQRC_NOT_OPEN_FOR_PASS_IDENT: MQLONG = 2094;
pub const MQRC_NOT_OPEN_FOR_SET_ALL: MQLONG = 2095;
pub const MQRC_NOT_OPEN_FOR_SET_IDENT: MQLONG = 2096;
pub const MQRC_CONTEXT_HANDLE_ERROR: MQLONG = 2097;
pub const MQRC_CONTEXT_NOT_AVAILABLE: MQLONG = 2098;
pub const MQRC_SIGNAL1_ERROR: MQLONG = 2099;
pub const MQRC_OBJECT_ALREADY_EXISTS: MQLONG = 2100;
pub const MQRC_OBJECT_DAMAGED: MQLONG = 2101;
pub const MQRC_RESOURCE_PROBLEM: MQLONG = 2102;
pub const MQRC_ANOTHER_Q_MGR_CONNECTED: MQLONG = 2103;
pub const MQRC_UNKNOWN_REPORT_OPTION: MQLONG = 2104;
pub const MQRC_STORAGE_CLASS_ERROR: MQLONG = 2105;
pub const MQRC_COD_NOT_VALID_FOR_XCF_Q: MQLONG = 2106;
pub const MQRC_XWAIT_CANCELED: MQLONG = 2107;
pub const MQRC_XWAIT_ERROR: MQLONG = 2108;
pub const MQRC_SUPPRESSED_BY_EXIT: MQLONG = 2109;
pub const MQRC_FORMAT_ERROR: MQLONG = 2110;
pub const MQRC_SOURCE_CCSID_ERROR: MQLONG = 2111;
pub const MQRC_SOURCE_INTEGER_ENC_ERROR: MQLONG = 2112;
pub const MQRC_SOURCE_DECIMAL_ENC_ERROR: MQLONG = 2113;
pub const MQRC_SOURCE_FLOAT_ENC_ERROR: MQLONG = 2114;
pub const MQRC_TARGET_CCSID_ERROR: MQLONG = 2115;
pub const MQRC_TARGET_INTEGER_ENC_ERROR: MQLONG = 2116;
pub const MQRC_TARGET_DECIMAL_ENC_ERROR: MQLONG = 2117;
pub const MQRC_TARGET_FLOAT_ENC_ERROR: MQLONG = 2118;
pub const MQRC_NOT_CONVERTED: MQLONG = 2119;
pub const MQRC_CONVERTED_MSG_TOO_BIG: MQLONG = 2120;
pub const MQRC_TRUNCATED: MQLONG = 2120;
pub const MQRC_NO_EXTERNAL_PARTICIPANTS: MQLONG = 2121;
pub const MQRC_PARTICIPANT_NOT_AVAILABLE: MQLONG = 2122;
pub const MQRC_OUTCOME_MIXED: MQLONG = 2123;
pub const MQRC_OUTCOME_PENDING: MQLONG = 2124;
pub const MQRC_BRIDGE_STARTED: MQLONG = 2125;
pub const MQRC_BRIDGE_STOPPED: MQLONG = 2126;
pub const MQRC_ADAPTER_STORAGE_SHORTAGE: MQLONG = 2127;
pub const MQRC_UOW_IN_PROGRESS: MQLONG = 2128;
pub const MQRC_ADAPTER_CONN_LOAD_ERROR: MQLONG = 2129;
pub const MQRC_ADAPTER_SERV_LOAD_ERROR: MQLONG = 2130;
pub const MQRC_ADAPTER_DEFS_ERROR: MQLONG = 2131;
pub const MQRC_ADAPTER_DEFS_LOAD_ERROR: MQLONG = 2132;
pub const MQRC_ADAPTER_CONV_LOAD_ERROR: MQLONG = 2133;
pub const MQRC_BO_ERROR: MQLONG = 2134;
pub const MQRC_DH_ERROR: MQLONG = 2135;
pub const MQRC_MULTIPLE_REASONS: MQLONG = 2136;
pub const MQRC_OPEN_FAILED: MQLONG = 2137;
pub const MQRC_ADAPTER_DISC_LOAD_ERROR: MQLONG = 2138;
pub const MQRC_CNO_ERROR: MQLONG = 2139;
pub const MQRC_CICS_WAIT_FAILED: MQLONG = 2140;
pub const MQRC_DLH_ERROR: MQLONG = 2141;
pub const MQRC_HEADER_ERROR: MQLONG = 2142;
pub const MQRC_SOURCE_LENGTH_ERROR: MQLONG = 2143;
pub const MQRC_TARGET_LENGTH_ERROR: MQLONG = 2144;
pub const MQRC_SOURCE_BUFFER_ERROR: MQLONG = 2145;
pub const MQRC_TARGET_BUFFER_ERROR: MQLONG = 2146;
pub const MQRC_INCOMPLETE_TRANSACTION: MQLONG = 2147;
pub const MQRC_IIH_ERROR: MQLONG = 2148;
pub const MQRC_PCF_ERROR: MQLONG = 2149;
pub const MQRC_DBCS_ERROR: MQLONG = 2150;
pub const MQRC_OBJECT_NAME_ERROR: MQLONG = 2152;
pub const MQRC_OBJECT_Q_MGR_NAME_ERROR: MQLONG = 2153;
pub const MQRC_RECS_PRESENT_ERROR: MQLONG = 2154;
pub const MQRC_OBJECT_RECORDS_ERROR: MQLONG = 2155;
pub const MQRC_RESPONSE_RECORDS_ERROR: MQLONG = 2156;
pub const MQRC_ASID_MISMATCH: MQLONG = 2157;
pub const MQRC_PMO_RECORD_FLAGS_ERROR: MQLONG = 2158;
pub const MQRC_PUT_MSG_RECORDS_ERROR: MQLONG = 2159;
pub const MQRC_CONN_ID_IN_USE: MQLONG = 2160;
pub const MQRC_Q_MGR_QUIESCING: MQLONG = 2161;
pub const MQRC_Q_MGR_STOPPING: MQLONG = 2162;
pub const MQRC_DUPLICATE_RECOV_COORD: MQLONG = 2163;
pub const MQRC_PMO_ERROR: MQLONG = 2173;
pub const MQRC_API_EXIT_NOT_FOUND: MQLONG = 2182;
pub const MQRC_API_EXIT_LOAD_ERROR: MQLONG = 2183;
pub const MQRC_REMOTE_Q_NAME_ERROR: MQLONG = 2184;
pub const MQRC_INCONSISTENT_PERSISTENCE: MQLONG = 2185;
pub const MQRC_GMO_ERROR: MQLONG = 2186;
pub const MQRC_CICS_BRIDGE_RESTRICTION: MQLONG = 2187;
pub const MQRC_STOPPED_BY_CLUSTER_EXIT: MQLONG = 2188;
pub const MQRC_CLUSTER_RESOLUTION_ERROR: MQLONG = 2189;
pub const MQRC_CONVERTED_STRING_TOO_BIG: MQLONG = 2190;
pub const MQRC_TMC_ERROR: MQLONG = 2191;
pub const MQRC_STORAGE_MEDIUM_FULL: MQLONG = 2192;
pub const MQRC_PAGESET_FULL: MQLONG = 2192;
pub const MQRC_PAGESET_ERROR: MQLONG = 2193;
pub const MQRC_NAME_NOT_VALID_FOR_TYPE: MQLONG = 2194;
pub const MQRC_UNEXPECTED_ERROR: MQLONG = 2195;
pub const MQRC_UNKNOWN_XMIT_Q: MQLONG = 2196;
pub const MQRC_UNKNOWN_DEF_XMIT_Q: MQLONG = 2197;
pub const MQRC_DEF_XMIT_Q_TYPE_ERROR: MQLONG = 2198;
pub const MQRC_DEF_XMIT_Q_USAGE_ERROR: MQLONG = 2199;
pub const MQRC_MSG_MARKED_BROWSE_CO_OP: MQLONG = 2200;
pub const MQRC_NAME_IN_USE: MQLONG = 2201;
pub const MQRC_CONNECTION_QUIESCING: MQLONG = 2202;
pub const MQRC_CONNECTION_STOPPING: MQLONG = 2203;
pub const MQRC_ADAPTER_NOT_AVAILABLE: MQLONG = 2204;
pub const MQRC_MSG_ID_ERROR: MQLONG = 2206;
pub const MQRC_CORREL_ID_ERROR: MQLONG = 2207;
pub const MQRC_FILE_SYSTEM_ERROR: MQLONG = 2208;
pub const MQRC_NO_MSG_LOCKED: MQLONG = 2209;
pub const MQRC_SOAP_DOTNET_ERROR: MQLONG = 2210;
pub const MQRC_SOAP_AXIS_ERROR: MQLONG = 2211;
pub const MQRC_SOAP_URL_ERROR: MQLONG = 2212;
pub const MQRC_FILE_NOT_AUDITED: MQLONG = 2216;
pub const MQRC_CONNECTION_NOT_AUTHORIZED: MQLONG = 2217;
pub const MQRC_MSG_TOO_BIG_FOR_CHANNEL: MQLONG = 2218;
pub const MQRC_CALL_IN_PROGRESS: MQLONG = 2219;
pub const MQRC_RMH_ERROR: MQLONG = 2220;
pub const MQRC_Q_MGR_ACTIVE: MQLONG = 2222;
pub const MQRC_Q_MGR_NOT_ACTIVE: MQLONG = 2223;
pub const MQRC_Q_DEPTH_HIGH: MQLONG = 2224;
pub const MQRC_Q_DEPTH_LOW: MQLONG = 2225;
pub const MQRC_Q_SERVICE_INTERVAL_HIGH: MQLONG = 2226;
pub const MQRC_Q_SERVICE_INTERVAL_OK: MQLONG = 2227;
pub const MQRC_RFH_HEADER_FIELD_ERROR: MQLONG = 2228;
pub const MQRC_RAS_PROPERTY_ERROR: MQLONG = 2229;
pub const MQRC_UNIT_OF_WORK_NOT_STARTED: MQLONG = 2232;
pub const MQRC_CHANNEL_AUTO_DEF_OK: MQLONG = 2233;
pub const MQRC_CHANNEL_AUTO_DEF_ERROR: MQLONG = 2234;
pub const MQRC_CFH_ERROR: MQLONG = 2235;
pub const MQRC_CFIL_ERROR: MQLONG = 2236;
pub const MQRC_CFIN_ERROR: MQLONG = 2237;
pub const MQRC_CFSL_ERROR: MQLONG = 2238;
pub const MQRC_CFST_ERROR: MQLONG = 2239;
pub const MQRC_INCOMPLETE_GROUP: MQLONG = 2241;
pub const MQRC_INCOMPLETE_MSG: MQLONG = 2242;
pub const MQRC_INCONSISTENT_CCSIDS: MQLONG = 2243;
pub const MQRC_INCONSISTENT_ENCODINGS: MQLONG = 2244;
pub const MQRC_INCONSISTENT_UOW: MQLONG = 2245;
pub const MQRC_INVALID_MSG_UNDER_CURSOR: MQLONG = 2246;
pub const MQRC_MATCH_OPTIONS_ERROR: MQLONG = 2247;
pub const MQRC_MDE_ERROR: MQLONG = 2248;
pub const MQRC_MSG_FLAGS_ERROR: MQLONG = 2249;
pub const MQRC_MSG_SEQ_NUMBER_ERROR: MQLONG = 2250;
pub const MQRC_OFFSET_ERROR: MQLONG = 2251;
pub const MQRC_ORIGINAL_LENGTH_ERROR: MQLONG = 2252;
pub const MQRC_SEGMENT_LENGTH_ZERO: MQLONG = 2253;
pub const MQRC_UOW_NOT_AVAILABLE: MQLONG = 2255;
pub const MQRC_WRONG_GMO_VERSION: MQLONG = 2256;
pub const MQRC_WRONG_MD_VERSION: MQLONG = 2257;
pub const MQRC_GROUP_ID_ERROR: MQLONG = 2258;
pub const MQRC_INCONSISTENT_BROWSE: MQLONG = 2259;
pub const MQRC_XQH_ERROR: MQLONG = 2260;
pub const MQRC_SRC_ENV_ERROR: MQLONG = 2261;
pub const MQRC_SRC_NAME_ERROR: MQLONG = 2262;
pub const MQRC_DEST_ENV_ERROR: MQLONG = 2263;
pub const MQRC_DEST_NAME_ERROR: MQLONG = 2264;
pub const MQRC_TM_ERROR: MQLONG = 2265;
pub const MQRC_CLUSTER_EXIT_ERROR: MQLONG = 2266;
pub const MQRC_CLUSTER_EXIT_LOAD_ERROR: MQLONG = 2267;
pub const MQRC_CLUSTER_PUT_INHIBITED: MQLONG = 2268;
pub const MQRC_CLUSTER_RESOURCE_ERROR: MQLONG = 2269;
pub const MQRC_NO_DESTINATIONS_AVAILABLE: MQLONG = 2270;
pub const MQRC_CONN_TAG_IN_USE: MQLONG = 2271;
pub const MQRC_PARTIALLY_CONVERTED: MQLONG = 2272;
pub const MQRC_CONNECTION_ERROR: MQLONG = 2273;
pub const MQRC_OPTION_ENVIRONMENT_ERROR: MQLONG = 2274;
pub const MQRC_CD_ERROR: MQLONG = 2277;
pub const MQRC_CLIENT_CONN_ERROR: MQLONG = 2278;
pub const MQRC_CHANNEL_STOPPED_BY_USER: MQLONG = 2279;
pub const MQRC_HCONFIG_ERROR: MQLONG = 2280;
pub const MQRC_FUNCTION_ERROR: MQLONG = 2281;
pub const MQRC_CHANNEL_STARTED: MQLONG = 2282;
pub const MQRC_CHANNEL_STOPPED: MQLONG = 2283;
pub const MQRC_CHANNEL_CONV_ERROR: MQLONG = 2284;
pub const MQRC_SERVICE_NOT_AVAILABLE: MQLONG = 2285;
pub const MQRC_INITIALIZATION_FAILED: MQLONG = 2286;
pub const MQRC_TERMINATION_FAILED: MQLONG = 2287;
pub const MQRC_UNKNOWN_Q_NAME: MQLONG = 2288;
pub const MQRC_SERVICE_ERROR: MQLONG = 2289;
pub const MQRC_Q_ALREADY_EXISTS: MQLONG = 2290;
pub const MQRC_USER_ID_NOT_AVAILABLE: MQLONG = 2291;
pub const MQRC_UNKNOWN_ENTITY: MQLONG = 2292;
pub const MQRC_UNKNOWN_AUTH_ENTITY: MQLONG = 2293;
pub const MQRC_UNKNOWN_REF_OBJECT: MQLONG = 2294;
pub const MQRC_CHANNEL_ACTIVATED: MQLONG = 2295;
pub const MQRC_CHANNEL_NOT_ACTIVATED: MQLONG = 2296;
pub const MQRC_UOW_CANCELED: MQLONG = 2297;
pub const MQRC_FUNCTION_NOT_SUPPORTED: MQLONG = 2298;
pub const MQRC_SELECTOR_TYPE_ERROR: MQLONG = 2299;
pub const MQRC_COMMAND_TYPE_ERROR: MQLONG = 2300;
pub const MQRC_MULTIPLE_INSTANCE_ERROR: MQLONG = 2301;
pub const MQRC_SYSTEM_ITEM_NOT_ALTERABLE: MQLONG = 2302;
pub const MQRC_BAG_CONVERSION_ERROR: MQLONG = 2303;
pub const MQRC_SELECTOR_OUT_OF_RANGE: MQLONG = 2304;
pub const MQRC_SELECTOR_NOT_UNIQUE: MQLONG = 2305;
pub const MQRC_INDEX_NOT_PRESENT: MQLONG = 2306;
pub const MQRC_STRING_ERROR: MQLONG = 2307;
pub const MQRC_ENCODING_NOT_SUPPORTED: MQLONG = 2308;
pub const MQRC_SELECTOR_NOT_PRESENT: MQLONG = 2309;
pub const MQRC_OUT_SELECTOR_ERROR: MQLONG = 2310;
pub const MQRC_STRING_TRUNCATED: MQLONG = 2311;
pub const MQRC_SELECTOR_WRONG_TYPE: MQLONG = 2312;
pub const MQRC_INCONSISTENT_ITEM_TYPE: MQLONG = 2313;
pub const MQRC_INDEX_ERROR: MQLONG = 2314;
pub const MQRC_SYSTEM_BAG_NOT_ALTERABLE: MQLONG = 2315;
pub const MQRC_ITEM_COUNT_ERROR: MQLONG = 2316;
pub const MQRC_FORMAT_NOT_SUPPORTED: MQLONG = 2317;
pub const MQRC_SELECTOR_NOT_SUPPORTED: MQLONG = 2318;
pub const MQRC_ITEM_VALUE_ERROR: MQLONG = 2319;
pub const MQRC_HBAG_ERROR: MQLONG = 2320;
pub const MQRC_PARAMETER_MISSING: MQLONG = 2321;
pub const MQRC_CMD_SERVER_NOT_AVAILABLE: MQLONG = 2322;
pub const MQRC_STRING_LENGTH_ERROR: MQLONG = 2323;
pub const MQRC_INQUIRY_COMMAND_ERROR: MQLONG = 2324;
pub const MQRC_NESTED_BAG_NOT_SUPPORTED: MQLONG = 2325;
pub const MQRC_BAG_WRONG_TYPE: MQLONG = 2326;
pub const MQRC_ITEM_TYPE_ERROR: MQLONG = 2327;
pub const MQRC_SYSTEM_BAG_NOT_DELETABLE: MQLONG = 2328;
pub const MQRC_SYSTEM_ITEM_NOT_DELETABLE: MQLONG = 2329;
pub const MQRC_CODED_CHAR_SET_ID_ERROR: MQLONG = 2330;
pub const MQRC_MSG_TOKEN_ERROR: MQLONG = 2331;
pub const MQRC_MISSING_WIH: MQLONG = 2332;
pub const MQRC_WIH_ERROR: MQLONG = 2333;
pub const MQRC_RFH_ERROR: MQLONG = 2334;
pub const MQRC_RFH_STRING_ERROR: MQLONG = 2335;
pub const MQRC_RFH_COMMAND_ERROR: MQLONG = 2336;
pub const MQRC_RFH_PARM_ERROR: MQLONG = 2337;
pub const MQRC_RFH_DUPLICATE_PARM: MQLONG = 2338;
pub const MQRC_RFH_PARM_MISSING: MQLONG = 2339;
pub const MQRC_CHAR_CONVERSION_ERROR: MQLONG = 2340;
pub const MQRC_UCS2_CONVERSION_ERROR: MQLONG = 2341;
pub const MQRC_DB2_NOT_AVAILABLE: MQLONG = 2342;
pub const MQRC_OBJECT_NOT_UNIQUE: MQLONG = 2343;
pub const MQRC_CONN_TAG_NOT_RELEASED: MQLONG = 2344;
pub const MQRC_CF_NOT_AVAILABLE: MQLONG = 2345;
pub const MQRC_CF_STRUC_IN_USE: MQLONG = 2346;
pub const MQRC_CF_STRUC_LIST_HDR_IN_USE: MQLONG = 2347;
pub const MQRC_CF_STRUC_AUTH_FAILED: MQLONG = 2348;
pub const MQRC_CF_STRUC_ERROR: MQLONG = 2349;
pub const MQRC_CONN_TAG_NOT_USABLE: MQLONG = 2350;
pub const MQRC_GLOBAL_UOW_CONFLICT: MQLONG = 2351;
pub const MQRC_LOCAL_UOW_CONFLICT: MQLONG = 2352;
pub const MQRC_HANDLE_IN_USE_FOR_UOW: MQLONG = 2353;
pub const MQRC_UOW_ENLISTMENT_ERROR: MQLONG = 2354;
pub const MQRC_UOW_MIX_NOT_SUPPORTED: MQLONG = 2355;
pub const MQRC_WXP_ERROR: MQLONG = 2356;
pub const MQRC_CURRENT_RECORD_ERROR: MQLONG = 2357;
pub const MQRC_NEXT_OFFSET_ERROR: MQLONG = 2358;
pub const MQRC_NO_RECORD_AVAILABLE: MQLONG = 2359;
pub const MQRC_OBJECT_LEVEL_INCOMPATIBLE: MQLONG = 2360;
pub const MQRC_NEXT_RECORD_ERROR: MQLONG = 2361;
pub const MQRC_BACKOUT_THRESHOLD_REACHED: MQLONG = 2362;
pub const MQRC_MSG_NOT_MATCHED: MQLONG = 2363;
pub const MQRC_JMS_FORMAT_ERROR: MQLONG = 2364;
pub const MQRC_SEGMENTS_NOT_SUPPORTED: MQLONG = 2365;
pub const MQRC_WRONG_CF_LEVEL: MQLONG = 2366;
pub const MQRC_CONFIG_CREATE_OBJECT: MQLONG = 2367;
pub const MQRC_CONFIG_CHANGE_OBJECT: MQLONG = 2368;
pub const MQRC_CONFIG_DELETE_OBJECT: MQLONG = 2369;
pub const MQRC_CONFIG_REFRESH_OBJECT: MQLONG = 2370;
pub const MQRC_CHANNEL_SSL_ERROR: MQLONG = 2371;
pub const MQRC_PARTICIPANT_NOT_DEFINED: MQLONG = 2372;
pub const MQRC_CF_STRUC_FAILED: MQLONG = 2373;
pub const MQRC_API_EXIT_ERROR: MQLONG = 2374;
pub const MQRC_API_EXIT_INIT_ERROR: MQLONG = 2375;
pub const MQRC_API_EXIT_TERM_ERROR: MQLONG = 2376;
pub const MQRC_EXIT_REASON_ERROR: MQLONG = 2377;
pub const MQRC_RESERVED_VALUE_ERROR: MQLONG = 2378;
pub const MQRC_NO_DATA_AVAILABLE: MQLONG = 2379;
pub const MQRC_SCO_ERROR: MQLONG = 2380;
pub const MQRC_KEY_REPOSITORY_ERROR: MQLONG = 2381;
pub const MQRC_CRYPTO_HARDWARE_ERROR: MQLONG = 2382;
pub const MQRC_AUTH_INFO_REC_COUNT_ERROR: MQLONG = 2383;
pub const MQRC_AUTH_INFO_REC_ERROR: MQLONG = 2384;
pub const MQRC_AIR_ERROR: MQLONG = 2385;
pub const MQRC_AUTH_INFO_TYPE_ERROR: MQLONG = 2386;
pub const MQRC_AUTH_INFO_CONN_NAME_ERROR: MQLONG = 2387;
pub const MQRC_LDAP_USER_NAME_ERROR: MQLONG = 2388;
pub const MQRC_LDAP_USER_NAME_LENGTH_ERR: MQLONG = 2389;
pub const MQRC_LDAP_PASSWORD_ERROR: MQLONG = 2390;
pub const MQRC_SSL_ALREADY_INITIALIZED: MQLONG = 2391;
pub const MQRC_SSL_CONFIG_ERROR: MQLONG = 2392;
pub const MQRC_SSL_INITIALIZATION_ERROR: MQLONG = 2393;
pub const MQRC_Q_INDEX_TYPE_ERROR: MQLONG = 2394;
pub const MQRC_CFBS_ERROR: MQLONG = 2395;
pub const MQRC_SSL_NOT_ALLOWED: MQLONG = 2396;
pub const MQRC_JSSE_ERROR: MQLONG = 2397;
pub const MQRC_SSL_PEER_NAME_MISMATCH: MQLONG = 2398;
pub const MQRC_SSL_PEER_NAME_ERROR: MQLONG = 2399;
pub const MQRC_UNSUPPORTED_CIPHER_SUITE: MQLONG = 2400;
pub const MQRC_SSL_CERTIFICATE_REVOKED: MQLONG = 2401;
pub const MQRC_SSL_CERT_STORE_ERROR: MQLONG = 2402;
pub const MQRC_CLIENT_EXIT_LOAD_ERROR: MQLONG = 2406;
pub const MQRC_CLIENT_EXIT_ERROR: MQLONG = 2407;
pub const MQRC_UOW_COMMITTED: MQLONG = 2408;
pub const MQRC_SSL_KEY_RESET_ERROR: MQLONG = 2409;
pub const MQRC_UNKNOWN_COMPONENT_NAME: MQLONG = 2410;
pub const MQRC_LOGGER_STATUS: MQLONG = 2411;
pub const MQRC_COMMAND_MQSC: MQLONG = 2412;
pub const MQRC_COMMAND_PCF: MQLONG = 2413;
pub const MQRC_CFIF_ERROR: MQLONG = 2414;
pub const MQRC_CFSF_ERROR: MQLONG = 2415;
pub const MQRC_CFGR_ERROR: MQLONG = 2416;
pub const MQRC_MSG_NOT_ALLOWED_IN_GROUP: MQLONG = 2417;
pub const MQRC_FILTER_OPERATOR_ERROR: MQLONG = 2418;
pub const MQRC_NESTED_SELECTOR_ERROR: MQLONG = 2419;
pub const MQRC_EPH_ERROR: MQLONG = 2420;
pub const MQRC_RFH_FORMAT_ERROR: MQLONG = 2421;
pub const MQRC_CFBF_ERROR: MQLONG = 2422;
pub const MQRC_CLIENT_CHANNEL_CONFLICT: MQLONG = 2423;
pub const MQRC_SD_ERROR: MQLONG = 2424;
pub const MQRC_TOPIC_STRING_ERROR: MQLONG = 2425;
pub const MQRC_STS_ERROR: MQLONG = 2426;
pub const MQRC_NO_SUBSCRIPTION: MQLONG = 2428;
pub const MQRC_SUBSCRIPTION_IN_USE: MQLONG = 2429;
pub const MQRC_STAT_TYPE_ERROR: MQLONG = 2430;
pub const MQRC_SUB_USER_DATA_ERROR: MQLONG = 2431;
pub const MQRC_SUB_ALREADY_EXISTS: MQLONG = 2432;
pub const MQRC_IDENTITY_MISMATCH: MQLONG = 2434;
pub const MQRC_ALTER_SUB_ERROR: MQLONG = 2435;
pub const MQRC_DURABILITY_NOT_ALLOWED: MQLONG = 2436;
pub const MQRC_NO_RETAINED_MSG: MQLONG = 2437;
pub const MQRC_SRO_ERROR: MQLONG = 2438;
pub const MQRC_SUB_NAME_ERROR: MQLONG = 2440;
pub const MQRC_OBJECT_STRING_ERROR: MQLONG = 2441;
pub const MQRC_PROPERTY_NAME_ERROR: MQLONG = 2442;
pub const MQRC_SEGMENTATION_NOT_ALLOWED: MQLONG = 2443;
pub const MQRC_CBD_ERROR: MQLONG = 2444;
pub const MQRC_CTLO_ERROR: MQLONG = 2445;
pub const MQRC_NO_CALLBACKS_ACTIVE: MQLONG = 2446;
pub const MQRC_CALLBACK_NOT_REGISTERED: MQLONG = 2448;
pub const MQRC_OPTIONS_CHANGED: MQLONG = 2457;
pub const MQRC_READ_AHEAD_MSGS: MQLONG = 2458;
pub const MQRC_SELECTOR_SYNTAX_ERROR: MQLONG = 2459;
pub const MQRC_HMSG_ERROR: MQLONG = 2460;
pub const MQRC_CMHO_ERROR: MQLONG = 2461;
pub const MQRC_DMHO_ERROR: MQLONG = 2462;
pub const MQRC_SMPO_ERROR: MQLONG = 2463;
pub const MQRC_IMPO_ERROR: MQLONG = 2464;
pub const MQRC_PROPERTY_NAME_TOO_BIG: MQLONG = 2465;
pub const MQRC_PROP_VALUE_NOT_CONVERTED: MQLONG = 2466;
pub const MQRC_PROP_TYPE_NOT_SUPPORTED: MQLONG = 2467;
pub const MQRC_PROPERTY_VALUE_TOO_BIG: MQLONG = 2469;
pub const MQRC_PROP_CONV_NOT_SUPPORTED: MQLONG = 2470;
pub const MQRC_PROPERTY_NOT_AVAILABLE: MQLONG = 2471;
pub const MQRC_PROP_NUMBER_FORMAT_ERROR: MQLONG = 2472;
pub const MQRC_PROPERTY_TYPE_ERROR: MQLONG = 2473;
pub const MQRC_PROPERTIES_TOO_BIG: MQLONG = 2478;
pub const MQRC_PUT_NOT_RETAINED: MQLONG = 2479;
pub const MQRC_ALIAS_TARGTYPE_CHANGED: MQLONG = 2480;
pub const MQRC_DMPO_ERROR: MQLONG = 2481;
pub const MQRC_PD_ERROR: MQLONG = 2482;
pub const MQRC_CALLBACK_TYPE_ERROR: MQLONG = 2483;
pub const MQRC_CBD_OPTIONS_ERROR: MQLONG = 2484;
pub const MQRC_MAX_MSG_LENGTH_ERROR: MQLONG = 2485;
pub const MQRC_CALLBACK_ROUTINE_ERROR: MQLONG = 2486;
pub const MQRC_CALLBACK_LINK_ERROR: MQLONG = 2487;
pub const MQRC_OPERATION_ERROR: MQLONG = 2488;
pub const MQRC_BMHO_ERROR: MQLONG = 2489;
pub const MQRC_UNSUPPORTED_PROPERTY: MQLONG = 2490;
pub const MQRC_MSG_LENGTH_ERROR: MQLONG = 2491;
pub const MQRC_PROP_NAME_NOT_CONVERTED: MQLONG = 2492;
pub const MQRC_GET_ENABLED: MQLONG = 2494;
pub const MQRC_MODULE_NOT_FOUND: MQLONG = 2495;
pub const MQRC_MODULE_INVALID: MQLONG = 2496;
pub const MQRC_MODULE_ENTRY_NOT_FOUND: MQLONG = 2497;
pub const MQRC_MIXED_CONTENT_NOT_ALLOWED: MQLONG = 2498;
pub const MQRC_MSG_HANDLE_IN_USE: MQLONG = 2499;
pub const MQRC_HCONN_ASYNC_ACTIVE: MQLONG = 2500;
pub const MQRC_MHBO_ERROR: MQLONG = 2501;
pub const MQRC_PUBLICATION_FAILURE: MQLONG = 2502;
pub const MQRC_SUB_INHIBITED: MQLONG = 2503;
pub const MQRC_SELECTOR_ALWAYS_FALSE: MQLONG = 2504;
pub const MQRC_XEPO_ERROR: MQLONG = 2507;
pub const MQRC_DURABILITY_NOT_ALTERABLE: MQLONG = 2509;
pub const MQRC_TOPIC_NOT_ALTERABLE: MQLONG = 2510;
pub const MQRC_SUBLEVEL_NOT_ALTERABLE: MQLONG = 2512;
pub const MQRC_PROPERTY_NAME_LENGTH_ERR: MQLONG = 2513;
pub const MQRC_DUPLICATE_GROUP_SUB: MQLONG = 2514;
pub const MQRC_GROUPING_NOT_ALTERABLE: MQLONG = 2515;
pub const MQRC_SELECTOR_INVALID_FOR_TYPE: MQLONG = 2516;
pub const MQRC_HOBJ_QUIESCED: MQLONG = 2517;
pub const MQRC_HOBJ_QUIESCED_NO_MSGS: MQLONG = 2518;
pub const MQRC_SELECTION_STRING_ERROR: MQLONG = 2519;
pub const MQRC_RES_OBJECT_STRING_ERROR: MQLONG = 2520;
pub const MQRC_CONNECTION_SUSPENDED: MQLONG = 2521;
pub const MQRC_INVALID_DESTINATION: MQLONG = 2522;
pub const MQRC_INVALID_SUBSCRIPTION: MQLONG = 2523;
pub const MQRC_SELECTOR_NOT_ALTERABLE: MQLONG = 2524;
pub const MQRC_RETAINED_MSG_Q_ERROR: MQLONG = 2525;
pub const MQRC_RETAINED_NOT_DELIVERED: MQLONG = 2526;
pub const MQRC_RFH_RESTRICTED_FORMAT_ERR: MQLONG = 2527;
pub const MQRC_CONNECTION_STOPPED: MQLONG = 2528;
pub const MQRC_ASYNC_UOW_CONFLICT: MQLONG = 2529;
pub const MQRC_ASYNC_XA_CONFLICT: MQLONG = 2530;
pub const MQRC_PUBSUB_INHIBITED: MQLONG = 2531;
pub const MQRC_MSG_HANDLE_COPY_FAILURE: MQLONG = 2532;
pub const MQRC_DEST_CLASS_NOT_ALTERABLE: MQLONG = 2533;
pub const MQRC_OPERATION_NOT_ALLOWED: MQLONG = 2534;
pub const MQRC_ACTION_ERROR: MQLONG = 2535;
pub const MQRC_CHANNEL_NOT_AVAILABLE: MQLONG = 2537;
pub const MQRC_HOST_NOT_AVAILABLE: MQLONG = 2538;
pub const MQRC_CHANNEL_CONFIG_ERROR: MQLONG = 2539;
pub const MQRC_UNKNOWN_CHANNEL_NAME: MQLONG = 2540;
pub const MQRC_LOOPING_PUBLICATION: MQLONG = 2541;
pub const MQRC_ALREADY_JOINED: MQLONG = 2542;
pub const MQRC_STANDBY_Q_MGR: MQLONG = 2543;
pub const MQRC_RECONNECTING: MQLONG = 2544;
pub const MQRC_RECONNECTED: MQLONG = 2545;
pub const MQRC_RECONNECT_QMID_MISMATCH: MQLONG = 2546;
pub const MQRC_RECONNECT_INCOMPATIBLE: MQLONG = 2547;
pub const MQRC_RECONNECT_FAILED: MQLONG = 2548;
pub const MQRC_CALL_INTERRUPTED: MQLONG = 2549;
pub const MQRC_NO_SUBS_MATCHED: MQLONG = 2550;
pub const MQRC_SELECTION_NOT_AVAILABLE: MQLONG = 2551;
pub const MQRC_CHANNEL_SSL_WARNING: MQLONG = 2552;
pub const MQRC_OCSP_URL_ERROR: MQLONG = 2553;
pub const MQRC_CONTENT_ERROR: MQLONG = 2554;
pub const MQRC_RECONNECT_Q_MGR_REQD: MQLONG = 2555;
pub const MQRC_RECONNECT_TIMED_OUT: MQLONG = 2556;
pub const MQRC_PUBLISH_EXIT_ERROR: MQLONG = 2557;
pub const MQRC_COMMINFO_ERROR: MQLONG = 2558;
pub const MQRC_DEF_SYNCPOINT_INHIBITED: MQLONG = 2559;
pub const MQRC_MULTICAST_ONLY: MQLONG = 2560;
pub const MQRC_DATA_SET_NOT_AVAILABLE: MQLONG = 2561;
pub const MQRC_GROUPING_NOT_ALLOWED: MQLONG = 2562;
pub const MQRC_GROUP_ADDRESS_ERROR: MQLONG = 2563;
pub const MQRC_MULTICAST_CONFIG_ERROR: MQLONG = 2564;
pub const MQRC_MULTICAST_INTERFACE_ERROR: MQLONG = 2565;
pub const MQRC_MULTICAST_SEND_ERROR: MQLONG = 2566;
pub const MQRC_MULTICAST_INTERNAL_ERROR: MQLONG = 2567;
pub const MQRC_CONNECTION_NOT_AVAILABLE: MQLONG = 2568;
pub const MQRC_SYNCPOINT_NOT_ALLOWED: MQLONG = 2569;
pub const MQRC_SSL_ALT_PROVIDER_REQUIRED: MQLONG = 2570;
pub const MQRC_MCAST_PUB_STATUS: MQLONG = 2571;
pub const MQRC_MCAST_SUB_STATUS: MQLONG = 2572;
pub const MQRC_PRECONN_EXIT_LOAD_ERROR: MQLONG = 2573;
pub const MQRC_PRECONN_EXIT_NOT_FOUND: MQLONG = 2574;
pub const MQRC_PRECONN_EXIT_ERROR: MQLONG = 2575;
pub const MQRC_CD_ARRAY_ERROR: MQLONG = 2576;
pub const MQRC_CHANNEL_BLOCKED: MQLONG = 2577;
pub const MQRC_CHANNEL_BLOCKED_WARNING: MQLONG = 2578;
pub const MQRC_SUBSCRIPTION_CREATE: MQLONG = 2579;
pub const MQRC_SUBSCRIPTION_DELETE: MQLONG = 2580;
pub const MQRC_SUBSCRIPTION_CHANGE: MQLONG = 2581;
pub const MQRC_SUBSCRIPTION_REFRESH: MQLONG = 2582;
pub const MQRC_INSTALLATION_MISMATCH: MQLONG = 2583;
pub const MQRC_NOT_PRIVILEGED: MQLONG = 2584;
pub const MQRC_PROPERTIES_DISABLED: MQLONG = 2586;
pub const MQRC_HMSG_NOT_AVAILABLE: MQLONG = 2587;
pub const MQRC_EXIT_PROPS_NOT_SUPPORTED: MQLONG = 2588;
pub const MQRC_INSTALLATION_MISSING: MQLONG = 2589;
pub const MQRC_FASTPATH_NOT_AVAILABLE: MQLONG = 2590;
pub const MQRC_CIPHER_SPEC_NOT_SUITE_B: MQLONG = 2591;
pub const MQRC_SUITE_B_ERROR: MQLONG = 2592;
pub const MQRC_CERT_VAL_POLICY_ERROR: MQLONG = 2593;
pub const MQRC_PASSWORD_PROTECTION_ERROR: MQLONG = 2594;
pub const MQRC_CSP_ERROR: MQLONG = 2595;
pub const MQRC_CERT_LABEL_NOT_ALLOWED: MQLONG = 2596;
pub const MQRC_ADMIN_TOPIC_STRING_ERROR: MQLONG = 2598;
pub const MQRC_AMQP_NOT_AVAILABLE: MQLONG = 2599;
pub const MQRC_CCDT_URL_ERROR: MQLONG = 2600;
pub const MQRC_Q_MGR_RECONNECT_REQUESTED: MQLONG = 2601;
pub const MQRC_BNO_ERROR: MQLONG = 2602;
pub const MQRC_OUTBOUND_SNI_NOT_VALID: MQLONG = 2603;
pub const MQRC_HTTPS_KEYSTORE_ERROR: MQLONG = 2604;
pub const MQRC_REOPEN_EXCL_INPUT_ERROR: MQLONG = 6100;
pub const MQRC_REOPEN_INQUIRE_ERROR: MQLONG = 6101;
pub const MQRC_REOPEN_SAVED_CONTEXT_ERR: MQLONG = 6102;
pub const MQRC_REOPEN_TEMPORARY_Q_ERROR: MQLONG = 6103;
pub const MQRC_ATTRIBUTE_LOCKED: MQLONG = 6104;
pub const MQRC_CURSOR_NOT_VALID: MQLONG = 6105;
pub const MQRC_ENCODING_ERROR: MQLONG = 6106;
pub const MQRC_STRUC_ID_ERROR: MQLONG = 6107;
pub const MQRC_NULL_POINTER: MQLONG = 6108;
pub const MQRC_NO_CONNECTION_REFERENCE: MQLONG = 6109;
pub const MQRC_NO_BUFFER: MQLONG = 6110;
pub const MQRC_BINARY_DATA_LENGTH_ERROR: MQLONG = 6111;
pub const MQRC_BUFFER_NOT_AUTOMATIC: MQLONG = 6112;
pub const MQRC_INSUFFICIENT_BUFFER: MQLONG = 6113;
pub const MQRC_INSUFFICIENT_DATA: MQLONG = 6114;
pub const MQRC_DATA_TRUNCATED: MQLONG = 6115;
pub const MQRC_ZERO_LENGTH: MQLONG = 6116;
pub const MQRC_NEGATIVE_LENGTH: MQLONG = 6117;
pub const MQRC_NEGATIVE_OFFSET: MQLONG = 6118;
pub const MQRC_INCONSISTENT_FORMAT: MQLONG = 6119;
pub const MQRC_INCONSISTENT_OBJECT_STATE: MQLONG = 6120;
pub const MQRC_CONTEXT_OBJECT_NOT_VALID: MQLONG = 6121;
pub const MQRC_CONTEXT_OPEN_ERROR: MQLONG = 6122;
pub const MQRC_STRUC_LENGTH_ERROR: MQLONG = 6123;
pub const MQRC_NOT_CONNECTED: MQLONG = 6124;
pub const MQRC_NOT_OPEN: MQLONG = 6125;
pub const MQRC_DISTRIBUTION_LIST_EMPTY: MQLONG = 6126;
pub const MQRC_INCONSISTENT_OPEN_OPTIONS: MQLONG = 6127;
pub const MQRC_WRONG_VERSION: MQLONG = 6128;
pub const MQRC_REFERENCE_ERROR: MQLONG = 6129;
pub const MQRC_XR_NOT_AVAILABLE: MQLONG = 6130;
pub const MQRC_SUB_JOIN_NOT_ALTERABLE: MQLONG = 29440;
pub const MQQT_LOCAL: MQLONG = 1;
pub const MQQT_MODEL: MQLONG = 2;
pub const MQQT_ALIAS: MQLONG = 3;
pub const MQQT_REMOTE: MQLONG = 6;
pub const MQQT_CLUSTER: MQLONG = 7;
pub const MQCQT_LOCAL_Q: MQLONG = 1;
pub const MQCQT_ALIAS_Q: MQLONG = 2;
pub const MQCQT_REMOTE_Q: MQLONG = 3;
pub const MQCQT_Q_MGR_ALIAS: MQLONG = 4;
pub const MQQT_ALL: MQLONG = 1001;
pub const MQQDT_PREDEFINED: MQLONG = 1;
pub const MQQDT_PERMANENT_DYNAMIC: MQLONG = 2;
pub const MQQDT_TEMPORARY_DYNAMIC: MQLONG = 3;
pub const MQQDT_SHARED_DYNAMIC: MQLONG = 4;
pub const MQQA_GET_INHIBITED: MQLONG = 1;
pub const MQQA_GET_ALLOWED: MQLONG = 0;
pub const MQQA_PUT_INHIBITED: MQLONG = 1;
pub const MQQA_PUT_ALLOWED: MQLONG = 0;
pub const MQQA_SHAREABLE: MQLONG = 1;
pub const MQQA_NOT_SHAREABLE: MQLONG = 0;
pub const MQQA_BACKOUT_HARDENED: MQLONG = 1;
pub const MQQA_BACKOUT_NOT_HARDENED: MQLONG = 0;
pub const MQMDS_PRIORITY: MQLONG = 0;
pub const MQMDS_FIFO: MQLONG = 1;
pub const MQNPM_CLASS_NORMAL: MQLONG = 0;
pub const MQNPM_CLASS_HIGH: MQLONG = 10;
pub const MQTC_OFF: MQLONG = 0;
pub const MQTC_ON: MQLONG = 1;
pub const MQTT_NONE: MQLONG = 0;
pub const MQTT_FIRST: MQLONG = 1;
pub const MQTT_EVERY: MQLONG = 2;
pub const MQTT_DEPTH: MQLONG = 3;
pub const MQTRIGGER_RESTART_NO: MQLONG = 0;
pub const MQTRIGGER_RESTART_YES: MQLONG = 1;
pub const MQUS_NORMAL: MQLONG = 0;
pub const MQUS_TRANSMISSION: MQLONG = 1;
pub const MQDL_SUPPORTED: MQLONG = 1;
pub const MQDL_NOT_SUPPORTED: MQLONG = 0;
pub const MQIT_NONE: MQLONG = 0;
pub const MQIT_MSG_ID: MQLONG = 1;
pub const MQIT_CORREL_ID: MQLONG = 2;
pub const MQIT_MSG_TOKEN: MQLONG = 4;
pub const MQIT_GROUP_ID: MQLONG = 5;
pub const MQBND_BIND_ON_OPEN: MQLONG = 0;
pub const MQBND_BIND_NOT_FIXED: MQLONG = 1;
pub const MQBND_BIND_ON_GROUP: MQLONG = 2;
pub const MQQSGD_ALL: MQLONG = -1;
pub const MQQSGD_Q_MGR: MQLONG = 0;
pub const MQQSGD_COPY: MQLONG = 1;
pub const MQQSGD_SHARED: MQLONG = 2;
pub const MQQSGD_GROUP: MQLONG = 3;
pub const MQQSGD_PRIVATE: MQLONG = 4;
pub const MQQSGD_LIVE: MQLONG = 6;
pub const MQREORG_DISABLED: MQLONG = 0;
pub const MQREORG_ENABLED: MQLONG = 1;
pub const MQQFS_DEFAULT: MQLONG = -1;
pub const MQCEX_NOLIMIT: MQLONG = -1;
pub const MQCEX_AS_PARENT: MQLONG = -2;
pub const MQOTEL_TRACE_QMGR: MQLONG = 0;
pub const MQOTEL_TRACE_OFF: MQLONG = 1;
pub const MQOTEL_TRACE_ON: MQLONG = 2;
pub const MQOTEL_TRACE_NONE: MQLONG = 3;
pub const MQOTEL_TRACE_AS_PARENT: MQLONG = 4;
pub const MQOTEL_PCTL_QMGR: MQLONG = 0;
pub const MQOTEL_PCTL_MANUAL: MQLONG = 1;
pub const MQOTEL_PCTL_AUTO: MQLONG = 2;
pub const MQOTEL_PCTL_AS_PARENT: MQLONG = 3;
pub const MQREADA_NO: MQLONG = 0;
pub const MQREADA_YES: MQLONG = 1;
pub const MQREADA_DISABLED: MQLONG = 2;
pub const MQREADA_INHIBITED: MQLONG = 3;
pub const MQREADA_BACKLOG: MQLONG = 4;
pub const MQPROP_COMPATIBILITY: MQLONG = 0;
pub const MQPROP_NONE: MQLONG = 1;
pub const MQPROP_ALL: MQLONG = 2;
pub const MQPROP_FORCE_MQRFH2: MQLONG = 3;
pub const MQPROP_V6COMPAT: MQLONG = 4;
pub const MQST_BEST_EFFORT: MQLONG = 0;
pub const MQST_MUST_DUP: MQLONG = 1;
pub const MQNC_MAX_NAMELIST_NAME_COUNT: MQLONG = 256;
pub const MQNT_NONE: MQLONG = 0;
pub const MQNT_Q: MQLONG = 1;
pub const MQNT_CLUSTER: MQLONG = 2;
pub const MQNT_AUTH_INFO: MQLONG = 4;
pub const MQNT_ALL: MQLONG = 1001;
pub const MQCFR_YES: MQLONG = 1;
pub const MQCFR_NO: MQLONG = 0;
pub const MQRECAUTO_NO: MQLONG = 0;
pub const MQRECAUTO_YES: MQLONG = 1;
pub const MQCFCONLOS_TERMINATE: MQLONG = 0;
pub const MQCFCONLOS_TOLERATE: MQLONG = 1;
pub const MQCFCONLOS_ASQMGR: MQLONG = 2;
pub const MQSVC_TYPE_COMMAND: MQLONG = 0;
pub const MQSVC_TYPE_SERVER: MQLONG = 1;
pub const MQADOPT_CHECK_NONE: MQLONG = 0;
pub const MQADOPT_CHECK_ALL: MQLONG = 1;
pub const MQADOPT_CHECK_Q_MGR_NAME: MQLONG = 2;
pub const MQADOPT_CHECK_NET_ADDR: MQLONG = 4;
pub const MQADOPT_CHECK_CHANNEL_NAME: MQLONG = 8;
pub const MQADOPT_TYPE_NO: MQLONG = 0;
pub const MQADOPT_TYPE_ALL: MQLONG = 1;
pub const MQADOPT_TYPE_SVR: MQLONG = 2;
pub const MQADOPT_TYPE_SDR: MQLONG = 4;
pub const MQADOPT_TYPE_RCVR: MQLONG = 8;
pub const MQADOPT_TYPE_CLUSRCVR: MQLONG = 16;
pub const MQAUTO_START_NO: MQLONG = 0;
pub const MQAUTO_START_YES: MQLONG = 1;
pub const MQCHAD_DISABLED: MQLONG = 0;
pub const MQCHAD_ENABLED: MQLONG = 1;
pub const MQCLWL_USEQ_LOCAL: MQLONG = 0;
pub const MQCLWL_USEQ_ANY: MQLONG = 1;
pub const MQCLWL_USEQ_AS_Q_MGR: MQLONG = -3;
pub const MQCMDL_LEVEL_1: MQLONG = 100;
pub const MQCMDL_LEVEL_101: MQLONG = 101;
pub const MQCMDL_LEVEL_110: MQLONG = 110;
pub const MQCMDL_LEVEL_114: MQLONG = 114;
pub const MQCMDL_LEVEL_120: MQLONG = 120;
pub const MQCMDL_LEVEL_200: MQLONG = 200;
pub const MQCMDL_LEVEL_201: MQLONG = 201;
pub const MQCMDL_LEVEL_210: MQLONG = 210;
pub const MQCMDL_LEVEL_211: MQLONG = 211;
pub const MQCMDL_LEVEL_220: MQLONG = 220;
pub const MQCMDL_LEVEL_221: MQLONG = 221;
pub const MQCMDL_LEVEL_230: MQLONG = 230;
pub const MQCMDL_LEVEL_320: MQLONG = 320;
pub const MQCMDL_LEVEL_420: MQLONG = 420;
pub const MQCMDL_LEVEL_500: MQLONG = 500;
pub const MQCMDL_LEVEL_510: MQLONG = 510;
pub const MQCMDL_LEVEL_520: MQLONG = 520;
pub const MQCMDL_LEVEL_530: MQLONG = 530;
pub const MQCMDL_LEVEL_531: MQLONG = 531;
pub const MQCMDL_LEVEL_600: MQLONG = 600;
pub const MQCMDL_LEVEL_700: MQLONG = 700;
pub const MQCMDL_LEVEL_701: MQLONG = 701;
pub const MQCMDL_LEVEL_710: MQLONG = 710;
pub const MQCMDL_LEVEL_711: MQLONG = 711;
pub const MQCMDL_LEVEL_750: MQLONG = 750;
pub const MQCMDL_LEVEL_800: MQLONG = 800;
pub const MQCMDL_LEVEL_801: MQLONG = 801;
pub const MQCMDL_LEVEL_802: MQLONG = 802;
pub const MQCMDL_LEVEL_900: MQLONG = 900;
pub const MQCMDL_LEVEL_901: MQLONG = 901;
pub const MQCMDL_LEVEL_902: MQLONG = 902;
pub const MQCMDL_LEVEL_903: MQLONG = 903;
pub const MQCMDL_LEVEL_904: MQLONG = 904;
pub const MQCMDL_LEVEL_905: MQLONG = 905;
pub const MQCMDL_LEVEL_910: MQLONG = 910;
pub const MQCMDL_LEVEL_911: MQLONG = 911;
pub const MQCMDL_LEVEL_912: MQLONG = 912;
pub const MQCMDL_LEVEL_913: MQLONG = 913;
pub const MQCMDL_LEVEL_914: MQLONG = 914;
pub const MQCMDL_LEVEL_915: MQLONG = 915;
pub const MQCMDL_LEVEL_920: MQLONG = 920;
pub const MQCMDL_LEVEL_921: MQLONG = 921;
pub const MQCMDL_LEVEL_922: MQLONG = 922;
pub const MQCMDL_LEVEL_923: MQLONG = 923;
pub const MQCMDL_LEVEL_924: MQLONG = 924;
pub const MQCMDL_LEVEL_925: MQLONG = 925;
pub const MQCMDL_LEVEL_930: MQLONG = 930;
pub const MQCMDL_LEVEL_931: MQLONG = 931;
pub const MQCMDL_LEVEL_932: MQLONG = 932;
pub const MQCMDL_LEVEL_933: MQLONG = 933;
pub const MQCMDL_LEVEL_934: MQLONG = 934;
pub const MQCMDL_LEVEL_935: MQLONG = 935;
pub const MQCMDL_LEVEL_940: MQLONG = 940;
pub const MQCMDL_LEVEL_941: MQLONG = 941;
pub const MQCMDL_LEVEL_942: MQLONG = 942;
pub const MQCMDL_LEVEL_943: MQLONG = 943;
pub const MQCMDL_CURRENT_LEVEL: MQLONG = 943;
pub const MQCSRV_CONVERT_NO: MQLONG = 0;
pub const MQCSRV_CONVERT_YES: MQLONG = 1;
pub const MQCSRV_DLQ_NO: MQLONG = 0;
pub const MQCSRV_DLQ_YES: MQLONG = 1;
pub const MQDNSWLM_NO: MQLONG = 0;
pub const MQDNSWLM_YES: MQLONG = 1;
pub const MQEXPI_OFF: MQLONG = 0;
pub const MQIGQ_DISABLED: MQLONG = 0;
pub const MQIGQ_ENABLED: MQLONG = 1;
pub const MQIGQPA_DEFAULT: MQLONG = 1;
pub const MQIGQPA_CONTEXT: MQLONG = 2;
pub const MQIGQPA_ONLY_IGQ: MQLONG = 3;
pub const MQIGQPA_ALTERNATE_OR_IGQ: MQLONG = 4;
pub const MQIPADDR_IPV4: MQLONG = 0;
pub const MQIPADDR_IPV6: MQLONG = 1;
pub const MQMMBI_UNLIMITED: MQLONG = -1;
pub const MQMON_NOT_AVAILABLE: MQLONG = -1;
pub const MQMON_NONE: MQLONG = -1;
pub const MQMON_Q_MGR: MQLONG = -3;
pub const MQMON_OFF: MQLONG = 0;
pub const MQMON_ON: MQLONG = 1;
pub const MQMON_DISABLED: MQLONG = 0;
pub const MQMON_ENABLED: MQLONG = 1;
pub const MQMON_LOW: MQLONG = 17;
pub const MQMON_MEDIUM: MQLONG = 33;
pub const MQMON_HIGH: MQLONG = 65;
pub const MQFUN_TYPE_UNKNOWN: MQLONG = 0;
pub const MQFUN_TYPE_JVM: MQLONG = 1;
pub const MQFUN_TYPE_PROGRAM: MQLONG = 2;
pub const MQFUN_TYPE_PROCEDURE: MQLONG = 3;
pub const MQFUN_TYPE_USERDEF: MQLONG = 4;
pub const MQFUN_TYPE_COMMAND: MQLONG = 5;
pub const MQACTV_DETAIL_LOW: MQLONG = 1;
pub const MQACTV_DETAIL_MEDIUM: MQLONG = 2;
pub const MQACTV_DETAIL_HIGH: MQLONG = 3;
pub const MQPL_MVS: MQLONG = 1;
pub const MQPL_OS390: MQLONG = 1;
pub const MQPL_ZOS: MQLONG = 1;
pub const MQPL_OS2: MQLONG = 2;
pub const MQPL_AIX: MQLONG = 3;
pub const MQPL_UNIX: MQLONG = 3;
pub const MQPL_OS400: MQLONG = 4;
pub const MQPL_WINDOWS: MQLONG = 5;
pub const MQPL_WINDOWS_NT: MQLONG = 11;
pub const MQPL_VMS: MQLONG = 12;
pub const MQPL_NSK: MQLONG = 13;
pub const MQPL_NSS: MQLONG = 13;
pub const MQPL_OPEN_TP1: MQLONG = 15;
pub const MQPL_VM: MQLONG = 18;
pub const MQPL_TPF: MQLONG = 23;
pub const MQPL_VSE: MQLONG = 27;
pub const MQPL_APPLIANCE: MQLONG = 28;
pub const MQPL_NATIVE: MQLONG = 3;
pub const MQPROP_UNRESTRICTED_LENGTH: MQLONG = -1;
pub const MQPSM_DISABLED: MQLONG = 0;
pub const MQPSM_COMPAT: MQLONG = 1;
pub const MQPSM_ENABLED: MQLONG = 2;
pub const MQPSCLUS_DISABLED: MQLONG = 0;
pub const MQPSCLUS_ENABLED: MQLONG = 1;
pub const MQQMOPT_DISABLED: MQLONG = 0;
pub const MQQMOPT_ENABLED: MQLONG = 1;
pub const MQQMOPT_REPLY: MQLONG = 2;
pub const MQRCVTIME_MULTIPLY: MQLONG = 0;
pub const MQRCVTIME_ADD: MQLONG = 1;
pub const MQRCVTIME_EQUAL: MQLONG = 2;
pub const MQRECORDING_DISABLED: MQLONG = 0;
pub const MQRECORDING_Q: MQLONG = 1;
pub const MQRECORDING_MSG: MQLONG = 2;
pub const MQSCYC_UPPER: MQLONG = 0;
pub const MQSCYC_MIXED: MQLONG = 1;
pub const MQSQQM_USE: MQLONG = 0;
pub const MQSQQM_IGNORE: MQLONG = 1;
pub const MQSSL_FIPS_NO: MQLONG = 0;
pub const MQSSL_FIPS_YES: MQLONG = 1;
pub const MQSP_AVAILABLE: MQLONG = 1;
pub const MQSP_NOT_AVAILABLE: MQLONG = 0;
pub const MQSVC_CONTROL_Q_MGR: MQLONG = 0;
pub const MQSVC_CONTROL_Q_MGR_START: MQLONG = 1;
pub const MQSVC_CONTROL_MANUAL: MQLONG = 2;
pub const MQSVC_STATUS_STOPPED: MQLONG = 0;
pub const MQSVC_STATUS_STARTING: MQLONG = 1;
pub const MQSVC_STATUS_RUNNING: MQLONG = 2;
pub const MQSVC_STATUS_STOPPING: MQLONG = 3;
pub const MQSVC_STATUS_RETRYING: MQLONG = 4;
pub const MQTCPKEEP_NO: MQLONG = 0;
pub const MQTCPKEEP_YES: MQLONG = 1;
pub const MQTCPSTACK_SINGLE: MQLONG = 0;
pub const MQTCPSTACK_MULTIPLE: MQLONG = 1;
pub const MQTRAXSTR_NO: MQLONG = 0;
pub const MQTRAXSTR_YES: MQLONG = 1;
pub const MQCAP_NOT_SUPPORTED: MQLONG = 0;
pub const MQCAP_SUPPORTED: MQLONG = 1;
pub const MQCAP_EXPIRED: MQLONG = 2;
pub const MQMEDIMGSCHED_MANUAL: MQLONG = 0;
pub const MQMEDIMGSCHED_AUTO: MQLONG = 1;
pub const MQMEDIMGINTVL_OFF: MQLONG = 0;
pub const MQMEDIMGLOGLN_OFF: MQLONG = 0;
pub const MQIMGRCOV_NO: MQLONG = 0;
pub const MQIMGRCOV_YES: MQLONG = 1;
pub const MQIMGRCOV_AS_Q_MGR: MQLONG = 2;
pub const MQDLV_AS_PARENT: MQLONG = 0;
pub const MQDLV_ALL: MQLONG = 1;
pub const MQDLV_ALL_DUR: MQLONG = 2;
pub const MQDLV_ALL_AVAIL: MQLONG = 3;
pub const MQMASTER_NO: MQLONG = 0;
pub const MQMASTER_YES: MQLONG = 1;
pub const MQSCOPE_ALL: MQLONG = 0;
pub const MQSCOPE_AS_PARENT: MQLONG = 1;
pub const MQSCOPE_QMGR: MQLONG = 4;
pub const MQSUB_DURABLE_AS_PARENT: MQLONG = 0;
pub const MQSUB_DURABLE_ALLOWED: MQLONG = 1;
pub const MQSUB_DURABLE_INHIBITED: MQLONG = 2;
pub const MQTA_BLOCK: MQLONG = 1;
pub const MQTA_PASSTHRU: MQLONG = 2;
pub const MQTA_SUB_AS_PARENT: MQLONG = 0;
pub const MQTA_SUB_INHIBITED: MQLONG = 1;
pub const MQTA_SUB_ALLOWED: MQLONG = 2;
pub const MQTA_PROXY_SUB_FORCE: MQLONG = 1;
pub const MQTA_PROXY_SUB_FIRSTUSE: MQLONG = 2;
pub const MQTA_PUB_AS_PARENT: MQLONG = 0;
pub const MQTA_PUB_INHIBITED: MQLONG = 1;
pub const MQTA_PUB_ALLOWED: MQLONG = 2;
pub const MQTOPT_LOCAL: MQLONG = 0;
pub const MQTOPT_CLUSTER: MQLONG = 1;
pub const MQTOPT_ALL: MQLONG = 2;
pub const MQMC_AS_PARENT: MQLONG = 0;
pub const MQMC_ENABLED: MQLONG = 1;
pub const MQMC_DISABLED: MQLONG = 2;
pub const MQMC_ONLY: MQLONG = 3;
pub const MQCIT_MULTICAST: MQLONG = 1;
pub const MQDC_MANAGED: MQLONG = 1;
pub const MQDC_PROVIDED: MQLONG = 2;
pub const MQPSPROP_NONE: MQLONG = 0;
pub const MQPSPROP_COMPAT: MQLONG = 1;
pub const MQPSPROP_RFH2: MQLONG = 2;
pub const MQPSPROP_MSGPROP: MQLONG = 3;
pub const MQRU_PUBLISH_ON_REQUEST: MQLONG = 1;
pub const MQRU_PUBLISH_ALL: MQLONG = 2;
pub const MQSUB_DURABLE_ALL: MQLONG = -1;
pub const MQSUB_DURABLE_YES: MQLONG = 1;
pub const MQSUB_DURABLE_NO: MQLONG = 2;
pub const MQTSCOPE_QMGR: MQLONG = 1;
pub const MQTSCOPE_ALL: MQLONG = 2;
pub const MQVU_FIXED_USER: MQLONG = 1;
pub const MQVU_ANY_USER: MQLONG = 2;
pub const MQWS_DEFAULT: MQLONG = 0;
pub const MQWS_CHAR: MQLONG = 1;
pub const MQWS_TOPIC: MQLONG = 2;
pub const MQUSRC_MAP: MQLONG = 0;
pub const MQUSRC_NOACCESS: MQLONG = 1;
pub const MQUSRC_CHANNEL: MQLONG = 2;
pub const MQWARN_YES: MQLONG = 1;
pub const MQWARN_NO: MQLONG = 0;
pub const MQDSB_DEFAULT: MQLONG = 0;
pub const MQDSB_8K: MQLONG = 1;
pub const MQDSB_16K: MQLONG = 2;
pub const MQDSB_32K: MQLONG = 3;
pub const MQDSB_64K: MQLONG = 4;
pub const MQDSB_128K: MQLONG = 5;
pub const MQDSB_256K: MQLONG = 6;
pub const MQDSB_512K: MQLONG = 7;
pub const MQDSB_1024K: MQLONG = 8;
pub const MQDSB_1M: MQLONG = 8;
pub const MQDSE_DEFAULT: MQLONG = 0;
pub const MQDSE_YES: MQLONG = 1;
pub const MQDSE_NO: MQLONG = 2;
pub const MQCFOFFLD_NONE: MQLONG = 0;
pub const MQCFOFFLD_SMDS: MQLONG = 1;
pub const MQCFOFFLD_DB2: MQLONG = 2;
pub const MQCFOFFLD_BOTH: MQLONG = 3;
pub const MQUSEDLQ_AS_PARENT: MQLONG = 0;
pub const MQUSEDLQ_NO: MQLONG = 1;
pub const MQUSEDLQ_YES: MQLONG = 2;
pub const MQ_MQTT_MAX_KEEP_ALIVE: MQLONG = 65536;
pub const MQ_SSL_KEY_PASSPHRASE_LENGTH: usize = 1024;
pub const MQHO_UNUSABLE_HOBJ: MQHOBJ = -1;
pub const MQHO_NONE: MQHOBJ = 0;
pub const MQCO_IMMEDIATE: MQLONG = 0;
pub const MQCO_NONE: MQLONG = 0;
pub const MQCO_DELETE: MQLONG = 1;
pub const MQCO_DELETE_PURGE: MQLONG = 2;
pub const MQCO_KEEP_SUB: MQLONG = 4;
pub const MQCO_REMOVE_SUB: MQLONG = 8;
pub const MQCO_QUIESCE: MQLONG = 32;
pub const MQOP_START: MQLONG = 1;
pub const MQOP_START_WAIT: MQLONG = 2;
pub const MQOP_STOP: MQLONG = 4;
pub const MQOP_REGISTER: MQLONG = 256;
pub const MQOP_DEREGISTER: MQLONG = 512;
pub const MQOP_SUSPEND: MQLONG = 65536;
pub const MQOP_RESUME: MQLONG = 131072;
pub const MQHM_UNUSABLE_HMSG: MQHMSG = -1;
pub const MQHM_NONE: MQHMSG = 0;
pub const MQBA_FIRST: MQLONG = 6001;
pub const MQBA_LAST: MQLONG = 8000;
pub const MQCA_ADMIN_TOPIC_NAME: MQLONG = 2105;
pub const MQCA_ALTERATION_DATE: MQLONG = 2027;
pub const MQCA_ALTERATION_TIME: MQLONG = 2028;
pub const MQCA_AMQP_SSL_CIPHER_SUITES: MQLONG = 2137;
pub const MQCA_AMQP_VERSION: MQLONG = 2136;
pub const MQCA_APPL_ID: MQLONG = 2001;
pub const MQCA_AUTH_INFO_CONN_NAME: MQLONG = 2053;
pub const MQCA_AUTH_INFO_DESC: MQLONG = 2046;
pub const MQCA_AUTH_INFO_NAME: MQLONG = 2045;
pub const MQCA_AUTH_INFO_OCSP_URL: MQLONG = 2109;
pub const MQCA_AUTO_REORG_CATALOG: MQLONG = 2091;
pub const MQCA_AUTO_REORG_START_TIME: MQLONG = 2090;
pub const MQCA_BACKOUT_REQ_Q_NAME: MQLONG = 2019;
pub const MQCA_BASE_OBJECT_NAME: MQLONG = 2002;
pub const MQCA_BASE_Q_NAME: MQLONG = 2002;
pub const MQCA_BATCH_INTERFACE_ID: MQLONG = 2068;
pub const MQCA_CERT_LABEL: MQLONG = 2121;
pub const MQCA_CF_STRUC_DESC: MQLONG = 2052;
pub const MQCA_CF_STRUC_NAME: MQLONG = 2039;
pub const MQCA_CHANNEL_AUTO_DEF_EXIT: MQLONG = 2026;
pub const MQCA_CHILD: MQLONG = 2101;
pub const MQCA_CHINIT_SERVICE_PARM: MQLONG = 2076;
pub const MQCA_CHLAUTH_DESC: MQLONG = 2118;
pub const MQCA_CICS_FILE_NAME: MQLONG = 2060;
pub const MQCA_CLUSTER_DATE: MQLONG = 2037;
pub const MQCA_CLUSTER_NAME: MQLONG = 2029;
pub const MQCA_CLUSTER_NAMELIST: MQLONG = 2030;
pub const MQCA_CLUSTER_Q_MGR_NAME: MQLONG = 2031;
pub const MQCA_CLUSTER_TIME: MQLONG = 2038;
pub const MQCA_CLUSTER_WORKLOAD_DATA: MQLONG = 2034;
pub const MQCA_CLUSTER_WORKLOAD_EXIT: MQLONG = 2033;
pub const MQCA_CLUS_CHL_NAME: MQLONG = 2124;
pub const MQCA_COMMAND_INPUT_Q_NAME: MQLONG = 2003;
pub const MQCA_COMMAND_REPLY_Q_NAME: MQLONG = 2067;
pub const MQCA_COMM_INFO_DESC: MQLONG = 2111;
pub const MQCA_COMM_INFO_NAME: MQLONG = 2110;
pub const MQCA_CONN_AUTH: MQLONG = 2125;
pub const MQCA_CREATION_DATE: MQLONG = 2004;
pub const MQCA_CREATION_TIME: MQLONG = 2005;
pub const MQCA_CUSTOM: MQLONG = 2119;
pub const MQCA_DEAD_LETTER_Q_NAME: MQLONG = 2006;
pub const MQCA_DEF_XMIT_Q_NAME: MQLONG = 2025;
pub const MQCA_DNS_GROUP: MQLONG = 2071;
pub const MQCA_ENV_DATA: MQLONG = 2007;
pub const MQCA_FIRST: MQLONG = 2001;
pub const MQCA_IGQ_USER_ID: MQLONG = 2041;
pub const MQCA_INITIAL_KEY: MQLONG = 2054;
pub const MQCA_INITIATION_Q_NAME: MQLONG = 2008;
pub const MQCA_INSTALLATION_DESC: MQLONG = 2115;
pub const MQCA_INSTALLATION_NAME: MQLONG = 2116;
pub const MQCA_INSTALLATION_PATH: MQLONG = 2117;
pub const MQCA_LAST: MQLONG = 4000;
pub const MQCA_LAST_USED: MQLONG = 2138;
pub const MQCA_LDAP_BASE_DN_GROUPS: MQLONG = 2132;
pub const MQCA_LDAP_BASE_DN_USERS: MQLONG = 2126;
pub const MQCA_LDAP_FIND_GROUP_FIELD: MQLONG = 2135;
pub const MQCA_LDAP_GROUP_ATTR_FIELD: MQLONG = 2134;
pub const MQCA_LDAP_GROUP_OBJECT_CLASS: MQLONG = 2133;
pub const MQCA_LDAP_PASSWORD: MQLONG = 2048;
pub const MQCA_LDAP_SHORT_USER_FIELD: MQLONG = 2127;
pub const MQCA_LDAP_USER_ATTR_FIELD: MQLONG = 2129;
pub const MQCA_LDAP_USER_NAME: MQLONG = 2047;
pub const MQCA_LDAP_USER_OBJECT_CLASS: MQLONG = 2128;
pub const MQCA_LU62_ARM_SUFFIX: MQLONG = 2074;
pub const MQCA_LU_GROUP_NAME: MQLONG = 2072;
pub const MQCA_LU_NAME: MQLONG = 2073;
pub const MQCA_MODEL_DURABLE_Q: MQLONG = 2096;
pub const MQCA_MODEL_NON_DURABLE_Q: MQLONG = 2097;
pub const MQCA_MONITOR_Q_NAME: MQLONG = 2066;
pub const MQCA_NAMELIST_DESC: MQLONG = 2009;
pub const MQCA_NAMELIST_NAME: MQLONG = 2010;
pub const MQCA_NAMES: MQLONG = 2020;
pub const MQCA_PARENT: MQLONG = 2102;
pub const MQCA_PASS_TICKET_APPL: MQLONG = 2086;
pub const MQCA_POLICY_NAME: MQLONG = 2112;
pub const MQCA_PROCESS_DESC: MQLONG = 2011;
pub const MQCA_PROCESS_NAME: MQLONG = 2012;
pub const MQCA_QSG_CERT_LABEL: MQLONG = 2131;
pub const MQCA_QSG_NAME: MQLONG = 2040;
pub const MQCA_Q_DESC: MQLONG = 2013;
pub const MQCA_Q_MGR_DESC: MQLONG = 2014;
pub const MQCA_Q_MGR_IDENTIFIER: MQLONG = 2032;
pub const MQCA_Q_MGR_NAME: MQLONG = 2015;
pub const MQCA_Q_NAME: MQLONG = 2016;
pub const MQCA_RECIPIENT_DN: MQLONG = 2114;
pub const MQCA_REMOTE_Q_MGR_NAME: MQLONG = 2017;
pub const MQCA_REMOTE_Q_NAME: MQLONG = 2018;
pub const MQCA_REPOSITORY_NAME: MQLONG = 2035;
pub const MQCA_REPOSITORY_NAMELIST: MQLONG = 2036;
pub const MQCA_RESUME_DATE: MQLONG = 2098;
pub const MQCA_RESUME_TIME: MQLONG = 2099;
pub const MQCA_SERVICE_DESC: MQLONG = 2078;
pub const MQCA_SERVICE_NAME: MQLONG = 2077;
pub const MQCA_SERVICE_START_ARGS: MQLONG = 2080;
pub const MQCA_SERVICE_START_COMMAND: MQLONG = 2079;
pub const MQCA_SERVICE_STOP_ARGS: MQLONG = 2082;
pub const MQCA_SERVICE_STOP_COMMAND: MQLONG = 2081;
pub const MQCA_SIGNER_DN: MQLONG = 2113;
pub const MQCA_SSL_CERT_ISSUER_NAME: MQLONG = 2130;
pub const MQCA_SSL_CRL_NAMELIST: MQLONG = 2050;
pub const MQCA_SSL_CRYPTO_HARDWARE: MQLONG = 2051;
pub const MQCA_SSL_KEY_LIBRARY: MQLONG = 2069;
pub const MQCA_SSL_KEY_MEMBER: MQLONG = 2070;
pub const MQCA_SSL_KEY_REPOSITORY: MQLONG = 2049;
pub const MQCA_SSL_KEY_REPO_PASSWORD: MQLONG = 2055;
pub const MQCA_STDERR_DESTINATION: MQLONG = 2084;
pub const MQCA_STDOUT_DESTINATION: MQLONG = 2083;
pub const MQCA_STORAGE_CLASS: MQLONG = 2022;
pub const MQCA_STORAGE_CLASS_DESC: MQLONG = 2042;
pub const MQCA_STREAM_QUEUE_NAME: MQLONG = 2138;
pub const MQCA_SYSTEM_LOG_Q_NAME: MQLONG = 2065;
pub const MQCA_TCP_NAME: MQLONG = 2075;
pub const MQCA_TOPIC_DESC: MQLONG = 2093;
pub const MQCA_TOPIC_NAME: MQLONG = 2092;
pub const MQCA_TOPIC_STRING: MQLONG = 2094;
pub const MQCA_TOPIC_STRING_FILTER: MQLONG = 2108;
pub const MQCA_TPIPE_NAME: MQLONG = 2085;
pub const MQCA_TRIGGER_CHANNEL_NAME: MQLONG = 2064;
pub const MQCA_TRIGGER_DATA: MQLONG = 2023;
pub const MQCA_TRIGGER_PROGRAM_NAME: MQLONG = 2062;
pub const MQCA_TRIGGER_TERM_ID: MQLONG = 2063;
pub const MQCA_TRIGGER_TRANS_ID: MQLONG = 2061;
pub const MQCA_USER_DATA: MQLONG = 2021;
pub const MQCA_USER_LIST: MQLONG = 4000;
pub const MQCA_VERSION: MQLONG = 2120;
pub const MQCA_XCF_GROUP_NAME: MQLONG = 2043;
pub const MQCA_XCF_MEMBER_NAME: MQLONG = 2044;
pub const MQCA_XMIT_Q_NAME: MQLONG = 2024;
pub const MQCA_XR_SSL_CIPHER_SUITES: MQLONG = 2123;
pub const MQCA_XR_VERSION: MQLONG = 2122;
pub const MQIA_ACCOUNTING_CONN_OVERRIDE: MQLONG = 136;
pub const MQIA_ACCOUNTING_INTERVAL: MQLONG = 135;
pub const MQIA_ACCOUNTING_MQI: MQLONG = 133;
pub const MQIA_ACCOUNTING_Q: MQLONG = 134;
pub const MQIA_ACTIVE_CHANNELS: MQLONG = 100;
pub const MQIA_ACTIVITY_CONN_OVERRIDE: MQLONG = 239;
pub const MQIA_ACTIVITY_RECORDING: MQLONG = 138;
pub const MQIA_ACTIVITY_TRACE: MQLONG = 240;
pub const MQIA_ADOPTNEWMCA_CHECK: MQLONG = 102;
pub const MQIA_ADOPTNEWMCA_INTERVAL: MQLONG = 104;
pub const MQIA_ADOPTNEWMCA_TYPE: MQLONG = 103;
pub const MQIA_ADOPT_CONTEXT: MQLONG = 260;
pub const MQIA_ADVANCED_CAPABILITY: MQLONG = 273;
pub const MQIA_AMQP_CAPABILITY: MQLONG = 265;
pub const MQIA_APPL_TYPE: MQLONG = 1;
pub const MQIA_ARCHIVE: MQLONG = 60;
pub const MQIA_AUTHENTICATION_FAIL_DELAY: MQLONG = 259;
pub const MQIA_AUTHENTICATION_METHOD: MQLONG = 266;
pub const MQIA_AUTHOREV_SCOPE: MQLONG = 277;
pub const MQIA_AUTHORITY_EVENT: MQLONG = 47;
pub const MQIA_AUTH_INFO_TYPE: MQLONG = 66;
pub const MQIA_AUTO_REORGANIZATION: MQLONG = 173;
pub const MQIA_AUTO_REORG_INTERVAL: MQLONG = 174;
pub const MQIA_BACKOUT_THRESHOLD: MQLONG = 22;
pub const MQIA_BASE_TYPE: MQLONG = 193;
pub const MQIA_BATCH_INTERFACE_AUTO: MQLONG = 86;
pub const MQIA_BRIDGE_EVENT: MQLONG = 74;
pub const MQIA_CAP_EXPIRY: MQLONG = 276;
pub const MQIA_CERT_VAL_POLICY: MQLONG = 252;
pub const MQIA_CF_CFCONLOS: MQLONG = 246;
pub const MQIA_CF_LEVEL: MQLONG = 70;
pub const MQIA_CF_OFFLDUSE: MQLONG = 229;
pub const MQIA_CF_OFFLOAD: MQLONG = 224;
pub const MQIA_CF_OFFLOAD_THRESHOLD1: MQLONG = 225;
pub const MQIA_CF_OFFLOAD_THRESHOLD2: MQLONG = 226;
pub const MQIA_CF_OFFLOAD_THRESHOLD3: MQLONG = 227;
pub const MQIA_CF_RECAUTO: MQLONG = 244;
pub const MQIA_CF_RECOVER: MQLONG = 71;
pub const MQIA_CF_SMDS_BUFFERS: MQLONG = 228;
pub const MQIA_CHANNEL_AUTO_DEF: MQLONG = 55;
pub const MQIA_CHANNEL_AUTO_DEF_EVENT: MQLONG = 56;
pub const MQIA_CHANNEL_EVENT: MQLONG = 73;
pub const MQIA_CHECK_CLIENT_BINDING: MQLONG = 258;
pub const MQIA_CHECK_LOCAL_BINDING: MQLONG = 257;
pub const MQIA_CHINIT_ADAPTERS: MQLONG = 101;
pub const MQIA_CHINIT_CONTROL: MQLONG = 119;
pub const MQIA_CHINIT_DISPATCHERS: MQLONG = 105;
pub const MQIA_CHINIT_TRACE_AUTO_START: MQLONG = 117;
pub const MQIA_CHINIT_TRACE_TABLE_SIZE: MQLONG = 118;
pub const MQIA_CHLAUTH_RECORDS: MQLONG = 248;
pub const MQIA_CLUSTER_OBJECT_STATE: MQLONG = 256;
pub const MQIA_CLUSTER_PUB_ROUTE: MQLONG = 255;
pub const MQIA_CLUSTER_Q_TYPE: MQLONG = 59;
pub const MQIA_CLUSTER_WORKLOAD_LENGTH: MQLONG = 58;
pub const MQIA_CLWL_MRU_CHANNELS: MQLONG = 97;
pub const MQIA_CLWL_Q_PRIORITY: MQLONG = 96;
pub const MQIA_CLWL_Q_RANK: MQLONG = 95;
pub const MQIA_CLWL_USEQ: MQLONG = 98;
pub const MQIA_CMD_SERVER_AUTO: MQLONG = 87;
pub const MQIA_CMD_SERVER_CONTROL: MQLONG = 120;
pub const MQIA_CMD_SERVER_CONVERT_MSG: MQLONG = 88;
pub const MQIA_CMD_SERVER_DLQ_MSG: MQLONG = 89;
pub const MQIA_CODED_CHAR_SET_ID: MQLONG = 2;
pub const MQIA_COMMAND_EVENT: MQLONG = 99;
pub const MQIA_COMMAND_LEVEL: MQLONG = 31;
pub const MQIA_COMM_EVENT: MQLONG = 232;
pub const MQIA_COMM_INFO_TYPE: MQLONG = 223;
pub const MQIA_CONFIGURATION_EVENT: MQLONG = 51;
pub const MQIA_CPI_LEVEL: MQLONG = 27;
pub const MQIA_CURRENT_Q_DEPTH: MQLONG = 3;
pub const MQIA_DEFINITION_TYPE: MQLONG = 7;
pub const MQIA_DEF_BIND: MQLONG = 61;
pub const MQIA_DEF_CLUSTER_XMIT_Q_TYPE: MQLONG = 250;
pub const MQIA_DEF_INPUT_OPEN_OPTION: MQLONG = 4;
pub const MQIA_DEF_PERSISTENCE: MQLONG = 5;
pub const MQIA_DEF_PRIORITY: MQLONG = 6;
pub const MQIA_DEF_PUT_RESPONSE_TYPE: MQLONG = 184;
pub const MQIA_DEF_READ_AHEAD: MQLONG = 188;
pub const MQIA_DISPLAY_TYPE: MQLONG = 262;
pub const MQIA_DIST_LISTS: MQLONG = 34;
pub const MQIA_DNS_WLM: MQLONG = 106;
pub const MQIA_DURABLE_SUB: MQLONG = 175;
pub const MQIA_ENCRYPTION_ALGORITHM: MQLONG = 237;
pub const MQIA_EXPIRY_INTERVAL: MQLONG = 39;
pub const MQIA_FIRST: MQLONG = 1;
pub const MQIA_GROUP_UR: MQLONG = 221;
pub const MQIA_HARDEN_GET_BACKOUT: MQLONG = 8;
pub const MQIA_HIGH_Q_DEPTH: MQLONG = 36;
pub const MQIA_IGQ_PUT_AUTHORITY: MQLONG = 65;
pub const MQIA_INDEX_TYPE: MQLONG = 57;
pub const MQIA_INHIBIT_EVENT: MQLONG = 48;
pub const MQIA_INHIBIT_GET: MQLONG = 9;
pub const MQIA_INHIBIT_PUB: MQLONG = 181;
pub const MQIA_INHIBIT_PUT: MQLONG = 10;
pub const MQIA_INHIBIT_SUB: MQLONG = 182;
pub const MQIA_INTRA_GROUP_QUEUING: MQLONG = 64;
pub const MQIA_IP_ADDRESS_VERSION: MQLONG = 93;
pub const MQIA_KEY_REUSE_COUNT: MQLONG = 267;
pub const MQIA_LAST: MQLONG = 2000;
pub const MQIA_LAST_USED: MQLONG = 279;
pub const MQIA_LDAP_AUTHORMD: MQLONG = 263;
pub const MQIA_LDAP_NESTGRP: MQLONG = 264;
pub const MQIA_LDAP_SECURE_COMM: MQLONG = 261;
pub const MQIA_LISTENER_PORT_NUMBER: MQLONG = 85;
pub const MQIA_LISTENER_TIMER: MQLONG = 107;
pub const MQIA_LOCAL_EVENT: MQLONG = 49;
pub const MQIA_LOGGER_EVENT: MQLONG = 94;
pub const MQIA_LU62_CHANNELS: MQLONG = 108;
pub const MQIA_MASTER_ADMIN: MQLONG = 186;
pub const MQIA_MAX_CHANNELS: MQLONG = 109;
pub const MQIA_MAX_CLIENTS: MQLONG = 172;
pub const MQIA_MAX_GLOBAL_LOCKS: MQLONG = 83;
pub const MQIA_MAX_HANDLES: MQLONG = 11;
pub const MQIA_MAX_LOCAL_LOCKS: MQLONG = 84;
pub const MQIA_MAX_MSG_LENGTH: MQLONG = 13;
pub const MQIA_MAX_OPEN_Q: MQLONG = 80;
pub const MQIA_MAX_PRIORITY: MQLONG = 14;
pub const MQIA_MAX_PROPERTIES_LENGTH: MQLONG = 192;
pub const MQIA_MAX_Q_DEPTH: MQLONG = 15;
pub const MQIA_MAX_Q_FILE_SIZE: MQLONG = 274;
pub const MQIA_MAX_Q_TRIGGERS: MQLONG = 90;
pub const MQIA_MAX_RECOVERY_TASKS: MQLONG = 171;
pub const MQIA_MAX_RESPONSES: MQLONG = 230;
pub const MQIA_MAX_UNCOMMITTED_MSGS: MQLONG = 33;
pub const MQIA_MCAST_BRIDGE: MQLONG = 233;
pub const MQIA_MEDIA_IMAGE_INTERVAL: MQLONG = 269;
pub const MQIA_MEDIA_IMAGE_LOG_LENGTH: MQLONG = 270;
pub const MQIA_MEDIA_IMAGE_RECOVER_OBJ: MQLONG = 271;
pub const MQIA_MEDIA_IMAGE_RECOVER_Q: MQLONG = 272;
pub const MQIA_MEDIA_IMAGE_SCHEDULING: MQLONG = 268;
pub const MQIA_MONITORING_AUTO_CLUSSDR: MQLONG = 124;
pub const MQIA_MONITORING_CHANNEL: MQLONG = 122;
pub const MQIA_MONITORING_Q: MQLONG = 123;
pub const MQIA_MONITOR_INTERVAL: MQLONG = 81;
pub const MQIA_MSG_DELIVERY_SEQUENCE: MQLONG = 16;
pub const MQIA_MSG_DEQ_COUNT: MQLONG = 38;
pub const MQIA_MSG_ENQ_COUNT: MQLONG = 37;
pub const MQIA_MSG_MARK_BROWSE_INTERVAL: MQLONG = 68;
pub const MQIA_MULTICAST: MQLONG = 176;
pub const MQIA_NAMELIST_TYPE: MQLONG = 72;
pub const MQIA_NAME_COUNT: MQLONG = 19;
pub const MQIA_NPM_CLASS: MQLONG = 78;
pub const MQIA_NPM_DELIVERY: MQLONG = 196;
pub const MQIA_OPEN_INPUT_COUNT: MQLONG = 17;
pub const MQIA_OPEN_OUTPUT_COUNT: MQLONG = 18;
pub const MQIA_OTEL_PROPAGATION_CONTROL: MQLONG = 279;
pub const MQIA_OTEL_TRACE: MQLONG = 278;
pub const MQIA_OUTBOUND_PORT_MAX: MQLONG = 140;
pub const MQIA_OUTBOUND_PORT_MIN: MQLONG = 110;
pub const MQIA_PAGESET_ID: MQLONG = 62;
pub const MQIA_PERFORMANCE_EVENT: MQLONG = 53;
pub const MQIA_PLATFORM: MQLONG = 32;
pub const MQIA_PM_DELIVERY: MQLONG = 195;
pub const MQIA_POLICY_VERSION: MQLONG = 238;
pub const MQIA_PROPERTY_CONTROL: MQLONG = 190;
pub const MQIA_PROT_POLICY_CAPABILITY: MQLONG = 251;
pub const MQIA_PROXY_SUB: MQLONG = 199;
pub const MQIA_PUBSUB_CLUSTER: MQLONG = 249;
pub const MQIA_PUBSUB_MAXMSG_RETRY_COUNT: MQLONG = 206;
pub const MQIA_PUBSUB_MODE: MQLONG = 187;
pub const MQIA_PUBSUB_NP_MSG: MQLONG = 203;
pub const MQIA_PUBSUB_NP_RESP: MQLONG = 205;
pub const MQIA_PUBSUB_SYNC_PT: MQLONG = 207;
pub const MQIA_PUB_COUNT: MQLONG = 215;
pub const MQIA_PUB_SCOPE: MQLONG = 219;
pub const MQIA_QMGR_CFCONLOS: MQLONG = 245;
pub const MQIA_QMOPT_CONS_COMMS_MSGS: MQLONG = 155;
pub const MQIA_QMOPT_CONS_CRITICAL_MSGS: MQLONG = 154;
pub const MQIA_QMOPT_CONS_ERROR_MSGS: MQLONG = 153;
pub const MQIA_QMOPT_CONS_INFO_MSGS: MQLONG = 151;
pub const MQIA_QMOPT_CONS_REORG_MSGS: MQLONG = 156;
pub const MQIA_QMOPT_CONS_SYSTEM_MSGS: MQLONG = 157;
pub const MQIA_QMOPT_CONS_WARNING_MSGS: MQLONG = 152;
pub const MQIA_QMOPT_CSMT_ON_ERROR: MQLONG = 150;
pub const MQIA_QMOPT_INTERNAL_DUMP: MQLONG = 170;
pub const MQIA_QMOPT_LOG_COMMS_MSGS: MQLONG = 162;
pub const MQIA_QMOPT_LOG_CRITICAL_MSGS: MQLONG = 161;
pub const MQIA_QMOPT_LOG_ERROR_MSGS: MQLONG = 160;
pub const MQIA_QMOPT_LOG_INFO_MSGS: MQLONG = 158;
pub const MQIA_QMOPT_LOG_REORG_MSGS: MQLONG = 163;
pub const MQIA_QMOPT_LOG_SYSTEM_MSGS: MQLONG = 164;
pub const MQIA_QMOPT_LOG_WARNING_MSGS: MQLONG = 159;
pub const MQIA_QMOPT_TRACE_COMMS: MQLONG = 166;
pub const MQIA_QMOPT_TRACE_CONVERSION: MQLONG = 168;
pub const MQIA_QMOPT_TRACE_MQI_CALLS: MQLONG = 165;
pub const MQIA_QMOPT_TRACE_REORG: MQLONG = 167;
pub const MQIA_QMOPT_TRACE_SYSTEM: MQLONG = 169;
pub const MQIA_QSG_DISP: MQLONG = 63;
pub const MQIA_Q_DEPTH_HIGH_EVENT: MQLONG = 43;
pub const MQIA_Q_DEPTH_HIGH_LIMIT: MQLONG = 40;
pub const MQIA_Q_DEPTH_LOW_EVENT: MQLONG = 44;
pub const MQIA_Q_DEPTH_LOW_LIMIT: MQLONG = 41;
pub const MQIA_Q_DEPTH_MAX_EVENT: MQLONG = 42;
pub const MQIA_Q_SERVICE_INTERVAL: MQLONG = 54;
pub const MQIA_Q_SERVICE_INTERVAL_EVENT: MQLONG = 46;
pub const MQIA_Q_TYPE: MQLONG = 20;
pub const MQIA_Q_USERS: MQLONG = 82;
pub const MQIA_READ_AHEAD: MQLONG = 189;
pub const MQIA_RECEIVE_TIMEOUT: MQLONG = 111;
pub const MQIA_RECEIVE_TIMEOUT_MIN: MQLONG = 113;
pub const MQIA_RECEIVE_TIMEOUT_TYPE: MQLONG = 112;
pub const MQIA_REMOTE_EVENT: MQLONG = 50;
pub const MQIA_RESPONSE_RESTART_POINT: MQLONG = 231;
pub const MQIA_RETENTION_INTERVAL: MQLONG = 21;
pub const MQIA_REVERSE_DNS_LOOKUP: MQLONG = 254;
pub const MQIA_SCOPE: MQLONG = 45;
pub const MQIA_SECURITY_CASE: MQLONG = 141;
pub const MQIA_SERVICE_CONTROL: MQLONG = 139;
pub const MQIA_SERVICE_TYPE: MQLONG = 121;
pub const MQIA_SHAREABILITY: MQLONG = 23;
pub const MQIA_SHARED_Q_Q_MGR_NAME: MQLONG = 77;
pub const MQIA_SIGNATURE_ALGORITHM: MQLONG = 236;
pub const MQIA_SSL_EVENT: MQLONG = 75;
pub const MQIA_SSL_FIPS_REQUIRED: MQLONG = 92;
pub const MQIA_SSL_RESET_COUNT: MQLONG = 76;
pub const MQIA_SSL_TASKS: MQLONG = 69;
pub const MQIA_START_STOP_EVENT: MQLONG = 52;
pub const MQIA_STATISTICS_AUTO_CLUSSDR: MQLONG = 130;
pub const MQIA_STATISTICS_CHANNEL: MQLONG = 129;
pub const MQIA_STATISTICS_INTERVAL: MQLONG = 131;
pub const MQIA_STATISTICS_MQI: MQLONG = 127;
pub const MQIA_STATISTICS_Q: MQLONG = 128;
pub const MQIA_STREAM_QUEUE_QOS: MQLONG = 275;
pub const MQIA_SUB_CONFIGURATION_EVENT: MQLONG = 242;
pub const MQIA_SUB_COUNT: MQLONG = 204;
pub const MQIA_SUB_SCOPE: MQLONG = 218;
pub const MQIA_SUITE_B_STRENGTH: MQLONG = 247;
pub const MQIA_SYNCPOINT: MQLONG = 30;
pub const MQIA_TCP_CHANNELS: MQLONG = 114;
pub const MQIA_TCP_KEEP_ALIVE: MQLONG = 115;
pub const MQIA_TCP_STACK_TYPE: MQLONG = 116;
pub const MQIA_TIME_SINCE_RESET: MQLONG = 35;
pub const MQIA_TOLERATE_UNPROTECTED: MQLONG = 235;
pub const MQIA_TOPIC_DEF_PERSISTENCE: MQLONG = 185;
pub const MQIA_TOPIC_NODE_COUNT: MQLONG = 253;
pub const MQIA_TOPIC_TYPE: MQLONG = 208;
pub const MQIA_TRACE_ROUTE_RECORDING: MQLONG = 137;
pub const MQIA_TREE_LIFE_TIME: MQLONG = 183;
pub const MQIA_TRIGGER_CONTROL: MQLONG = 24;
pub const MQIA_TRIGGER_DEPTH: MQLONG = 29;
pub const MQIA_TRIGGER_INTERVAL: MQLONG = 25;
pub const MQIA_TRIGGER_MSG_PRIORITY: MQLONG = 26;
pub const MQIA_TRIGGER_RESTART: MQLONG = 91;
pub const MQIA_TRIGGER_TYPE: MQLONG = 28;
pub const MQIA_UR_DISP: MQLONG = 222;
pub const MQIA_USAGE: MQLONG = 12;
pub const MQIA_USER_LIST: MQLONG = 2000;
pub const MQIA_USE_DEAD_LETTER_Q: MQLONG = 234;
pub const MQIA_WILDCARD_OPERATION: MQLONG = 216;
pub const MQIA_XR_CAPABILITY: MQLONG = 243;
pub const MQIAV_NOT_APPLICABLE: MQLONG = -1;
pub const MQIAV_UNDEFINED: MQLONG = -2;
pub const MQMCB_DISABLED: MQLONG = 0;
pub const MQMCB_ENABLED: MQLONG = 1;
pub const MQKEY_REUSE_DISABLED: MQLONG = 0;
pub const MQKEY_REUSE_UNLIMITED: MQLONG = -1;
pub const MQGA_FIRST: MQLONG = 8001;
pub const MQGA_LAST: MQLONG = 9000;
pub const MQOO_BIND_AS_Q_DEF: MQLONG = 0;
pub const MQOO_READ_AHEAD_AS_Q_DEF: MQLONG = 0;
pub const MQOO_INPUT_AS_Q_DEF: MQLONG = 1;
pub const MQOO_INPUT_SHARED: MQLONG = 2;
pub const MQOO_INPUT_EXCLUSIVE: MQLONG = 4;
pub const MQOO_BROWSE: MQLONG = 8;
pub const MQOO_OUTPUT: MQLONG = 16;
pub const MQOO_INQUIRE: MQLONG = 32;
pub const MQOO_SET: MQLONG = 64;
pub const MQOO_SAVE_ALL_CONTEXT: MQLONG = 128;
pub const MQOO_PASS_IDENTITY_CONTEXT: MQLONG = 256;
pub const MQOO_PASS_ALL_CONTEXT: MQLONG = 512;
pub const MQOO_SET_IDENTITY_CONTEXT: MQLONG = 1024;
pub const MQOO_SET_ALL_CONTEXT: MQLONG = 2048;
pub const MQOO_ALTERNATE_USER_AUTHORITY: MQLONG = 4096;
pub const MQOO_FAIL_IF_QUIESCING: MQLONG = 8192;
pub const MQOO_BIND_ON_OPEN: MQLONG = 16384;
pub const MQOO_BIND_ON_GROUP: MQLONG = 4194304;
pub const MQOO_BIND_NOT_FIXED: MQLONG = 32768;
pub const MQOO_CO_OP: MQLONG = 131072;
pub const MQOO_NO_READ_AHEAD: MQLONG = 524288;
pub const MQOO_READ_AHEAD: MQLONG = 1048576;
pub const MQOO_NO_MULTICAST: MQLONG = 2097152;
pub const MQOO_RESOLVE_LOCAL_Q: MQLONG = 262144;
pub const MQOO_RESOLVE_LOCAL_TOPIC: MQLONG = 262144;
pub const MQOO_RESOLVE_NAMES: MQLONG = 65536;
pub const MQTYPE_AS_SET: MQLONG = 0;
pub const MQTYPE_NULL: MQLONG = 2;
pub const MQTYPE_BOOLEAN: MQLONG = 4;
pub const MQTYPE_BYTE_STRING: MQLONG = 8;
pub const MQTYPE_INT8: MQLONG = 16;
pub const MQTYPE_INT16: MQLONG = 32;
pub const MQTYPE_INT32: MQLONG = 64;
pub const MQTYPE_LONG: MQLONG = 64;
pub const MQTYPE_INT64: MQLONG = 128;
pub const MQTYPE_FLOAT32: MQLONG = 256;
pub const MQTYPE_FLOAT64: MQLONG = 512;
pub const MQTYPE_STRING: MQLONG = 1024;
pub const MQVL_NULL_TERMINATED: MQLONG = -1;
pub const MQVL_EMPTY_STRING: MQLONG = 0;
pub const MQSTAT_TYPE_ASYNC_ERROR: MQLONG = 0;
pub const MQSTAT_TYPE_RECONNECTION: MQLONG = 1;
pub const MQSTAT_TYPE_RECONNECTION_ERROR: MQLONG = 2;
pub const MQSO_NONE: MQLONG = 0;
pub const MQSO_NON_DURABLE: MQLONG = 0;
pub const MQSO_READ_AHEAD_AS_Q_DEF: MQLONG = 0;
pub const MQSO_ALTER: MQLONG = 1;
pub const MQSO_CREATE: MQLONG = 2;
pub const MQSO_RESUME: MQLONG = 4;
pub const MQSO_DURABLE: MQLONG = 8;
pub const MQSO_GROUP_SUB: MQLONG = 16;
pub const MQSO_MANAGED: MQLONG = 32;
pub const MQSO_SET_IDENTITY_CONTEXT: MQLONG = 64;
pub const MQSO_NO_MULTICAST: MQLONG = 128;
pub const MQSO_FIXED_USERID: MQLONG = 256;
pub const MQSO_ANY_USERID: MQLONG = 512;
pub const MQSO_PUBLICATIONS_ON_REQUEST: MQLONG = 2048;
pub const MQSO_NEW_PUBLICATIONS_ONLY: MQLONG = 4096;
pub const MQSO_FAIL_IF_QUIESCING: MQLONG = 8192;
pub const MQSO_ALTERNATE_USER_AUTHORITY: MQLONG = 262144;
pub const MQSO_WILDCARD_CHAR: MQLONG = 1048576;
pub const MQSO_WILDCARD_TOPIC: MQLONG = 2097152;
pub const MQSO_SET_CORREL_ID: MQLONG = 4194304;
pub const MQSO_SCOPE_QMGR: MQLONG = 67108864;
pub const MQSO_NO_READ_AHEAD: MQLONG = 134217728;
pub const MQSO_READ_AHEAD: MQLONG = 268435456;
pub const MQSR_ACTION_PUBLICATION: MQLONG = 1;
unsafe extern "C" {
    /// Back Out Changes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqback-back-out-changes)
    pub fn MQBACK(Hconn: MQHCONN, pCompCode: &mut MQLONG, pReason: &mut MQLONG);
    /// Begin Unit of Work
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pBeginOptions` (Input/Output): Options that control the action of MQBEGIN
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbegin-begin-unit-work)
    pub fn MQBEGIN(
        Hconn: MQHCONN,
        pBeginOptions: Option<&mut MQBO>,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Buffer To Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pBufMsgHOpts`: Options that control the action of MQBUFMH
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `pBuffer` (Input/Output): Area to contain the message buffer
    /// * `pDataLength` (Output): Length of the output buffer
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbufmh-convert-buffer-into-message-handle)
    pub fn MQBUFMH(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pBufMsgHOpts: &MQBMHO,
        pMsgDesc: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Register Message consumer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Operation`: Operation
    /// * `pCallbackDesc`: Callback descriptor
    /// * `Hobj`: Object handle
    /// * `pMsgDesc`: Message Descriptor
    /// * `pGetMsgOpts`: Get options
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcb-manage-callback)
    pub fn MQCB(
        Hconn: MQHCONN,
        Operation: MQLONG,
        pCallbackDesc: Option<&MQCBD>,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: Option<&MQGMO>,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Close Object
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pHobj` (Input/Output): Object handle
    /// * `Options`: Options that control the action of MQCLOSE
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqclose-close-object)
    pub fn MQCLOSE(
        Hconn: MQHCONN,
        pHobj: &mut MQHOBJ,
        Options: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Commit Changes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcmit-commit-changes)
    pub fn MQCMIT(Hconn: MQHCONN, pCompCode: &mut MQLONG, pReason: &mut MQLONG);
    /// Connect Queue Manager
    ///
    /// # Arguments
    /// * `pQMgrName`: Name of queue manager
    /// * `pHconn` (Output): Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqconn-connect-queue-manager)
    pub fn MQCONN(
        pQMgrName: &MQCHAR48,
        pHconn: &mut MQHCONN,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Connect Queue Manager (Extended)
    ///
    /// # Arguments
    /// * `pQMgrName`: Name of queue manager
    /// * `pConnectOpts` (Input/Output): Options that control the action of MQCONNX
    /// * `pHconn` (Output): Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqconnx-connect-queue-manager-extended)
    pub fn MQCONNX(
        pQMgrName: &MQCHAR48,
        pConnectOpts: &mut MQCNO,
        pHconn: &mut MQHCONN,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Create Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pCrtMsgHOpts`: Options that control the action of MQCRTMH
    /// * `pHmsg` (Output): Message handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcrtmh-create-message-handle)
    pub fn MQCRTMH(
        Hconn: MQHCONN,
        pCrtMsgHOpts: &MQCMHO,
        pHmsg: &mut MQHMSG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Control Consumer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Operation`: Operation
    /// * `pControlOpts`: Control options
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqctl-control-callbacks)
    pub fn MQCTL(
        Hconn: MQHCONN,
        Operation: MQLONG,
        pControlOpts: &MQCTLO,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Disconnect Queue Manager
    ///
    /// # Arguments
    /// * `pHconn` (Input/Output): Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdisc-disconnect-queue-manager)
    pub fn MQDISC(pHconn: &mut MQHCONN, pCompCode: &mut MQLONG, pReason: &mut MQLONG);
    /// Delete Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pHmsg` (Input/Output): Message handle
    /// * `pDltMsgHOpts`: Options that control the action of MQDLTMH
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdltmh-delete-message-handle)
    pub fn MQDLTMH(
        Hconn: MQHCONN,
        pHmsg: &mut MQHMSG,
        pDltMsgHOpts: &MQDMHO,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Delete Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pDltPropOpts`: Options that control the action of MQDLTMP
    /// * `pName`: Property name
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdltmp-delete-message-property)
    pub fn MQDLTMP(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pDltPropOpts: &MQDMPO,
        pName: &MQCHARV,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Get Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `pGetMsgOpts` (Input/Output): Options that control the action of MQGET
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `pBuffer` (Output): Area to contain the message data
    /// * `pDataLength` (Output): Length of the message
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqget-get-message)
    pub fn MQGET(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: &mut MQGMO,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire Object Attributes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `SelectorCount`: Count of selectors
    /// * `pSelectors`: Array of attribute selectors
    /// * `IntAttrCount`: Count of integer attributes
    /// * `pIntAttrs` (Output): Array of integer attributes
    /// * `CharAttrLength`: Length of character attributes buffer
    /// * `pCharAttrs` (Output): Character attributes
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinq-inquire-object-attributes)
    pub fn MQINQ(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pInqPropOpts`: Options that control the action of MQINQMP
    /// * `pName`: Property name
    /// * `pPropDesc` (Output): Property descriptor
    /// * `pType` (Input/Output): Property data type
    /// * `ValueLength`: Length in bytes of the Value area
    /// * `pValue` (Output): Property value
    /// * `pDataLength` (Output): Length of the property value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinqmp-inquire-message-property)
    pub fn MQINQMP(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pInqPropOpts: &mut MQIMPO,
        pName: &MQCHARV,
        pPropDesc: &mut MQPD,
        pType: &mut MQLONG,
        ValueLength: MQLONG,
        pValue: PMQVOID,
        pDataLength: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Message Handle To Buffer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pMsgHBufOpts`: Options that control the action of MQMHBUF
    /// * `pName`: Property name
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `pBuffer` (Output): Area to contain the properties
    /// * `pDataLength` (Output): Length of the properties
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqmhbuf-convert-message-handle-into-buffer)
    pub fn MQMHBUF(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pMsgHBufOpts: &MQMHBO,
        pName: &MQCHARV,
        pMsgDesc: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Open Object
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pObjDesc` (Input/Output): Object descriptor
    /// * `Options`: Options that control the action of MQOPEN
    /// * `pHobj` (Output): Object handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqopen-open-object)
    pub fn MQOPEN(
        Hconn: MQHCONN,
        pObjDesc: &mut MQOD,
        Options: MQLONG,
        pHobj: &mut MQHOBJ,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Put Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `pPutMsgOpts` (Input/Output): Options that control the action of MQPUT
    /// * `BufferLength`: Length of the message in Buffer
    /// * `pBuffer`: Message data
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqput-put-message)
    pub fn MQPUT(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: &mut MQPMO,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Put One Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pObjDesc` (Input/Output): Object descriptor
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `pPutMsgOpts` (Input/Output): Options that control the action of MQPUT1
    /// * `BufferLength`: Length of the message in Buffer
    /// * `pBuffer`: Message data
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqput1-put-one-message)
    pub fn MQPUT1(
        Hconn: MQHCONN,
        pObjDesc: &mut MQOD,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: &mut MQPMO,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Set Object Attributes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `SelectorCount`: Count of selectors
    /// * `pSelectors`: Array of attribute selectors
    /// * `IntAttrCount`: Count of integer attributes
    /// * `pIntAttrs`: Array of integer attributes
    /// * `CharAttrLength`: Length of character attributes buffer
    /// * `pCharAttrs`: Character attributes
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqset-set-object-attributes)
    pub fn MQSET(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Set Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pSetPropOpts`: Options that control the action of MQSETMP
    /// * `pName`: Property name
    /// * `pPropDesc` (Input/Output): Property descriptor
    /// * `Type`: Property data type
    /// * `ValueLength`: Length of the Value area
    /// * `pValue`: Property value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetmp-set-message-property)
    pub fn MQSETMP(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pSetPropOpts: &MQSMPO,
        pName: &MQCHARV,
        pPropDesc: &mut MQPD,
        Type: MQLONG,
        ValueLength: MQLONG,
        pValue: PMQVOID,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Get Status Information
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Type`: Status information type
    /// * `pStatus` (Input/Output): Status information
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqstat-retrieve-status-information)
    pub fn MQSTAT(
        Hconn: MQHCONN,
        Type: MQLONG,
        pStatus: &mut MQSTS,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Subscribe to topic
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pSubDesc` (Input/Output): Subscription descriptor
    /// * `pHobj` (Input/Output): Object handle for queue
    /// * `pHsub` (Output): Subscription object handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsub-register-subscription)
    pub fn MQSUB(
        Hconn: MQHCONN,
        pSubDesc: &mut MQSD,
        pHobj: Option<&mut MQHOBJ>,
        pHsub: &mut MQHOBJ,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Subscription Request
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hsub`: Subscription handle
    /// * `Action`: Action requested on the subscription
    /// * `pSubRqOpts` (Input/Output): Subscription Request Options
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsubrq-subscription-request)
    pub fn MQSUBRQ(
        Hconn: MQHCONN,
        Hsub: MQHOBJ,
        Action: MQLONG,
        pSubRqOpts: Option<&mut MQSRO>,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
}
/// Channel Definition
pub type MQCD = tagMQCD;
pub type PMQCD = *mut MQCD;
pub type PPMQCD = *mut PMQCD;
/// Channel Definition
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCD {
    /// Channel definition name
    pub ChannelName: [MQCHAR; 20usize],
    /// Structure version number
    pub Version: MQLONG,
    /// Channel type
    pub ChannelType: MQLONG,
    /// Transport type
    pub TransportType: MQLONG,
    /// Channel description
    pub Desc: [MQCHAR; 64usize],
    /// Queue-manager name
    pub QMgrName: [MQCHAR; 48usize],
    /// Transmission queue name
    pub XmitQName: [MQCHAR; 48usize],
    /// First 20 bytes of connection name
    pub ShortConnectionName: [MQCHAR; 20usize],
    /// Reserved
    pub MCAName: [MQCHAR; 20usize],
    /// LU 6.2 Mode name
    pub ModeName: [MQCHAR; 8usize],
    /// LU 6.2 transaction program name
    pub TpName: [MQCHAR; 64usize],
    /// Batch size
    pub BatchSize: MQLONG,
    /// Disconnect interval
    pub DiscInterval: MQLONG,
    /// Short retry count
    pub ShortRetryCount: MQLONG,
    /// Short retry wait interval
    pub ShortRetryInterval: MQLONG,
    /// Long retry count
    pub LongRetryCount: MQLONG,
    /// Long retry wait interval
    pub LongRetryInterval: MQLONG,
    /// Channel security exit name
    pub SecurityExit: [MQCHAR; 128usize],
    /// Channel message exit name
    pub MsgExit: [MQCHAR; 128usize],
    /// Channel send exit name
    pub SendExit: [MQCHAR; 128usize],
    /// Channel receive exit name
    pub ReceiveExit: [MQCHAR; 128usize],
    /// Highest allowable message sequence number
    pub SeqNumberWrap: MQLONG,
    /// Maximum message length
    pub MaxMsgLength: MQLONG,
    /// Put authority
    pub PutAuthority: MQLONG,
    /// Data conversion
    pub DataConversion: MQLONG,
    /// Channel security exit user data
    pub SecurityUserData: [MQCHAR; 32usize],
    /// Channel message exit user data
    pub MsgUserData: [MQCHAR; 32usize],
    /// Channel send exit user data
    pub SendUserData: [MQCHAR; 32usize],
    /// Channel receive exit user data
    pub ReceiveUserData: [MQCHAR; 32usize],
    /// User identifier
    pub UserIdentifier: [MQCHAR; 12usize],
    /// Password
    pub Password: [MQCHAR; 12usize],
    /// First 12 bytes of MCA user identifier
    pub MCAUserIdentifier: [MQCHAR; 12usize],
    /// Message channel agent type
    pub MCAType: MQLONG,
    /// Connection name
    pub ConnectionName: [MQCHAR; 264usize],
    /// First 12 bytes of user identifier from partner
    pub RemoteUserIdentifier: [MQCHAR; 12usize],
    /// Password from partner
    pub RemotePassword: [MQCHAR; 12usize],
    /// Channel message retry exit name
    pub MsgRetryExit: [MQCHAR; 128usize],
    /// Channel message retry exit user data
    pub MsgRetryUserData: [MQCHAR; 32usize],
    /// Number of times MCA will try to put the message, after first attempt has failed
    pub MsgRetryCount: MQLONG,
    /// Minimum interval in milliseconds after which the open or put operation will be retried
    pub MsgRetryInterval: MQLONG,
    /// Time in seconds between heartbeat flows
    pub HeartbeatInterval: MQLONG,
    /// Batch duration
    pub BatchInterval: MQLONG,
    /// Speed at which nonpersistent messages are sent
    pub NonPersistentMsgSpeed: MQLONG,
    /// Length of MQCD structure
    pub StrucLength: MQLONG,
    /// Length of exit name
    pub ExitNameLength: MQLONG,
    /// Length of exit user data
    pub ExitDataLength: MQLONG,
    /// Number of message exits defined
    pub MsgExitsDefined: MQLONG,
    /// Number of send exits defined
    pub SendExitsDefined: MQLONG,
    /// Number of receive exits defined
    pub ReceiveExitsDefined: MQLONG,
    /// Address of first MsgExit field
    pub MsgExitPtr: MQPTR,
    /// Address of first MsgUserData field
    pub MsgUserDataPtr: MQPTR,
    /// Address of first SendExit field
    pub SendExitPtr: MQPTR,
    /// Address of first SendUserData field
    pub SendUserDataPtr: MQPTR,
    /// Address of first ReceiveExit field
    pub ReceiveExitPtr: MQPTR,
    /// Address of first ReceiveUserData field
    pub ReceiveUserDataPtr: MQPTR,
    /// Address of a list of cluster names
    pub ClusterPtr: MQPTR,
    /// Number of clusters to which the channel belongs
    pub ClustersDefined: MQLONG,
    /// Network priority
    pub NetworkPriority: MQLONG,
    /// Length of long MCA user identifier
    pub LongMCAUserIdLength: MQLONG,
    /// Length of long remote user identifier
    pub LongRemoteUserIdLength: MQLONG,
    /// Address of long MCA user identifier
    pub LongMCAUserIdPtr: MQPTR,
    /// Address of long remote user identifier
    pub LongRemoteUserIdPtr: MQPTR,
    /// MCA security identifier
    pub MCASecurityId: MQBYTE40,
    /// Remote security identifier
    pub RemoteSecurityId: MQBYTE40,
    /// SSL CipherSpec
    pub SSLCipherSpec: [MQCHAR; 32usize],
    /// Address of SSL peer name
    pub SSLPeerNamePtr: MQPTR,
    /// Length of SSL peer name
    pub SSLPeerNameLength: MQLONG,
    /// Whether SSL client authentication is required
    pub SSLClientAuth: MQLONG,
    /// Keepalive interval
    pub KeepAliveInterval: MQLONG,
    /// Local communications address
    pub LocalAddress: [MQCHAR; 48usize],
    /// Batch heartbeat interval
    pub BatchHeartbeat: MQLONG,
    /// Header data compression list
    pub HdrCompList: [MQLONG; 2usize],
    /// Message data compression list
    pub MsgCompList: [MQLONG; 16usize],
    /// Channel rank
    pub CLWLChannelRank: MQLONG,
    /// Channel priority
    pub CLWLChannelPriority: MQLONG,
    /// Channel weight
    pub CLWLChannelWeight: MQLONG,
    /// Channel monitoring
    pub ChannelMonitoring: MQLONG,
    /// Channel statistics
    pub ChannelStatistics: MQLONG,
    /// Limit on sharing conversations
    pub SharingConversations: MQLONG,
    /// Message property control
    pub PropertyControl: MQLONG,
    /// Limit on SVRCONN channel instances
    pub MaxInstances: MQLONG,
    /// Limit on SVRCONN channel instances per client
    pub MaxInstancesPerClient: MQLONG,
    /// Client channel weight
    pub ClientChannelWeight: MQLONG,
    /// Connection affinity
    pub ConnectionAffinity: MQLONG,
    /// Batch data limit
    pub BatchDataLimit: MQLONG,
    /// Use Dead Letter Queue
    pub UseDLQ: MQLONG,
    /// Default client reconnect option
    pub DefReconnect: MQLONG,
    /// Certificate label
    pub CertificateLabel: [MQCHAR; 64usize],
    /// SPL Protection
    pub SPLProtection: MQLONG,
}
pub const MQCD_VERSION_1: MQLONG = 1;
pub const MQCD_VERSION_2: MQLONG = 2;
pub const MQCD_VERSION_3: MQLONG = 3;
pub const MQCD_VERSION_4: MQLONG = 4;
pub const MQCD_VERSION_5: MQLONG = 5;
pub const MQCD_VERSION_6: MQLONG = 6;
pub const MQCD_VERSION_7: MQLONG = 7;
pub const MQCD_VERSION_8: MQLONG = 8;
pub const MQCD_VERSION_9: MQLONG = 9;
pub const MQCD_VERSION_10: MQLONG = 10;
pub const MQCD_VERSION_11: MQLONG = 11;
pub const MQCD_VERSION_12: MQLONG = 12;
pub const MQCD_CURRENT_VERSION: MQLONG = 12;
pub const MQCD_LENGTH_1: usize = 984;
pub const MQCD_LENGTH_2: usize = 1312;
pub const MQCD_LENGTH_3: usize = 1480;
pub const MQCD_LENGTH_4: usize = 1568;
pub const MQCD_LENGTH_5: usize = 1584;
pub const MQCD_LENGTH_6: usize = 1688;
pub const MQCD_LENGTH_7: usize = 1792;
pub const MQCD_LENGTH_8: usize = 1888;
pub const MQCD_LENGTH_9: usize = 1912;
pub const MQCD_LENGTH_10: usize = 1920;
pub const MQCD_LENGTH_11: usize = 1984;
pub const MQCD_LENGTH_12: usize = 1992;
pub const MQCD_CURRENT_LENGTH: usize = 1992;
pub const MQCHT_SENDER: MQLONG = 1;
pub const MQCHT_SERVER: MQLONG = 2;
pub const MQCHT_RECEIVER: MQLONG = 3;
pub const MQCHT_REQUESTER: MQLONG = 4;
pub const MQCHT_ALL: MQLONG = 5;
pub const MQCHT_CLNTCONN: MQLONG = 6;
pub const MQCHT_SVRCONN: MQLONG = 7;
pub const MQCHT_CLUSRCVR: MQLONG = 8;
pub const MQCHT_CLUSSDR: MQLONG = 9;
pub const MQCHT_MQTT: MQLONG = 10;
pub const MQCHT_AMQP: MQLONG = 11;
pub const MQCOMPRESS_NOT_AVAILABLE: MQLONG = -1;
pub const MQCOMPRESS_NONE: MQLONG = 0;
pub const MQCOMPRESS_RLE: MQLONG = 1;
pub const MQCOMPRESS_ZLIBFAST: MQLONG = 2;
pub const MQCOMPRESS_ZLIBHIGH: MQLONG = 4;
pub const MQCOMPRESS_SYSTEM: MQLONG = 8;
pub const MQCOMPRESS_LZ4FAST: MQLONG = 16;
pub const MQCOMPRESS_LZ4HIGH: MQLONG = 32;
pub const MQCOMPRESS_ANY: MQLONG = 268435455;
pub const MQXPT_ALL: MQLONG = -1;
pub const MQXPT_LOCAL: MQLONG = 0;
pub const MQXPT_LU62: MQLONG = 1;
pub const MQXPT_TCP: MQLONG = 2;
pub const MQXPT_NETBIOS: MQLONG = 3;
pub const MQXPT_SPX: MQLONG = 4;
pub const MQXPT_DECNET: MQLONG = 5;
pub const MQXPT_UDP: MQLONG = 6;
pub const MQCAFTY_NONE: MQLONG = 0;
pub const MQCAFTY_PREFERRED: MQLONG = 1;
pub const MQDCC_DEFAULT_CONVERSION: MQLONG = 1;
pub const MQDCC_FILL_TARGET_BUFFER: MQLONG = 2;
pub const MQDCC_INT_DEFAULT_CONVERSION: MQLONG = 4;
pub const MQDCC_SOURCE_ENC_NATIVE: MQLONG = 32;
pub const MQDCC_SOURCE_ENC_NORMAL: MQLONG = 16;
pub const MQDCC_SOURCE_ENC_REVERSED: MQLONG = 32;
pub const MQDCC_SOURCE_ENC_UNDEFINED: MQLONG = 0;
pub const MQDCC_TARGET_ENC_NATIVE: MQLONG = 512;
pub const MQDCC_TARGET_ENC_NORMAL: MQLONG = 256;
pub const MQDCC_TARGET_ENC_REVERSED: MQLONG = 512;
pub const MQDCC_TARGET_ENC_UNDEFINED: MQLONG = 0;
pub const MQDCC_NONE: MQLONG = 0;
pub const MQDCC_SOURCE_ENC_MASK: MQLONG = 240;
pub const MQDCC_TARGET_ENC_MASK: MQLONG = 3840;
pub const MQDCC_SOURCE_ENC_FACTOR: MQLONG = 16;
pub const MQDCC_TARGET_ENC_FACTOR: MQLONG = 256;
unsafe extern "C" {
    /// Convert Characters
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Options`: Options that control the action of MQXCNVC
    /// * `SourceCCSID`: Coded character set identifier of string before conversion
    /// * `SourceLength`: Length of string before conversion
    /// * `pSourceBuffer`: String to be converted
    /// * `TargetCCSID`: Coded character set identifier of string after conversion
    /// * `TargetLength`: Length of output buffer
    /// * `pTargetBuffer` (Output): String after conversion
    /// * `pDataLength` (Output): Length of output string
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=exit-mqxcnvc-convert-characters)
    pub fn MQXCNVC(
        Hconn: MQHCONN,
        Options: MQLONG,
        SourceCCSID: MQLONG,
        SourceLength: MQLONG,
        pSourceBuffer: PMQCHAR,
        TargetCCSID: MQLONG,
        TargetLength: MQLONG,
        pTargetBuffer: PMQCHAR,
        pDataLength: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
}
