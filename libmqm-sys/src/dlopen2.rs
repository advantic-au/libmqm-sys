#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_variables)]
/*!
 * Dynamic loading of the MQI library using dlopen2
 *
 * Example
 * -------
 *
 *  Dynamically load the `libmqm_r` library and issue an `MQCONN`
 *
 * ```no_run
 *  use dlopen2::wrapper::Container;
 *  use libmqm_sys::{lib, dlopen2::MqWrapper};
 *
 * # fn main() -> Result<(), dlopen2::Error> {
 * #
 * // Dynamically load the libmqm_r library
 * let mq: Container<MqWrapper> = unsafe { Container::load("libmqm_r") }?;
 *
 * // Connect to MQ
 * let mut hconn = lib::MQHC_DEF_HCONN;
 * let mut comp_code = lib::MQCC_UNKNOWN;
 * let mut reason = lib::MQRC_NONE;
 * let mut qmgr: [lib::MQCHAR; 48] = [32; 48]; // All spaces
 * unsafe {
 *    mq.MQCONN(
 *      &qmgr,
 *      &mut hconn,
 *      &mut comp_code,
 *      &mut reason,
 *    );
 * }
 * #
 * # Ok(())
 * # }
 * ```
 */

use dlopen2::wrapper::Container;

use crate::lib as mqsys;

/// A dlopen2 [`WrapperApi`] implementation for MQI and MQAI function calls
pub use super::generated::dlopen2::MqWrapper;

/// Name of the platform dependent MQM dynamic library
pub const MQM_LIB: &str = if cfg!(windows) { "mqm.dll" } else { "libmqm_r.so" };

/// A [dlopen2] [Container] for the MQI library
pub type MqmContainer = Container<MqWrapper>;

/// Extension trait for [`MqmContainer`] to load the MQM library using dlopen2
pub trait LoadMqm {
    /// Loads the MQM library using the platform dependent search rules
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

impl crate::Mqi for MqmContainer {
    unsafe fn MQCONNX(
        &self,
        pQMgrName: &mqsys::MQCHAR48,
        pConnectOpts: &mut mqsys::MQCNO,
        pHconn: &mut mqsys::MQHCONN,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQCONNX(self, pQMgrName, pConnectOpts, pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQCONN(
        &self,
        pQMgrName: &mqsys::MQCHAR48,
        pHconn: &mut mqsys::MQHCONN,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQCONN(self, pQMgrName, pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQDISC(&self, pHconn: &mut mqsys::MQHCONN, pCompCode: &mut mqsys::MQLONG, pReason: &mut mqsys::MQLONG) {
        unsafe {
            MqWrapper::MQDISC(self, pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQOPEN(
        &self,
        Hconn: mqsys::MQHCONN,
        pObjDesc: mqsys::PMQVOID,
        Options: mqsys::MQLONG,
        pHobj: &mut mqsys::MQHOBJ,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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

    unsafe fn MQCMIT(&self, Hconn: mqsys::MQHCONN, pCompCode: &mut mqsys::MQLONG, pReason: &mut mqsys::MQLONG) {
        unsafe { MqWrapper::MQCMIT(self, Hconn, pCompCode, pReason) };
    }

    unsafe fn MQCLOSE(
        &self,
        Hconn: mqsys::MQHCONN,
        pHobj: &mut mqsys::MQHOBJ,
        Options: mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pDataLength: &mut mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pHobj: Option<&mut mqsys::MQHOBJ>,
        pHsub: mqsys::PMQHOBJ,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQSUBRQ(self, Hconn, Hsub, Action, pSubRqOpts, pCompCode, pReason);
        }
    }

    unsafe fn MQBEGIN(
        &self,
        Hconn: mqsys::MQHCONN,
        pBeginOptions: Option<&mut mqsys::MQBO>,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQBEGIN(self, Hconn, pBeginOptions, pCompCode, pReason);
        }
    }

    unsafe fn MQBACK(&self, Hconn: mqsys::MQHCONN, pCompCode: &mut mqsys::MQLONG, pReason: &mut mqsys::MQLONG) {
        unsafe {
            MqWrapper::MQBACK(self, Hconn, pCompCode, pReason);
        }
    }

    unsafe fn MQCRTMH(
        &self,
        Hconn: mqsys::MQHCONN,
        pCrtMsgHOpts: mqsys::PMQVOID,
        pHmsg: &mut mqsys::MQHMSG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQCRTMH(self, Hconn, pCrtMsgHOpts, pHmsg, pCompCode, pReason);
        }
    }

    unsafe fn MQDLTMH(
        &self,
        Hconn: mqsys::MQHCONN,
        pHmsg: &mut mqsys::MQHMSG,
        pDltMsgHOpts: mqsys::PMQVOID,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pDataLength: &mut mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pDataLength: &mut mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pBufMsgHOpts: &mqsys::MQBMHO,
        pMsgDesc: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: &mut mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCallbackDesc: Option<&mqsys::MQCBD>,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: Option<&mqsys::MQGMO>,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQDLTMP(self, Hconn, Hmsg, pDltPropOpts, pName, pCompCode, pReason);
        }
    }
}

#[cfg(feature = "exits")]
impl crate::Exits for MqmContainer {
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
        pDataLength: &mut mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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

    unsafe fn MQXEP(
        &self,
        Hconfig: mqsys::MQHCONFIG,
        ExitReason: mqsys::MQLONG,
        Function: mqsys::MQLONG,
        pEntryPoint: mqsys::PMQFUNC,
        pExitOpts: mqsys::PMQXEPO,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        todo!()
    }

    unsafe fn MQXCLWLN(
        &self,
        pExitParms: mqsys::PMQWXP,
        CurrentRecord: mqsys::MQPTR,
        NextOffset: mqsys::MQLONG,
        pNextRecord: mqsys::PMQPTR,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        todo!()
    }

    unsafe fn MQXDX(
        &self,
        pDataConvExitParms: mqsys::PMQDXP,
        pMsgDesc: mqsys::PMQMD,
        InBufferLength: mqsys::MQLONG,
        pInBuffer: mqsys::PMQVOID,
        OutBufferLength: mqsys::MQLONG,
        pOutBuffer: mqsys::PMQVOID,
    ) {
        todo!()
    }

    unsafe fn MQZEP(
        &self,
        Hconfig: mqsys::MQHCONFIG,
        Function: mqsys::MQLONG,
        pEntryPoint: mqsys::PMQFUNC,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        todo!()
    }
}

#[cfg(feature = "mqai")]
impl crate::Mqai for MqmContainer {
    unsafe fn mqCreateBag(
        &self,
        Options: mqsys::MQLONG,
        pBag: mqsys::PMQHBAG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqCreateBag(self, Options, pBag, pCompCode, pReason);
        }
    }

    unsafe fn mqDeleteBag(&self, pBag: mqsys::PMQHBAG, pCompCode: &mut mqsys::MQLONG, pReason: &mut mqsys::MQLONG) {
        unsafe {
            MqWrapper::mqDeleteBag(self, pBag, pCompCode, pReason);
        }
    }

    unsafe fn mqAddInquiry(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetInteger64(self, Bag, Selector, ItemIndex, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqClearBag(&self, Bag: mqsys::MQHBAG, pCompCode: &mut mqsys::MQLONG, pReason: &mut mqsys::MQLONG) {
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
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
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqPutBag(self, Hconn, Hobj, pMsgDesc, pPutMsgOpts, Bag, pCompCode, pReason);
        }
    }

    unsafe fn mqTruncateBag(
        &self,
        Bag: mqsys::MQHBAG,
        ItemCount: mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqTruncateBag(self, Bag, ItemCount, pCompCode, pReason);
        }
    }

    unsafe fn mqBagToBuffer(
        &self,
        OptionsBag: mqsys::MQHBAG,
        DataBag: mqsys::MQHBAG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: &mut mqsys::MQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqBagToBuffer(
                self,
                OptionsBag,
                DataBag,
                BufferLength,
                pBuffer,
                pDataLength,
                pCompCode,
                pReason,
            );
        }
    }

    unsafe fn mqBufferToBag(
        &self,
        OptionsBag: mqsys::MQHBAG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        DataBag: mqsys::MQHBAG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqBufferToBag(self, OptionsBag, BufferLength, pBuffer, DataBag, pCompCode, pReason);
        }
    }

    unsafe fn mqInquireItemInfo(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pOutSelector: mqsys::PMQLONG,
        pItemType: mqsys::PMQLONG,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireItemInfo(self, Bag, Selector, ItemIndex, pOutSelector, pItemType, pCompCode, pReason);
        }
    }

    unsafe fn mqPad(
        &self,
        pString: mqsys::PMQCHAR,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        todo!()
    }

    unsafe fn mqTrim(
        &self,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pString: mqsys::PMQCHAR,
        pCompCode: &mut mqsys::MQLONG,
        pReason: &mut mqsys::MQLONG,
    ) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use dlopen2::wrapper::Container;

    use crate::lib;

    use super::*;

    #[test]
    fn mqdist_load_default() {
        let _ = unsafe { MqmContainer::load_mqm_default() }.expect("MQM library to be loaded");
    }

    #[test]
    fn mqredist_load() -> Result<(), dlopen2::Error> {
        // Dynamically load the mqm library
        let mq: Container<MqWrapper> = unsafe { Container::load(MQM_LIB) }?;

        let mut hconn = lib::MQHC_DEF_HCONN;
        let mut comp_code = lib::MQCC_UNKNOWN;
        let mut reason = lib::MQRC_NONE;
        let qmgr: [lib::MQCHAR; 48] = [32; 48]; // All spaces
        unsafe {
            mq.MQCONN(&qmgr, &mut hconn, &mut comp_code, &mut reason);
        }

        Ok(())
    }
}
