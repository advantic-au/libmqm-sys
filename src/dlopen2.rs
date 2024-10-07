#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use dlopen2::wrapper::{Container, WrapperApi};

use crate::{function, lib as mqsys};

/// Name of the platform dependent MQM dynamic library
pub const MQM_LIB: &str = if cfg!(windows) { "mqm.dll" } else { "libmqm_r.so" };

/// A [dlopen2] [Container] for the MQI library
pub type MqmContainer = Container<MqWrapper>;

pub trait LoadMqm {
    /// Loads the mqm library using the platform dependent search rules
    ///
    /// # Safety
    /// Loading the dynamic library is inherently unsafe
    ///
    /// # Errors
    /// Will return `Err` if the dynamic library could not be loaded
    unsafe fn load_mqm_default() -> Result<Self, dlopen2::Error>
    where
        Self: std::marker::Sized;
}

impl LoadMqm for MqmContainer {
    unsafe fn load_mqm_default() -> Result<Self, dlopen2::Error> {
        unsafe { Self::load(MQM_LIB) }
    }
}

/// A [`WrapperApi`] implementation for MQI and MQAI function calls
#[derive(WrapperApi, Debug)]
pub struct MqWrapper {
    MQCONNX: unsafe extern "C" fn(
        pQMgrName: mqsys::PMQCHAR,
        pConnectOpts: mqsys::PMQCNO,
        pHconn: mqsys::PMQHCONN,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQCONN: unsafe extern "C" fn(
        pQMgrName: mqsys::PMQCHAR,
        pHconn: mqsys::PMQHCONN,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQDISC: unsafe extern "C" fn(pHconn: mqsys::PMQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG),
    MQOPEN: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        pObjDesc: mqsys::PMQVOID,
        Options: mqsys::MQLONG,
        pHobj: mqsys::PMQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQPUT: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQPUT1: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        pObjDesc: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQCLOSE: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        pHobj: mqsys::PMQHOBJ,
        Options: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQCMIT: unsafe extern "C" fn(Hconn: mqsys::MQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG),
    MQGET: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQINQ: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        SelectorCount: mqsys::MQLONG,
        pSelectors: mqsys::PMQLONG,
        IntAttrCount: mqsys::MQLONG,
        pIntAttrs: mqsys::PMQLONG,
        CharAttrLength: mqsys::MQLONG,
        pCharAttrs: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQCRTMH: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        pCrtMsgHOpts: mqsys::PMQVOID,
        pHmsg: mqsys::PMQHMSG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQDLTMH: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        pHmsg: mqsys::PMQHMSG,
        pDltMsgHOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQINQMP: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pInqPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pPropDesc: mqsys::PMQVOID,
        pType: mqsys::PMQLONG,
        ValueLength: mqsys::MQLONG,
        pValue: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQSUB: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        pSubDesc: mqsys::PMQVOID,
        pHobj: mqsys::PMQHOBJ,
        pHsub: mqsys::PMQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQSUBRQ: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hsub: mqsys::MQHOBJ,
        Action: mqsys::MQLONG,
        pSubRqOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQBEGIN: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        pBeginOptions: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQBACK: unsafe extern "C" fn(Hconn: mqsys::MQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG),
    MQMHBUF: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pMsgHBufOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQBUFMH: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pBufMsgHOpts: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQCB: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Operation: mqsys::MQLONG,
        pCallbackDesc: mqsys::PMQVOID,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQCTL: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Operation: mqsys::MQLONG,
        pControlOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQSET: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        SelectorCount: mqsys::MQLONG,
        pSelectors: mqsys::PMQLONG,
        IntAttrCount: mqsys::MQLONG,
        pIntAttrs: mqsys::PMQLONG,
        CharAttrLength: mqsys::MQLONG,
        pCharAttrs: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQSETMP: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pSetPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pPropDesc: mqsys::PMQVOID,
        Type: mqsys::MQLONG,
        ValueLength: mqsys::MQLONG,
        pValue: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQSTAT: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Type: mqsys::MQLONG,
        pStatus: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQDLTMP: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pDltPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    MQXCNVC: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Options: mqsys::MQLONG,
        SourceCCSID: mqsys::MQLONG,
        SourceLength: mqsys::MQLONG,
        pSourceBuffer: mqsys::PMQCHAR,
        TargetCCSID: mqsys::MQLONG,
        TargetLength: mqsys::MQLONG,
        pTargetBuffer: mqsys::PMQCHAR,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqCreateBag:
        unsafe extern "C" fn(Options: mqsys::MQLONG, pBag: mqsys::PMQHBAG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG),
    #[cfg(feature = "mqai")]
    mqDeleteBag: unsafe extern "C" fn(pBag: mqsys::PMQHBAG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG),
    #[cfg(feature = "mqai")]
    mqClearBag: unsafe extern "C" fn(Bag: mqsys::MQHBAG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG),
    #[cfg(feature = "mqai")]
    mqGetBag: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        Bag: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqPutBag: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        Bag: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInquiry:
        unsafe extern "C" fn(Bag: mqsys::MQHBAG, Selector: mqsys::MQLONG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG),
    #[cfg(feature = "mqai")]
    mqDeleteItem: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInteger: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddIntegerFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetInteger: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireInteger: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddString: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqExecute: unsafe extern "C" fn(
        Hconn: mqsys::MQHCONN,
        Command: mqsys::MQLONG,
        OptionsBag: mqsys::MQHBAG,
        AdminBag: mqsys::MQHBAG,
        ResponseBag: mqsys::MQHBAG,
        AdminQ: mqsys::MQHOBJ,
        ResponseQ: mqsys::MQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqCountItems: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        pItemCount: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireBag: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireString: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pStringLength: mqsys::PMQLONG,
        pCodedCharSetId: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddBag: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInteger64: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddStringFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddByteString: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddByteStringFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetIntegerFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetString: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetStringFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetByteString: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetByteStringFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireIntegerFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireInteger64: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireByteString: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pByteStringLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireStringFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pStringLength: mqsys::PMQLONG,
        pCodedCharSetId: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireByteStringFilter: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pByteStringLength: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetInteger64: unsafe extern "C" fn(
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqTruncateBag:
        unsafe extern "C" fn(Bag: mqsys::MQHBAG, ItemCount: mqsys::MQLONG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG),
}

impl function::Mqi for MqmContainer {
    unsafe fn MQCONNX(
        &self,
        pQMgrName: mqsys::PMQCHAR,
        pConnectOpts: mqsys::PMQCNO,
        pHconn: mqsys::PMQHCONN,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQCONNX(self, pQMgrName, pConnectOpts, pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQCONN(
        &self,
        pQMgrName: mqsys::PMQCHAR,
        pHconn: mqsys::PMQHCONN,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQCONN(self, pQMgrName, pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQDISC(&self, pHconn: mqsys::PMQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG) {
        unsafe {
            MqWrapper::MQDISC(self, pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQOPEN(
        &self,
        Hconn: mqsys::MQHCONN,
        pObjDesc: mqsys::PMQVOID,
        Options: mqsys::MQLONG,
        pHobj: mqsys::PMQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQOPEN(self, Hconn, pObjDesc, Options, pHobj, pCompCode, pReason);
        }
    }

    unsafe fn MQPUT1(
        &self,
        Hconn: mqsys::MQHCONN,
        pObjDesc: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQPUT1(
                self,
                Hconn,
                pObjDesc,
                pMsgDesc,
                pPutMsgOpts,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn MQCMIT(&self, Hconn: mqsys::MQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG) {
        unsafe { MqWrapper::MQCMIT(self, Hconn, pCompCode, pReason) };
    }

    unsafe fn MQCLOSE(
        &self,
        Hconn: mqsys::MQHCONN,
        pHobj: mqsys::PMQHOBJ,
        Options: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe { MqWrapper::MQCLOSE(self, Hconn, pHobj, Options, pCompCode, pReason) };
    }

    unsafe fn MQGET(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQGET(
                self,
                Hconn,
                Hobj,
                pMsgDesc,
                pGetMsgOpts,
                BufferLength,
                pBuffer,
                pDataLength,
                pCompCode,
                pReason,
            );
        };
    }

    unsafe fn MQPUT(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQPUT(
                self,
                Hconn,
                Hobj,
                pMsgDesc,
                pPutMsgOpts,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            );
        };
    }

    unsafe fn MQINQ(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        SelectorCount: mqsys::MQLONG,
        pSelectors: mqsys::PMQLONG,
        IntAttrCount: mqsys::MQLONG,
        pIntAttrs: mqsys::PMQLONG,
        CharAttrLength: mqsys::MQLONG,
        pCharAttrs: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQINQ(
                self,
                Hconn,
                Hobj,
                SelectorCount,
                pSelectors,
                IntAttrCount,
                pIntAttrs,
                CharAttrLength,
                pCharAttrs,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn MQSUB(
        &self,
        Hconn: mqsys::MQHCONN,
        pSubDesc: mqsys::PMQVOID,
        pHobj: mqsys::PMQHOBJ,
        pHsub: mqsys::PMQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQSUB(self, Hconn, pSubDesc, pHobj, pHsub, pCompCode, pReason);
        }
    }

    unsafe fn MQSUBRQ(
        &self,
        Hconn: mqsys::MQHCONN,
        Hsub: mqsys::MQHOBJ,
        Action: mqsys::MQLONG,
        pSubRqOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQSUBRQ(self, Hconn, Hsub, Action, pSubRqOpts, pCompCode, pReason);
        }
    }

    unsafe fn MQBEGIN(
        &self,
        Hconn: mqsys::MQHCONN,
        pBeginOptions: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQBEGIN(self, Hconn, pBeginOptions, pCompCode, pReason);
        }
    }

    unsafe fn MQBACK(&self, Hconn: mqsys::MQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG) {
        unsafe {
            MqWrapper::MQBACK(self, Hconn, pCompCode, pReason);
        }
    }

    unsafe fn MQCRTMH(
        &self,
        Hconn: mqsys::MQHCONN,
        pCrtMsgHOpts: mqsys::PMQVOID,
        pHmsg: mqsys::PMQHMSG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQCRTMH(self, Hconn, pCrtMsgHOpts, pHmsg, pCompCode, pReason);
        }
    }

    unsafe fn MQDLTMH(
        &self,
        Hconn: mqsys::MQHCONN,
        pHmsg: mqsys::PMQHMSG,
        pDltMsgHOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQDLTMH(self, Hconn, pHmsg, pDltMsgHOpts, pCompCode, pReason);
        }
    }

    unsafe fn MQINQMP(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pInqPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pPropDesc: mqsys::PMQVOID,
        pType: mqsys::PMQLONG,
        ValueLength: mqsys::MQLONG,
        pValue: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQINQMP(
                self,
                Hconn,
                Hmsg,
                pInqPropOpts,
                pName,
                pPropDesc,
                pType,
                ValueLength,
                pValue,
                pDataLength,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn MQMHBUF(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pMsgHBufOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQMHBUF(
                self,
                Hconn,
                Hmsg,
                pMsgHBufOpts,
                pName,
                pMsgDesc,
                BufferLength,
                pBuffer,
                pDataLength,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn MQBUFMH(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pBufMsgHOpts: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQBUFMH(
                self,
                Hconn,
                Hmsg,
                pBufMsgHOpts,
                pMsgDesc,
                BufferLength,
                pBuffer,
                pDataLength,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn MQCB(
        &self,
        Hconn: mqsys::MQHCONN,
        Operation: mqsys::MQLONG,
        pCallbackDesc: mqsys::PMQVOID,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQCB(
                self,
                Hconn,
                Operation,
                pCallbackDesc,
                Hobj,
                pMsgDesc,
                pGetMsgOpts,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn MQCTL(
        &self,
        Hconn: mqsys::MQHCONN,
        Operation: mqsys::MQLONG,
        pControlOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQCTL(self, Hconn, Operation, pControlOpts, pCompCode, pReason);
        }
    }

    unsafe fn MQSET(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        SelectorCount: mqsys::MQLONG,
        pSelectors: mqsys::PMQLONG,
        IntAttrCount: mqsys::MQLONG,
        pIntAttrs: mqsys::PMQLONG,
        CharAttrLength: mqsys::MQLONG,
        pCharAttrs: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQSET(
                self,
                Hconn,
                Hobj,
                SelectorCount,
                pSelectors,
                IntAttrCount,
                pIntAttrs,
                CharAttrLength,
                pCharAttrs,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn MQSETMP(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pSetPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pPropDesc: mqsys::PMQVOID,
        Type: mqsys::MQLONG,
        ValueLength: mqsys::MQLONG,
        pValue: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQSETMP(
                self,
                Hconn,
                Hmsg,
                pSetPropOpts,
                pName,
                pPropDesc,
                Type,
                ValueLength,
                pValue,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn MQSTAT(
        &self,
        Hconn: mqsys::MQHCONN,
        Type: mqsys::MQLONG,
        pStatus: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQSTAT(self, Hconn, Type, pStatus, pCompCode, pReason);
        }
    }

    unsafe fn MQDLTMP(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pDltPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQDLTMP(self, Hconn, Hmsg, pDltPropOpts, pName, pCompCode, pReason);
        }
    }

    unsafe fn MQXCNVC(
        &self,
        Hconn: mqsys::MQHCONN,
        Options: mqsys::MQLONG,
        SourceCCSID: mqsys::MQLONG,
        SourceLength: mqsys::MQLONG,
        pSourceBuffer: mqsys::PMQCHAR,
        TargetCCSID: mqsys::MQLONG,
        TargetLength: mqsys::MQLONG,
        pTargetBuffer: mqsys::PMQCHAR,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::MQXCNVC(
                self,
                Hconn,
                Options,
                SourceCCSID,
                SourceLength,
                pSourceBuffer,
                TargetCCSID,
                TargetLength,
                pTargetBuffer,
                pDataLength,
                pCompCode,
                pReason,
            );
        }
    }
}

#[cfg(feature = "mqai")]
impl function::Mqai for MqmContainer {
    unsafe fn mqCreateBag(
        &self,
        Options: mqsys::MQLONG,
        pBag: mqsys::PMQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqCreateBag(self, Options, pBag, pCompCode, pReason);
        }
    }

    unsafe fn mqDeleteBag(&self, pBag: mqsys::PMQHBAG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG) {
        unsafe {
            MqWrapper::mqDeleteBag(self, pBag, pCompCode, pReason);
        }
    }

    unsafe fn mqAddInquiry(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddInquiry(self, Bag, Selector, pCompCode, pReason);
        }
    }

    unsafe fn mqDeleteItem(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqDeleteItem(self, Bag, Selector, ItemIndex, pCompCode, pReason);
        }
    }

    unsafe fn mqAddInteger(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddInteger(self, Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddIntegerFilter(self, Bag, Selector, ItemValue, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqAddInteger64(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddInteger64(self, Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqAddString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddString(self, Bag, Selector, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn mqAddStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddStringFilter(self, Bag, Selector, BufferLength, pBuffer, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqAddByteString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddByteString(self, Bag, Selector, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddByteStringFilter(self, Bag, Selector, BufferLength, pBuffer, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqSetInteger(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetInteger(self, Bag, Selector, ItemIndex, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetIntegerFilter(self, Bag, Selector, ItemIndex, ItemValue, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqAddBag(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddBag(self, Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqSetString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetString(self, Bag, Selector, ItemIndex, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn mqSetStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetStringFilter(
                self,
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                Operator,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn mqSetByteString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetByteString(self, Bag, Selector, ItemIndex, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn mqSetByteStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetByteStringFilter(
                self,
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                Operator,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn mqInquireInteger(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireInteger(self, Bag, Selector, ItemIndex, pItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqInquireIntegerFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireIntegerFilter(self, Bag, Selector, ItemIndex, pItemValue, pOperator, pCompCode, pReason);
        }
    }

    unsafe fn mqInquireInteger64(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireInteger64(self, Bag, Selector, ItemIndex, pItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqInquireByteString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pByteStringLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireByteString(
                self,
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                pByteStringLength,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn mqInquireString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pStringLength: mqsys::PMQLONG,
        pCodedCharSetId: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireString(
                self,
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                pStringLength,
                pCodedCharSetId,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn mqInquireStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pStringLength: mqsys::PMQLONG,
        pCodedCharSetId: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireStringFilter(
                self,
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                pStringLength,
                pCodedCharSetId,
                pOperator,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn mqInquireByteStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pByteStringLength: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireByteStringFilter(
                self,
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                pByteStringLength,
                pOperator,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn mqInquireBag(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireBag(self, Bag, Selector, ItemIndex, pItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqCountItems(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        pItemCount: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqCountItems(self, Bag, Selector, pItemCount, pCompCode, pReason);
        }
    }

    unsafe fn mqExecute(
        &self,
        Hconn: mqsys::MQHCONN,
        Command: mqsys::MQLONG,
        OptionsBag: mqsys::MQHBAG,
        AdminBag: mqsys::MQHBAG,
        ResponseBag: mqsys::MQHBAG,
        AdminQ: mqsys::MQHOBJ,
        ResponseQ: mqsys::MQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqExecute(
                self,
                Hconn,
                Command,
                OptionsBag,
                AdminBag,
                ResponseBag,
                AdminQ,
                ResponseQ,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn mqSetInteger64(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetInteger64(self, Bag, Selector, ItemIndex, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqClearBag(&self, Bag: mqsys::MQHBAG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG) {
        unsafe {
            MqWrapper::mqClearBag(self, Bag, pCompCode, pReason);
        }
    }

    unsafe fn mqGetBag(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        Bag: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqGetBag(self, Hconn, Hobj, pMsgDesc, pGetMsgOpts, Bag, pCompCode, pReason);
        }
    }

    unsafe fn mqPutBag(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        Bag: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqPutBag(self, Hconn, Hobj, pMsgDesc, pPutMsgOpts, Bag, pCompCode, pReason);
        }
    }

    unsafe fn mqTruncateBag(
        &self,
        Bag: mqsys::MQHBAG,
        ItemCount: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    ) {
        unsafe {
            MqWrapper::mqTruncateBag(self, Bag, ItemCount, pCompCode, pReason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadMqm, MqmContainer};

    #[test]
    fn mqdist_load_default() {
        unsafe { MqmContainer::load_mqm_default() }.expect("Could not open library or load symbols");
    }
}
