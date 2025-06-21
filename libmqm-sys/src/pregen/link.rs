use crate::lib;
/// Provides access to compile time linked MQI and MQAI functions
#[derive(Debug, Clone, Copy)]
pub struct LinkedMq;
#[cfg(feature = "exits")]
impl crate::Exits for LinkedMq {
    unsafe fn MQXEP(
        &self,
        Hconfig: lib::MQHCONFIG,
        ExitReason: lib::MQLONG,
        Function: lib::MQLONG,
        pEntryPoint: lib::PMQFUNC,
        pExitOpts: Option<&lib::MQXEPO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQXEP(
                Hconfig,
                ExitReason,
                Function,
                pEntryPoint,
                pExitOpts,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn MQXCLWLN(
        &self,
        pExitParms: &mut lib::MQWXP,
        CurrentRecord: lib::MQPTR,
        NextOffset: lib::MQLONG,
        pNextRecord: &mut lib::MQPTR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQXCLWLN(
                pExitParms,
                CurrentRecord,
                NextOffset,
                pNextRecord,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn MQXDX(
        &self,
        pDataConvExitParms: &mut lib::MQDXP,
        pMsgDesc: lib::PMQMD,
        InBufferLength: lib::MQLONG,
        pInBuffer: lib::PMQVOID,
        OutBufferLength: lib::MQLONG,
        pOutBuffer: lib::PMQVOID,
    ) {
        unsafe {
            lib::MQXDX(
                pDataConvExitParms,
                pMsgDesc,
                InBufferLength,
                pInBuffer,
                OutBufferLength,
                pOutBuffer,
            );
        }
    }
    unsafe fn MQZEP(
        &self,
        Hconfig: lib::MQHCONFIG,
        Function: lib::MQLONG,
        pEntryPoint: lib::PMQFUNC,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQZEP(Hconfig, Function, pEntryPoint, pCompCode, pReason);
        }
    }
}
#[cfg(feature = "mqai")]
impl crate::Mqai for LinkedMq {
    unsafe fn mqAddBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddBag(Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }
    unsafe fn mqAddByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddByteString(
                Bag,
                Selector,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddByteStringFilter(
                Bag,
                Selector,
                BufferLength,
                pBuffer,
                Operator,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqAddInquiry(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddInquiry(Bag, Selector, pCompCode, pReason);
        }
    }
    unsafe fn mqAddInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddInteger(Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }
    unsafe fn mqAddInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddInteger64(Bag, Selector, ItemValue, pCompCode, pReason);
        }
    }
    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddIntegerFilter(
                Bag,
                Selector,
                ItemValue,
                Operator,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqAddString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddStringFilter(
                Bag,
                Selector,
                BufferLength,
                pBuffer,
                Operator,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqBagToBuffer(
        &self,
        OptionsBag: lib::MQHBAG,
        DataBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqBagToBuffer(
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
        OptionsBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        DataBag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqBufferToBag(
                OptionsBag,
                BufferLength,
                pBuffer,
                DataBag,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqClearBag(
        &self,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqClearBag(Bag, pCompCode, pReason);
        }
    }
    unsafe fn mqCountItems(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        pItemCount: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqCountItems(Bag, Selector, pItemCount, pCompCode, pReason);
        }
    }
    unsafe fn mqCreateBag(
        &self,
        Options: lib::MQLONG,
        pBag: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqCreateBag(Options, pBag, pCompCode, pReason);
        }
    }
    unsafe fn mqDeleteBag(
        &self,
        pBag: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqDeleteBag(pBag, pCompCode, pReason);
        }
    }
    unsafe fn mqDeleteItem(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqDeleteItem(Bag, Selector, ItemIndex, pCompCode, pReason);
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
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqGetBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: &mut lib::MQGMO,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqGetBag(Hconn, Hobj, pMsgDesc, pGetMsgOpts, Bag, pCompCode, pReason);
        }
    }
    unsafe fn mqInquireBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireBag(Bag, Selector, ItemIndex, pItemValue, pCompCode, pReason);
        }
    }
    unsafe fn mqInquireByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pByteStringLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqInquireByteStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pByteStringLength: &mut lib::MQLONG,
        pOperator: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqInquireInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireInteger(
                Bag,
                Selector,
                ItemIndex,
                pItemValue,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqInquireInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireInteger64(
                Bag,
                Selector,
                ItemIndex,
                pItemValue,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqInquireIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQLONG,
        pOperator: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireIntegerFilter(
                Bag,
                Selector,
                ItemIndex,
                pItemValue,
                pOperator,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqInquireItemInfo(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pOutSelector: &mut lib::MQLONG,
        pItemType: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireItemInfo(
                Bag,
                Selector,
                ItemIndex,
                pOutSelector,
                pItemType,
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
        pStringLength: &mut lib::MQLONG,
        pCodedCharSetId: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pStringLength: &mut lib::MQLONG,
        pCodedCharSetId: &mut lib::MQLONG,
        pOperator: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqPad(
        &self,
        pString: lib::PMQCHAR,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqPad(pString, BufferLength, pBuffer, pCompCode, pReason);
        }
    }
    unsafe fn mqPutBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: &mut lib::MQPMO,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqPutBag(Hconn, Hobj, pMsgDesc, pPutMsgOpts, Bag, pCompCode, pReason);
        }
    }
    unsafe fn mqSetByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetByteString(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            );
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
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetByteStringFilter(
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
    unsafe fn mqSetInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetInteger(Bag, Selector, ItemIndex, ItemValue, pCompCode, pReason);
        }
    }
    unsafe fn mqSetInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetInteger64(Bag, Selector, ItemIndex, ItemValue, pCompCode, pReason);
        }
    }
    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetIntegerFilter(
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                Operator,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn mqSetString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetString(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            );
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
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetStringFilter(
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
    unsafe fn mqTrim(
        &self,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pString: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqTrim(BufferLength, pBuffer, pString, pCompCode, pReason);
        }
    }
    unsafe fn mqTruncateBag(
        &self,
        Bag: lib::MQHBAG,
        ItemCount: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqTruncateBag(Bag, ItemCount, pCompCode, pReason);
        }
    }
}
impl crate::Mqi for LinkedMq {
    unsafe fn MQBACK(
        &self,
        Hconn: lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQBACK(Hconn, pCompCode, pReason);
        }
    }
    unsafe fn MQBEGIN(
        &self,
        Hconn: lib::MQHCONN,
        pBeginOptions: Option<&mut lib::MQBO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQBEGIN(Hconn, pBeginOptions, pCompCode, pReason);
        }
    }
    unsafe fn MQBUFMH(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pBufMsgHOpts: &lib::MQBMHO,
        pMsgDesc: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pCallbackDesc: Option<&lib::MQCBD>,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: Option<&lib::MQGMO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn MQCLOSE(
        &self,
        Hconn: lib::MQHCONN,
        pHobj: &mut lib::MQHOBJ,
        Options: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCLOSE(Hconn, pHobj, Options, pCompCode, pReason);
        }
    }
    unsafe fn MQCMIT(
        &self,
        Hconn: lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCMIT(Hconn, pCompCode, pReason);
        }
    }
    unsafe fn MQCONN(
        &self,
        pQMgrName: &lib::MQCHAR48,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCONN(pQMgrName, pHconn, pCompCode, pReason);
        }
    }
    unsafe fn MQCONNX(
        &self,
        pQMgrName: &lib::MQCHAR48,
        pConnectOpts: &mut lib::MQCNO,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCONNX(pQMgrName, pConnectOpts, pHconn, pCompCode, pReason);
        }
    }
    unsafe fn MQCRTMH(
        &self,
        Hconn: lib::MQHCONN,
        pCrtMsgHOpts: &lib::MQCMHO,
        pHmsg: &mut lib::MQHMSG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCRTMH(Hconn, pCrtMsgHOpts, pHmsg, pCompCode, pReason);
        }
    }
    unsafe fn MQCTL(
        &self,
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        pControlOpts: &lib::MQCTLO,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCTL(Hconn, Operation, pControlOpts, pCompCode, pReason);
        }
    }
    unsafe fn MQDISC(
        &self,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQDISC(pHconn, pCompCode, pReason);
        }
    }
    unsafe fn MQDLTMH(
        &self,
        Hconn: lib::MQHCONN,
        pHmsg: &mut lib::MQHMSG,
        pDltMsgHOpts: &lib::MQDMHO,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQDLTMH(Hconn, pHmsg, pDltMsgHOpts, pCompCode, pReason);
        }
    }
    unsafe fn MQDLTMP(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pDltPropOpts: &lib::MQDMPO,
        pName: &lib::MQCHARV,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQDLTMP(Hconn, Hmsg, pDltPropOpts, pName, pCompCode, pReason);
        }
    }
    unsafe fn MQGET(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: &mut lib::MQGMO,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn MQINQMP(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pInqPropOpts: &mut lib::MQIMPO,
        pName: &lib::MQCHARV,
        pPropDesc: &mut lib::MQPD,
        pType: &mut lib::MQLONG,
        ValueLength: lib::MQLONG,
        pValue: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pMsgHBufOpts: &lib::MQMHBO,
        pName: &lib::MQCHARV,
        pMsgDesc: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn MQOPEN(
        &self,
        Hconn: lib::MQHCONN,
        pObjDesc: &mut lib::MQOD,
        Options: lib::MQLONG,
        pHobj: &mut lib::MQHOBJ,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQOPEN(Hconn, pObjDesc, Options, pHobj, pCompCode, pReason);
        }
    }
    unsafe fn MQPUT(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: &mut lib::MQPMO,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQPUT(
                Hconn,
                Hobj,
                pMsgDesc,
                pPutMsgOpts,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            );
        }
    }
    unsafe fn MQPUT1(
        &self,
        Hconn: lib::MQHCONN,
        pObjDesc: &mut lib::MQOD,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: &mut lib::MQPMO,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pSetPropOpts: &lib::MQSMPO,
        pName: &lib::MQCHARV,
        pPropDesc: &mut lib::MQPD,
        Type: lib::MQLONG,
        ValueLength: lib::MQLONG,
        pValue: lib::PMQVOID,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pStatus: &mut lib::MQSTS,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQSTAT(Hconn, Type, pStatus, pCompCode, pReason);
        }
    }
    unsafe fn MQSUB(
        &self,
        Hconn: lib::MQHCONN,
        pSubDesc: &mut lib::MQSD,
        pHobj: Option<&mut lib::MQHOBJ>,
        pHsub: &mut lib::MQHOBJ,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pSubRqOpts: Option<&mut lib::MQSRO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQSUBRQ(Hconn, Hsub, Action, pSubRqOpts, pCompCode, pReason);
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
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
