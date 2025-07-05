/* Generated with MQ client version 9.4.3.0 */

pub type PPMQCHAR = *mut crate::PMQCHAR;
pub type PPMQLONG = *mut crate::PMQLONG;
pub type PMQIEP = *mut MQIEP;
pub type PPMQIEP = *mut PMQIEP;
pub type MQHCONFIG = PMQIEP;
pub type PMQHCONFIG = *mut MQHCONFIG;
pub type PPMQHCONN = *mut crate::PMQHCONN;
pub type PPMQHOBJ = *mut crate::PMQHOBJ;
pub type PPMQVOID = *mut crate::PMQVOID;
pub type PPMQBO = *mut crate::PMQBO;
pub type PPMQCBC = *mut crate::PMQCBC;
pub type PPMQCBD = *mut crate::PMQCBD;
pub type PPMQCTLO = *mut crate::PMQCTLO;
pub type PPMQCNO = *mut crate::PMQCNO;
pub type PPMQGMO = *mut crate::PMQGMO;
pub type PPMQMD = *mut crate::PMQMD;
pub type PPMQOD = *mut crate::PMQOD;
pub type PPMQPMO = *mut crate::PMQPMO;
pub type PPMQSD = *mut crate::PMQSD;
pub type PPMQSRO = *mut crate::PMQSRO;
pub type PPMQSTS = *mut crate::PMQSTS;
/// Back Out Changes
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_BACK_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_BACK_CALL = MQ_BACK_CALL;
/// Begin Unit of Work
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `BeginOptions` (Input/Output): Options that control the action of [`MQBEGIN`](crate::MQBEGIN)
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_BEGIN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        BeginOptions: Option<&mut crate::MQBO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_BEGIN_CALL = MQ_BEGIN_CALL;
/// Buffer To Message Handle
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hmsg`: Message handle
/// * `BufMsgHOpts`: Options that control the action of [`MQBUFMH`](crate::MQBUFMH)
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `BufferLength`: Length in bytes of the Buffer area
/// * `Buffer` (Input/Output): Area to contain the message buffer
/// * `DataLength` (Output): Length of the output buffer
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_BUFMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        BufMsgHOpts: &crate::MQBMHO,
        MsgDesc: crate::PMQVOID,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_BUFMH_CALL = MQ_BUFMH_CALL;
/// Register Message consumer
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Operation`: Operation
/// * `CallbackDesc`: Callback descriptor
/// * `Hobj`: Object handle
/// * `MsgDesc`: Message Descriptor
/// * `GetMsgOpts`: Get options
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_CB_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Operation: crate::MQLONG,
        CallbackDesc: Option<&crate::MQCBD>,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: Option<&crate::MQGMO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CB_CALL = MQ_CB_CALL;
/// Close Object
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hobj` (Input/Output): Object handle
/// * `Options`: Options that control the action of [`MQCLOSE`](crate::MQCLOSE)
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_CLOSE_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        Options: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CLOSE_CALL = MQ_CLOSE_CALL;
/// Commit Changes
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_CMIT_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CMIT_CALL = MQ_CMIT_CALL;
/// Connect Queue Manager
///
/// # Arguments
/// * `QMgrName`: Name of queue manager
/// * `Hconn` (Output): Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_CONN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: &crate::MQCHAR48,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CONN_CALL = MQ_CONN_CALL;
/// Connect Queue Manager (Extended)
///
/// # Arguments
/// * `QMgrName`: Name of queue manager
/// * `ConnectOpts` (Input/Output): Options that control the action of [`MQCONNX`](crate::MQCONNX)
/// * `Hconn` (Output): Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_CONNX_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: &crate::MQCHAR48,
        ConnectOpts: &mut crate::MQCNO,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CONNX_CALL = MQ_CONNX_CALL;
/// Create Message Handle
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `CrtMsgHOpts`: Options that control the action of [`MQCRTMH`](crate::MQCRTMH)
/// * `Hmsg` (Output): Message handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_CRTMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        CrtMsgHOpts: &crate::MQCMHO,
        Hmsg: &mut crate::MQHMSG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CRTMH_CALL = MQ_CRTMH_CALL;
/// Control Consumer
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Operation`: Operation
/// * `ControlOpts`: Control options
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_CTL_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Operation: crate::MQLONG,
        ControlOpts: &crate::MQCTLO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CTL_CALL = MQ_CTL_CALL;
/// Disconnect Queue Manager
///
/// # Arguments
/// * `Hconn` (Input/Output): Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_DISC_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_DISC_CALL = MQ_DISC_CALL;
/// Delete Message Handle
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hmsg` (Input/Output): Message handle
/// * `DltMsgHOpts`: Options that control the action of [`MQDLTMH`](crate::MQDLTMH)
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_DLTMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hmsg: &mut crate::MQHMSG,
        DltMsgHOpts: &crate::MQDMHO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_DLTMH_CALL = MQ_DLTMH_CALL;
/// Delete Message Property
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hmsg`: Message handle
/// * `DltPropOpts`: Options that control the action of [`MQDLTMP`](crate::MQDLTMP)
/// * `Name`: Property name
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_DLTMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        DltPropOpts: &crate::MQDMPO,
        Name: &crate::MQCHARV,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_DLTMP_CALL = MQ_DLTMP_CALL;
/// Get Message
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hobj`: Object handle
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `GetMsgOpts` (Input/Output): Options that control the action of [`MQGET`](crate::MQGET)
/// * `BufferLength`: Length in bytes of the Buffer area
/// * `Buffer` (Output): Area to contain the message data
/// * `DataLength` (Output): Length of the message
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_GET_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: &mut crate::MQGMO,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_GET_CALL = MQ_GET_CALL;
/// Inquire Object Attributes
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hobj`: Object handle
/// * `SelectorCount`: Count of selectors
/// * `Selectors`: Array of attribute selectors
/// * `IntAttrCount`: Count of integer attributes
/// * `IntAttrs` (Output): Array of integer attributes
/// * `CharAttrLength`: Length of character attributes buffer
/// * `CharAttrs` (Output): Character attributes
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_INQ_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        SelectorCount: crate::MQLONG,
        Selectors: crate::PMQLONG,
        IntAttrCount: crate::MQLONG,
        IntAttrs: crate::PMQLONG,
        CharAttrLength: crate::MQLONG,
        CharAttrs: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_INQ_CALL = MQ_INQ_CALL;
/// Inquire Message Property
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hmsg`: Message handle
/// * `InqPropOpts`: Options that control the action of [`MQINQMP`](crate::MQINQMP)
/// * `Name`: Property name
/// * `PropDesc` (Output): Property descriptor
/// * `Type` (Input/Output): Property data type
/// * `ValueLength`: Length in bytes of the Value area
/// * `Value` (Output): Property value
/// * `DataLength` (Output): Length of the property value
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_INQMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        InqPropOpts: &mut crate::MQIMPO,
        Name: &crate::MQCHARV,
        PropDesc: &mut crate::MQPD,
        Type: &mut crate::MQLONG,
        ValueLength: crate::MQLONG,
        Value: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_INQMP_CALL = MQ_INQMP_CALL;
/// Message Handle To Buffer
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hmsg`: Message handle
/// * `MsgHBufOpts`: Options that control the action of [`MQMHBUF`](crate::MQMHBUF)
/// * `Name`: Property name
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `BufferLength`: Length in bytes of the Buffer area
/// * `Buffer` (Output): Area to contain the properties
/// * `DataLength` (Output): Length of the properties
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_MHBUF_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        MsgHBufOpts: &crate::MQMHBO,
        Name: &crate::MQCHARV,
        MsgDesc: crate::PMQVOID,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_MHBUF_CALL = MQ_MHBUF_CALL;
/// Open Object
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `ObjDesc` (Input/Output): Object descriptor
/// * `Options`: Options that control the action of [`MQOPEN`](crate::MQOPEN)
/// * `Hobj` (Output): Object handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_OPEN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        Options: crate::MQLONG,
        Hobj: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_OPEN_CALL = MQ_OPEN_CALL;
/// Put Message
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hobj`: Object handle
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `PutMsgOpts` (Input/Output): Options that control the action of [`MQPUT`](crate::MQPUT)
/// * `BufferLength`: Length of the message in Buffer
/// * `Buffer`: Message data
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_PUT_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_PUT_CALL = MQ_PUT_CALL;
/// Put One Message
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `ObjDesc` (Input/Output): Object descriptor
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `PutMsgOpts` (Input/Output): Options that control the action of [`MQPUT1`](crate::MQPUT1)
/// * `BufferLength`: Length of the message in Buffer
/// * `Buffer`: Message data
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_PUT1_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_PUT1_CALL = MQ_PUT1_CALL;
/// Set Object Attributes
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hobj`: Object handle
/// * `SelectorCount`: Count of selectors
/// * `Selectors`: Array of attribute selectors
/// * `IntAttrCount`: Count of integer attributes
/// * `IntAttrs`: Array of integer attributes
/// * `CharAttrLength`: Length of character attributes buffer
/// * `CharAttrs`: Character attributes
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_SET_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        SelectorCount: crate::MQLONG,
        Selectors: crate::PMQLONG,
        IntAttrCount: crate::MQLONG,
        IntAttrs: crate::PMQLONG,
        CharAttrLength: crate::MQLONG,
        CharAttrs: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_SET_CALL = MQ_SET_CALL;
/// Set Message Property
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hmsg`: Message handle
/// * `SetPropOpts`: Options that control the action of [`MQSETMP`](crate::MQSETMP)
/// * `Name`: Property name
/// * `PropDesc` (Input/Output): Property descriptor
/// * `Type`: Property data type
/// * `ValueLength`: Length of the Value area
/// * `Value`: Property value
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_SETMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        SetPropOpts: &crate::MQSMPO,
        Name: &crate::MQCHARV,
        PropDesc: &mut crate::MQPD,
        Type: crate::MQLONG,
        ValueLength: crate::MQLONG,
        Value: crate::PMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_SETMP_CALL = MQ_SETMP_CALL;
/// Get Status Information
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Type`: Status information type
/// * `Status` (Input/Output): Status information
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_STAT_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Type: crate::MQLONG,
        Status: &mut crate::MQSTS,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_STAT_CALL = MQ_STAT_CALL;
/// Subscribe to topic
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `SubDesc` (Input/Output): Subscription descriptor
/// * `Hobj` (Input/Output): Object handle for queue
/// * `Hsub` (Output): Subscription object handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_SUB_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        SubDesc: &mut crate::MQSD,
        Hobj: Option<&mut crate::MQHOBJ>,
        Hsub: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_SUB_CALL = MQ_SUB_CALL;
/// Subscription Request
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Hsub`: Subscription handle
/// * `Action`: Action requested on the subscription
/// * `SubRqOpts` (Input/Output): Subscription Request Options
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_SUBRQ_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hsub: crate::MQHOBJ,
        Action: crate::MQLONG,
        SubRqOpts: Option<&mut crate::MQSRO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_SUBRQ_CALL = MQ_SUBRQ_CALL;
pub type PMQACH = *mut MQACH;
pub type PMQAXC = *mut MQAXC;
pub type PMQAXP = *mut MQAXP;
pub type PMQCXP = *mut MQCXP;
pub type PMQDXP = *mut MQDXP;
pub type PMQNXP = *mut MQNXP;
pub type PMQPBC = *mut MQPBC;
pub type PMQPSXP = *mut MQPSXP;
pub type PMQSBC = *mut MQSBC;
pub type PMQWCR = *mut MQWCR;
pub type PMQWDR = *mut MQWDR;
pub type PPMQWDR = *mut PMQWDR;
pub type PMQWDR1 = *mut MQWDR1;
pub type PMQWDR2 = *mut MQWDR2;
pub type PMQWQR = *mut MQWQR;
pub type PPMQWQR = *mut PMQWQR;
pub type PMQWQR1 = *mut MQWQR1;
pub type PMQWQR2 = *mut MQWQR2;
pub type PMQWQR3 = *mut MQWQR3;
pub type PMQWQR4 = *mut MQWQR4;
pub type PMQWXP = *mut MQWXP;
pub type PMQWXP1 = *mut MQWXP1;
pub type PMQWXP2 = *mut MQWXP2;
pub type PMQWXP3 = *mut MQWXP3;
pub type PMQWXP4 = *mut MQWXP4;
pub type PMQXEPO = *mut MQXEPO;
/// Register Entry Point
///
/// # Arguments
/// * `Hconfig`: Configuration handle
/// * `ExitReason`: Exit reason
/// * `Function`: Function identifier
/// * `EntryPoint`: Exit function entry point
/// * `ExitOpts`: Options that control the action of [`MQXEP`]
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_XEP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        ExitReason: crate::MQLONG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        ExitOpts: Option<&MQXEPO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_XEP_CALL = MQ_XEP_CALL;
/// Back Out Changes Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_BACK_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109700_.html)
pub type MQ_BACK_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_BACK_EXIT = MQ_BACK_EXIT;
/// Begin Unit of Work Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `BeginOptions` (Input/Output): Options that control the action of [`MQBEGIN`](crate::MQBEGIN)
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_BEGIN_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109710_.html)
pub type MQ_BEGIN_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        BeginOptions: PPMQBO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_BEGIN_EXIT = MQ_BEGIN_EXIT;
/// Callback Function Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `GetMsgOpts` (Input/Output): Options that define the operation of the consumer
/// * `Buffer` (Input/Output): Area to contain the message data
/// * `MQCBContext` (Input/Output): Context data for the callback
///
/// # References
/// * [IBM `MQ_CALLBACK_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109720_.html)
pub type MQ_CALLBACK_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        MsgDesc: PPMQMD,
        GetMsgOpts: PPMQGMO,
        Buffer: PPMQVOID,
        MQCBContext: PPMQCBC,
    ),
>;
pub type PMQ_CALLBACK_EXIT = MQ_CALLBACK_EXIT;
/// Register Callback Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Operation` (Input/Output): Operation
/// * `CallbackDesc` (Input/Output): Callback descriptor
/// * `Hobj` (Input/Output): Object handle
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `GetMsgOpts` (Input/Output): Get message options
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_CB_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109730_.html)
pub type MQ_CB_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Operation: crate::PMQLONG,
        CallbackDesc: PPMQCBD,
        Hobj: &mut crate::MQHOBJ,
        MsgDesc: PPMQMD,
        GetMsgOpts: PPMQGMO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CB_EXIT = MQ_CB_EXIT;
/// Close Object Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Hobj` (Input/Output): Object handle
/// * `Options` (Input/Output): Options that control the action of [`MQCLOSE`](crate::MQCLOSE)
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_CLOSE_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109740_.html)
pub type MQ_CLOSE_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Hobj: PPMQHOBJ,
        Options: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CLOSE_EXIT = MQ_CLOSE_EXIT;
/// Commit Changes Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_CMIT_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109750_.html)
pub type MQ_CMIT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CMIT_EXIT = MQ_CMIT_EXIT;
/// Connect Queue Manager Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `QMgrName` (Input/Output): Name of queue manager
/// * `ConnectOpts` (Input/Output): Options that control the action of [`MQCONNX`](crate::MQCONNX)
/// * `Hconn` (Input/Output): Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_CONNX_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109760_.html)
pub type MQ_CONNX_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        QMgrName: crate::PMQCHAR,
        ConnectOpts: PPMQCNO,
        Hconn: PPMQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CONNX_EXIT = MQ_CONNX_EXIT;
/// Control Asynchronous Operations Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Operation` (Input/Output): Operation
/// * `CtlOpts` (Input/Output): Control options
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_CTL_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109770_.html)
pub type MQ_CTL_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Operation: crate::PMQLONG,
        CtlOpts: PPMQCTLO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_CTL_EXIT = MQ_CTL_EXIT;
/// Disconnect Queue Manager Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_DISC_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109780_.html)
pub type MQ_DISC_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: PPMQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_DISC_EXIT = MQ_DISC_EXIT;
/// Get Message Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Hobj` (Input/Output): Object handle
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `GetMsgOpts` (Input/Output): Options that control the action of [`MQGET`](crate::MQGET)
/// * `BufferLength` (Input/Output): Length in bytes of pBuffer area
/// * `Buffer` (Input/Output): Area to contain the message data
/// * `DataLength` (Output): Length of the message
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_GET_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109790_.html)
pub type MQ_GET_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        MsgDesc: PPMQMD,
        GetMsgOpts: PPMQGMO,
        BufferLength: crate::PMQLONG,
        Buffer: PPMQVOID,
        DataLength: PPMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_GET_EXIT = MQ_GET_EXIT;
/// Initialization Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_INIT_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109810_.html)
pub type MQ_INIT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_INIT_EXIT = MQ_INIT_EXIT;
/// Inquire Object Attributes Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Hobj` (Input/Output): Object handle
/// * `SelectorCount` (Input/Output): Count of selectors
/// * `Selectors` (Input/Output): Array of attribute selectors
/// * `IntAttrCount` (Input/Output): Count of integer attributes
/// * `IntAttrs` (Input/Output): Array of integer attributes
/// * `CharAttrLength` (Output): Length of character attributes
/// * `CharAttrs` (Output): Character attributes
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_INQ_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109820_.html)
pub type MQ_INQ_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        SelectorCount: crate::PMQLONG,
        Selectors: PPMQLONG,
        IntAttrCount: crate::PMQLONG,
        IntAttrs: PPMQLONG,
        CharAttrLength: crate::PMQLONG,
        CharAttrs: PPMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_INQ_EXIT = MQ_INQ_EXIT;
/// Open Object Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `ObjDesc` (Input/Output): Object descriptor
/// * `Options` (Input/Output): Options that control the action of [`MQOPEN`](crate::MQOPEN)
/// * `Hobj` (Input/Output): Object handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_OPEN_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109830_.html)
pub type MQ_OPEN_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        ObjDesc: PPMQOD,
        Options: crate::PMQLONG,
        Hobj: PPMQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_OPEN_EXIT = MQ_OPEN_EXIT;
/// Put Message Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Hobj` (Input/Output): Object handle
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `PutMsgOpts` (Input/Output): Options that control the action of [`MQPUT`](crate::MQPUT)
/// * `BufferLength` (Input/Output): Length of the message in pBuffer
/// * `Buffer` (Input/Output): Message data
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_PUT_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109840_.html)
pub type MQ_PUT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        MsgDesc: PPMQMD,
        PutMsgOpts: PPMQPMO,
        BufferLength: crate::PMQLONG,
        Buffer: PPMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_PUT_EXIT = MQ_PUT_EXIT;
/// Put One Message Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `ObjDesc` (Input/Output): Object descriptor
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `PutMsgOpts` (Input/Output): Options that control the action of [`MQPUT1`](crate::MQPUT1)
/// * `BufferLength` (Input/Output): Length of the message in pBuffer
/// * `Buffer` (Input/Output): Message data
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_PUT1_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109850_.html)
pub type MQ_PUT1_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        ObjDesc: PPMQOD,
        MsgDesc: PPMQMD,
        PutMsgOpts: PPMQPMO,
        BufferLength: crate::PMQLONG,
        Buffer: PPMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_PUT1_EXIT = MQ_PUT1_EXIT;
/// Set Object Attributes Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Hobj` (Input/Output): Object handle
/// * `SelectorCount` (Input/Output): Count of selectors
/// * `Selectors` (Input/Output): Array of attribute selectors
/// * `IntAttrCount` (Input/Output): Count of integer attributes
/// * `IntAttrs` (Input/Output): Array of integer attributes
/// * `CharAttrLength` (Output): Length of character attributes buffer
/// * `CharAttrs` (Output): Character attributes
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_SET_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109860_.html)
pub type MQ_SET_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        SelectorCount: crate::PMQLONG,
        Selectors: PPMQLONG,
        IntAttrCount: crate::PMQLONG,
        IntAttrs: PPMQLONG,
        CharAttrLength: crate::PMQLONG,
        CharAttrs: PPMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_SET_EXIT = MQ_SET_EXIT;
/// Get Status Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Type` (Input/Output): Status Type
/// * `Status` (Input/Output): Status Buffer
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_STAT_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109870_.html)
pub type MQ_STAT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Type: crate::PMQLONG,
        Status: PPMQSTS,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_STAT_EXIT = MQ_STAT_EXIT;
/// Subscribe Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `Hsub` (Input/Output): Subscription handle
/// * `Action` (Input/Output): Request action
/// * `SubRqOpts` (Input/Output): Subscription Request options
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_SUBRQ_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109900_.html)
pub type MQ_SUBRQ_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        Hsub: crate::PMQHOBJ,
        Action: crate::PMQLONG,
        SubRqOpts: PPMQSRO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_SUBRQ_EXIT = MQ_SUBRQ_EXIT;
/// Subscribe Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `Hconn` (Input/Output): Connection handle
/// * `SubDesc` (Input/Output): Subscription descriptor
/// * `Hobj` (Input/Output): Queue object handle
/// * `Hsub` (Input/Output): Subscription object handle
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_SUB_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109890_.html)
pub type MQ_SUB_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        Hconn: &mut crate::MQHCONN,
        SubDesc: PPMQSD,
        Hobj: PPMQHOBJ,
        Hsub: PPMQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_SUB_EXIT = MQ_SUB_EXIT;
/// Termination Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `ExitContext` (Input/Output): Exit context structure
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQ_TERM_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109880_.html)
pub type MQ_TERM_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQAXP,
        ExitContext: &mut MQAXC,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_TERM_EXIT = MQ_TERM_EXIT;
/// Channel Exit
///
/// # Arguments
/// * `ChannelExitParms` (Input/Output): Channel exit parameter block
/// * `ChannelDefinition` (Input/Output): Channel definition
/// * `DataLength` (Input/Output): Length of data
/// * `AgentBufferLength`: Length of agent buffer
/// * `AgentBuffer` (Input/Output): Agent buffer
/// * `ExitBufferLength` (Input/Output): Length of exit buffer
/// * `ExitBufferAddr` (Input/Output): Address of exit buffer
///
/// # References
/// * [IBM `MQ_CHANNEL_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108180_.html)
pub type MQ_CHANNEL_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ChannelExitParms: crate::PMQVOID,
        ChannelDefinition: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        AgentBufferLength: crate::PMQLONG,
        AgentBuffer: crate::PMQVOID,
        ExitBufferLength: crate::PMQLONG,
        ExitBufferAddr: crate::PMQPTR,
    ),
>;
pub type PMQ_CHANNEL_EXIT = MQ_CHANNEL_EXIT;
/// Channel Auto Definition Exit
///
/// # Arguments
/// * `ChannelExitParms` (Input/Output): Channel exit parameter block
/// * `ChannelDefinition` (Input/Output): Channel definition
///
/// # References
/// * [IBM `MQ_CHANNEL_AUTO_DEF_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108190_.html)
pub type MQ_CHANNEL_AUTO_DEF_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ChannelExitParms: crate::PMQVOID,
        ChannelDefinition: crate::PMQVOID,
    ),
>;
pub type PMQ_CHANNEL_AUTO_DEF_EXIT = MQ_CHANNEL_AUTO_DEF_EXIT;
/// Cluster Workload Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter block
///
/// # References
/// * [IBM `MQ_CLUSTER_WORKLOAD_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q082500_.html)
pub type MQ_CLUSTER_WORKLOAD_EXIT = ::std::option::Option<
    unsafe extern "C" fn(ExitParms: &mut MQWXP),
>;
pub type PMQ_CLUSTER_WORKLOAD_EXIT = MQ_CLUSTER_WORKLOAD_EXIT;
/// Data Conversion Exit
///
/// # Arguments
/// * `DataConvExitParms` (Input/Output): Data-conversion exit parameter block
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `InBufferLength`: Length in bytes of `InBuffer`
/// * `InBuffer`: Buffer containing the unconverted message
/// * `OutBufferLength`: Length in bytes of `OutBuffer`
/// * `OutBuffer` (Output): Buffer containing the converted message
///
/// # References
/// * [IBM `MQ_DATA_CONV_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q104120_.html)
pub type MQ_DATA_CONV_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        DataConvExitParms: PMQDXP,
        MsgDesc: crate::PMQMD,
        InBufferLength: crate::MQLONG,
        InBuffer: crate::PMQVOID,
        OutBufferLength: crate::MQLONG,
        OutBuffer: crate::PMQVOID,
    ),
>;
pub type PMQ_DATA_CONV_EXIT = MQ_DATA_CONV_EXIT;
/// Publish Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter block
/// * `PubContext`: Publication context structure
/// * `SubContext`: Subscription context structure
///
/// # References
/// * [IBM `MQ_PUBLISH_EXIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108130_.html)
pub type MQ_PUBLISH_EXIT = ::std::option::Option<
    unsafe extern "C" fn(ExitParms: &mut MQPSXP, PubContext: PMQPBC, SubContext: PMQSBC),
>;
pub type PMQ_PUBLISH_EXIT = MQ_PUBLISH_EXIT;
/// Transport Retry Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter block
/// * `DestAddressLength`: Length in bytes of destination IP address
/// * `DestAddress`: Destination IP address
pub type MQ_TRANSPORT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: crate::PMQVOID,
        DestAddressLength: crate::MQLONG,
        DestAddress: crate::PMQCHAR,
    ),
>;
pub type PMQ_TRANSPORT_EXIT = MQ_TRANSPORT_EXIT;
/// Preconnect Exit
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `QMgrName` (Input/Output): Name of queue manager
/// * `ConnectOpts` (Input/Output): Options that control the action of [`MQCONNX`](crate::MQCONNX)
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_PRECONNECT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQNXP,
        QMgrName: crate::PMQCHAR,
        ConnectOpts: PPMQCNO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_PRECONNECT_EXIT = MQ_PRECONNECT_EXIT;
/// Cluster Workload Navigate Records
///
/// # Arguments
/// * `ExitParms` (Input/Output): Exit parameter structure
/// * `CurrentRecord`: Address of current record
/// * `NextOffset`: Offset of next record
/// * `NextRecord` (Output): Address of next record or structure
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_XCLWLN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        ExitParms: &mut MQWXP,
        CurrentRecord: crate::MQPTR,
        NextOffset: crate::MQLONG,
        NextRecord: &mut crate::MQPTR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_XCLWLN_CALL = MQ_XCLWLN_CALL;
/// Convert Characters
///
/// # Arguments
/// * `Hconn`: Connection handle
/// * `Options`: Options that control the action of [`MQXCNVC`](crate::MQXCNVC)
/// * `SourceCCSID`: Coded character set identifier of string before conversion
/// * `SourceLength`: Length of string before conversion
/// * `SourceBuffer`: String to be converted
/// * `TargetCCSID`: Coded character set identifier of string after conversion
/// * `TargetLength`: Length of output buffer
/// * `TargetBuffer` (Output): String after conversion
/// * `DataLength` (Output): Length of output string
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_XCNVC_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Options: crate::MQLONG,
        SourceCCSID: crate::MQLONG,
        SourceLength: crate::MQLONG,
        SourceBuffer: crate::PMQCHAR,
        TargetCCSID: crate::MQLONG,
        TargetLength: crate::MQLONG,
        TargetBuffer: crate::PMQCHAR,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_XCNVC_CALL = MQ_XCNVC_CALL;
/// Convert Message Data
///
/// # Arguments
/// * `DataConvExitParms` (Input/Output): Data-conversion exit parameter block
/// * `MsgDesc` (Input/Output): Message descriptor
/// * `InBufferLength`: Length in bytes of `InBuffer`
/// * `InBuffer`: Buffer containing the unconverted message
/// * `OutBufferLength`: Length in bytes of `OutBuffer`
/// * `OutBuffer` (Output): Buffer containing the converted message
pub type MQ_XDX_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        DataConvExitParms: PMQDXP,
        MsgDesc: crate::PMQMD,
        InBufferLength: crate::MQLONG,
        InBuffer: crate::PMQVOID,
        OutBufferLength: crate::MQLONG,
        OutBuffer: crate::PMQVOID,
    ),
>;
pub type PMQ_XDX_CALL = MQ_XDX_CALL;
pub type PMQZED = *mut MQZED;
pub type PMQZAC = *mut MQZAC;
pub type PMQZAD = *mut MQZAD;
pub type PMQZFP = *mut MQZFP;
pub type PMQZIC = *mut MQZIC;
/// Add Component Entry Point
///
/// # Arguments
/// * `Hconfig`: Configuration handle
/// * `Function`: Function identifier
/// * `EntryPoint`: Function entry point
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQ_ZEP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQ_ZEP_CALL = MQ_ZEP_CALL;
/// Initialize Authority-Services
///
/// # Arguments
/// * `Hconfig`: Configuration handle
/// * `Options`: Initialization options
/// * `QMgrName`: Queue manager name
/// * `ComponentDataLength`: Length of component data
/// * `ComponentData` (Input/Output): Component data
/// * `Version`: Version number
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_INIT_AUTHORITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110210_.html)
pub type MQZ_INIT_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: crate::MQLONG,
        QMgrName: crate::PMQCHAR,
        ComponentDataLength: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Version: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_INIT_AUTHORITY = MQZ_INIT_AUTHORITY;
/// Terminate Authority-Services
///
/// # Arguments
/// * `Hconfig`: Configuration handle
/// * `Options`: Termination options
/// * `QMgrName`: Queue manager name
/// * `ComponentData`: Component data
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_TERM_AUTHORITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110260_.html)
pub type MQZ_TERM_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: crate::MQLONG,
        QMgrName: crate::PMQCHAR,
        ComponentData: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_TERM_AUTHORITY = MQZ_TERM_AUTHORITY;
/// Delete Authority
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_DELETE_AUTHORITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110140_.html)
pub type MQZ_DELETE_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_DELETE_AUTHORITY = MQZ_DELETE_AUTHORITY;
/// Get Authority
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityName`: Entity name
/// * `EntityType`: Entity type
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `Authority` (Output): Authority of entity
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_GET_AUTHORITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110170_.html)
pub type MQZ_GET_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityName: crate::PMQCHAR,
        EntityType: crate::MQLONG,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        Authority: crate::PMQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_GET_AUTHORITY = MQZ_GET_AUTHORITY;
/// Get Authority Version 2
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityData`: Entity data
/// * `EntityType`: Entity type
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `Authority` (Output): Authority of entity
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_GET_AUTHORITY_2` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110180_.html)
pub type MQZ_GET_AUTHORITY_2 = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityData: PMQZED,
        EntityType: crate::MQLONG,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        Authority: crate::PMQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_GET_AUTHORITY_2 = MQZ_GET_AUTHORITY_2;
/// Get Explicit Authority
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityName`: Entity name
/// * `EntityType`: Entity type
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `Authority` (Output): Authority of entity
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_GET_EXPLICIT_AUTHORITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110190_.html)
pub type MQZ_GET_EXPLICIT_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityName: crate::PMQCHAR,
        EntityType: crate::MQLONG,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        Authority: crate::PMQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_GET_EXPLICIT_AUTHORITY = MQZ_GET_EXPLICIT_AUTHORITY;
/// Get Explicit Authority Version 2
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityData`: Entity data
/// * `EntityType`: Entity type
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `Authority` (Output): Authority of entity
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_GET_EXPLICIT_AUTHORITY_2` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110200_.html)
pub type MQZ_GET_EXPLICIT_AUTHORITY_2 = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityData: PMQZED,
        EntityType: crate::MQLONG,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        Authority: crate::PMQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_GET_EXPLICIT_AUTHORITY_2 = MQZ_GET_EXPLICIT_AUTHORITY_2;
/// Enumerate Authority Data
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `StartEnumeration`: Flag indicating whether call should start enumeration
/// * `Filter`: Filter
/// * `AuthorityBufferLength`: Length of AuthorityBuffer
/// * `AuthorityBuffer` (Output): Authority data
/// * `AuthorityDataLength` (Output): Length of data returned in AuthorityBuffer
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_ENUMERATE_AUTHORITY_DATA` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110150_.html)
pub type MQZ_ENUMERATE_AUTHORITY_DATA = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        StartEnumeration: crate::MQLONG,
        Filter: PMQZAD,
        AuthorityBufferLength: crate::MQLONG,
        AuthorityBuffer: PMQZAD,
        AuthorityDataLength: crate::PMQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_ENUMERATE_AUTHORITY_DATA = MQZ_ENUMERATE_AUTHORITY_DATA;
/// Set Authority
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityName`: Entity name
/// * `EntityType`: Entity type
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `Authority`: Authority to be checked
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_SET_AUTHORITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110240_.html)
pub type MQZ_SET_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityName: crate::PMQCHAR,
        EntityType: crate::MQLONG,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        Authority: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_SET_AUTHORITY = MQZ_SET_AUTHORITY;
/// Set Authority Version 2
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityData`: Entity data
/// * `EntityType`: Entity type
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `Authority`: Authority to be checked
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_SET_AUTHORITY_2` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110250_.html)
pub type MQZ_SET_AUTHORITY_2 = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityData: PMQZED,
        EntityType: crate::MQLONG,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        Authority: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_SET_AUTHORITY_2 = MQZ_SET_AUTHORITY_2;
/// Copy All Authority
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `RefObjectName`: Reference object name
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_COPY_ALL_AUTHORITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110130_.html)
pub type MQZ_COPY_ALL_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        RefObjectName: crate::PMQCHAR,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_COPY_ALL_AUTHORITY = MQZ_COPY_ALL_AUTHORITY;
/// Check Authority
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityName`: Entity name
/// * `EntityType`: Entity type
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `Authority`: Authority to be checked
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_CHECK_AUTHORITY` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110100_.html)
pub type MQZ_CHECK_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityName: crate::PMQCHAR,
        EntityType: crate::MQLONG,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        Authority: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_CHECK_AUTHORITY = MQZ_CHECK_AUTHORITY;
/// Check Authority Version 2
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityData`: Entity data
/// * `EntityType`: Entity type
/// * `ObjectName`: Object name
/// * `ObjectType`: Object type
/// * `Authority`: Authority to be checked
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_CHECK_AUTHORITY_2` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110110_.html)
pub type MQZ_CHECK_AUTHORITY_2 = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityData: PMQZED,
        EntityType: crate::MQLONG,
        ObjectName: crate::PMQCHAR,
        ObjectType: crate::MQLONG,
        Authority: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_CHECK_AUTHORITY_2 = MQZ_CHECK_AUTHORITY_2;
/// Authenticate User
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `SecurityParms`: Security parameters
/// * `ApplicationContext`: Application context
/// * `IdentityContext`: Identity context
/// * `CorrelationPtr`: Correlation data
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_AUTHENTICATE_USER` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110090_.html)
pub type MQZ_AUTHENTICATE_USER = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        SecurityParms: crate::PMQCSP,
        ApplicationContext: PMQZAC,
        IdentityContext: PMQZIC,
        CorrelationPtr: crate::PMQPTR,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_AUTHENTICATE_USER = MQZ_AUTHENTICATE_USER;
/// Free User
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `FreeParms`: Free parameters
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_FREE_USER` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110160_.html)
pub type MQZ_FREE_USER = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        FreeParms: PMQZFP,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_FREE_USER = MQZ_FREE_USER;
/// Inquire
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `SelectorCount`: Count of selectors
/// * `Selectors`: Array of attribute selectors
/// * `IntAttrCount`: Count of integer attributes
/// * `IntAttrs`: Array of integer attributes
/// * `CharAttrLength`: Length of character attributes buffer
/// * `CharAttrs`: Character attributes
/// * `SelectorReturned` (Output): Array of returned selector indicators
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_INQUIRE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110220_.html)
pub type MQZ_INQUIRE = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        SelectorCount: crate::MQLONG,
        Selectors: crate::PMQLONG,
        IntAttrCount: crate::MQLONG,
        IntAttrs: crate::PMQLONG,
        CharAttrLength: crate::MQLONG,
        CharAttrs: crate::PMQCHAR,
        SelectorReturned: crate::PMQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_INQUIRE = MQZ_INQUIRE;
/// Refresh Cache
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_REFRESH_CACHE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110230_.html)
pub type MQZ_REFRESH_CACHE = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_REFRESH_CACHE = MQZ_REFRESH_CACHE;
/// Check if User is Privileged
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `EntityData`: Entity data
/// * `EntityType`: Entity type
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_CHECK_PRIVILEGED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110120_.html)
pub type MQZ_CHECK_PRIVILEGED = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        EntityData: PMQZED,
        EntityType: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_CHECK_PRIVILEGED = MQZ_CHECK_PRIVILEGED;
/// Initialize Name-Services
///
/// # Arguments
/// * `Hconfig`: Configuration handle
/// * `Options`: Initialization options
/// * `QMgrName`: Queue manager name
/// * `ComponentDataLength`: Length of component data
/// * `ComponentData` (Input/Output): Component data
/// * `Version`: Version number
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_INIT_NAME` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110280_.html)
pub type MQZ_INIT_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: crate::MQLONG,
        QMgrName: crate::PMQCHAR,
        ComponentDataLength: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Version: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_INIT_NAME = MQZ_INIT_NAME;
/// Terminate Name-Services
///
/// # Arguments
/// * `Hconfig`: Configuration handle
/// * `Options`: Termination options
/// * `QMgrName`: Queue manager name
/// * `ComponentData`: Component data
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_TERM_NAME` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110310_.html)
pub type MQZ_TERM_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: crate::MQLONG,
        QMgrName: crate::PMQCHAR,
        ComponentData: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_TERM_NAME = MQZ_TERM_NAME;
/// Look-Up Name
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `QName`: Queue name
/// * `ResolvedQMgrName`: Resolved queue manager name
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_LOOKUP_NAME` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110300_.html)
pub type MQZ_LOOKUP_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        QName: crate::PMQCHAR,
        ResolvedQMgrName: crate::PMQCHAR,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_LOOKUP_NAME = MQZ_LOOKUP_NAME;
/// Insert Name
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `QName`: Queue name
/// * `ResolvedQMgrName`: Resolved queue manager name
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_INSERT_NAME` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110290_.html)
pub type MQZ_INSERT_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        QName: crate::PMQCHAR,
        ResolvedQMgrName: crate::PMQCHAR,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_INSERT_NAME = MQZ_INSERT_NAME;
/// Delete Name
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `QName`: Queue name
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
///
/// # References
/// * [IBM `MQZ_DELETE_NAME` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110270_.html)
pub type MQZ_DELETE_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        QName: crate::PMQCHAR,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_DELETE_NAME = MQZ_DELETE_NAME;
/// Initialize Userid-Services
///
/// # Arguments
/// * `Hconfig`: Configuration handle
/// * `Options`: Initialization options
/// * `QMgrName`: Queue manager name
/// * `ComponentDataLength`: Length of component data
/// * `ComponentData` (Input/Output): Component data
/// * `Version`: Version number
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQZ_INIT_USERID = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: crate::MQLONG,
        QMgrName: crate::PMQCHAR,
        ComponentDataLength: crate::MQLONG,
        ComponentData: crate::PMQBYTE,
        Version: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_INIT_USERID = MQZ_INIT_USERID;
/// Terminate Userid-Services
///
/// # Arguments
/// * `Hconfig`: Configuration handle
/// * `Options`: Termination options
/// * `QMgrName`: Queue manager name
/// * `ComponentData`: Component data
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQZ_TERM_USERID = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: crate::MQLONG,
        QMgrName: crate::PMQCHAR,
        ComponentData: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_TERM_USERID = MQZ_TERM_USERID;
/// Find Userid
///
/// # Arguments
/// * `QMgrName`: Queue manager name
/// * `UserId`: User identifier
/// * `Password`: Password
/// * `ComponentData` (Input/Output): Component data
/// * `Continuation` (Output): Continuation indicator set by component
/// * `CompCode` (Output): Completion code
/// * `Reason` (Output): Reason code qualifying `CompCode`
pub type MQZ_FIND_USERID = ::std::option::Option<
    unsafe extern "C" fn(
        QMgrName: crate::PMQCHAR,
        UserId: crate::PMQCHAR,
        Password: crate::PMQCHAR,
        ComponentData: crate::PMQBYTE,
        Continuation: crate::PMQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
>;
pub type PMQZ_FIND_USERID = MQZ_FIND_USERID;
/// [IBM `MQACH` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109630_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQACH {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQACH`] structure
    pub StrucLength: crate::MQLONG,
    /// Total length of chain area
    pub ChainAreaLength: crate::MQLONG,
    /// Exit information name
    pub ExitInfoName: crate::MQCHAR48,
    /// Address of next [`MQACH`] structure in chain
    pub NextChainAreaPtr: PMQACH,
}
/// API Exit Context
///
/// # References
/// * [IBM `MQAXC` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109620_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQAXC {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Environment
    pub Environment: crate::MQLONG,
    /// User identifier
    pub UserId: crate::MQCHAR12,
    /// Security identifier
    pub SecurityId: crate::MQBYTE40,
    /// Connection name
    pub ConnectionName: [crate::MQCHAR; 264usize],
    /// Length of long MCA user identifier
    pub LongMCAUserIdLength: crate::MQLONG,
    /// Length of long remote user identifier
    pub LongRemoteUserIdLength: crate::MQLONG,
    /// Address of long MCA user identifier
    pub LongMCAUserIdPtr: crate::MQPTR,
    /// Address of long remote user identifier
    pub LongRemoteUserIdPtr: crate::MQPTR,
    /// Application name
    pub ApplName: crate::MQCHAR28,
    /// Application type
    pub ApplType: crate::MQLONG,
    /// Process identifier
    pub ProcessId: crate::MQPID,
    /// Thread identifier
    pub ThreadId: crate::MQTID,
    /// Channel Name
    ///
    /// [`MQAXC::Version`] >= 2
    pub ChannelName: [crate::MQCHAR; 20usize],
    /// Reserved
    ///
    /// [`MQAXC::Version`] >= 2
    pub Reserved1: crate::MQBYTE4,
    /// Pointer to Channel Definition
    ///
    /// [`MQAXC::Version`] >= 2
    pub pChannelDefinition: crate::PMQCD,
}
/// API Exit Parameter
///
/// # References
/// * [IBM `MQAXP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109610_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQAXP {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Type of exit
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: crate::MQLONG,
    /// Feedback
    pub Feedback: crate::MQLONG,
    /// API caller type
    pub APICallerType: crate::MQLONG,
    /// Exit user area
    pub ExitUserArea: crate::MQBYTE16,
    /// Exit data
    pub ExitData: crate::MQCHAR32,
    /// Exit information name
    pub ExitInfoName: crate::MQCHAR48,
    /// Problem determination area
    pub ExitPDArea: crate::MQBYTE48,
    /// Name of local queue manager
    pub QMgrName: crate::MQCHAR48,
    /// Address of first [`MQACH`] structure in chain
    pub ExitChainAreaPtr: PMQACH,
    /// Configuration handle
    pub Hconfig: MQHCONFIG,
    /// API function identifier
    pub Function: crate::MQLONG,
    /// Exit message handle
    pub ExitMsgHandle: crate::MQHMSG,
}
/// Channel Exit Parameter
///
/// # References
/// * [IBM `MQCXP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109150_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQCXP {
    /// Structure identifier
    ///
    /// [`MQCXP::Version`] >= 3
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    ///
    /// [`MQCXP::Version`] >= 3
    pub Version: crate::MQLONG,
    /// Type of exit
    ///
    /// [`MQCXP::Version`] >= 3
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    ///
    /// [`MQCXP::Version`] >= 3
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    ///
    /// [`MQCXP::Version`] >= 3
    pub ExitResponse: crate::MQLONG,
    /// Secondary response from exit
    ///
    /// [`MQCXP::Version`] >= 3
    pub ExitResponse2: crate::MQLONG,
    /// Feedback code
    ///
    /// [`MQCXP::Version`] >= 3
    pub Feedback: crate::MQLONG,
    /// Maximum segment length
    ///
    /// [`MQCXP::Version`] >= 3
    pub MaxSegmentLength: crate::MQLONG,
    /// Exit user area
    ///
    /// [`MQCXP::Version`] >= 3
    pub ExitUserArea: crate::MQBYTE16,
    /// Exit data
    ///
    /// [`MQCXP::Version`] >= 3
    pub ExitData: crate::MQCHAR32,
    /// Number of times the message has been retried
    ///
    /// [`MQCXP::Version`] >= 3
    pub MsgRetryCount: crate::MQLONG,
    /// Minimum interval in milliseconds after which the put operation should be retried
    ///
    /// [`MQCXP::Version`] >= 3
    pub MsgRetryInterval: crate::MQLONG,
    /// Reason code from previous attempt to put the message
    ///
    /// [`MQCXP::Version`] >= 3
    pub MsgRetryReason: crate::MQLONG,
    /// Length of header information
    ///
    /// [`MQCXP::Version`] >= 3
    pub HeaderLength: crate::MQLONG,
    /// Partner Name
    ///
    /// [`MQCXP::Version`] >= 3
    pub PartnerName: crate::MQCHAR48,
    /// Negotiated Formats and Protocols level
    ///
    /// [`MQCXP::Version`] >= 3
    pub FAPLevel: crate::MQLONG,
    /// Capability flags
    ///
    /// [`MQCXP::Version`] >= 3
    pub CapabilityFlags: crate::MQLONG,
    /// Exit number
    ///
    /// [`MQCXP::Version`] >= 3
    pub ExitNumber: crate::MQLONG,
    /// Number of bytes in transmission buffer reserved for exit to use
    ///
    /// [`MQCXP::Version`] >= 5
    pub ExitSpace: crate::MQLONG,
    /// User identifier associated with remote SSL certificate
    ///
    /// [`MQCXP::Version`] >= 6
    pub SSLCertUserid: crate::MQCHAR12,
    /// Length of distinguished name of issuer of remote SSL certificate
    ///
    /// [`MQCXP::Version`] >= 6
    pub SSLRemCertIssNameLength: crate::MQLONG,
    /// Address of distinguished name of issuer of remote SSL certificate
    ///
    /// [`MQCXP::Version`] >= 6
    pub SSLRemCertIssNamePtr: crate::MQPTR,
    /// Address of security parameters
    ///
    /// [`MQCXP::Version`] >= 6
    pub SecurityParms: crate::PMQCSP,
    /// Header data compression used for current message
    ///
    /// [`MQCXP::Version`] >= 6
    pub CurHdrCompression: crate::MQLONG,
    /// Message data compression used for current message
    ///
    /// [`MQCXP::Version`] >= 6
    pub CurMsgCompression: crate::MQLONG,
    /// Connection handle
    ///
    /// [`MQCXP::Version`] >= 7
    pub Hconn: crate::MQHCONN,
    /// Multiple conversations allowed
    ///
    /// [`MQCXP::Version`] >= 7
    pub SharingConversations: crate::MQBOOL,
    /// The source of the run-time user ID
    ///
    /// [`MQCXP::Version`] >= 8
    pub MCAUserSource: crate::MQLONG,
    /// Interface entry points
    ///
    /// [`MQCXP::Version`] >= 8
    pub pEntryPoints: PMQIEP,
    /// The identifier for the remote product
    ///
    /// [`MQCXP::Version`] >= 9
    pub RemoteProduct: crate::MQCHAR4,
    /// The version of the remote product
    ///
    /// [`MQCXP::Version`] >= 9
    pub RemoteVersion: crate::MQCHAR8,
}
/// Data Conversion Exit Parameter
///
/// # References
/// * [IBM `MQDXP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q104100_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQDXP {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Reserved
    pub ExitOptions: crate::MQLONG,
    /// Application options
    pub AppOptions: crate::MQLONG,
    /// Numeric encoding required by application
    pub Encoding: crate::MQLONG,
    /// Character set required by application
    pub CodedCharSetId: crate::MQLONG,
    /// Length in bytes of message data
    pub DataLength: crate::MQLONG,
    /// Completion code
    pub CompCode: crate::MQLONG,
    /// Reason code qualifying `CompCode`
    pub Reason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Connection handle
    pub Hconn: crate::MQHCONN,
    /// Interface entry points
    ///
    /// [`MQDXP::Version`] >= 2
    pub pEntryPoints: PMQIEP,
}
/// PreConnect Exit options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQNXP {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Type of exit
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: crate::MQLONG,
    /// Feedback
    pub Feedback: crate::MQLONG,
    /// Length of exit data
    pub ExitDataLength: crate::MQLONG,
    /// Address of exit data
    pub pExitDataPtr: crate::PMQCHAR,
    /// Address of exit user area
    pub pExitUserAreaPtr: crate::MQPTR,
    /// Address of pointers referencing [`MQCD`](crate::MQCD)
    pub ppMQCDArrayPtr: crate::PPMQCD,
    /// Count of [`MQCD`](crate::MQCD)  referenced
    pub MQCDArrayCount: crate::MQLONG,
    /// Maximum [`MQCD`](crate::MQCD) version requested
    pub MaxMQCDVersion: crate::MQLONG,
    /// Interface entry points
    ///
    /// [`MQNXP::Version`] >= 2
    pub pEntryPoints: PMQIEP,
}
/// Publish Exit Publication Context
///
/// # References
/// * [IBM `MQPBC` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108150_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQPBC {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Publish topic string
    pub PubTopicString: crate::MQCHARV,
    /// Address of publisher message descriptor
    ///
    /// [`MQPBC::Version`] >= 2
    pub MsgDescPtr: crate::PMQMD,
}
/// Publish Exit Parameter
///
/// # References
/// * [IBM `MQPSXP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108140_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQPSXP {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Type of exit
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Reserved
    pub ExitResponse2: crate::MQLONG,
    /// Feedback code
    pub Feedback: crate::MQLONG,
    /// Connection handle
    pub Hconn: crate::MQHCONN,
    /// Exit user area
    pub ExitUserArea: crate::MQBYTE16,
    /// Exit data
    pub ExitData: crate::MQCHAR32,
    /// Name of local queue manager
    pub QMgrName: crate::MQCHAR48,
    /// Handle to message properties
    pub MsgHandle: crate::MQHMSG,
    /// Address of message descriptor
    pub MsgDescPtr: crate::PMQMD,
    /// Address of input message data
    pub MsgInPtr: crate::PMQVOID,
    /// Length of input message data
    pub MsgInLength: crate::MQLONG,
    /// Address of output message data
    pub MsgOutPtr: crate::PMQVOID,
    /// Length of output message data
    pub MsgOutLength: crate::MQLONG,
    /// Interface entry points
    ///
    /// [`MQPSXP::Version`] >= 2
    pub pEntryPoints: PMQIEP,
}
/// Publish Exit Subscription Context
///
/// # References
/// * [IBM `MQSBC` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108160_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQSBC {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Destination queue manager
    pub DestinationQMgrName: crate::MQCHAR48,
    /// Destination queue name
    pub DestinationQName: crate::MQCHAR48,
    /// Type of subscription
    pub SubType: crate::MQLONG,
    /// Subscription options
    pub SubOptions: crate::MQLONG,
    /// Object name
    pub ObjectName: crate::MQCHAR48,
    /// Object string
    pub ObjectString: crate::MQCHARV,
    /// Subscription topic string
    pub SubTopicString: crate::MQCHARV,
    /// Subscription name
    pub SubName: crate::MQCHARV,
    /// Subscription identifier
    pub SubId: crate::MQBYTE24,
    /// Subscription selection string
    pub SelectionString: crate::MQCHARV,
    /// Subscription level
    pub SubLevel: crate::MQLONG,
    /// Publish/subscribe properties
    pub PSProperties: crate::MQLONG,
}
/// Cluster Workload Exit Cluster Record
///
/// # References
/// * [IBM `MQWCR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q082670_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWCR {
    /// Cluster name
    pub ClusterName: crate::MQCHAR48,
    /// Offset of next cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Cluster flags
    pub ClusterFlags: crate::MQLONG,
}
/// Cluster Workload Exit Destination Record
///
/// # References
/// * [IBM `MQWDR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q082610_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWDR {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQWDR`] structure
    pub StrucLength: crate::MQLONG,
    /// Queue-manager flags
    pub QMgrFlags: crate::MQLONG,
    /// Queue-manager identifier
    pub QMgrIdentifier: crate::MQCHAR48,
    /// Queue-manager name
    pub QMgrName: crate::MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Channel state
    pub ChannelState: crate::MQLONG,
    /// Offset of channel definition structure
    pub ChannelDefOffset: crate::MQLONG,
    /// Cluster channel destination sequence number
    ///
    /// [`MQWDR::Version`] >= 2
    pub DestSeqNumber: crate::MQLONG,
    /// Cluster channel destination sequence factor
    ///
    /// [`MQWDR::Version`] >= 2
    pub DestSeqFactor: crate::MQINT64,
}
/// Version-1 CLWL Exit Destination Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWDR1 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQWDR`] structure
    pub StrucLength: crate::MQLONG,
    /// Queue-manager flags
    pub QMgrFlags: crate::MQLONG,
    /// Queue-manager identifier
    pub QMgrIdentifier: crate::MQCHAR48,
    /// Queue-manager name
    pub QMgrName: crate::MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Channel state
    pub ChannelState: crate::MQLONG,
    /// Offset of channel definition structure
    pub ChannelDefOffset: crate::MQLONG,
}
/// Version-2 CLWL Exit Destination Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWDR2 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQWDR`] structure
    pub StrucLength: crate::MQLONG,
    /// Queue-manager flags
    pub QMgrFlags: crate::MQLONG,
    /// Queue-manager identifier
    pub QMgrIdentifier: crate::MQCHAR48,
    /// Queue-manager name
    pub QMgrName: crate::MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Channel state
    pub ChannelState: crate::MQLONG,
    /// Offset of channel definition structure
    pub ChannelDefOffset: crate::MQLONG,
    /// Cluster channel destination sequence number
    ///
    /// [`MQWDR2::Version`] >= 2
    pub DestSeqNumber: crate::MQLONG,
    /// Cluster channel destination sequence factor
    ///
    /// [`MQWDR2::Version`] >= 2
    pub DestSeqFactor: crate::MQINT64,
}
/// Cluster Workload Exit Queue Record
///
/// # References
/// * [IBM `MQWQR` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q082640_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWQR {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQWQR`] structure
    pub StrucLength: crate::MQLONG,
    /// Queue flags
    pub QFlags: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: crate::MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Queue type
    pub QType: crate::MQLONG,
    /// Queue description
    pub QDesc: crate::MQCHAR64,
    /// Default binding
    pub DefBind: crate::MQLONG,
    /// Default message persistence
    pub DefPersistence: crate::MQLONG,
    /// Default message priority
    pub DefPriority: crate::MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: crate::MQLONG,
    /// Queue priority
    ///
    /// [`MQWQR::Version`] >= 2
    pub CLWLQueuePriority: crate::MQLONG,
    /// Queue rank
    ///
    /// [`MQWQR::Version`] >= 2
    pub CLWLQueueRank: crate::MQLONG,
    /// Default put response
    ///
    /// [`MQWQR::Version`] >= 3
    pub DefPutResponse: crate::MQLONG,
    /// CapExpiry
    ///
    /// [`MQWQR::Version`] >= 4
    pub CapExpiry: crate::MQLONG,
}
/// Version-1 CLWL Exit Queue Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWQR1 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQWQR`] structure
    pub StrucLength: crate::MQLONG,
    /// Queue flags
    pub QFlags: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: crate::MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Queue type
    pub QType: crate::MQLONG,
    /// Queue description
    pub QDesc: crate::MQCHAR64,
    /// Default binding
    pub DefBind: crate::MQLONG,
    /// Default message persistence
    pub DefPersistence: crate::MQLONG,
    /// Default message priority
    pub DefPriority: crate::MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: crate::MQLONG,
}
/// Version-2 CLWL Exit Queue Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWQR2 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQWQR`] structure
    pub StrucLength: crate::MQLONG,
    /// Queue flags
    pub QFlags: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: crate::MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Queue type
    pub QType: crate::MQLONG,
    /// Queue description
    pub QDesc: crate::MQCHAR64,
    /// Default binding
    pub DefBind: crate::MQLONG,
    /// Default message persistence
    pub DefPersistence: crate::MQLONG,
    /// Default message priority
    pub DefPriority: crate::MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: crate::MQLONG,
    /// Queue priority
    ///
    /// [`MQWQR2::Version`] >= 2
    pub CLWLQueuePriority: crate::MQLONG,
    /// Queue rank
    ///
    /// [`MQWQR2::Version`] >= 2
    pub CLWLQueueRank: crate::MQLONG,
}
/// Version-3 CLWL Exit Queue Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWQR3 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQWQR`] structure
    pub StrucLength: crate::MQLONG,
    /// Queue flags
    pub QFlags: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: crate::MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Queue type
    pub QType: crate::MQLONG,
    /// Queue description
    pub QDesc: crate::MQCHAR64,
    /// Default binding
    pub DefBind: crate::MQLONG,
    /// Default message persistence
    pub DefPersistence: crate::MQLONG,
    /// Default message priority
    pub DefPriority: crate::MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: crate::MQLONG,
    /// Queue priority
    ///
    /// [`MQWQR3::Version`] >= 2
    pub CLWLQueuePriority: crate::MQLONG,
    /// Queue rank
    ///
    /// [`MQWQR3::Version`] >= 2
    pub CLWLQueueRank: crate::MQLONG,
    /// Default put response
    ///
    /// [`MQWQR3::Version`] >= 3
    pub DefPutResponse: crate::MQLONG,
}
/// Version-4 CLWL Exit Queue Record
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWQR4 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQWQR`] structure
    pub StrucLength: crate::MQLONG,
    /// Queue flags
    pub QFlags: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: crate::MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: crate::MQLONG,
    /// Queue type
    pub QType: crate::MQLONG,
    /// Queue description
    pub QDesc: crate::MQCHAR64,
    /// Default binding
    pub DefBind: crate::MQLONG,
    /// Default message persistence
    pub DefPersistence: crate::MQLONG,
    /// Default message priority
    pub DefPriority: crate::MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: crate::MQLONG,
    /// Queue priority
    ///
    /// [`MQWQR4::Version`] >= 2
    pub CLWLQueuePriority: crate::MQLONG,
    /// Queue rank
    ///
    /// [`MQWQR4::Version`] >= 2
    pub CLWLQueueRank: crate::MQLONG,
    /// Default put response
    ///
    /// [`MQWQR4::Version`] >= 3
    pub DefPutResponse: crate::MQLONG,
    /// CapExpiry
    ///
    /// [`MQWQR4::Version`] >= 4
    pub CapExpiry: crate::MQLONG,
}
/// Cluster Workload Exit Parameter
///
/// # References
/// * [IBM `MQWXP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q082580_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWXP {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Type of exit
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: crate::MQLONG,
    /// Reserved
    pub Feedback: crate::MQLONG,
    /// Flags
    pub Flags: crate::MQLONG,
    /// Exit user area
    pub ExitUserArea: crate::MQBYTE16,
    /// Exit data
    pub ExitData: crate::MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: crate::PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: crate::PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: crate::MQLONG,
    /// Length of complete message
    pub MsgLength: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: crate::MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: crate::MQLONG,
    /// Destination chosen
    pub DestinationChosen: crate::MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
    /// Context information
    ///
    /// [`MQWXP::Version`] >= 2
    pub CacheContext: crate::MQPTR,
    /// Type of cluster cache
    ///
    /// [`MQWXP::Version`] >= 2
    pub CacheType: crate::MQLONG,
    /// Number of allowed active outbound channels
    ///
    /// [`MQWXP::Version`] >= 3
    pub CLWLMRUChannels: crate::MQLONG,
    /// Interface entry points
    ///
    /// [`MQWXP::Version`] >= 4
    pub pEntryPoints: PMQIEP,
}
/// Version-1 CLWL Exit Parameter
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWXP1 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Type of exit
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: crate::MQLONG,
    /// Reserved
    pub Feedback: crate::MQLONG,
    /// Flags
    pub Flags: crate::MQLONG,
    /// Exit user area
    pub ExitUserArea: crate::MQBYTE16,
    /// Exit data
    pub ExitData: crate::MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: crate::PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: crate::PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: crate::MQLONG,
    /// Length of complete message
    pub MsgLength: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: crate::MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: crate::MQLONG,
    /// Destination chosen
    pub DestinationChosen: crate::MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
}
/// Version-2 CLWL Exit Parameter
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWXP2 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Type of exit
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: crate::MQLONG,
    /// Reserved
    pub Feedback: crate::MQLONG,
    /// Flags
    pub Flags: crate::MQLONG,
    /// Exit user area
    pub ExitUserArea: crate::MQBYTE16,
    /// Exit data
    pub ExitData: crate::MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: crate::PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: crate::PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: crate::MQLONG,
    /// Length of complete message
    pub MsgLength: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: crate::MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: crate::MQLONG,
    /// Destination chosen
    pub DestinationChosen: crate::MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
    /// Context information
    ///
    /// [`MQWXP2::Version`] >= 2
    pub CacheContext: crate::MQPTR,
    /// Type of cluster cache
    ///
    /// [`MQWXP2::Version`] >= 2
    pub CacheType: crate::MQLONG,
}
/// Version-3 CLWL Exit Parameter
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWXP3 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Type of exit
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: crate::MQLONG,
    /// Reserved
    pub Feedback: crate::MQLONG,
    /// Flags
    pub Flags: crate::MQLONG,
    /// Exit user area
    pub ExitUserArea: crate::MQBYTE16,
    /// Exit data
    pub ExitData: crate::MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: crate::PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: crate::PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: crate::MQLONG,
    /// Length of complete message
    pub MsgLength: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: crate::MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: crate::MQLONG,
    /// Destination chosen
    pub DestinationChosen: crate::MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
    /// Context information
    ///
    /// [`MQWXP3::Version`] >= 2
    pub CacheContext: crate::MQPTR,
    /// Type of cluster cache
    ///
    /// [`MQWXP3::Version`] >= 2
    pub CacheType: crate::MQLONG,
    /// Number of allowed active outbound channels
    ///
    /// [`MQWXP3::Version`] >= 3
    pub CLWLMRUChannels: crate::MQLONG,
}
/// Version-4 CLWL Exit Parameter
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQWXP4 {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Type of exit
    pub ExitId: crate::MQLONG,
    /// Reason for invoking exit
    pub ExitReason: crate::MQLONG,
    /// Response from exit
    pub ExitResponse: crate::MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: crate::MQLONG,
    /// Reserved
    pub Feedback: crate::MQLONG,
    /// Flags
    pub Flags: crate::MQLONG,
    /// Exit user area
    pub ExitUserArea: crate::MQBYTE16,
    /// Exit data
    pub ExitData: crate::MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: crate::PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: crate::PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: crate::MQLONG,
    /// Length of complete message
    pub MsgLength: crate::MQLONG,
    /// Queue name
    pub QName: crate::MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: crate::MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: crate::MQLONG,
    /// Destination chosen
    pub DestinationChosen: crate::MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
    /// Context information
    ///
    /// [`MQWXP4::Version`] >= 2
    pub CacheContext: crate::MQPTR,
    /// Type of cluster cache
    ///
    /// [`MQWXP4::Version`] >= 2
    pub CacheType: crate::MQLONG,
    /// Number of allowed active outbound channels
    ///
    /// [`MQWXP4::Version`] >= 3
    pub CLWLMRUChannels: crate::MQLONG,
    /// Interface entry points
    ///
    /// [`MQWXP4::Version`] >= 4
    pub pEntryPoints: PMQIEP,
}
/// Register entry point options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQXEPO {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Options that control the action of [`MQXEP`]
    pub Options: crate::MQLONG,
    /// Exit properties
    pub ExitProperties: crate::MQCHARV,
}
/// Entity Data
///
/// # References
/// * [IBM `MQZED` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110340_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQZED {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Address of entity name
    pub EntityNamePtr: crate::PMQCHAR,
    /// Address of entity domain name
    pub EntityDomainPtr: crate::PMQCHAR,
    /// Security identifier
    pub SecurityId: crate::MQBYTE40,
    /// Address of correlational data
    ///
    /// [`MQZED::Version`] >= 2
    pub CorrelationPtr: crate::MQPTR,
}
/// Application Context
///
/// # References
/// * [IBM `MQZAC` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110320_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQZAC {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Process identifier of application
    pub ProcessId: crate::MQPID,
    /// Thread identifier of application
    pub ThreadId: crate::MQTID,
    /// Application name
    pub ApplName: crate::MQCHAR28,
    /// User ID of application
    pub UserID: crate::MQCHAR12,
    /// Effective user ID of application
    pub EffectiveUserID: crate::MQCHAR12,
    /// Environment of caller
    pub Environment: crate::MQLONG,
    /// Type of caller
    pub CallerType: crate::MQLONG,
    /// Type of authentication being performed
    pub AuthenticationType: crate::MQLONG,
    /// Type of bindings in use
    pub BindType: crate::MQLONG,
}
/// Authority Data
///
/// # References
/// * [IBM `MQZAD` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110330_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQZAD {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Profile name
    pub ProfileName: crate::MQCHAR48,
    /// Object type
    pub ObjectType: crate::MQLONG,
    /// Authority
    pub Authority: crate::MQLONG,
    /// Address of [`MQZED`] structure identifying an entity
    pub EntityDataPtr: PMQZED,
    /// Entity type
    pub EntityType: crate::MQLONG,
    /// Options
    ///
    /// [`MQZAD::Version`] >= 2
    pub Options: crate::MQLONG,
}
/// Free Parameters
///
/// # References
/// * [IBM `MQZFP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110360_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQZFP {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Reserved
    pub Reserved: crate::MQBYTE8,
    /// Address of correlational data
    pub CorrelationPtr: crate::MQPTR,
}
/// Identity Context
///
/// # References
/// * [IBM `MQZIC` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110370_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQZIC {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// User identifier
    pub UserIdentifier: crate::MQCHAR12,
    /// Accounting token
    pub AccountingToken: crate::MQBYTE32,
    /// Application data relating to identity
    pub ApplIdentityData: crate::MQCHAR32,
    /// Long user identifier
    ///
    /// [`MQZIC::Version`] >= 2
    pub LongUserIdentifier: crate::MQCHAR1024,
}
/// Interface Entry Points
///
/// # References
/// * [IBM `MQIEP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q108070_.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MQIEP {
    /// Structure identifier
    pub StrucId: crate::MQCHAR4,
    /// Structure version number
    pub Version: crate::MQLONG,
    /// Length of [`MQIEP`] structure
    pub StrucLength: crate::MQLONG,
    /// Flags containing information about the interface entry points
    pub Flags: crate::MQLONG,
    /// Reserved
    pub Reserved: crate::MQPTR,
    /// [`MQBACK`](crate::MQBACK) entry point
    pub MQBACK_Call: PMQ_BACK_CALL,
    /// [`MQBEGIN`](crate::MQBEGIN) entry point
    pub MQBEGIN_Call: PMQ_BEGIN_CALL,
    /// [`MQBUFMH`](crate::MQBUFMH) entry point
    pub MQBUFMH_Call: PMQ_BUFMH_CALL,
    /// [`MQCB`](crate::MQCB) entry point
    pub MQCB_Call: PMQ_CB_CALL,
    /// [`MQCLOSE`](crate::MQCLOSE) entry point
    pub MQCLOSE_Call: PMQ_CLOSE_CALL,
    /// [`MQCMIT`](crate::MQCMIT) entry point
    pub MQCMIT_Call: PMQ_CMIT_CALL,
    /// [`MQCONN`](crate::MQCONN) entry point
    pub MQCONN_Call: PMQ_CONN_CALL,
    /// [`MQCONNX`](crate::MQCONNX) entry point
    pub MQCONNX_Call: PMQ_CONNX_CALL,
    /// [`MQCRTMH`](crate::MQCRTMH) entry point
    pub MQCRTMH_Call: PMQ_CRTMH_CALL,
    /// [`MQCTL`](crate::MQCTL) entry point
    pub MQCTL_Call: PMQ_CTL_CALL,
    /// [`MQDISC`](crate::MQDISC) entry point
    pub MQDISC_Call: PMQ_DISC_CALL,
    /// [`MQDLTMH`](crate::MQDLTMH) entry point
    pub MQDLTMH_Call: PMQ_DLTMH_CALL,
    /// [`MQDLTMP`](crate::MQDLTMP) entry point
    pub MQDLTMP_Call: PMQ_DLTMP_CALL,
    /// [`MQGET`](crate::MQGET) entry point
    pub MQGET_Call: PMQ_GET_CALL,
    /// [`MQINQ`](crate::MQINQ) entry point
    pub MQINQ_Call: PMQ_INQ_CALL,
    /// [`MQINQMP`](crate::MQINQMP) entry point
    pub MQINQMP_Call: PMQ_INQMP_CALL,
    /// [`MQMHBUF`](crate::MQMHBUF) entry point
    pub MQMHBUF_Call: PMQ_MHBUF_CALL,
    /// [`MQOPEN`](crate::MQOPEN) entry point
    pub MQOPEN_Call: PMQ_OPEN_CALL,
    /// [`MQPUT`](crate::MQPUT) entry point
    pub MQPUT_Call: PMQ_PUT_CALL,
    /// [`MQPUT1`](crate::MQPUT1) entry point
    pub MQPUT1_Call: PMQ_PUT1_CALL,
    /// [`MQSET`](crate::MQSET) entry point
    pub MQSET_Call: PMQ_SET_CALL,
    /// [`MQSETMP`](crate::MQSETMP) entry point
    pub MQSETMP_Call: PMQ_SETMP_CALL,
    /// [`MQSTAT`](crate::MQSTAT) entry point
    pub MQSTAT_Call: PMQ_STAT_CALL,
    /// [`MQSUB`](crate::MQSUB) entry point
    pub MQSUB_Call: PMQ_SUB_CALL,
    /// [`MQSUBRQ`](crate::MQSUBRQ) entry point
    pub MQSUBRQ_Call: PMQ_SUBRQ_CALL,
    /// [`MQXCLWLN`] entry point
    pub MQXCLWLN_Call: PMQ_XCLWLN_CALL,
    /// [`MQXCNVC`](crate::MQXCNVC) entry point
    pub MQXCNVC_Call: PMQ_XCNVC_CALL,
    /// [`MQXDX`] entry point
    pub MQXDX_Call: PMQ_XDX_CALL,
    /// [`MQXEP`] entry point
    pub MQXEP_Call: PMQ_XEP_CALL,
    /// [`MQZEP`] entry point
    pub MQZEP_Call: PMQ_ZEP_CALL,
}
pub const MQPA_DEFAULT: crate::MQLONG = 1;
pub const MQPA_CONTEXT: crate::MQLONG = 2;
pub const MQPA_ONLY_MCA: crate::MQLONG = 3;
pub const MQPA_ALTERNATE_OR_MCA: crate::MQLONG = 4;
pub const MQCDC_SENDER_CONVERSION: crate::MQLONG = 1;
pub const MQCDC_NO_SENDER_CONVERSION: crate::MQLONG = 0;
pub const MQMCAT_PROCESS: crate::MQLONG = 1;
pub const MQMCAT_THREAD: crate::MQLONG = 2;
pub const MQNPMS_NORMAL: crate::MQLONG = 1;
pub const MQNPMS_FAST: crate::MQLONG = 2;
pub const MQSCA_REQUIRED: crate::MQLONG = 0;
pub const MQSCA_OPTIONAL: crate::MQLONG = 1;
pub const MQSCA_NEVER_REQUIRED: crate::MQLONG = 2;
pub const MQKAI_AUTO: crate::MQLONG = -1;
pub const MQRCN_NO: crate::MQLONG = 0;
pub const MQRCN_YES: crate::MQLONG = 1;
pub const MQRCN_Q_MGR: crate::MQLONG = 2;
pub const MQRCN_DISABLED: crate::MQLONG = 3;
pub const MQPROTO_MQTTV3: crate::MQLONG = 1;
pub const MQPROTO_HTTP: crate::MQLONG = 2;
pub const MQPROTO_AMQP: crate::MQLONG = 3;
pub const MQPROTO_MQTTV311: crate::MQLONG = 4;
pub const MQSECPROT_NONE: crate::MQLONG = 0;
pub const MQSECPROT_SSLV30: crate::MQLONG = 1;
pub const MQSECPROT_TLSV10: crate::MQLONG = 2;
pub const MQSECPROT_TLSV12: crate::MQLONG = 4;
pub const MQSECPROT_TLSV13: crate::MQLONG = 8;
pub const MQSPL_PASSTHRU: crate::MQLONG = 0;
pub const MQSPL_REMOVE: crate::MQLONG = 1;
pub const MQSPL_AS_POLICY: crate::MQLONG = 2;
pub const MQACH_STRUC_ID: &::std::ffi::CStr = c"ACH ";
pub const MQACH_VERSION_1: crate::MQLONG = 1;
pub const MQACH_CURRENT_VERSION: crate::MQLONG = 1;
pub const MQACH_LENGTH_1: usize = 72;
pub const MQACH_CURRENT_LENGTH: usize = 72;
pub const MQAXC_STRUC_ID: &::std::ffi::CStr = c"AXC ";
pub const MQAXC_VERSION_1: crate::MQLONG = 1;
pub const MQAXC_VERSION_2: crate::MQLONG = 2;
pub const MQAXC_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQAXC_LENGTH_1: usize = 392;
pub const MQAXC_LENGTH_2: usize = 424;
pub const MQAXC_CURRENT_LENGTH: usize = 424;
pub const MQXE_OTHER: crate::MQLONG = 0;
pub const MQXE_MCA: crate::MQLONG = 1;
pub const MQXE_MCA_SVRCONN: crate::MQLONG = 2;
pub const MQXE_COMMAND_SERVER: crate::MQLONG = 3;
pub const MQXE_MQSC: crate::MQLONG = 4;
pub const MQXE_MCA_CLNTCONN: crate::MQLONG = 5;
pub const MQAXP_STRUC_ID: &::std::ffi::CStr = c"AXP ";
pub const MQAXP_VERSION_1: crate::MQLONG = 1;
pub const MQAXP_VERSION_2: crate::MQLONG = 2;
pub const MQAXP_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQAXP_LENGTH_1: usize = 256;
pub const MQAXP_CURRENT_LENGTH: usize = 256;
pub const MQXACT_EXTERNAL: crate::MQLONG = 1;
pub const MQXACT_INTERNAL: crate::MQLONG = 2;
pub const MQXPDA_NONE: &[u8; 49] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQXF_INIT: crate::MQLONG = 1;
pub const MQXF_TERM: crate::MQLONG = 2;
pub const MQXF_CONN: crate::MQLONG = 3;
pub const MQXF_CONNX: crate::MQLONG = 4;
pub const MQXF_DISC: crate::MQLONG = 5;
pub const MQXF_OPEN: crate::MQLONG = 6;
pub const MQXF_CLOSE: crate::MQLONG = 7;
pub const MQXF_PUT1: crate::MQLONG = 8;
pub const MQXF_PUT: crate::MQLONG = 9;
pub const MQXF_GET: crate::MQLONG = 10;
/// [IBM `MQXF_DATA_CONV_ON_GET` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109800_.html)
pub const MQXF_DATA_CONV_ON_GET: crate::MQLONG = 11;
pub const MQXF_INQ: crate::MQLONG = 12;
pub const MQXF_SET: crate::MQLONG = 13;
pub const MQXF_BEGIN: crate::MQLONG = 14;
pub const MQXF_CMIT: crate::MQLONG = 15;
pub const MQXF_BACK: crate::MQLONG = 16;
pub const MQXF_STAT: crate::MQLONG = 18;
pub const MQXF_CB: crate::MQLONG = 19;
pub const MQXF_CTL: crate::MQLONG = 20;
pub const MQXF_CALLBACK: crate::MQLONG = 21;
pub const MQXF_SUB: crate::MQLONG = 22;
pub const MQXF_SUBRQ: crate::MQLONG = 23;
pub const MQXF_XACLOSE: crate::MQLONG = 24;
pub const MQXF_XACOMMIT: crate::MQLONG = 25;
pub const MQXF_XACOMPLETE: crate::MQLONG = 26;
pub const MQXF_XAEND: crate::MQLONG = 27;
pub const MQXF_XAFORGET: crate::MQLONG = 28;
pub const MQXF_XAOPEN: crate::MQLONG = 29;
pub const MQXF_XAPREPARE: crate::MQLONG = 30;
pub const MQXF_XARECOVER: crate::MQLONG = 31;
pub const MQXF_XAROLLBACK: crate::MQLONG = 32;
pub const MQXF_XASTART: crate::MQLONG = 33;
pub const MQXF_AXREG: crate::MQLONG = 34;
pub const MQXF_AXUNREG: crate::MQLONG = 35;
pub const MQCXP_STRUC_ID: &::std::ffi::CStr = c"CXP ";
pub const MQCXP_VERSION_1: crate::MQLONG = 1;
pub const MQCXP_VERSION_2: crate::MQLONG = 2;
pub const MQCXP_VERSION_3: crate::MQLONG = 3;
pub const MQCXP_VERSION_4: crate::MQLONG = 4;
pub const MQCXP_VERSION_5: crate::MQLONG = 5;
pub const MQCXP_VERSION_6: crate::MQLONG = 6;
pub const MQCXP_VERSION_7: crate::MQLONG = 7;
pub const MQCXP_VERSION_8: crate::MQLONG = 8;
pub const MQCXP_VERSION_9: crate::MQLONG = 9;
pub const MQCXP_CURRENT_VERSION: crate::MQLONG = 9;
pub const MQCXP_LENGTH_3: usize = 156;
pub const MQCXP_LENGTH_4: usize = 156;
pub const MQCXP_LENGTH_5: usize = 160;
pub const MQCXP_LENGTH_6: usize = 200;
pub const MQCXP_LENGTH_7: usize = 208;
pub const MQCXP_LENGTH_8: usize = 224;
pub const MQCXP_LENGTH_9: usize = 240;
pub const MQCXP_CURRENT_LENGTH: usize = 240;
pub const MQXR2_PUT_WITH_DEF_ACTION: crate::MQLONG = 0;
pub const MQXR2_PUT_WITH_DEF_USERID: crate::MQLONG = 1;
pub const MQXR2_PUT_WITH_MSG_USERID: crate::MQLONG = 2;
pub const MQXR2_USE_AGENT_BUFFER: crate::MQLONG = 0;
pub const MQXR2_USE_EXIT_BUFFER: crate::MQLONG = 4;
pub const MQXR2_DEFAULT_CONTINUATION: crate::MQLONG = 0;
pub const MQXR2_CONTINUE_CHAIN: crate::MQLONG = 8;
pub const MQXR2_SUPPRESS_CHAIN: crate::MQLONG = 16;
pub const MQXR2_STATIC_CACHE: crate::MQLONG = 0;
pub const MQXR2_DYNAMIC_CACHE: crate::MQLONG = 32;
pub const MQCF_NONE: crate::MQLONG = 0;
pub const MQCF_DIST_LISTS: crate::MQLONG = 1;
pub const MQDXP_STRUC_ID: &::std::ffi::CStr = c"DXP ";
pub const MQDXP_VERSION_1: crate::MQLONG = 1;
pub const MQDXP_VERSION_2: crate::MQLONG = 2;
pub const MQDXP_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQDXP_LENGTH_1: usize = 44;
pub const MQDXP_LENGTH_2: usize = 56;
pub const MQDXP_CURRENT_LENGTH: usize = 56;
pub const MQXDR_OK: crate::MQLONG = 0;
pub const MQXDR_CONVERSION_FAILED: crate::MQLONG = 1;
pub const MQNXP_STRUC_ID: &::std::ffi::CStr = c"NXP ";
pub const MQNXP_VERSION_1: crate::MQLONG = 1;
pub const MQNXP_VERSION_2: crate::MQLONG = 2;
pub const MQNXP_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQNXP_LENGTH_1: usize = 64;
pub const MQNXP_LENGTH_2: usize = 72;
pub const MQNXP_CURRENT_LENGTH: usize = 72;
pub const MQPBC_STRUC_ID: &::std::ffi::CStr = c"PBC ";
pub const MQPBC_VERSION_1: crate::MQLONG = 1;
pub const MQPBC_VERSION_2: crate::MQLONG = 2;
pub const MQPBC_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQPBC_LENGTH_1: usize = 32;
pub const MQPBC_LENGTH_2: usize = 40;
pub const MQPBC_CURRENT_LENGTH: usize = 40;
pub const MQPSXP_STRUC_ID: &::std::ffi::CStr = c"PSXP";
pub const MQPSXP_VERSION_1: crate::MQLONG = 1;
pub const MQPSXP_VERSION_2: crate::MQLONG = 2;
pub const MQPSXP_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQPSXP_LENGTH_1: usize = 176;
pub const MQPSXP_LENGTH_2: usize = 184;
pub const MQPSXP_CURRENT_LENGTH: usize = 184;
pub const MQSBC_STRUC_ID: &::std::ffi::CStr = c"SBC ";
pub const MQSBC_VERSION_1: crate::MQLONG = 1;
pub const MQSBC_CURRENT_VERSION: crate::MQLONG = 1;
pub const MQSBC_LENGTH_1: usize = 288;
pub const MQSBC_CURRENT_LENGTH: usize = 288;
pub const MQWDR_STRUC_ID: &::std::ffi::CStr = c"WDR ";
pub const MQWDR_VERSION_1: crate::MQLONG = 1;
pub const MQWDR_VERSION_2: crate::MQLONG = 2;
pub const MQWDR_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQWDR_LENGTH_1: usize = 124;
pub const MQWDR_LENGTH_2: usize = 136;
pub const MQWDR_CURRENT_LENGTH: usize = 136;
pub const MQQMF_REPOSITORY_Q_MGR: crate::MQLONG = 2;
pub const MQQMF_CLUSSDR_USER_DEFINED: crate::MQLONG = 8;
pub const MQQMF_CLUSSDR_AUTO_DEFINED: crate::MQLONG = 16;
pub const MQQMF_AVAILABLE: crate::MQLONG = 32;
pub const MQWDR1_LENGTH_1: usize = 124;
pub const MQWDR1_CURRENT_LENGTH: usize = 124;
pub const MQWDR2_LENGTH_1: usize = 124;
pub const MQWDR2_LENGTH_2: usize = 136;
pub const MQWDR2_CURRENT_LENGTH: usize = 136;
pub const MQWQR_STRUC_ID: &::std::ffi::CStr = c"WQR ";
pub const MQWQR_VERSION_1: crate::MQLONG = 1;
pub const MQWQR_VERSION_2: crate::MQLONG = 2;
pub const MQWQR_VERSION_3: crate::MQLONG = 3;
pub const MQWQR_VERSION_4: crate::MQLONG = 4;
pub const MQWQR_CURRENT_VERSION: crate::MQLONG = 4;
pub const MQWQR_LENGTH_1: usize = 200;
pub const MQWQR_LENGTH_2: usize = 208;
pub const MQWQR_LENGTH_3: usize = 212;
pub const MQWQR_LENGTH_4: usize = 216;
pub const MQWQR_CURRENT_LENGTH: usize = 216;
pub const MQQF_LOCAL_Q: crate::MQLONG = 1;
pub const MQQF_CLWL_USEQ_ANY: crate::MQLONG = 64;
pub const MQQF_CLWL_USEQ_LOCAL: crate::MQLONG = 128;
pub const MQWQR1_LENGTH_1: usize = 200;
pub const MQWQR1_CURRENT_LENGTH: usize = 200;
pub const MQWQR2_LENGTH_1: usize = 200;
pub const MQWQR2_LENGTH_2: usize = 208;
pub const MQWQR2_CURRENT_LENGTH: usize = 208;
pub const MQWQR3_LENGTH_1: usize = 200;
pub const MQWQR3_LENGTH_2: usize = 208;
pub const MQWQR3_LENGTH_3: usize = 212;
pub const MQWQR3_CURRENT_LENGTH: usize = 212;
pub const MQWQR4_LENGTH_1: usize = 200;
pub const MQWQR4_LENGTH_2: usize = 208;
pub const MQWQR4_LENGTH_3: usize = 212;
pub const MQWQR4_LENGTH_4: usize = 216;
pub const MQWQR4_CURRENT_LENGTH: usize = 216;
pub const MQWXP_STRUC_ID: &::std::ffi::CStr = c"WXP ";
pub const MQWXP_VERSION_1: crate::MQLONG = 1;
pub const MQWXP_VERSION_2: crate::MQLONG = 2;
pub const MQWXP_VERSION_3: crate::MQLONG = 3;
pub const MQWXP_VERSION_4: crate::MQLONG = 4;
pub const MQWXP_CURRENT_VERSION: crate::MQLONG = 4;
pub const MQWXP_LENGTH_1: usize = 224;
pub const MQWXP_LENGTH_2: usize = 240;
pub const MQWXP_LENGTH_3: usize = 240;
pub const MQWXP_LENGTH_4: usize = 248;
pub const MQWXP_CURRENT_LENGTH: usize = 248;
pub const MQWXP_PUT_BY_CLUSTER_CHL: crate::MQLONG = 2;
pub const MQWXP1_LENGTH_1: usize = 224;
pub const MQWXP1_CURRENT_LENGTH: usize = 224;
pub const MQWXP2_LENGTH_1: usize = 224;
pub const MQWXP2_LENGTH_2: usize = 240;
pub const MQWXP2_CURRENT_LENGTH: usize = 240;
pub const MQWXP3_LENGTH_1: usize = 224;
pub const MQWXP3_LENGTH_2: usize = 240;
pub const MQWXP3_LENGTH_3: usize = 240;
pub const MQWXP3_CURRENT_LENGTH: usize = 240;
pub const MQWXP4_LENGTH_1: usize = 224;
pub const MQWXP4_LENGTH_2: usize = 240;
pub const MQWXP4_LENGTH_3: usize = 240;
pub const MQWXP4_LENGTH_4: usize = 248;
pub const MQWXP4_CURRENT_LENGTH: usize = 248;
pub const MQXEPO_STRUC_ID: &::std::ffi::CStr = c"XEPO";
pub const MQXEPO_VERSION_1: crate::MQLONG = 1;
pub const MQXEPO_CURRENT_VERSION: crate::MQLONG = 1;
pub const MQXEPO_LENGTH_1: usize = 40;
pub const MQXEPO_CURRENT_LENGTH: usize = 40;
pub const MQXEPO_NONE: crate::MQLONG = 0;
pub const MQXT_API_CROSSING_EXIT: crate::MQLONG = 1;
pub const MQXT_API_EXIT: crate::MQLONG = 2;
pub const MQXT_CHANNEL_SEC_EXIT: crate::MQLONG = 11;
pub const MQXT_CHANNEL_MSG_EXIT: crate::MQLONG = 12;
pub const MQXT_CHANNEL_SEND_EXIT: crate::MQLONG = 13;
pub const MQXT_CHANNEL_RCV_EXIT: crate::MQLONG = 14;
pub const MQXT_CHANNEL_MSG_RETRY_EXIT: crate::MQLONG = 15;
pub const MQXT_CHANNEL_AUTO_DEF_EXIT: crate::MQLONG = 16;
pub const MQXT_CLUSTER_WORKLOAD_EXIT: crate::MQLONG = 20;
pub const MQXT_PUBSUB_ROUTING_EXIT: crate::MQLONG = 21;
pub const MQXT_PUBLISH_EXIT: crate::MQLONG = 22;
pub const MQXT_PRECONNECT_EXIT: crate::MQLONG = 23;
pub const MQXR_BEFORE: crate::MQLONG = 1;
pub const MQXR_AFTER: crate::MQLONG = 2;
pub const MQXR_CONNECTION: crate::MQLONG = 3;
pub const MQXR_BEFORE_CONVERT: crate::MQLONG = 4;
pub const MQXR_INIT: crate::MQLONG = 11;
pub const MQXR_TERM: crate::MQLONG = 12;
pub const MQXR_MSG: crate::MQLONG = 13;
pub const MQXR_XMIT: crate::MQLONG = 14;
pub const MQXR_SEC_MSG: crate::MQLONG = 15;
pub const MQXR_INIT_SEC: crate::MQLONG = 16;
pub const MQXR_RETRY: crate::MQLONG = 17;
pub const MQXR_AUTO_CLUSSDR: crate::MQLONG = 18;
pub const MQXR_AUTO_RECEIVER: crate::MQLONG = 19;
pub const MQXR_CLWL_OPEN: crate::MQLONG = 20;
pub const MQXR_CLWL_PUT: crate::MQLONG = 21;
pub const MQXR_CLWL_MOVE: crate::MQLONG = 22;
pub const MQXR_CLWL_REPOS: crate::MQLONG = 23;
pub const MQXR_CLWL_REPOS_MOVE: crate::MQLONG = 24;
pub const MQXR_END_BATCH: crate::MQLONG = 25;
pub const MQXR_ACK_RECEIVED: crate::MQLONG = 26;
pub const MQXR_AUTO_SVRCONN: crate::MQLONG = 27;
pub const MQXR_AUTO_CLUSRCVR: crate::MQLONG = 28;
pub const MQXR_SEC_PARMS: crate::MQLONG = 29;
pub const MQXR_PUBLICATION: crate::MQLONG = 30;
pub const MQXR_PRECONNECT: crate::MQLONG = 31;
pub const MQXCC_OK: crate::MQLONG = 0;
pub const MQXCC_SUPPRESS_FUNCTION: crate::MQLONG = -1;
pub const MQXCC_SKIP_FUNCTION: crate::MQLONG = -2;
pub const MQXCC_SEND_AND_REQUEST_SEC_MSG: crate::MQLONG = -3;
pub const MQXCC_SEND_SEC_MSG: crate::MQLONG = -4;
pub const MQXCC_SUPPRESS_EXIT: crate::MQLONG = -5;
pub const MQXCC_CLOSE_CHANNEL: crate::MQLONG = -6;
pub const MQXCC_REQUEST_ACK: crate::MQLONG = -7;
pub const MQXCC_FAILED: crate::MQLONG = -8;
pub const MQXUA_NONE: &[u8; 17] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQCLCT_STATIC: crate::MQLONG = 0;
pub const MQCLCT_DYNAMIC: crate::MQLONG = 1;
pub const MQMCEV_PACKET_LOSS: crate::MQLONG = 1;
pub const MQMCEV_HEARTBEAT_TIMEOUT: crate::MQLONG = 2;
pub const MQMCEV_VERSION_CONFLICT: crate::MQLONG = 3;
pub const MQMCEV_RELIABILITY: crate::MQLONG = 4;
pub const MQMCEV_CLOSED_TRANS: crate::MQLONG = 5;
pub const MQMCEV_STREAM_ERROR: crate::MQLONG = 6;
pub const MQMCEV_NEW_SOURCE: crate::MQLONG = 10;
pub const MQMCEV_RECEIVE_QUEUE_TRIMMED: crate::MQLONG = 11;
pub const MQMCEV_PACKET_LOSS_NACK_EXPIRE: crate::MQLONG = 12;
pub const MQMCEV_ACK_RETRIES_EXCEEDED: crate::MQLONG = 13;
pub const MQMCEV_STREAM_SUSPEND_NACK: crate::MQLONG = 14;
pub const MQMCEV_STREAM_RESUME_NACK: crate::MQLONG = 15;
pub const MQMCEV_STREAM_EXPELLED: crate::MQLONG = 16;
pub const MQMCEV_FIRST_MESSAGE: crate::MQLONG = 20;
pub const MQMCEV_LATE_JOIN_FAILURE: crate::MQLONG = 21;
pub const MQMCEV_MESSAGE_LOSS: crate::MQLONG = 22;
pub const MQMCEV_SEND_PACKET_FAILURE: crate::MQLONG = 23;
pub const MQMCEV_REPAIR_DELAY: crate::MQLONG = 24;
pub const MQMCEV_MEMORY_ALERT_ON: crate::MQLONG = 25;
pub const MQMCEV_MEMORY_ALERT_OFF: crate::MQLONG = 26;
pub const MQMCEV_NACK_ALERT_ON: crate::MQLONG = 27;
pub const MQMCEV_NACK_ALERT_OFF: crate::MQLONG = 28;
pub const MQMCEV_REPAIR_ALERT_ON: crate::MQLONG = 29;
pub const MQMCEV_REPAIR_ALERT_OFF: crate::MQLONG = 30;
pub const MQMCEV_RELIABILITY_CHANGED: crate::MQLONG = 31;
pub const MQMCEV_SHM_DEST_UNUSABLE: crate::MQLONG = 80;
pub const MQMCEV_SHM_PORT_UNUSABLE: crate::MQLONG = 81;
pub const MQMCEV_CCT_GETTIME_FAILED: crate::MQLONG = 110;
pub const MQMCEV_DEST_INTERFACE_FAILURE: crate::MQLONG = 120;
pub const MQMCEV_DEST_INTERFACE_FAILOVER: crate::MQLONG = 121;
pub const MQMCEV_PORT_INTERFACE_FAILURE: crate::MQLONG = 122;
pub const MQMCEV_PORT_INTERFACE_FAILOVER: crate::MQLONG = 123;
pub const MQZED_STRUC_ID: &::std::ffi::CStr = c"ZED ";
pub const MQZED_VERSION_1: crate::MQLONG = 1;
pub const MQZED_VERSION_2: crate::MQLONG = 2;
pub const MQZED_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQZED_LENGTH_1: usize = 64;
pub const MQZED_LENGTH_2: usize = 72;
pub const MQZED_CURRENT_LENGTH: usize = 72;
pub const MQZAC_STRUC_ID: &::std::ffi::CStr = c"ZAC ";
pub const MQZAC_VERSION_1: crate::MQLONG = 1;
pub const MQZAC_CURRENT_VERSION: crate::MQLONG = 1;
pub const MQZAC_LENGTH_1: usize = 84;
pub const MQZAC_CURRENT_LENGTH: usize = 84;
pub const MQZAT_INITIAL_CONTEXT: crate::MQLONG = 0;
pub const MQZAT_CHANGE_CONTEXT: crate::MQLONG = 1;
pub const MQZAD_STRUC_ID: &::std::ffi::CStr = c"ZAD ";
pub const MQZAD_VERSION_1: crate::MQLONG = 1;
pub const MQZAD_VERSION_2: crate::MQLONG = 2;
pub const MQZAD_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQZAD_LENGTH_1: usize = 80;
pub const MQZAD_LENGTH_2: usize = 80;
pub const MQZAD_CURRENT_LENGTH: usize = 80;
pub const MQZFP_STRUC_ID: &::std::ffi::CStr = c"ZFP ";
pub const MQZFP_VERSION_1: crate::MQLONG = 1;
pub const MQZFP_CURRENT_VERSION: crate::MQLONG = 1;
pub const MQZFP_LENGTH_1: usize = 24;
pub const MQZFP_CURRENT_LENGTH: usize = 24;
pub const MQZIC_STRUC_ID: &::std::ffi::CStr = c"ZIC ";
pub const MQZIC_VERSION_1: crate::MQLONG = 1;
pub const MQZIC_VERSION_2: crate::MQLONG = 2;
pub const MQZIC_CURRENT_VERSION: crate::MQLONG = 2;
pub const MQZIC_LENGTH_1: usize = 84;
pub const MQZIC_LENGTH_2: usize = 1108;
pub const MQZIC_CURRENT_LENGTH: usize = 1108;
pub const MQZIO_PRIMARY: crate::MQLONG = 0;
pub const MQZIO_SECONDARY: crate::MQLONG = 1;
pub const MQZTO_PRIMARY: crate::MQLONG = 0;
pub const MQZTO_SECONDARY: crate::MQLONG = 1;
pub const MQZCI_DEFAULT: crate::MQLONG = 0;
pub const MQZCI_CONTINUE: crate::MQLONG = 0;
pub const MQZCI_STOP: crate::MQLONG = 1;
pub const MQZAS_VERSION_1: crate::MQLONG = 1;
pub const MQZAS_VERSION_2: crate::MQLONG = 2;
pub const MQZAS_VERSION_3: crate::MQLONG = 3;
pub const MQZAS_VERSION_4: crate::MQLONG = 4;
pub const MQZAS_VERSION_5: crate::MQLONG = 5;
pub const MQZAS_VERSION_6: crate::MQLONG = 6;
pub const MQZAO_CONNECT: crate::MQLONG = 1;
pub const MQZAO_BROWSE: crate::MQLONG = 2;
pub const MQZAO_INPUT: crate::MQLONG = 4;
pub const MQZAO_OUTPUT: crate::MQLONG = 8;
pub const MQZAO_INQUIRE: crate::MQLONG = 16;
pub const MQZAO_SET: crate::MQLONG = 32;
pub const MQZAO_PASS_IDENTITY_CONTEXT: crate::MQLONG = 64;
pub const MQZAO_PASS_ALL_CONTEXT: crate::MQLONG = 128;
pub const MQZAO_SET_IDENTITY_CONTEXT: crate::MQLONG = 256;
pub const MQZAO_SET_ALL_CONTEXT: crate::MQLONG = 512;
pub const MQZAO_ALTERNATE_USER_AUTHORITY: crate::MQLONG = 1024;
pub const MQZAO_PUBLISH: crate::MQLONG = 2048;
pub const MQZAO_SUBSCRIBE: crate::MQLONG = 4096;
pub const MQZAO_RESUME: crate::MQLONG = 8192;
pub const MQZAO_ALL_MQI: crate::MQLONG = 16383;
pub const MQZAO_CREATE: crate::MQLONG = 65536;
pub const MQZAO_DELETE: crate::MQLONG = 131072;
pub const MQZAO_DISPLAY: crate::MQLONG = 262144;
pub const MQZAO_CHANGE: crate::MQLONG = 524288;
pub const MQZAO_CLEAR: crate::MQLONG = 1048576;
pub const MQZAO_CONTROL: crate::MQLONG = 2097152;
pub const MQZAO_CONTROL_EXTENDED: crate::MQLONG = 4194304;
pub const MQZAO_AUTHORIZE: crate::MQLONG = 8388608;
pub const MQZAO_ALL_ADMIN: crate::MQLONG = 16646144;
pub const MQZAO_SYSTEM: crate::MQLONG = 33554432;
pub const MQZAO_ALL: crate::MQLONG = 50216959;
pub const MQZAO_REMOVE: crate::MQLONG = 16777216;
pub const MQZAO_NONE: crate::MQLONG = 0;
pub const MQZAO_CREATE_ONLY: crate::MQLONG = 67108864;
pub const MQZAET_NONE: crate::MQLONG = 0;
pub const MQZAET_PRINCIPAL: crate::MQLONG = 1;
pub const MQZAET_GROUP: crate::MQLONG = 2;
pub const MQZAET_UNKNOWN: crate::MQLONG = 3;
pub const MQZSE_START: crate::MQLONG = 1;
pub const MQZSE_CONTINUE: crate::MQLONG = 0;
pub const MQZSL_NOT_RETURNED: crate::MQLONG = 0;
pub const MQZSL_RETURNED: crate::MQLONG = 1;
pub const MQZNS_VERSION_1: crate::MQLONG = 1;
pub const MQZUS_VERSION_1: crate::MQLONG = 1;
pub const MQZID_INIT: crate::MQLONG = 0;
pub const MQZID_TERM: crate::MQLONG = 1;
pub const MQZID_INIT_AUTHORITY: crate::MQLONG = 0;
pub const MQZID_TERM_AUTHORITY: crate::MQLONG = 1;
pub const MQZID_CHECK_AUTHORITY: crate::MQLONG = 2;
pub const MQZID_COPY_ALL_AUTHORITY: crate::MQLONG = 3;
pub const MQZID_DELETE_AUTHORITY: crate::MQLONG = 4;
pub const MQZID_SET_AUTHORITY: crate::MQLONG = 5;
pub const MQZID_GET_AUTHORITY: crate::MQLONG = 6;
pub const MQZID_GET_EXPLICIT_AUTHORITY: crate::MQLONG = 7;
pub const MQZID_REFRESH_CACHE: crate::MQLONG = 8;
pub const MQZID_ENUMERATE_AUTHORITY_DATA: crate::MQLONG = 9;
pub const MQZID_AUTHENTICATE_USER: crate::MQLONG = 10;
pub const MQZID_FREE_USER: crate::MQLONG = 11;
pub const MQZID_INQUIRE: crate::MQLONG = 12;
pub const MQZID_CHECK_PRIVILEGED: crate::MQLONG = 13;
pub const MQZID_INIT_NAME: crate::MQLONG = 0;
pub const MQZID_TERM_NAME: crate::MQLONG = 1;
pub const MQZID_LOOKUP_NAME: crate::MQLONG = 2;
pub const MQZID_INSERT_NAME: crate::MQLONG = 3;
pub const MQZID_DELETE_NAME: crate::MQLONG = 4;
pub const MQZID_INIT_USERID: crate::MQLONG = 0;
pub const MQZID_TERM_USERID: crate::MQLONG = 1;
pub const MQZID_FIND_USERID: crate::MQLONG = 2;
pub const MQIEP_STRUC_ID: &::std::ffi::CStr = c"IEP ";
pub const MQIEP_VERSION_1: crate::MQLONG = 1;
pub const MQIEP_CURRENT_VERSION: crate::MQLONG = 1;
pub const MQIEP_LENGTH_1: usize = 264;
pub const MQIEP_CURRENT_LENGTH: usize = 264;
pub const MQIEPF_NONE: crate::MQLONG = 0;
pub const MQIEPF_NON_THREADED_LIBRARY: crate::MQLONG = 0;
pub const MQIEPF_THREADED_LIBRARY: crate::MQLONG = 1;
pub const MQIEPF_CLIENT_LIBRARY: crate::MQLONG = 0;
pub const MQIEPF_LOCAL_LIBRARY: crate::MQLONG = 2;
unsafe extern "C" {
    /// Register Entry Point
    ///
    /// # Arguments
    /// * `Hconfig`: Configuration handle
    /// * `ExitReason`: Exit reason
    /// * `Function`: Function identifier
    /// * `EntryPoint`: Exit function entry point
    /// * `ExitOpts`: Options that control the action of [`MQXEP`]
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `MQXEP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109660_.html)
    pub fn MQXEP(
        Hconfig: MQHCONFIG,
        ExitReason: crate::MQLONG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        ExitOpts: Option<&MQXEPO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    );
    /// Cluster Workload Navigate Records
    ///
    /// # Arguments
    /// * `ExitParms` (Input/Output): Exit parameter structure
    /// * `CurrentRecord`: Address of current record
    /// * `NextOffset`: Offset of next record
    /// * `NextRecord` (Output): Address of next record or structure
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `MQXCLWLN` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q082540_.html)
    pub fn MQXCLWLN(
        ExitParms: &mut MQWXP,
        CurrentRecord: crate::MQPTR,
        NextOffset: crate::MQLONG,
        NextRecord: &mut crate::MQPTR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    );
    /// Convert Message Data
    ///
    /// # Arguments
    /// * `DataConvExitParms` (Input/Output): Data-conversion exit parameter block
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `InBufferLength`: Length in bytes of `InBuffer`
    /// * `InBuffer`: Buffer containing the unconverted message
    /// * `OutBufferLength`: Length in bytes of `OutBuffer`
    /// * `OutBuffer` (Output): Buffer containing the converted message
    pub fn MQXDX(
        DataConvExitParms: &mut MQDXP,
        MsgDesc: crate::PMQMD,
        InBufferLength: crate::MQLONG,
        InBuffer: crate::PMQVOID,
        OutBufferLength: crate::MQLONG,
        OutBuffer: crate::PMQVOID,
    );
    /// Add Component Entry Point
    ///
    /// # Arguments
    /// * `Hconfig`: Configuration handle
    /// * `Function`: Function identifier
    /// * `EntryPoint`: Function entry point
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `MQZEP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110350_.html)
    pub fn MQZEP(
        Hconfig: MQHCONFIG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    );
}
