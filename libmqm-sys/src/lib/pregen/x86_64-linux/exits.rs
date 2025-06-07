/* Generated with MQ client version 9.4.2.1 */

pub type PPMQCHAR = *mut PMQCHAR;
pub type PPMQLONG = *mut PMQLONG;
/// Interface Entry Points
pub type MQIEP = tagMQIEP;
pub type PMQIEP = *mut MQIEP;
pub type PPMQIEP = *mut PMQIEP;
pub type MQHCONFIG = PMQIEP;
pub type PMQHCONFIG = *mut MQHCONFIG;
pub type PPMQHCONN = *mut PMQHCONN;
pub type PPMQHOBJ = *mut PMQHOBJ;
pub type PPMQVOID = *mut PMQVOID;
pub type PPMQBO = *mut PMQBO;
pub type PPMQCBC = *mut PMQCBC;
pub type PPMQCBD = *mut PMQCBD;
pub type PPMQCTLO = *mut PMQCTLO;
pub type PPMQCNO = *mut PMQCNO;
pub type PPMQGMO = *mut PMQGMO;
pub type PPMQMD = *mut PMQMD;
pub type PPMQOD = *mut PMQOD;
pub type PPMQPMO = *mut PMQPMO;
pub type PPMQSD = *mut PMQSD;
pub type PPMQSRO = *mut PMQSRO;
pub type PPMQSTS = *mut PMQSTS;
pub type MQ_BACK_CALL = ::std::option::Option<
    unsafe extern "C" fn(Hconn: MQHCONN, pCompCode: PMQLONG, pReason: PMQLONG),
>;
pub type PMQ_BACK_CALL = MQ_BACK_CALL;
pub type MQ_BEGIN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pBeginOptions: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_BEGIN_CALL = MQ_BEGIN_CALL;
pub type MQ_BUFMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pBufMsgHOpts: PMQVOID,
        pMsgDesc: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_BUFMH_CALL = MQ_BUFMH_CALL;
pub type MQ_CB_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Operation: MQLONG,
        pCallbackDesc: PMQVOID,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CB_CALL = MQ_CB_CALL;
pub type MQ_CLOSE_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pHobj: PMQHOBJ,
        Options: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CLOSE_CALL = MQ_CLOSE_CALL;
pub type MQ_CMIT_CALL = ::std::option::Option<
    unsafe extern "C" fn(Hconn: MQHCONN, pCompCode: PMQLONG, pReason: PMQLONG),
>;
pub type PMQ_CMIT_CALL = MQ_CMIT_CALL;
pub type MQ_CONN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CONN_CALL = MQ_CONN_CALL;
pub type MQ_CONNX_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pConnectOpts: PMQCNO,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CONNX_CALL = MQ_CONNX_CALL;
pub type MQ_CRTMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pCrtMsgHOpts: PMQVOID,
        pHmsg: PMQHMSG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CRTMH_CALL = MQ_CRTMH_CALL;
pub type MQ_CTL_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Operation: MQLONG,
        pControlOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CTL_CALL = MQ_CTL_CALL;
pub type MQ_DISC_CALL = ::std::option::Option<
    unsafe extern "C" fn(pHconn: PMQHCONN, pCompCode: PMQLONG, pReason: PMQLONG),
>;
pub type PMQ_DISC_CALL = MQ_DISC_CALL;
pub type MQ_DLTMH_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pHmsg: PMQHMSG,
        pDltMsgHOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_DLTMH_CALL = MQ_DLTMH_CALL;
pub type MQ_DLTMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pDltPropOpts: PMQVOID,
        pName: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_DLTMP_CALL = MQ_DLTMP_CALL;
pub type MQ_GET_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_GET_CALL = MQ_GET_CALL;
pub type MQ_INQ_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_INQ_CALL = MQ_INQ_CALL;
pub type MQ_INQMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pInqPropOpts: PMQVOID,
        pName: PMQVOID,
        pPropDesc: PMQVOID,
        pType: PMQLONG,
        ValueLength: MQLONG,
        pValue: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_INQMP_CALL = MQ_INQMP_CALL;
pub type MQ_MHBUF_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pMsgHBufOpts: PMQVOID,
        pName: PMQVOID,
        pMsgDesc: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_MHBUF_CALL = MQ_MHBUF_CALL;
pub type MQ_OPEN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pObjDesc: PMQVOID,
        Options: MQLONG,
        pHobj: PMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_OPEN_CALL = MQ_OPEN_CALL;
pub type MQ_PUT_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PUT_CALL = MQ_PUT_CALL;
pub type MQ_PUT1_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pObjDesc: PMQVOID,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: PMQVOID,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PUT1_CALL = MQ_PUT1_CALL;
pub type MQ_SET_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SET_CALL = MQ_SET_CALL;
pub type MQ_SETMP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hmsg: MQHMSG,
        pSetPropOpts: PMQVOID,
        pName: PMQVOID,
        pPropDesc: PMQVOID,
        Type: MQLONG,
        ValueLength: MQLONG,
        pValue: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SETMP_CALL = MQ_SETMP_CALL;
pub type MQ_STAT_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Type: MQLONG,
        pStatus: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_STAT_CALL = MQ_STAT_CALL;
pub type MQ_SUB_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        pSubDesc: PMQVOID,
        pHobj: PMQHOBJ,
        pHsub: PMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SUB_CALL = MQ_SUB_CALL;
pub type MQ_SUBRQ_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Hsub: MQHOBJ,
        Action: MQLONG,
        pSubRqOpts: PMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SUBRQ_CALL = MQ_SUBRQ_CALL;
pub type MQACH = tagMQACH;
pub type PMQACH = *mut MQACH;
/// API Exit Context
pub type MQAXC = tagMQAXC;
pub type PMQAXC = *mut MQAXC;
/// API Exit Parameter
pub type MQAXP = tagMQAXP;
pub type PMQAXP = *mut MQAXP;
/// Channel Exit Parameter
pub type MQCXP = tagMQCXP;
pub type PMQCXP = *mut MQCXP;
/// Data Conversion Exit Parameter
pub type MQDXP = tagMQDXP;
pub type PMQDXP = *mut MQDXP;
/// PreConnect Exit options
pub type MQNXP = tagMQNXP;
pub type PMQNXP = *mut MQNXP;
/// Publish Exit Publication Context
pub type MQPBC = tagMQPBC;
pub type PMQPBC = *mut MQPBC;
/// Publish Exit Parameter
pub type MQPSXP = tagMQPSXP;
pub type PMQPSXP = *mut MQPSXP;
/// Publish Exit Subscription Context
pub type MQSBC = tagMQSBC;
pub type PMQSBC = *mut MQSBC;
/// Cluster Workload Exit Cluster Record
pub type MQWCR = tagMQWCR;
pub type PMQWCR = *mut MQWCR;
/// Cluster Workload Exit Destination Record
pub type MQWDR = tagMQWDR;
pub type PMQWDR = *mut MQWDR;
pub type PPMQWDR = *mut PMQWDR;
/// Version-1 CLWL Exit Destination Record
pub type MQWDR1 = tagMQWDR1;
pub type PMQWDR1 = *mut MQWDR1;
/// Version-2 CLWL Exit Destination Record
pub type MQWDR2 = tagMQWDR2;
pub type PMQWDR2 = *mut MQWDR2;
/// Cluster Workload Exit Queue Record
pub type MQWQR = tagMQWQR;
pub type PMQWQR = *mut MQWQR;
pub type PPMQWQR = *mut PMQWQR;
/// Version-1 CLWL Exit Queue Record
pub type MQWQR1 = tagMQWQR1;
pub type PMQWQR1 = *mut MQWQR1;
/// Version-2 CLWL Exit Queue Record
pub type MQWQR2 = tagMQWQR2;
pub type PMQWQR2 = *mut MQWQR2;
/// Version-3 CLWL Exit Queue Record
pub type MQWQR3 = tagMQWQR3;
pub type PMQWQR3 = *mut MQWQR3;
/// Version-4 CLWL Exit Queue Record
pub type MQWQR4 = tagMQWQR4;
pub type PMQWQR4 = *mut MQWQR4;
/// Cluster Workload Exit Parameter
pub type MQWXP = tagMQWXP;
pub type PMQWXP = *mut MQWXP;
/// Version-1 CLWL Exit Parameter
pub type MQWXP1 = tagMQWXP1;
pub type PMQWXP1 = *mut MQWXP1;
/// Version-2 CLWL Exit Parameter
pub type MQWXP2 = tagMQWXP2;
pub type PMQWXP2 = *mut MQWXP2;
/// Version-3 CLWL Exit Parameter
pub type MQWXP3 = tagMQWXP3;
pub type PMQWXP3 = *mut MQWXP3;
/// Version-4 CLWL Exit Parameter
pub type MQWXP4 = tagMQWXP4;
pub type PMQWXP4 = *mut MQWXP4;
/// Register entry point options
pub type MQXEPO = tagMQXEPO;
pub type PMQXEPO = *mut MQXEPO;
pub type MQ_XEP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        ExitReason: MQLONG,
        Function: MQLONG,
        pEntryPoint: PMQFUNC,
        pExitOpts: PMQXEPO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_XEP_CALL = MQ_XEP_CALL;
/// Back Out Changes Exit
pub type MQ_BACK_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_BACK_EXIT = MQ_BACK_EXIT;
/// Begin Unit of Work Exit
pub type MQ_BEGIN_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppBeginOptions: PPMQBO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_BEGIN_EXIT = MQ_BEGIN_EXIT;
/// Callback Function Exit
pub type MQ_CALLBACK_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppMsgDesc: PPMQMD,
        ppGetMsgOpts: PPMQGMO,
        ppBuffer: PPMQVOID,
        ppMQCBContext: PPMQCBC,
    ),
>;
pub type PMQ_CALLBACK_EXIT = MQ_CALLBACK_EXIT;
/// Register Callback Exit
pub type MQ_CB_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pOperation: PMQLONG,
        ppCallbackDesc: PPMQCBD,
        pHobj: PMQHOBJ,
        ppMsgDesc: PPMQMD,
        ppGetMsgOpts: PPMQGMO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CB_EXIT = MQ_CB_EXIT;
/// Close Object Exit
pub type MQ_CLOSE_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppHobj: PPMQHOBJ,
        pOptions: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CLOSE_EXIT = MQ_CLOSE_EXIT;
/// Commit Changes Exit
pub type MQ_CMIT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CMIT_EXIT = MQ_CMIT_EXIT;
/// Connect Queue Manager Exit
pub type MQ_CONNX_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pQMgrName: PMQCHAR,
        ppConnectOpts: PPMQCNO,
        ppHconn: PPMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CONNX_EXIT = MQ_CONNX_EXIT;
/// Control Asynchronous Operations Exit
pub type MQ_CTL_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pOperation: PMQLONG,
        ppCtlOpts: PPMQCTLO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_CTL_EXIT = MQ_CTL_EXIT;
/// Disconnect Queue Manager Exit
pub type MQ_DISC_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        ppHconn: PPMQHCONN,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_DISC_EXIT = MQ_DISC_EXIT;
/// Get Message Exit
pub type MQ_GET_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHobj: PMQHOBJ,
        ppMsgDesc: PPMQMD,
        ppGetMsgOpts: PPMQGMO,
        pBufferLength: PMQLONG,
        ppBuffer: PPMQVOID,
        ppDataLength: PPMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_GET_EXIT = MQ_GET_EXIT;
/// Initialization Exit
pub type MQ_INIT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_INIT_EXIT = MQ_INIT_EXIT;
/// Inquire Object Attributes Exit
pub type MQ_INQ_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHobj: PMQHOBJ,
        pSelectorCount: PMQLONG,
        ppSelectors: PPMQLONG,
        pIntAttrCount: PMQLONG,
        ppIntAttrs: PPMQLONG,
        pCharAttrLength: PMQLONG,
        ppCharAttrs: PPMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_INQ_EXIT = MQ_INQ_EXIT;
/// Open Object Exit
pub type MQ_OPEN_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppObjDesc: PPMQOD,
        pOptions: PMQLONG,
        ppHobj: PPMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_OPEN_EXIT = MQ_OPEN_EXIT;
/// Put Message Exit
pub type MQ_PUT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHobj: PMQHOBJ,
        ppMsgDesc: PPMQMD,
        ppPutMsgOpts: PPMQPMO,
        pBufferLength: PMQLONG,
        ppBuffer: PPMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PUT_EXIT = MQ_PUT_EXIT;
/// Put One Message Exit
pub type MQ_PUT1_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppObjDesc: PPMQOD,
        ppMsgDesc: PPMQMD,
        ppPutMsgOpts: PPMQPMO,
        pBufferLength: PMQLONG,
        ppBuffer: PPMQVOID,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PUT1_EXIT = MQ_PUT1_EXIT;
/// Set Object Attributes Exit
pub type MQ_SET_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHobj: PMQHOBJ,
        pSelectorCount: PMQLONG,
        ppSelectors: PPMQLONG,
        pIntAttrCount: PMQLONG,
        ppIntAttrs: PPMQLONG,
        pCharAttrLength: PMQLONG,
        ppCharAttrs: PPMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SET_EXIT = MQ_SET_EXIT;
/// Get Status Exit
pub type MQ_STAT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pType: PMQLONG,
        ppStatus: PPMQSTS,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_STAT_EXIT = MQ_STAT_EXIT;
/// Subscribe Exit
pub type MQ_SUBRQ_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        pHsub: PMQHOBJ,
        pAction: PMQLONG,
        ppSubRqOpts: PPMQSRO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SUBRQ_EXIT = MQ_SUBRQ_EXIT;
/// Subscribe Exit
pub type MQ_SUB_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pHconn: PMQHCONN,
        ppSubDesc: PPMQSD,
        ppHobj: PPMQHOBJ,
        ppHsub: PPMQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_SUB_EXIT = MQ_SUB_EXIT;
/// Termination Exit
pub type MQ_TERM_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQAXP,
        pExitContext: PMQAXC,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_TERM_EXIT = MQ_TERM_EXIT;
/// Channel Exit
pub type MQ_CHANNEL_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pChannelExitParms: PMQVOID,
        pChannelDefinition: PMQVOID,
        pDataLength: PMQLONG,
        pAgentBufferLength: PMQLONG,
        pAgentBuffer: PMQVOID,
        pExitBufferLength: PMQLONG,
        pExitBufferAddr: PMQPTR,
    ),
>;
pub type PMQ_CHANNEL_EXIT = MQ_CHANNEL_EXIT;
/// Channel Auto Definition Exit
pub type MQ_CHANNEL_AUTO_DEF_EXIT = ::std::option::Option<
    unsafe extern "C" fn(pChannelExitParms: PMQVOID, pChannelDefinition: PMQVOID),
>;
pub type PMQ_CHANNEL_AUTO_DEF_EXIT = MQ_CHANNEL_AUTO_DEF_EXIT;
/// Cluster Workload Exit
pub type MQ_CLUSTER_WORKLOAD_EXIT = ::std::option::Option<
    unsafe extern "C" fn(pExitParms: PMQWXP),
>;
pub type PMQ_CLUSTER_WORKLOAD_EXIT = MQ_CLUSTER_WORKLOAD_EXIT;
/// Data Conversion Exit
pub type MQ_DATA_CONV_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pDataConvExitParms: PMQDXP,
        pMsgDesc: PMQMD,
        InBufferLength: MQLONG,
        pInBuffer: PMQVOID,
        OutBufferLength: MQLONG,
        pOutBuffer: PMQVOID,
    ),
>;
pub type PMQ_DATA_CONV_EXIT = MQ_DATA_CONV_EXIT;
/// Publish Exit
pub type MQ_PUBLISH_EXIT = ::std::option::Option<
    unsafe extern "C" fn(pExitParms: PMQPSXP, pPubContext: PMQPBC, pSubContext: PMQSBC),
>;
pub type PMQ_PUBLISH_EXIT = MQ_PUBLISH_EXIT;
/// Transport Retry Exit
pub type MQ_TRANSPORT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQVOID,
        DestAddressLength: MQLONG,
        pDestAddress: PMQCHAR,
    ),
>;
pub type PMQ_TRANSPORT_EXIT = MQ_TRANSPORT_EXIT;
/// Preconnect Exit
pub type MQ_PRECONNECT_EXIT = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQNXP,
        pQMgrName: PMQCHAR,
        ppConnectOpts: PPMQCNO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_PRECONNECT_EXIT = MQ_PRECONNECT_EXIT;
pub type MQ_XCLWLN_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        pExitParms: PMQWXP,
        CurrentRecord: MQPTR,
        NextOffset: MQLONG,
        pNextRecord: PMQPTR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_XCLWLN_CALL = MQ_XCLWLN_CALL;
pub type MQ_XCNVC_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconn: MQHCONN,
        Options: MQLONG,
        SourceCCSID: MQLONG,
        SourceLength: MQLONG,
        pSourceBuffer: PMQCHAR,
        TargetCCSID: MQLONG,
        TargetLength: MQLONG,
        pTargetBuffer: PMQCHAR,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_XCNVC_CALL = MQ_XCNVC_CALL;
pub type MQ_XDX_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        pDataConvExitParms: PMQDXP,
        pMsgDesc: PMQMD,
        InBufferLength: MQLONG,
        pInBuffer: PMQVOID,
        OutBufferLength: MQLONG,
        pOutBuffer: PMQVOID,
    ),
>;
pub type PMQ_XDX_CALL = MQ_XDX_CALL;
/// Entity Data
pub type MQZED = tagMQZED;
pub type PMQZED = *mut MQZED;
/// Application Context
pub type MQZAC = tagMQZAC;
pub type PMQZAC = *mut MQZAC;
/// Authority Data
pub type MQZAD = tagMQZAD;
pub type PMQZAD = *mut MQZAD;
/// Free Parameters
pub type MQZFP = tagMQZFP;
pub type PMQZFP = *mut MQZFP;
/// Identity Context
pub type MQZIC = tagMQZIC;
pub type PMQZIC = *mut MQZIC;
pub type MQ_ZEP_CALL = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Function: MQLONG,
        pEntryPoint: PMQFUNC,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQ_ZEP_CALL = MQ_ZEP_CALL;
/// Initialize Authority-Services
pub type MQZ_INIT_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: MQLONG,
        pQMgrName: PMQCHAR,
        ComponentDataLength: MQLONG,
        pComponentData: PMQBYTE,
        pVersion: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_INIT_AUTHORITY = MQZ_INIT_AUTHORITY;
/// Terminate Authority-Services
pub type MQZ_TERM_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: MQLONG,
        pQMgrName: PMQCHAR,
        pComponentData: PMQBYTE,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_TERM_AUTHORITY = MQZ_TERM_AUTHORITY;
/// Delete Authority
pub type MQZ_DELETE_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_DELETE_AUTHORITY = MQZ_DELETE_AUTHORITY;
/// Get Authority
pub type MQZ_GET_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityName: PMQCHAR,
        EntityType: MQLONG,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        pAuthority: PMQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_GET_AUTHORITY = MQZ_GET_AUTHORITY;
/// Get Authority Version 2
pub type MQZ_GET_AUTHORITY_2 = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityData: PMQZED,
        EntityType: MQLONG,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        pAuthority: PMQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_GET_AUTHORITY_2 = MQZ_GET_AUTHORITY_2;
/// Get Explicit Authority
pub type MQZ_GET_EXPLICIT_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityName: PMQCHAR,
        EntityType: MQLONG,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        pAuthority: PMQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_GET_EXPLICIT_AUTHORITY = MQZ_GET_EXPLICIT_AUTHORITY;
/// Get Explicit Authority Version 2
pub type MQZ_GET_EXPLICIT_AUTHORITY_2 = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityData: PMQZED,
        EntityType: MQLONG,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        pAuthority: PMQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_GET_EXPLICIT_AUTHORITY_2 = MQZ_GET_EXPLICIT_AUTHORITY_2;
/// Enumerate Authority Data
pub type MQZ_ENUMERATE_AUTHORITY_DATA = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        StartEnumeration: MQLONG,
        pFilter: PMQZAD,
        AuthorityBufferLength: MQLONG,
        pAuthorityBuffer: PMQZAD,
        pAuthorityDataLength: PMQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_ENUMERATE_AUTHORITY_DATA = MQZ_ENUMERATE_AUTHORITY_DATA;
/// Set Authority
pub type MQZ_SET_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityName: PMQCHAR,
        EntityType: MQLONG,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        Authority: MQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_SET_AUTHORITY = MQZ_SET_AUTHORITY;
/// Set Authority Version 2
pub type MQZ_SET_AUTHORITY_2 = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityData: PMQZED,
        EntityType: MQLONG,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        Authority: MQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_SET_AUTHORITY_2 = MQZ_SET_AUTHORITY_2;
/// Copy All Authority
pub type MQZ_COPY_ALL_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pRefObjectName: PMQCHAR,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_COPY_ALL_AUTHORITY = MQZ_COPY_ALL_AUTHORITY;
/// Check Authority
pub type MQZ_CHECK_AUTHORITY = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityName: PMQCHAR,
        EntityType: MQLONG,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        Authority: MQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_CHECK_AUTHORITY = MQZ_CHECK_AUTHORITY;
/// Check Authority Version 2
pub type MQZ_CHECK_AUTHORITY_2 = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityData: PMQZED,
        EntityType: MQLONG,
        pObjectName: PMQCHAR,
        ObjectType: MQLONG,
        Authority: MQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_CHECK_AUTHORITY_2 = MQZ_CHECK_AUTHORITY_2;
/// Authenticate User
pub type MQZ_AUTHENTICATE_USER = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pSecurityParms: PMQCSP,
        pApplicationContext: PMQZAC,
        pIdentityContext: PMQZIC,
        pCorrelationPtr: PMQPTR,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_AUTHENTICATE_USER = MQZ_AUTHENTICATE_USER;
/// Free User
pub type MQZ_FREE_USER = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pFreeParms: PMQZFP,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_FREE_USER = MQZ_FREE_USER;
/// Inquire
pub type MQZ_INQUIRE = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        SelectorCount: MQLONG,
        pSelectors: PMQLONG,
        IntAttrCount: MQLONG,
        pIntAttrs: PMQLONG,
        CharAttrLength: MQLONG,
        pCharAttrs: PMQCHAR,
        pSelectorReturned: PMQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_INQUIRE = MQZ_INQUIRE;
/// Refresh Cache
pub type MQZ_REFRESH_CACHE = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_REFRESH_CACHE = MQZ_REFRESH_CACHE;
/// Check if User is Privileged
pub type MQZ_CHECK_PRIVILEGED = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pEntityData: PMQZED,
        EntityType: MQLONG,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_CHECK_PRIVILEGED = MQZ_CHECK_PRIVILEGED;
/// Initialize Name-Services
pub type MQZ_INIT_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: MQLONG,
        pQMgrName: PMQCHAR,
        ComponentDataLength: MQLONG,
        pComponentData: PMQBYTE,
        pVersion: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_INIT_NAME = MQZ_INIT_NAME;
/// Terminate Name-Services
pub type MQZ_TERM_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: MQLONG,
        pQMgrName: PMQCHAR,
        pComponentData: PMQBYTE,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_TERM_NAME = MQZ_TERM_NAME;
/// Look-Up Name
pub type MQZ_LOOKUP_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pQName: PMQCHAR,
        pResolvedQMgrName: PMQCHAR,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_LOOKUP_NAME = MQZ_LOOKUP_NAME;
/// Insert Name
pub type MQZ_INSERT_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pQName: PMQCHAR,
        pResolvedQMgrName: PMQCHAR,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_INSERT_NAME = MQZ_INSERT_NAME;
/// Delete Name
pub type MQZ_DELETE_NAME = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pQName: PMQCHAR,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_DELETE_NAME = MQZ_DELETE_NAME;
/// Initialize Userid-Services
pub type MQZ_INIT_USERID = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: MQLONG,
        pQMgrName: PMQCHAR,
        ComponentDataLength: MQLONG,
        pComponentData: PMQBYTE,
        pVersion: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_INIT_USERID = MQZ_INIT_USERID;
/// Terminate Userid-Services
pub type MQZ_TERM_USERID = ::std::option::Option<
    unsafe extern "C" fn(
        Hconfig: MQHCONFIG,
        Options: MQLONG,
        pQMgrName: PMQCHAR,
        pComponentData: PMQBYTE,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_TERM_USERID = MQZ_TERM_USERID;
/// Find Userid
pub type MQZ_FIND_USERID = ::std::option::Option<
    unsafe extern "C" fn(
        pQMgrName: PMQCHAR,
        pUserId: PMQCHAR,
        pPassword: PMQCHAR,
        pComponentData: PMQBYTE,
        pContinuation: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    ),
>;
pub type PMQZ_FIND_USERID = MQZ_FIND_USERID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQACH {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQACH structure
    pub StrucLength: MQLONG,
    /// Total length of chain area
    pub ChainAreaLength: MQLONG,
    /// Exit information name
    pub ExitInfoName: MQCHAR48,
    /// Address of next MQACH structure in chain
    pub NextChainAreaPtr: PMQACH,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQAXC {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Environment
    pub Environment: MQLONG,
    /// User identifier
    pub UserId: MQCHAR12,
    /// Security identifier
    pub SecurityId: MQBYTE40,
    /// Connection name
    pub ConnectionName: [MQCHAR; 264usize],
    /// Length of long MCA user identifier
    pub LongMCAUserIdLength: MQLONG,
    /// Length of long remote user identifier
    pub LongRemoteUserIdLength: MQLONG,
    /// Address of long MCA user identifier
    pub LongMCAUserIdPtr: MQPTR,
    /// Address of long remote user identifier
    pub LongRemoteUserIdPtr: MQPTR,
    /// Application name
    pub ApplName: MQCHAR28,
    /// Application type
    pub ApplType: MQLONG,
    /// Process identifier
    pub ProcessId: MQPID,
    /// Thread identifier
    pub ThreadId: MQTID,
    /// Channel Name
    pub ChannelName: [MQCHAR; 20usize],
    /// Reserved
    pub Reserved1: MQBYTE4,
    /// Pointer to Channel Definition
    pub pChannelDefinition: PMQCD,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQAXP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: MQLONG,
    /// Feedback
    pub Feedback: MQLONG,
    /// API caller type
    pub APICallerType: MQLONG,
    /// Exit user area
    pub ExitUserArea: MQBYTE16,
    /// Exit data
    pub ExitData: MQCHAR32,
    /// Exit information name
    pub ExitInfoName: MQCHAR48,
    /// Problem determination area
    pub ExitPDArea: MQBYTE48,
    /// Name of local queue manager
    pub QMgrName: MQCHAR48,
    /// Address of first MQACH structure in chain
    pub ExitChainAreaPtr: PMQACH,
    /// Configuration handle
    pub Hconfig: MQHCONFIG,
    /// API function identifier
    pub Function: MQLONG,
    /// Exit message handle
    pub ExitMsgHandle: MQHMSG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCXP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: MQLONG,
    /// Feedback code
    pub Feedback: MQLONG,
    /// Maximum segment length
    pub MaxSegmentLength: MQLONG,
    /// Exit user area
    pub ExitUserArea: MQBYTE16,
    /// Exit data
    pub ExitData: MQCHAR32,
    /// Number of times the message has been retried
    pub MsgRetryCount: MQLONG,
    /// Minimum interval in milliseconds after which the put operation should be retried
    pub MsgRetryInterval: MQLONG,
    /// Reason code from previous attempt to put the message
    pub MsgRetryReason: MQLONG,
    /// Length of header information
    pub HeaderLength: MQLONG,
    /// Partner Name
    pub PartnerName: MQCHAR48,
    /// Negotiated Formats and Protocols level
    pub FAPLevel: MQLONG,
    /// Capability flags
    pub CapabilityFlags: MQLONG,
    /// Exit number
    pub ExitNumber: MQLONG,
    /// Number of bytes in transmission buffer reserved for exit to use
    pub ExitSpace: MQLONG,
    /// User identifier associated with remote SSL certificate
    pub SSLCertUserid: MQCHAR12,
    /// Length of distinguished name of issuer of remote SSL certificate
    pub SSLRemCertIssNameLength: MQLONG,
    /// Address of distinguished name of issuer of remote SSL certificate
    pub SSLRemCertIssNamePtr: MQPTR,
    /// Address of security parameters
    pub SecurityParms: PMQCSP,
    /// Header data compression used for current message
    pub CurHdrCompression: MQLONG,
    /// Message data compression used for current message
    pub CurMsgCompression: MQLONG,
    /// Connection handle
    pub Hconn: MQHCONN,
    /// Multiple conversations allowed
    pub SharingConversations: MQBOOL,
    /// The source of the run-time user ID
    pub MCAUserSource: MQLONG,
    /// Interface entry points
    pub pEntryPoints: PMQIEP,
    /// The identifier for the remote product
    pub RemoteProduct: MQCHAR4,
    /// The version of the remote product
    pub RemoteVersion: MQCHAR8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQDXP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Reserved
    pub ExitOptions: MQLONG,
    /// Application options
    pub AppOptions: MQLONG,
    /// Numeric encoding required by application
    pub Encoding: MQLONG,
    /// Character set required by application
    pub CodedCharSetId: MQLONG,
    /// Length in bytes of message data
    pub DataLength: MQLONG,
    /// Completion code
    pub CompCode: MQLONG,
    /// Reason code qualifying CompCode
    pub Reason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Connection handle
    pub Hconn: MQHCONN,
    /// Interface entry points
    pub pEntryPoints: PMQIEP,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQNXP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: MQLONG,
    /// Feedback
    pub Feedback: MQLONG,
    /// Length of exit data
    pub ExitDataLength: MQLONG,
    /// Address of exit data
    pub pExitDataPtr: PMQCHAR,
    /// Address of exit user area
    pub pExitUserAreaPtr: MQPTR,
    /// Address of pointers referencing MQCDs
    pub ppMQCDArrayPtr: PPMQCD,
    /// Count of MQCDs referenced
    pub MQCDArrayCount: MQLONG,
    /// Maximum MQCD version requested
    pub MaxMQCDVersion: MQLONG,
    /// Interface entry points
    pub pEntryPoints: PMQIEP,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQPBC {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Publish topic string
    pub PubTopicString: MQCHARV,
    /// Address of publisher message descriptor
    pub MsgDescPtr: PMQMD,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQPSXP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Reserved
    pub ExitResponse2: MQLONG,
    /// Feedback code
    pub Feedback: MQLONG,
    /// Connection handle
    pub Hconn: MQHCONN,
    /// Exit user area
    pub ExitUserArea: MQBYTE16,
    /// Exit data
    pub ExitData: MQCHAR32,
    /// Name of local queue manager
    pub QMgrName: MQCHAR48,
    /// Handle to message properties
    pub MsgHandle: MQHMSG,
    /// Address of message descriptor
    pub MsgDescPtr: PMQMD,
    /// Address of input message data
    pub MsgInPtr: PMQVOID,
    /// Length of input message data
    pub MsgInLength: MQLONG,
    /// Address of output message data
    pub MsgOutPtr: PMQVOID,
    /// Length of output message data
    pub MsgOutLength: MQLONG,
    /// Interface entry points
    pub pEntryPoints: PMQIEP,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQSBC {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Destination queue manager
    pub DestinationQMgrName: MQCHAR48,
    /// Destination queue name
    pub DestinationQName: MQCHAR48,
    /// Type of subscription
    pub SubType: MQLONG,
    /// Subscription options
    pub SubOptions: MQLONG,
    /// Object name
    pub ObjectName: MQCHAR48,
    /// Object string
    pub ObjectString: MQCHARV,
    /// Subscription topic string
    pub SubTopicString: MQCHARV,
    /// Subscription name
    pub SubName: MQCHARV,
    /// Subscription identifier
    pub SubId: MQBYTE24,
    /// Subscription selection string
    pub SelectionString: MQCHARV,
    /// Subscription level
    pub SubLevel: MQLONG,
    /// Publish/subscribe properties
    pub PSProperties: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWCR {
    /// Cluster name
    pub ClusterName: MQCHAR48,
    /// Offset of next cluster record
    pub ClusterRecOffset: MQLONG,
    /// Cluster flags
    pub ClusterFlags: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWDR {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWDR structure
    pub StrucLength: MQLONG,
    /// Queue-manager flags
    pub QMgrFlags: MQLONG,
    /// Queue-manager identifier
    pub QMgrIdentifier: MQCHAR48,
    /// Queue-manager name
    pub QMgrName: MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: MQLONG,
    /// Channel state
    pub ChannelState: MQLONG,
    /// Offset of channel definition structure
    pub ChannelDefOffset: MQLONG,
    /// Cluster channel destination sequence number
    pub DestSeqNumber: MQLONG,
    /// Cluster channel destination sequence factor
    pub DestSeqFactor: MQINT64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWDR1 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWDR structure
    pub StrucLength: MQLONG,
    /// Queue-manager flags
    pub QMgrFlags: MQLONG,
    /// Queue-manager identifier
    pub QMgrIdentifier: MQCHAR48,
    /// Queue-manager name
    pub QMgrName: MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: MQLONG,
    /// Channel state
    pub ChannelState: MQLONG,
    /// Offset of channel definition structure
    pub ChannelDefOffset: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWDR2 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWDR structure
    pub StrucLength: MQLONG,
    /// Queue-manager flags
    pub QMgrFlags: MQLONG,
    /// Queue-manager identifier
    pub QMgrIdentifier: MQCHAR48,
    /// Queue-manager name
    pub QMgrName: MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: MQLONG,
    /// Channel state
    pub ChannelState: MQLONG,
    /// Offset of channel definition structure
    pub ChannelDefOffset: MQLONG,
    /// Cluster channel destination sequence number
    pub DestSeqNumber: MQLONG,
    /// Cluster channel destination sequence factor
    pub DestSeqFactor: MQINT64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWQR {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWQR structure
    pub StrucLength: MQLONG,
    /// Queue flags
    pub QFlags: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: MQLONG,
    /// Queue type
    pub QType: MQLONG,
    /// Queue description
    pub QDesc: MQCHAR64,
    /// Default binding
    pub DefBind: MQLONG,
    /// Default message persistence
    pub DefPersistence: MQLONG,
    /// Default message priority
    pub DefPriority: MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: MQLONG,
    /// Queue priority
    pub CLWLQueuePriority: MQLONG,
    /// Queue rank
    pub CLWLQueueRank: MQLONG,
    /// Default put response
    pub DefPutResponse: MQLONG,
    /// CapExpiry
    pub CapExpiry: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWQR1 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWQR structure
    pub StrucLength: MQLONG,
    /// Queue flags
    pub QFlags: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: MQLONG,
    /// Queue type
    pub QType: MQLONG,
    /// Queue description
    pub QDesc: MQCHAR64,
    /// Default binding
    pub DefBind: MQLONG,
    /// Default message persistence
    pub DefPersistence: MQLONG,
    /// Default message priority
    pub DefPriority: MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWQR2 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWQR structure
    pub StrucLength: MQLONG,
    /// Queue flags
    pub QFlags: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: MQLONG,
    /// Queue type
    pub QType: MQLONG,
    /// Queue description
    pub QDesc: MQCHAR64,
    /// Default binding
    pub DefBind: MQLONG,
    /// Default message persistence
    pub DefPersistence: MQLONG,
    /// Default message priority
    pub DefPriority: MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: MQLONG,
    /// Queue priority
    pub CLWLQueuePriority: MQLONG,
    /// Queue rank
    pub CLWLQueueRank: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWQR3 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWQR structure
    pub StrucLength: MQLONG,
    /// Queue flags
    pub QFlags: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: MQLONG,
    /// Queue type
    pub QType: MQLONG,
    /// Queue description
    pub QDesc: MQCHAR64,
    /// Default binding
    pub DefBind: MQLONG,
    /// Default message persistence
    pub DefPersistence: MQLONG,
    /// Default message priority
    pub DefPriority: MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: MQLONG,
    /// Queue priority
    pub CLWLQueuePriority: MQLONG,
    /// Queue rank
    pub CLWLQueueRank: MQLONG,
    /// Default put response
    pub DefPutResponse: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWQR4 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQWQR structure
    pub StrucLength: MQLONG,
    /// Queue flags
    pub QFlags: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Queue-manager identifier
    pub QMgrIdentifier: MQCHAR48,
    /// Offset of first cluster record
    pub ClusterRecOffset: MQLONG,
    /// Queue type
    pub QType: MQLONG,
    /// Queue description
    pub QDesc: MQCHAR64,
    /// Default binding
    pub DefBind: MQLONG,
    /// Default message persistence
    pub DefPersistence: MQLONG,
    /// Default message priority
    pub DefPriority: MQLONG,
    /// Whether put operations on the queue are allowed
    pub InhibitPut: MQLONG,
    /// Queue priority
    pub CLWLQueuePriority: MQLONG,
    /// Queue rank
    pub CLWLQueueRank: MQLONG,
    /// Default put response
    pub DefPutResponse: MQLONG,
    /// CapExpiry
    pub CapExpiry: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWXP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: MQLONG,
    /// Reserved
    pub Feedback: MQLONG,
    /// Flags
    pub Flags: MQLONG,
    /// Exit user area
    pub ExitUserArea: MQBYTE16,
    /// Exit data
    pub ExitData: MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: MQLONG,
    /// Length of complete message
    pub MsgLength: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: MQLONG,
    /// Destination chosen
    pub DestinationChosen: MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
    /// Context information
    pub CacheContext: MQPTR,
    /// Type of cluster cache
    pub CacheType: MQLONG,
    /// Number of allowed active outbound channels
    pub CLWLMRUChannels: MQLONG,
    /// Interface entry points
    pub pEntryPoints: PMQIEP,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWXP1 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: MQLONG,
    /// Reserved
    pub Feedback: MQLONG,
    /// Flags
    pub Flags: MQLONG,
    /// Exit user area
    pub ExitUserArea: MQBYTE16,
    /// Exit data
    pub ExitData: MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: MQLONG,
    /// Length of complete message
    pub MsgLength: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: MQLONG,
    /// Destination chosen
    pub DestinationChosen: MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWXP2 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: MQLONG,
    /// Reserved
    pub Feedback: MQLONG,
    /// Flags
    pub Flags: MQLONG,
    /// Exit user area
    pub ExitUserArea: MQBYTE16,
    /// Exit data
    pub ExitData: MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: MQLONG,
    /// Length of complete message
    pub MsgLength: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: MQLONG,
    /// Destination chosen
    pub DestinationChosen: MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
    /// Context information
    pub CacheContext: MQPTR,
    /// Type of cluster cache
    pub CacheType: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWXP3 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: MQLONG,
    /// Reserved
    pub Feedback: MQLONG,
    /// Flags
    pub Flags: MQLONG,
    /// Exit user area
    pub ExitUserArea: MQBYTE16,
    /// Exit data
    pub ExitData: MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: MQLONG,
    /// Length of complete message
    pub MsgLength: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: MQLONG,
    /// Destination chosen
    pub DestinationChosen: MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
    /// Context information
    pub CacheContext: MQPTR,
    /// Type of cluster cache
    pub CacheType: MQLONG,
    /// Number of allowed active outbound channels
    pub CLWLMRUChannels: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQWXP4 {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Type of exit
    pub ExitId: MQLONG,
    /// Reason for invoking exit
    pub ExitReason: MQLONG,
    /// Response from exit
    pub ExitResponse: MQLONG,
    /// Secondary response from exit
    pub ExitResponse2: MQLONG,
    /// Reserved
    pub Feedback: MQLONG,
    /// Flags
    pub Flags: MQLONG,
    /// Exit user area
    pub ExitUserArea: MQBYTE16,
    /// Exit data
    pub ExitData: MQCHAR32,
    /// Address of message descriptor
    pub MsgDescPtr: PMQMD,
    /// Address of buffer containing some or all of the message data
    pub MsgBufferPtr: PMQVOID,
    /// Length of buffer containing message data
    pub MsgBufferLength: MQLONG,
    /// Length of complete message
    pub MsgLength: MQLONG,
    /// Queue name
    pub QName: MQCHAR48,
    /// Name of local queue manager
    pub QMgrName: MQCHAR48,
    /// Number of possible destinations
    pub DestinationCount: MQLONG,
    /// Destination chosen
    pub DestinationChosen: MQLONG,
    /// Address of an array of pointers to destination records
    pub DestinationArrayPtr: PPMQWDR,
    /// Address of an array of pointers to queue records
    pub QArrayPtr: PPMQWQR,
    /// Context information
    pub CacheContext: MQPTR,
    /// Type of cluster cache
    pub CacheType: MQLONG,
    /// Number of allowed active outbound channels
    pub CLWLMRUChannels: MQLONG,
    /// Interface entry points
    pub pEntryPoints: PMQIEP,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQXEPO {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Options that control the action of MQXEP
    pub Options: MQLONG,
    /// Exit properties
    pub ExitProperties: MQCHARV,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQZED {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Address of entity name
    pub EntityNamePtr: PMQCHAR,
    /// Address of entity domain name
    pub EntityDomainPtr: PMQCHAR,
    /// Security identifier
    pub SecurityId: MQBYTE40,
    /// Address of correlational data
    pub CorrelationPtr: MQPTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQZAC {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Process identifier of application
    pub ProcessId: MQPID,
    /// Thread identifier of application
    pub ThreadId: MQTID,
    /// Application name
    pub ApplName: MQCHAR28,
    /// User ID of application
    pub UserID: MQCHAR12,
    /// Effective user ID of application
    pub EffectiveUserID: MQCHAR12,
    /// Environment of caller
    pub Environment: MQLONG,
    /// Type of caller
    pub CallerType: MQLONG,
    /// Type of authentication being performed
    pub AuthenticationType: MQLONG,
    /// Type of bindings in use
    pub BindType: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQZAD {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Profile name
    pub ProfileName: MQCHAR48,
    /// Object type
    pub ObjectType: MQLONG,
    /// Authority
    pub Authority: MQLONG,
    /// Address of MQZED structure identifying an entity
    pub EntityDataPtr: PMQZED,
    /// Entity type
    pub EntityType: MQLONG,
    /// Options
    pub Options: MQLONG,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQZFP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Reserved
    pub Reserved: MQBYTE8,
    /// Address of correlational data
    pub CorrelationPtr: MQPTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQZIC {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// User identifier
    pub UserIdentifier: MQCHAR12,
    /// Accounting token
    pub AccountingToken: MQBYTE32,
    /// Application data relating to identity
    pub ApplIdentityData: MQCHAR32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQIEP {
    /// Structure identifier
    pub StrucId: MQCHAR4,
    /// Structure version number
    pub Version: MQLONG,
    /// Length of MQIEP structure
    pub StrucLength: MQLONG,
    /// Flags containing information about the interface entry points
    pub Flags: MQLONG,
    /// Reserved
    pub Reserved: MQPTR,
    /// MQBACK entry point
    pub MQBACK_Call: PMQ_BACK_CALL,
    /// MQBEGIN entry point
    pub MQBEGIN_Call: PMQ_BEGIN_CALL,
    /// MQBUFMH entry point
    pub MQBUFMH_Call: PMQ_BUFMH_CALL,
    /// MQCB entry point
    pub MQCB_Call: PMQ_CB_CALL,
    /// MQCLOSE entry point
    pub MQCLOSE_Call: PMQ_CLOSE_CALL,
    /// MQCMIT entry point
    pub MQCMIT_Call: PMQ_CMIT_CALL,
    /// MQCONN entry point
    pub MQCONN_Call: PMQ_CONN_CALL,
    /// MQCONNX entry point
    pub MQCONNX_Call: PMQ_CONNX_CALL,
    /// MQCRTMH entry point
    pub MQCRTMH_Call: PMQ_CRTMH_CALL,
    /// MQCTL entry point
    pub MQCTL_Call: PMQ_CTL_CALL,
    /// MQDISC entry point
    pub MQDISC_Call: PMQ_DISC_CALL,
    /// MQDLTMH entry point
    pub MQDLTMH_Call: PMQ_DLTMH_CALL,
    /// MQDLTMP entry point
    pub MQDLTMP_Call: PMQ_DLTMP_CALL,
    /// MQGET entry point
    pub MQGET_Call: PMQ_GET_CALL,
    /// MQINQ entry point
    pub MQINQ_Call: PMQ_INQ_CALL,
    /// MQINQMP entry point
    pub MQINQMP_Call: PMQ_INQMP_CALL,
    /// MQMHBUF entry point
    pub MQMHBUF_Call: PMQ_MHBUF_CALL,
    /// MQOPEN entry point
    pub MQOPEN_Call: PMQ_OPEN_CALL,
    /// MQPUT entry point
    pub MQPUT_Call: PMQ_PUT_CALL,
    /// MQPUT1 entry point
    pub MQPUT1_Call: PMQ_PUT1_CALL,
    /// MQSET entry point
    pub MQSET_Call: PMQ_SET_CALL,
    /// MQSETMP entry point
    pub MQSETMP_Call: PMQ_SETMP_CALL,
    /// MQSTAT entry point
    pub MQSTAT_Call: PMQ_STAT_CALL,
    /// MQSUB entry point
    pub MQSUB_Call: PMQ_SUB_CALL,
    /// MQSUBRQ entry point
    pub MQSUBRQ_Call: PMQ_SUBRQ_CALL,
    /// MQXCLWLN entry point
    pub MQXCLWLN_Call: PMQ_XCLWLN_CALL,
    /// MQXCNVC entry point
    pub MQXCNVC_Call: PMQ_XCNVC_CALL,
    /// MQXDX entry point
    pub MQXDX_Call: PMQ_XDX_CALL,
    /// MQXEP entry point
    pub MQXEP_Call: PMQ_XEP_CALL,
    /// MQZEP entry point
    pub MQZEP_Call: PMQ_ZEP_CALL,
}
pub const MQPA_DEFAULT: MQLONG = 1;
pub const MQPA_CONTEXT: MQLONG = 2;
pub const MQPA_ONLY_MCA: MQLONG = 3;
pub const MQPA_ALTERNATE_OR_MCA: MQLONG = 4;
pub const MQCDC_SENDER_CONVERSION: MQLONG = 1;
pub const MQCDC_NO_SENDER_CONVERSION: MQLONG = 0;
pub const MQMCAT_PROCESS: MQLONG = 1;
pub const MQMCAT_THREAD: MQLONG = 2;
pub const MQNPMS_NORMAL: MQLONG = 1;
pub const MQNPMS_FAST: MQLONG = 2;
pub const MQSCA_REQUIRED: MQLONG = 0;
pub const MQSCA_OPTIONAL: MQLONG = 1;
pub const MQSCA_NEVER_REQUIRED: MQLONG = 2;
pub const MQKAI_AUTO: MQLONG = -1;
pub const MQRCN_NO: MQLONG = 0;
pub const MQRCN_YES: MQLONG = 1;
pub const MQRCN_Q_MGR: MQLONG = 2;
pub const MQRCN_DISABLED: MQLONG = 3;
pub const MQPROTO_MQTTV3: MQLONG = 1;
pub const MQPROTO_HTTP: MQLONG = 2;
pub const MQPROTO_AMQP: MQLONG = 3;
pub const MQPROTO_MQTTV311: MQLONG = 4;
pub const MQSECPROT_NONE: MQLONG = 0;
pub const MQSECPROT_SSLV30: MQLONG = 1;
pub const MQSECPROT_TLSV10: MQLONG = 2;
pub const MQSECPROT_TLSV12: MQLONG = 4;
pub const MQSECPROT_TLSV13: MQLONG = 8;
pub const MQSPL_PASSTHRU: MQLONG = 0;
pub const MQSPL_REMOVE: MQLONG = 1;
pub const MQSPL_AS_POLICY: MQLONG = 2;
pub const MQACH_STRUC_ID: &::std::ffi::CStr = c"ACH ";
pub const MQACH_VERSION_1: MQLONG = 1;
pub const MQACH_CURRENT_VERSION: MQLONG = 1;
pub const MQACH_LENGTH_1: usize = 72;
pub const MQACH_CURRENT_LENGTH: usize = 72;
pub const MQAXC_STRUC_ID: &::std::ffi::CStr = c"AXC ";
pub const MQAXC_VERSION_1: MQLONG = 1;
pub const MQAXC_VERSION_2: MQLONG = 2;
pub const MQAXC_CURRENT_VERSION: MQLONG = 2;
pub const MQAXC_LENGTH_1: usize = 392;
pub const MQAXC_LENGTH_2: usize = 424;
pub const MQAXC_CURRENT_LENGTH: usize = 424;
pub const MQXE_OTHER: MQLONG = 0;
pub const MQXE_MCA: MQLONG = 1;
pub const MQXE_MCA_SVRCONN: MQLONG = 2;
pub const MQXE_COMMAND_SERVER: MQLONG = 3;
pub const MQXE_MQSC: MQLONG = 4;
pub const MQXE_MCA_CLNTCONN: MQLONG = 5;
pub const MQAXP_STRUC_ID: &::std::ffi::CStr = c"AXP ";
pub const MQAXP_VERSION_1: MQLONG = 1;
pub const MQAXP_VERSION_2: MQLONG = 2;
pub const MQAXP_CURRENT_VERSION: MQLONG = 2;
pub const MQAXP_LENGTH_1: usize = 256;
pub const MQAXP_CURRENT_LENGTH: usize = 256;
pub const MQXACT_EXTERNAL: MQLONG = 1;
pub const MQXACT_INTERNAL: MQLONG = 2;
pub const MQXPDA_NONE: &[u8; 49] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQXF_INIT: MQLONG = 1;
pub const MQXF_TERM: MQLONG = 2;
pub const MQXF_CONN: MQLONG = 3;
pub const MQXF_CONNX: MQLONG = 4;
pub const MQXF_DISC: MQLONG = 5;
pub const MQXF_OPEN: MQLONG = 6;
pub const MQXF_CLOSE: MQLONG = 7;
pub const MQXF_PUT1: MQLONG = 8;
pub const MQXF_PUT: MQLONG = 9;
pub const MQXF_GET: MQLONG = 10;
pub const MQXF_DATA_CONV_ON_GET: MQLONG = 11;
pub const MQXF_INQ: MQLONG = 12;
pub const MQXF_SET: MQLONG = 13;
pub const MQXF_BEGIN: MQLONG = 14;
pub const MQXF_CMIT: MQLONG = 15;
pub const MQXF_BACK: MQLONG = 16;
pub const MQXF_STAT: MQLONG = 18;
pub const MQXF_CB: MQLONG = 19;
pub const MQXF_CTL: MQLONG = 20;
pub const MQXF_CALLBACK: MQLONG = 21;
pub const MQXF_SUB: MQLONG = 22;
pub const MQXF_SUBRQ: MQLONG = 23;
pub const MQXF_XACLOSE: MQLONG = 24;
pub const MQXF_XACOMMIT: MQLONG = 25;
pub const MQXF_XACOMPLETE: MQLONG = 26;
pub const MQXF_XAEND: MQLONG = 27;
pub const MQXF_XAFORGET: MQLONG = 28;
pub const MQXF_XAOPEN: MQLONG = 29;
pub const MQXF_XAPREPARE: MQLONG = 30;
pub const MQXF_XARECOVER: MQLONG = 31;
pub const MQXF_XAROLLBACK: MQLONG = 32;
pub const MQXF_XASTART: MQLONG = 33;
pub const MQXF_AXREG: MQLONG = 34;
pub const MQXF_AXUNREG: MQLONG = 35;
pub const MQCXP_STRUC_ID: &::std::ffi::CStr = c"CXP ";
pub const MQCXP_VERSION_1: MQLONG = 1;
pub const MQCXP_VERSION_2: MQLONG = 2;
pub const MQCXP_VERSION_3: MQLONG = 3;
pub const MQCXP_VERSION_4: MQLONG = 4;
pub const MQCXP_VERSION_5: MQLONG = 5;
pub const MQCXP_VERSION_6: MQLONG = 6;
pub const MQCXP_VERSION_7: MQLONG = 7;
pub const MQCXP_VERSION_8: MQLONG = 8;
pub const MQCXP_VERSION_9: MQLONG = 9;
pub const MQCXP_CURRENT_VERSION: MQLONG = 9;
pub const MQCXP_LENGTH_3: usize = 156;
pub const MQCXP_LENGTH_4: usize = 156;
pub const MQCXP_LENGTH_5: usize = 160;
pub const MQCXP_LENGTH_6: usize = 200;
pub const MQCXP_LENGTH_7: usize = 208;
pub const MQCXP_LENGTH_8: usize = 224;
pub const MQCXP_LENGTH_9: usize = 240;
pub const MQCXP_CURRENT_LENGTH: usize = 240;
pub const MQXR2_PUT_WITH_DEF_ACTION: MQLONG = 0;
pub const MQXR2_PUT_WITH_DEF_USERID: MQLONG = 1;
pub const MQXR2_PUT_WITH_MSG_USERID: MQLONG = 2;
pub const MQXR2_USE_AGENT_BUFFER: MQLONG = 0;
pub const MQXR2_USE_EXIT_BUFFER: MQLONG = 4;
pub const MQXR2_DEFAULT_CONTINUATION: MQLONG = 0;
pub const MQXR2_CONTINUE_CHAIN: MQLONG = 8;
pub const MQXR2_SUPPRESS_CHAIN: MQLONG = 16;
pub const MQXR2_STATIC_CACHE: MQLONG = 0;
pub const MQXR2_DYNAMIC_CACHE: MQLONG = 32;
pub const MQCF_NONE: MQLONG = 0;
pub const MQCF_DIST_LISTS: MQLONG = 1;
pub const MQDXP_STRUC_ID: &::std::ffi::CStr = c"DXP ";
pub const MQDXP_VERSION_1: MQLONG = 1;
pub const MQDXP_VERSION_2: MQLONG = 2;
pub const MQDXP_CURRENT_VERSION: MQLONG = 2;
pub const MQDXP_LENGTH_1: usize = 44;
pub const MQDXP_LENGTH_2: usize = 56;
pub const MQDXP_CURRENT_LENGTH: usize = 56;
pub const MQXDR_OK: MQLONG = 0;
pub const MQXDR_CONVERSION_FAILED: MQLONG = 1;
pub const MQNXP_STRUC_ID: &::std::ffi::CStr = c"NXP ";
pub const MQNXP_VERSION_1: MQLONG = 1;
pub const MQNXP_VERSION_2: MQLONG = 2;
pub const MQNXP_CURRENT_VERSION: MQLONG = 2;
pub const MQNXP_LENGTH_1: usize = 64;
pub const MQNXP_LENGTH_2: usize = 72;
pub const MQNXP_CURRENT_LENGTH: usize = 72;
pub const MQPBC_STRUC_ID: &::std::ffi::CStr = c"PBC ";
pub const MQPBC_VERSION_1: MQLONG = 1;
pub const MQPBC_VERSION_2: MQLONG = 2;
pub const MQPBC_CURRENT_VERSION: MQLONG = 2;
pub const MQPBC_LENGTH_1: usize = 32;
pub const MQPBC_LENGTH_2: usize = 40;
pub const MQPBC_CURRENT_LENGTH: usize = 40;
pub const MQPSXP_STRUC_ID: &::std::ffi::CStr = c"PSXP";
pub const MQPSXP_VERSION_1: MQLONG = 1;
pub const MQPSXP_VERSION_2: MQLONG = 2;
pub const MQPSXP_CURRENT_VERSION: MQLONG = 2;
pub const MQPSXP_LENGTH_1: usize = 176;
pub const MQPSXP_LENGTH_2: usize = 184;
pub const MQPSXP_CURRENT_LENGTH: usize = 184;
pub const MQSBC_STRUC_ID: &::std::ffi::CStr = c"SBC ";
pub const MQSBC_VERSION_1: MQLONG = 1;
pub const MQSBC_CURRENT_VERSION: MQLONG = 1;
pub const MQSBC_LENGTH_1: usize = 288;
pub const MQSBC_CURRENT_LENGTH: usize = 288;
pub const MQWDR_STRUC_ID: &::std::ffi::CStr = c"WDR ";
pub const MQWDR_VERSION_1: MQLONG = 1;
pub const MQWDR_VERSION_2: MQLONG = 2;
pub const MQWDR_CURRENT_VERSION: MQLONG = 2;
pub const MQWDR_LENGTH_1: usize = 124;
pub const MQWDR_LENGTH_2: usize = 136;
pub const MQWDR_CURRENT_LENGTH: usize = 136;
pub const MQQMF_REPOSITORY_Q_MGR: MQLONG = 2;
pub const MQQMF_CLUSSDR_USER_DEFINED: MQLONG = 8;
pub const MQQMF_CLUSSDR_AUTO_DEFINED: MQLONG = 16;
pub const MQQMF_AVAILABLE: MQLONG = 32;
pub const MQWDR1_LENGTH_1: usize = 124;
pub const MQWDR1_CURRENT_LENGTH: usize = 124;
pub const MQWDR2_LENGTH_1: usize = 124;
pub const MQWDR2_LENGTH_2: usize = 136;
pub const MQWDR2_CURRENT_LENGTH: usize = 136;
pub const MQWQR_STRUC_ID: &::std::ffi::CStr = c"WQR ";
pub const MQWQR_VERSION_1: MQLONG = 1;
pub const MQWQR_VERSION_2: MQLONG = 2;
pub const MQWQR_VERSION_3: MQLONG = 3;
pub const MQWQR_VERSION_4: MQLONG = 4;
pub const MQWQR_CURRENT_VERSION: MQLONG = 4;
pub const MQWQR_LENGTH_1: usize = 200;
pub const MQWQR_LENGTH_2: usize = 208;
pub const MQWQR_LENGTH_3: usize = 212;
pub const MQWQR_LENGTH_4: usize = 216;
pub const MQWQR_CURRENT_LENGTH: usize = 216;
pub const MQQF_LOCAL_Q: MQLONG = 1;
pub const MQQF_CLWL_USEQ_ANY: MQLONG = 64;
pub const MQQF_CLWL_USEQ_LOCAL: MQLONG = 128;
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
pub const MQWXP_VERSION_1: MQLONG = 1;
pub const MQWXP_VERSION_2: MQLONG = 2;
pub const MQWXP_VERSION_3: MQLONG = 3;
pub const MQWXP_VERSION_4: MQLONG = 4;
pub const MQWXP_CURRENT_VERSION: MQLONG = 4;
pub const MQWXP_LENGTH_1: usize = 224;
pub const MQWXP_LENGTH_2: usize = 240;
pub const MQWXP_LENGTH_3: usize = 240;
pub const MQWXP_LENGTH_4: usize = 248;
pub const MQWXP_CURRENT_LENGTH: usize = 248;
pub const MQWXP_PUT_BY_CLUSTER_CHL: MQLONG = 2;
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
pub const MQXEPO_VERSION_1: MQLONG = 1;
pub const MQXEPO_CURRENT_VERSION: MQLONG = 1;
pub const MQXEPO_LENGTH_1: usize = 40;
pub const MQXEPO_CURRENT_LENGTH: usize = 40;
pub const MQXEPO_NONE: MQLONG = 0;
pub const MQXT_API_CROSSING_EXIT: MQLONG = 1;
pub const MQXT_API_EXIT: MQLONG = 2;
pub const MQXT_CHANNEL_SEC_EXIT: MQLONG = 11;
pub const MQXT_CHANNEL_MSG_EXIT: MQLONG = 12;
pub const MQXT_CHANNEL_SEND_EXIT: MQLONG = 13;
pub const MQXT_CHANNEL_RCV_EXIT: MQLONG = 14;
pub const MQXT_CHANNEL_MSG_RETRY_EXIT: MQLONG = 15;
pub const MQXT_CHANNEL_AUTO_DEF_EXIT: MQLONG = 16;
pub const MQXT_CLUSTER_WORKLOAD_EXIT: MQLONG = 20;
pub const MQXT_PUBSUB_ROUTING_EXIT: MQLONG = 21;
pub const MQXT_PUBLISH_EXIT: MQLONG = 22;
pub const MQXT_PRECONNECT_EXIT: MQLONG = 23;
pub const MQXR_BEFORE: MQLONG = 1;
pub const MQXR_AFTER: MQLONG = 2;
pub const MQXR_CONNECTION: MQLONG = 3;
pub const MQXR_BEFORE_CONVERT: MQLONG = 4;
pub const MQXR_INIT: MQLONG = 11;
pub const MQXR_TERM: MQLONG = 12;
pub const MQXR_MSG: MQLONG = 13;
pub const MQXR_XMIT: MQLONG = 14;
pub const MQXR_SEC_MSG: MQLONG = 15;
pub const MQXR_INIT_SEC: MQLONG = 16;
pub const MQXR_RETRY: MQLONG = 17;
pub const MQXR_AUTO_CLUSSDR: MQLONG = 18;
pub const MQXR_AUTO_RECEIVER: MQLONG = 19;
pub const MQXR_CLWL_OPEN: MQLONG = 20;
pub const MQXR_CLWL_PUT: MQLONG = 21;
pub const MQXR_CLWL_MOVE: MQLONG = 22;
pub const MQXR_CLWL_REPOS: MQLONG = 23;
pub const MQXR_CLWL_REPOS_MOVE: MQLONG = 24;
pub const MQXR_END_BATCH: MQLONG = 25;
pub const MQXR_ACK_RECEIVED: MQLONG = 26;
pub const MQXR_AUTO_SVRCONN: MQLONG = 27;
pub const MQXR_AUTO_CLUSRCVR: MQLONG = 28;
pub const MQXR_SEC_PARMS: MQLONG = 29;
pub const MQXR_PUBLICATION: MQLONG = 30;
pub const MQXR_PRECONNECT: MQLONG = 31;
pub const MQXCC_OK: MQLONG = 0;
pub const MQXCC_SUPPRESS_FUNCTION: MQLONG = -1;
pub const MQXCC_SKIP_FUNCTION: MQLONG = -2;
pub const MQXCC_SEND_AND_REQUEST_SEC_MSG: MQLONG = -3;
pub const MQXCC_SEND_SEC_MSG: MQLONG = -4;
pub const MQXCC_SUPPRESS_EXIT: MQLONG = -5;
pub const MQXCC_CLOSE_CHANNEL: MQLONG = -6;
pub const MQXCC_REQUEST_ACK: MQLONG = -7;
pub const MQXCC_FAILED: MQLONG = -8;
pub const MQXUA_NONE: &[u8; 17] = b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const MQCLCT_STATIC: MQLONG = 0;
pub const MQCLCT_DYNAMIC: MQLONG = 1;
pub const MQMCEV_PACKET_LOSS: MQLONG = 1;
pub const MQMCEV_HEARTBEAT_TIMEOUT: MQLONG = 2;
pub const MQMCEV_VERSION_CONFLICT: MQLONG = 3;
pub const MQMCEV_RELIABILITY: MQLONG = 4;
pub const MQMCEV_CLOSED_TRANS: MQLONG = 5;
pub const MQMCEV_STREAM_ERROR: MQLONG = 6;
pub const MQMCEV_NEW_SOURCE: MQLONG = 10;
pub const MQMCEV_RECEIVE_QUEUE_TRIMMED: MQLONG = 11;
pub const MQMCEV_PACKET_LOSS_NACK_EXPIRE: MQLONG = 12;
pub const MQMCEV_ACK_RETRIES_EXCEEDED: MQLONG = 13;
pub const MQMCEV_STREAM_SUSPEND_NACK: MQLONG = 14;
pub const MQMCEV_STREAM_RESUME_NACK: MQLONG = 15;
pub const MQMCEV_STREAM_EXPELLED: MQLONG = 16;
pub const MQMCEV_FIRST_MESSAGE: MQLONG = 20;
pub const MQMCEV_LATE_JOIN_FAILURE: MQLONG = 21;
pub const MQMCEV_MESSAGE_LOSS: MQLONG = 22;
pub const MQMCEV_SEND_PACKET_FAILURE: MQLONG = 23;
pub const MQMCEV_REPAIR_DELAY: MQLONG = 24;
pub const MQMCEV_MEMORY_ALERT_ON: MQLONG = 25;
pub const MQMCEV_MEMORY_ALERT_OFF: MQLONG = 26;
pub const MQMCEV_NACK_ALERT_ON: MQLONG = 27;
pub const MQMCEV_NACK_ALERT_OFF: MQLONG = 28;
pub const MQMCEV_REPAIR_ALERT_ON: MQLONG = 29;
pub const MQMCEV_REPAIR_ALERT_OFF: MQLONG = 30;
pub const MQMCEV_RELIABILITY_CHANGED: MQLONG = 31;
pub const MQMCEV_SHM_DEST_UNUSABLE: MQLONG = 80;
pub const MQMCEV_SHM_PORT_UNUSABLE: MQLONG = 81;
pub const MQMCEV_CCT_GETTIME_FAILED: MQLONG = 110;
pub const MQMCEV_DEST_INTERFACE_FAILURE: MQLONG = 120;
pub const MQMCEV_DEST_INTERFACE_FAILOVER: MQLONG = 121;
pub const MQMCEV_PORT_INTERFACE_FAILURE: MQLONG = 122;
pub const MQMCEV_PORT_INTERFACE_FAILOVER: MQLONG = 123;
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
pub const MQZED_STRUC_ID: &::std::ffi::CStr = c"ZED ";
pub const MQZED_VERSION_1: MQLONG = 1;
pub const MQZED_VERSION_2: MQLONG = 2;
pub const MQZED_CURRENT_VERSION: MQLONG = 2;
pub const MQZED_LENGTH_1: usize = 64;
pub const MQZED_LENGTH_2: usize = 72;
pub const MQZED_CURRENT_LENGTH: usize = 72;
pub const MQZAC_STRUC_ID: &::std::ffi::CStr = c"ZAC ";
pub const MQZAC_VERSION_1: MQLONG = 1;
pub const MQZAC_CURRENT_VERSION: MQLONG = 1;
pub const MQZAC_LENGTH_1: usize = 84;
pub const MQZAC_CURRENT_LENGTH: usize = 84;
pub const MQZAT_INITIAL_CONTEXT: MQLONG = 0;
pub const MQZAT_CHANGE_CONTEXT: MQLONG = 1;
pub const MQZAD_STRUC_ID: &::std::ffi::CStr = c"ZAD ";
pub const MQZAD_VERSION_1: MQLONG = 1;
pub const MQZAD_VERSION_2: MQLONG = 2;
pub const MQZAD_CURRENT_VERSION: MQLONG = 2;
pub const MQZAD_LENGTH_1: usize = 80;
pub const MQZAD_LENGTH_2: usize = 80;
pub const MQZAD_CURRENT_LENGTH: usize = 80;
pub const MQZFP_STRUC_ID: &::std::ffi::CStr = c"ZFP ";
pub const MQZFP_VERSION_1: MQLONG = 1;
pub const MQZFP_CURRENT_VERSION: MQLONG = 1;
pub const MQZFP_LENGTH_1: usize = 24;
pub const MQZFP_CURRENT_LENGTH: usize = 24;
pub const MQZIC_STRUC_ID: &::std::ffi::CStr = c"ZIC ";
pub const MQZIC_VERSION_1: MQLONG = 1;
pub const MQZIC_CURRENT_VERSION: MQLONG = 1;
pub const MQZIC_LENGTH_1: usize = 84;
pub const MQZIC_CURRENT_LENGTH: usize = 84;
pub const MQZIO_PRIMARY: MQLONG = 0;
pub const MQZIO_SECONDARY: MQLONG = 1;
pub const MQZTO_PRIMARY: MQLONG = 0;
pub const MQZTO_SECONDARY: MQLONG = 1;
pub const MQZCI_DEFAULT: MQLONG = 0;
pub const MQZCI_CONTINUE: MQLONG = 0;
pub const MQZCI_STOP: MQLONG = 1;
pub const MQZAS_VERSION_1: MQLONG = 1;
pub const MQZAS_VERSION_2: MQLONG = 2;
pub const MQZAS_VERSION_3: MQLONG = 3;
pub const MQZAS_VERSION_4: MQLONG = 4;
pub const MQZAS_VERSION_5: MQLONG = 5;
pub const MQZAS_VERSION_6: MQLONG = 6;
pub const MQZAO_CONNECT: MQLONG = 1;
pub const MQZAO_BROWSE: MQLONG = 2;
pub const MQZAO_INPUT: MQLONG = 4;
pub const MQZAO_OUTPUT: MQLONG = 8;
pub const MQZAO_INQUIRE: MQLONG = 16;
pub const MQZAO_SET: MQLONG = 32;
pub const MQZAO_PASS_IDENTITY_CONTEXT: MQLONG = 64;
pub const MQZAO_PASS_ALL_CONTEXT: MQLONG = 128;
pub const MQZAO_SET_IDENTITY_CONTEXT: MQLONG = 256;
pub const MQZAO_SET_ALL_CONTEXT: MQLONG = 512;
pub const MQZAO_ALTERNATE_USER_AUTHORITY: MQLONG = 1024;
pub const MQZAO_PUBLISH: MQLONG = 2048;
pub const MQZAO_SUBSCRIBE: MQLONG = 4096;
pub const MQZAO_RESUME: MQLONG = 8192;
pub const MQZAO_ALL_MQI: MQLONG = 16383;
pub const MQZAO_CREATE: MQLONG = 65536;
pub const MQZAO_DELETE: MQLONG = 131072;
pub const MQZAO_DISPLAY: MQLONG = 262144;
pub const MQZAO_CHANGE: MQLONG = 524288;
pub const MQZAO_CLEAR: MQLONG = 1048576;
pub const MQZAO_CONTROL: MQLONG = 2097152;
pub const MQZAO_CONTROL_EXTENDED: MQLONG = 4194304;
pub const MQZAO_AUTHORIZE: MQLONG = 8388608;
pub const MQZAO_ALL_ADMIN: MQLONG = 16646144;
pub const MQZAO_SYSTEM: MQLONG = 33554432;
pub const MQZAO_ALL: MQLONG = 50216959;
pub const MQZAO_REMOVE: MQLONG = 16777216;
pub const MQZAO_NONE: MQLONG = 0;
pub const MQZAO_CREATE_ONLY: MQLONG = 67108864;
pub const MQZAET_NONE: MQLONG = 0;
pub const MQZAET_PRINCIPAL: MQLONG = 1;
pub const MQZAET_GROUP: MQLONG = 2;
pub const MQZAET_UNKNOWN: MQLONG = 3;
pub const MQZSE_START: MQLONG = 1;
pub const MQZSE_CONTINUE: MQLONG = 0;
pub const MQZSL_NOT_RETURNED: MQLONG = 0;
pub const MQZSL_RETURNED: MQLONG = 1;
pub const MQZNS_VERSION_1: MQLONG = 1;
pub const MQZUS_VERSION_1: MQLONG = 1;
pub const MQZID_INIT: MQLONG = 0;
pub const MQZID_TERM: MQLONG = 1;
pub const MQZID_INIT_AUTHORITY: MQLONG = 0;
pub const MQZID_TERM_AUTHORITY: MQLONG = 1;
pub const MQZID_CHECK_AUTHORITY: MQLONG = 2;
pub const MQZID_COPY_ALL_AUTHORITY: MQLONG = 3;
pub const MQZID_DELETE_AUTHORITY: MQLONG = 4;
pub const MQZID_SET_AUTHORITY: MQLONG = 5;
pub const MQZID_GET_AUTHORITY: MQLONG = 6;
pub const MQZID_GET_EXPLICIT_AUTHORITY: MQLONG = 7;
pub const MQZID_REFRESH_CACHE: MQLONG = 8;
pub const MQZID_ENUMERATE_AUTHORITY_DATA: MQLONG = 9;
pub const MQZID_AUTHENTICATE_USER: MQLONG = 10;
pub const MQZID_FREE_USER: MQLONG = 11;
pub const MQZID_INQUIRE: MQLONG = 12;
pub const MQZID_CHECK_PRIVILEGED: MQLONG = 13;
pub const MQZID_INIT_NAME: MQLONG = 0;
pub const MQZID_TERM_NAME: MQLONG = 1;
pub const MQZID_LOOKUP_NAME: MQLONG = 2;
pub const MQZID_INSERT_NAME: MQLONG = 3;
pub const MQZID_DELETE_NAME: MQLONG = 4;
pub const MQZID_INIT_USERID: MQLONG = 0;
pub const MQZID_TERM_USERID: MQLONG = 1;
pub const MQZID_FIND_USERID: MQLONG = 2;
pub const MQIEP_STRUC_ID: &::std::ffi::CStr = c"IEP ";
pub const MQIEP_VERSION_1: MQLONG = 1;
pub const MQIEP_CURRENT_VERSION: MQLONG = 1;
pub const MQIEP_LENGTH_1: usize = 264;
pub const MQIEP_CURRENT_LENGTH: usize = 264;
pub const MQIEPF_NONE: MQLONG = 0;
pub const MQIEPF_NON_THREADED_LIBRARY: MQLONG = 0;
pub const MQIEPF_THREADED_LIBRARY: MQLONG = 1;
pub const MQIEPF_CLIENT_LIBRARY: MQLONG = 0;
pub const MQIEPF_LOCAL_LIBRARY: MQLONG = 2;
unsafe extern "C" {
    /// Register Entry Point
    /// # Arguments
    /// * `Hconfig`: I: Configuration handle
    /// * `ExitReason`: I: Exit reason
    /// * `Function`: I: Function identifier
    /// * `pEntryPoint`: I: Exit function entry point
    /// * `pExitOpts`: I: Options that control the action of MQXEP
    /// * `pCompCode`: O: Completion code
    /// * `pReason`: O: Reason code qualifying CompCode
    pub fn MQXEP(
        Hconfig: MQHCONFIG,
        ExitReason: MQLONG,
        Function: MQLONG,
        pEntryPoint: PMQFUNC,
        pExitOpts: PMQXEPO,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Cluster Workload Navigate Records
    /// # Arguments
    /// * `pExitParms`: IO: Exit parameter structure
    /// * `CurrentRecord`: I: Address of current record
    /// * `NextOffset`: I: Offset of next record
    /// * `pNextRecord`: O: Address of next record or structure
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn MQXCLWLN(
        pExitParms: PMQWXP,
        CurrentRecord: MQPTR,
        NextOffset: MQLONG,
        pNextRecord: PMQPTR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Convert Characters
    /// # Arguments
    /// * `Hconn`: I: Connection handle
    /// * `Options`: I: Options that control the action of MQXCNVC
    /// * `SourceCCSID`: I: Coded character set identifier of string before conversion
    /// * `SourceLength`: IL: Length of string before conversion
    /// * `pSourceBuffer`: IB: String to be converted
    /// * `TargetCCSID`: I: Coded character set identifier of string after conversion
    /// * `TargetLength`: IL: Length of output buffer
    /// * `pTargetBuffer`: OB: String after conversion
    /// * `pDataLength`: O: Length of output string
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn MQXCNVC(
        Hconn: MQHCONN,
        Options: MQLONG,
        SourceCCSID: MQLONG,
        SourceLength: MQLONG,
        pSourceBuffer: PMQCHAR,
        TargetCCSID: MQLONG,
        TargetLength: MQLONG,
        pTargetBuffer: PMQCHAR,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Convert Message Data
    /// # Arguments
    /// * `pDataConvExitParms`: IO: Data-conversion exit parameter block
    /// * `pMsgDesc`: IO: Message descriptor
    /// * `InBufferLength`: IL: Length in bytes of InBuffer
    /// * `pInBuffer`: IB: Buffer containing the unconverted message
    /// * `OutBufferLength`: IL: Length in bytes of OutBuffer
    /// * `pOutBuffer`: OB: Buffer containing the converted message
    pub fn MQXDX(
        pDataConvExitParms: PMQDXP,
        pMsgDesc: PMQMD,
        InBufferLength: MQLONG,
        pInBuffer: PMQVOID,
        OutBufferLength: MQLONG,
        pOutBuffer: PMQVOID,
    );
    /// Add Component Entry Point
    /// # Arguments
    /// * `Hconfig`: I: Configuration handle
    /// * `Function`: I: Function identifier
    /// * `pEntryPoint`: I: Function entry point
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn MQZEP(
        Hconfig: MQHCONFIG,
        Function: MQLONG,
        pEntryPoint: PMQFUNC,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
