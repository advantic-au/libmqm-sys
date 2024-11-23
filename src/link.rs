/*!
 * Compile time linking of the MQI library
 *
 * Example
 * -------
 *
 *  Use the compile time linked MQ library and issue an `MQCONN`
 *
 * ```no_run
 * use std::ptr::addr_of_mut;
 * use libmqm_sys::{lib, Mqi as _};
 *
 * // Use the compile time linked MQ library
 * let mq = libmqm_sys::link::LinkedMq;
 *
 * // Connect to MQ
 * let mut hconn = lib::MQHC_DEF_HCONN;
 * let mut comp_code = lib::MQCC_UNKNOWN;
 * let mut reason = lib::MQRC_NONE;
 * let mut qmgr: [lib::MQCHAR; 48] = [32; 48]; // All spaces
 * unsafe {
 *    mq.MQCONN(
 *      addr_of_mut!(qmgr).cast(),
 *      addr_of_mut!(hconn),
 *      addr_of_mut!(comp_code),
 *      addr_of_mut!(reason),
 *    );
 * }
 * ```
 *
 */

use crate::function;
use crate::lib;

/// Provides access to compile time linked MQI and MQAI functions
#[derive(Debug, Clone, Copy)]
pub struct LinkedMq;

impl function::Mqi for LinkedMq {
    unsafe fn MQCONNX(
        &self,
        pQMgrName: lib::PMQCHAR,
        pConnectOpts: lib::PMQCNO,
        pHconn: lib::PMQHCONN,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQCONNX(pQMgrName, pConnectOpts, pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQCONN(&self, pQMgrName: lib::PMQCHAR, pHconn: lib::PMQHCONN, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::MQCONN(pQMgrName, pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQDISC(&self, pHconn: lib::PMQHCONN, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::MQDISC(pHconn, pCompCode, pReason);
        }
    }

    unsafe fn MQOPEN(
        &self,
        Hconn: lib::MQHCONN,
        pObjDesc: lib::PMQVOID,
        Options: lib::MQLONG,
        pHobj: lib::PMQHOBJ,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQOPEN(Hconn, pObjDesc, Options, pHobj, pCompCode, pReason);
        }
    }

    unsafe fn MQPUT1(
        &self,
        Hconn: lib::MQHCONN,
        pObjDesc: lib::PMQVOID,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQPUT1(
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

    unsafe fn MQCLOSE(
        &self,
        Hconn: lib::MQHCONN,
        pHobj: lib::PMQHOBJ,
        Options: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQCLOSE(Hconn, pHobj, Options, pCompCode, pReason);
        }
    }

    unsafe fn MQCMIT(&self, Hconn: lib::MQHCONN, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::MQCMIT(Hconn, pCompCode, pReason);
        }
    }

    unsafe fn MQGET(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQGET(
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
        }
    }

    unsafe fn MQPUT(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQPUT(Hconn, Hobj, pMsgDesc, pPutMsgOpts, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn MQINQ(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        SelectorCount: lib::MQLONG,
        pSelectors: lib::PMQLONG,
        IntAttrCount: lib::MQLONG,
        pIntAttrs: lib::PMQLONG,
        CharAttrLength: lib::MQLONG,
        pCharAttrs: lib::PMQCHAR,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQINQ(
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
        Hconn: lib::MQHCONN,
        pSubDesc: lib::PMQVOID,
        pHobj: lib::PMQHOBJ,
        pHsub: lib::PMQHOBJ,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQSUB(Hconn, pSubDesc, pHobj, pHsub, pCompCode, pReason);
        }
    }

    unsafe fn MQSUBRQ(
        &self,
        Hconn: lib::MQHCONN,
        Hsub: lib::MQHOBJ,
        Action: lib::MQLONG,
        pSubRqOpts: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQSUBRQ(Hconn, Hsub, Action, pSubRqOpts, pCompCode, pReason);
        }
    }

    unsafe fn MQBEGIN(&self, Hconn: lib::MQHCONN, pBeginOptions: lib::PMQVOID, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::MQBEGIN(Hconn, pBeginOptions, pCompCode, pReason);
        }
    }

    unsafe fn MQBACK(&self, Hconn: lib::MQHCONN, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::MQBACK(Hconn, pCompCode, pReason);
        }
    }

    unsafe fn MQCRTMH(
        &self,
        Hconn: lib::MQHCONN,
        pCrtMsgHOpts: lib::PMQVOID,
        pHmsg: lib::PMQHMSG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQCRTMH(Hconn, pCrtMsgHOpts, pHmsg, pCompCode, pReason);
        }
    }

    unsafe fn MQDLTMH(
        &self,
        Hconn: lib::MQHCONN,
        pHmsg: lib::PMQHMSG,
        pDltMsgHOpts: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQDLTMH(Hconn, pHmsg, pDltMsgHOpts, pCompCode, pReason);
        }
    }

    unsafe fn MQINQMP(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pInqPropOpts: lib::PMQVOID,
        pName: lib::PMQVOID,
        pPropDesc: lib::PMQVOID,
        pType: lib::PMQLONG,
        ValueLength: lib::MQLONG,
        pValue: lib::PMQVOID,
        pDataLength: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQINQMP(
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
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pMsgHBufOpts: lib::PMQVOID,
        pName: lib::PMQVOID,
        pMsgDesc: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQMHBUF(
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
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pBufMsgHOpts: lib::PMQVOID,
        pMsgDesc: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQBUFMH(
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
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        pCallbackDesc: lib::PMQVOID,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQCB(
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
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        pControlOpts: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQCTL(Hconn, Operation, pControlOpts, pCompCode, pReason);
        }
    }

    unsafe fn MQSET(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        SelectorCount: lib::MQLONG,
        pSelectors: lib::PMQLONG,
        IntAttrCount: lib::MQLONG,
        pIntAttrs: lib::PMQLONG,
        CharAttrLength: lib::MQLONG,
        pCharAttrs: lib::PMQCHAR,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQSET(
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
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pSetPropOpts: lib::PMQVOID,
        pName: lib::PMQVOID,
        pPropDesc: lib::PMQVOID,
        Type: lib::MQLONG,
        ValueLength: lib::MQLONG,
        pValue: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQSETMP(
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
        Hconn: lib::MQHCONN,
        Type: lib::MQLONG,
        pStatus: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQSTAT(Hconn, Type, pStatus, pCompCode, pReason);
        }
    }

    unsafe fn MQDLTMP(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pDltPropOpts: lib::PMQVOID,
        pName: lib::PMQVOID,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQDLTMP(Hconn, Hmsg, pDltPropOpts, pName, pCompCode, pReason);
        }
    }

    unsafe fn MQXCNVC(
        &self,
        Hconn: lib::MQHCONN,
        Options: lib::MQLONG,
        SourceCCSID: lib::MQLONG,
        SourceLength: lib::MQLONG,
        pSourceBuffer: lib::PMQCHAR,
        TargetCCSID: lib::MQLONG,
        TargetLength: lib::MQLONG,
        pTargetBuffer: lib::PMQCHAR,
        pDataLength: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::MQXCNVC(
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
impl function::Mqai for LinkedMq {
    unsafe fn mqCreateBag(&self, Options: lib::MQLONG, pBag: lib::PMQHBAG, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::mqCreateBag(Options, pBag, pCompCode, pReason);
        }
    }

    unsafe fn mqDeleteBag(&self, pBag: lib::PMQHBAG, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::mqDeleteBag(pBag, pCompCode, pReason);
        }
    }

    unsafe fn mqAddInquiry(&self, Bag: lib::MQHBAG, Selector: lib::MQLONG, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::mqAddInquiry(Bag, Selector, pCompCode, pReason);
        }
    }

    unsafe fn mqDeleteItem(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqDeleteItem(Bag, Selector, ItemIndex, pCompCode, pReason);
        }
    }

    unsafe fn mqAddInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqAddInteger(Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqAddIntegerFilter(Bag, Selector, ItemValue, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqAddInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQINT64,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqAddInteger64(Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqAddString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqAddString(Bag, Selector, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn mqAddStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqAddStringFilter(Bag, Selector, BufferLength, pBuffer, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqAddByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqAddByteString(Bag, Selector, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqAddByteStringFilter(Bag, Selector, BufferLength, pBuffer, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqSetInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqSetInteger(Bag, Selector, ItemIndex, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqSetIntegerFilter(Bag, Selector, ItemIndex, ItemValue, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqSetInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQINT64,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqSetInteger64(Bag, Selector, ItemIndex, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqAddBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQHBAG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqAddBag(Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqSetString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqSetString(Bag, Selector, ItemIndex, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn mqSetStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqSetStringFilter(Bag, Selector, ItemIndex, BufferLength, pBuffer, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqSetByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqSetByteString(Bag, Selector, ItemIndex, BufferLength, pBuffer, pCompCode, pReason);
        }
    }

    unsafe fn mqSetByteStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqSetByteStringFilter(Bag, Selector, ItemIndex, BufferLength, pBuffer, Operator, pCompCode, pReason);
        }
    }

    unsafe fn mqInquireInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqInquireInteger(Bag, Selector, ItemIndex, pItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqInquireIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: lib::PMQLONG,
        pOperator: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqInquireIntegerFilter(Bag, Selector, ItemIndex, pItemValue, pOperator, pCompCode, pReason);
        }
    }

    unsafe fn mqInquireInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: lib::PMQINT64,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqInquireInteger64(Bag, Selector, ItemIndex, pItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqInquireByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pByteStringLength: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqInquireByteString(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pStringLength: lib::PMQLONG,
        pCodedCharSetId: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqInquireString(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pStringLength: lib::PMQLONG,
        pCodedCharSetId: lib::PMQLONG,
        pOperator: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqInquireStringFilter(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pByteStringLength: lib::PMQLONG,
        pOperator: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqInquireByteStringFilter(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: lib::PMQHBAG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqInquireBag(Bag, Selector, ItemIndex, pItemValue, pCompCode, pReason);
        }
    }

    unsafe fn mqCountItems(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        pItemCount: lib::PMQLONG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqCountItems(Bag, Selector, pItemCount, pCompCode, pReason);
        }
    }

    unsafe fn mqExecute(
        &self,
        Hconn: lib::MQHCONN,
        Command: lib::MQLONG,
        OptionsBag: lib::MQHBAG,
        AdminBag: lib::MQHBAG,
        ResponseBag: lib::MQHBAG,
        AdminQ: lib::MQHOBJ,
        ResponseQ: lib::MQHOBJ,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqExecute(
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

    unsafe fn mqClearBag(&self, Bag: lib::MQHBAG, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::mqClearBag(Bag, pCompCode, pReason);
        }
    }

    unsafe fn mqGetBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: lib::PMQVOID,
        Bag: lib::MQHBAG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqGetBag(Hconn, Hobj, pMsgDesc, pGetMsgOpts, Bag, pCompCode, pReason);
        }
    }

    unsafe fn mqPutBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: lib::PMQVOID,
        Bag: lib::MQHBAG,
        pCompCode: lib::PMQLONG,
        pReason: lib::PMQLONG,
    ) {
        unsafe {
            lib::mqPutBag(Hconn, Hobj, pMsgDesc, pPutMsgOpts, Bag, pCompCode, pReason);
        }
    }

    unsafe fn mqTruncateBag(&self, Bag: lib::MQHBAG, ItemCount: lib::MQLONG, pCompCode: lib::PMQLONG, pReason: lib::PMQLONG) {
        unsafe {
            lib::mqTruncateBag(Bag, ItemCount, pCompCode, pReason);
        }
    }
}
