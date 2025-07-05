/// Provides access to compile time linked MQI and MQAI functions
#[derive(Debug, Clone, Copy)]
pub struct LinkedMq;
#[cfg(feature = "exits")]
impl crate::Exits for LinkedMq {
    unsafe fn MQXEP(
        &self,
        Hconfig: crate::exits::MQHCONFIG,
        ExitReason: crate::MQLONG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        ExitOpts: Option<&crate::exits::MQXEPO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::exits::MQXEP(
                Hconfig,
                ExitReason,
                Function,
                EntryPoint,
                ExitOpts,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQXCLWLN(
        &self,
        ExitParms: &mut crate::exits::MQWXP,
        CurrentRecord: crate::MQPTR,
        NextOffset: crate::MQLONG,
        NextRecord: &mut crate::MQPTR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::exits::MQXCLWLN(
                ExitParms,
                CurrentRecord,
                NextOffset,
                NextRecord,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQXDX(
        &self,
        DataConvExitParms: &mut crate::exits::MQDXP,
        MsgDesc: crate::PMQMD,
        InBufferLength: crate::MQLONG,
        InBuffer: crate::PMQVOID,
        OutBufferLength: crate::MQLONG,
        OutBuffer: crate::PMQVOID,
    ) {
        unsafe {
            crate::exits::MQXDX(
                DataConvExitParms,
                MsgDesc,
                InBufferLength,
                InBuffer,
                OutBufferLength,
                OutBuffer,
            );
        }
    }
    unsafe fn MQZEP(
        &self,
        Hconfig: crate::exits::MQHCONFIG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::exits::MQZEP(Hconfig, Function, EntryPoint, CompCode, Reason);
        }
    }
}
#[cfg(feature = "mqai")]
impl crate::Mqai for LinkedMq {
    unsafe fn mqAddBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddBag(Bag, Selector, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqAddByteString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddByteString(
                Bag,
                Selector,
                BufferLength,
                Buffer,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddByteStringFilter(
                Bag,
                Selector,
                BufferLength,
                Buffer,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqAddInquiry(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddInquiry(Bag, Selector, CompCode, Reason);
        }
    }
    unsafe fn mqAddInteger(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddInteger(Bag, Selector, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqAddInteger64(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddInteger64(Bag, Selector, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQLONG,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddIntegerFilter(
                Bag,
                Selector,
                ItemValue,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqAddString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddString(
                Bag,
                Selector,
                BufferLength,
                Buffer,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqAddStringFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqAddStringFilter(
                Bag,
                Selector,
                BufferLength,
                Buffer,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqBagToBuffer(
        &self,
        OptionsBag: crate::mqai::MQHBAG,
        DataBag: crate::mqai::MQHBAG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqBagToBuffer(
                OptionsBag,
                DataBag,
                BufferLength,
                Buffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqBufferToBag(
        &self,
        OptionsBag: crate::mqai::MQHBAG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataBag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqBufferToBag(
                OptionsBag,
                BufferLength,
                Buffer,
                DataBag,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqClearBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqClearBag(Bag, CompCode, Reason);
        }
    }
    unsafe fn mqCountItems(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemCount: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqCountItems(Bag, Selector, ItemCount, CompCode, Reason);
        }
    }
    unsafe fn mqCreateBag(
        &self,
        Options: crate::MQLONG,
        Bag: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqCreateBag(Options, Bag, CompCode, Reason);
        }
    }
    unsafe fn mqDeleteBag(
        &self,
        Bag: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqDeleteBag(Bag, CompCode, Reason);
        }
    }
    unsafe fn mqDeleteItem(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqDeleteItem(Bag, Selector, ItemIndex, CompCode, Reason);
        }
    }
    unsafe fn mqExecute(
        &self,
        Hconn: crate::MQHCONN,
        Command: crate::MQLONG,
        OptionsBag: crate::mqai::MQHBAG,
        AdminBag: crate::mqai::MQHBAG,
        ResponseBag: crate::mqai::MQHBAG,
        AdminQ: crate::MQHOBJ,
        ResponseQ: crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqExecute(
                Hconn,
                Command,
                OptionsBag,
                AdminBag,
                ResponseBag,
                AdminQ,
                ResponseQ,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqGetBag(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: &mut crate::MQGMO,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqGetBag(
                Hconn,
                Hobj,
                MsgDesc,
                GetMsgOpts,
                Bag,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireBag(
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireByteString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        ByteStringLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireByteString(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                Buffer,
                ByteStringLength,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireByteStringFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        ByteStringLength: &mut crate::MQLONG,
        Operator: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireByteStringFilter(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                Buffer,
                ByteStringLength,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireInteger(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireInteger(
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireInteger64(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireInteger64(
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireIntegerFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQLONG,
        Operator: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireIntegerFilter(
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireItemInfo(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        OutSelector: &mut crate::MQLONG,
        ItemType: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireItemInfo(
                Bag,
                Selector,
                ItemIndex,
                OutSelector,
                ItemType,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        StringLength: &mut crate::MQLONG,
        CodedCharSetId: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireString(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                Buffer,
                StringLength,
                CodedCharSetId,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqInquireStringFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        StringLength: &mut crate::MQLONG,
        CodedCharSetId: &mut crate::MQLONG,
        Operator: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqInquireStringFilter(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                Buffer,
                StringLength,
                CodedCharSetId,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqPad(
        &self,
        String: crate::PMQCHAR,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqPad(String, BufferLength, Buffer, CompCode, Reason);
        }
    }
    unsafe fn mqPutBag(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqPutBag(
                Hconn,
                Hobj,
                MsgDesc,
                PutMsgOpts,
                Bag,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqSetByteString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqSetByteString(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                Buffer,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqSetByteStringFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqSetByteStringFilter(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                Buffer,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqSetInteger(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqSetInteger(
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqSetInteger64(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqSetInteger64(
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQLONG,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqSetIntegerFilter(
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqSetString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqSetString(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                Buffer,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqSetStringFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqSetStringFilter(
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                Buffer,
                Operator,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqTrim(
        &self,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        String: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqTrim(BufferLength, Buffer, String, CompCode, Reason);
        }
    }
    unsafe fn mqTruncateBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        ItemCount: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::mqai::mqTruncateBag(Bag, ItemCount, CompCode, Reason);
        }
    }
}
impl crate::Mqi for LinkedMq {
    unsafe fn MQBACK(
        &self,
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQBACK(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQBEGIN(
        &self,
        Hconn: crate::MQHCONN,
        BeginOptions: Option<&mut crate::MQBO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQBEGIN(Hconn, BeginOptions, CompCode, Reason);
        }
    }
    unsafe fn MQBUFMH(
        &self,
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        BufMsgHOpts: &crate::MQBMHO,
        MsgDesc: crate::PMQVOID,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQBUFMH(
                Hconn,
                Hmsg,
                BufMsgHOpts,
                MsgDesc,
                BufferLength,
                Buffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQCB(
        &self,
        Hconn: crate::MQHCONN,
        Operation: crate::MQLONG,
        CallbackDesc: Option<&crate::MQCBD>,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: Option<&crate::MQGMO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQCB(
                Hconn,
                Operation,
                CallbackDesc,
                Hobj,
                MsgDesc,
                GetMsgOpts,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQCLOSE(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        Options: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQCLOSE(Hconn, Hobj, Options, CompCode, Reason);
        }
    }
    unsafe fn MQCMIT(
        &self,
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQCMIT(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCONN(
        &self,
        QMgrName: &crate::MQCHAR48,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQCONN(QMgrName, Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCONNX(
        &self,
        QMgrName: &crate::MQCHAR48,
        ConnectOpts: &mut crate::MQCNO,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQCONNX(QMgrName, ConnectOpts, Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCRTMH(
        &self,
        Hconn: crate::MQHCONN,
        CrtMsgHOpts: &crate::MQCMHO,
        Hmsg: &mut crate::MQHMSG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQCRTMH(Hconn, CrtMsgHOpts, Hmsg, CompCode, Reason);
        }
    }
    unsafe fn MQCTL(
        &self,
        Hconn: crate::MQHCONN,
        Operation: crate::MQLONG,
        ControlOpts: &crate::MQCTLO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQCTL(Hconn, Operation, ControlOpts, CompCode, Reason);
        }
    }
    unsafe fn MQDISC(
        &self,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQDISC(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQDLTMH(
        &self,
        Hconn: crate::MQHCONN,
        Hmsg: &mut crate::MQHMSG,
        DltMsgHOpts: &crate::MQDMHO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQDLTMH(Hconn, Hmsg, DltMsgHOpts, CompCode, Reason);
        }
    }
    unsafe fn MQDLTMP(
        &self,
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        DltPropOpts: &crate::MQDMPO,
        Name: &crate::MQCHARV,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQDLTMP(Hconn, Hmsg, DltPropOpts, Name, CompCode, Reason);
        }
    }
    unsafe fn MQGET(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: &mut crate::MQGMO,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQGET(
                Hconn,
                Hobj,
                MsgDesc,
                GetMsgOpts,
                BufferLength,
                Buffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQINQ(
        &self,
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
    ) {
        unsafe {
            crate::MQINQ(
                Hconn,
                Hobj,
                SelectorCount,
                Selectors,
                IntAttrCount,
                IntAttrs,
                CharAttrLength,
                CharAttrs,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQINQMP(
        &self,
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
    ) {
        unsafe {
            crate::MQINQMP(
                Hconn,
                Hmsg,
                InqPropOpts,
                Name,
                PropDesc,
                Type,
                ValueLength,
                Value,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQMHBUF(
        &self,
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
    ) {
        unsafe {
            crate::MQMHBUF(
                Hconn,
                Hmsg,
                MsgHBufOpts,
                Name,
                MsgDesc,
                BufferLength,
                Buffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQOPEN(
        &self,
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        Options: crate::MQLONG,
        Hobj: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQOPEN(Hconn, ObjDesc, Options, Hobj, CompCode, Reason);
        }
    }
    unsafe fn MQPUT(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQPUT(
                Hconn,
                Hobj,
                MsgDesc,
                PutMsgOpts,
                BufferLength,
                Buffer,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQPUT1(
        &self,
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQPUT1(
                Hconn,
                ObjDesc,
                MsgDesc,
                PutMsgOpts,
                BufferLength,
                Buffer,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQSET(
        &self,
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
    ) {
        unsafe {
            crate::MQSET(
                Hconn,
                Hobj,
                SelectorCount,
                Selectors,
                IntAttrCount,
                IntAttrs,
                CharAttrLength,
                CharAttrs,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQSETMP(
        &self,
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
    ) {
        unsafe {
            crate::MQSETMP(
                Hconn,
                Hmsg,
                SetPropOpts,
                Name,
                PropDesc,
                Type,
                ValueLength,
                Value,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQSTAT(
        &self,
        Hconn: crate::MQHCONN,
        Type: crate::MQLONG,
        Status: &mut crate::MQSTS,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQSTAT(Hconn, Type, Status, CompCode, Reason);
        }
    }
    unsafe fn MQSUB(
        &self,
        Hconn: crate::MQHCONN,
        SubDesc: &mut crate::MQSD,
        Hobj: Option<&mut crate::MQHOBJ>,
        Hsub: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQSUB(Hconn, SubDesc, Hobj, Hsub, CompCode, Reason);
        }
    }
    unsafe fn MQSUBRQ(
        &self,
        Hconn: crate::MQHCONN,
        Hsub: crate::MQHOBJ,
        Action: crate::MQLONG,
        SubRqOpts: Option<&mut crate::MQSRO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            crate::MQSUBRQ(Hconn, Hsub, Action, SubRqOpts, CompCode, Reason);
        }
    }
    unsafe fn MQXCNVC(
        &self,
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
    ) {
        unsafe {
            crate::MQXCNVC(
                Hconn,
                Options,
                SourceCCSID,
                SourceLength,
                SourceBuffer,
                TargetCCSID,
                TargetLength,
                TargetBuffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
}
