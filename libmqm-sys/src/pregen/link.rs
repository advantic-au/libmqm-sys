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
        EntryPoint: lib::PMQFUNC,
        ExitOpts: Option<&lib::MQXEPO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQXEP(
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
        ExitParms: &mut lib::MQWXP,
        CurrentRecord: lib::MQPTR,
        NextOffset: lib::MQLONG,
        NextRecord: &mut lib::MQPTR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQXCLWLN(
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
        DataConvExitParms: &mut lib::MQDXP,
        MsgDesc: lib::PMQMD,
        InBufferLength: lib::MQLONG,
        InBuffer: lib::PMQVOID,
        OutBufferLength: lib::MQLONG,
        OutBuffer: lib::PMQVOID,
    ) {
        unsafe {
            lib::MQXDX(
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
        Hconfig: lib::MQHCONFIG,
        Function: lib::MQLONG,
        EntryPoint: lib::PMQFUNC,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQZEP(Hconfig, Function, EntryPoint, CompCode, Reason);
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
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddBag(Bag, Selector, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqAddByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddByteString(Bag, Selector, BufferLength, Buffer, CompCode, Reason);
        }
    }
    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddByteStringFilter(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddInquiry(Bag, Selector, CompCode, Reason);
        }
    }
    unsafe fn mqAddInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddInteger(Bag, Selector, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqAddInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddInteger64(Bag, Selector, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddIntegerFilter(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddString(Bag, Selector, BufferLength, Buffer, CompCode, Reason);
        }
    }
    unsafe fn mqAddStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqAddStringFilter(
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
        OptionsBag: lib::MQHBAG,
        DataBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqBagToBuffer(
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
        OptionsBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataBag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqBufferToBag(
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
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqClearBag(Bag, CompCode, Reason);
        }
    }
    unsafe fn mqCountItems(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemCount: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqCountItems(Bag, Selector, ItemCount, CompCode, Reason);
        }
    }
    unsafe fn mqCreateBag(
        &self,
        Options: lib::MQLONG,
        Bag: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqCreateBag(Options, Bag, CompCode, Reason);
        }
    }
    unsafe fn mqDeleteBag(
        &self,
        Bag: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqDeleteBag(Bag, CompCode, Reason);
        }
    }
    unsafe fn mqDeleteItem(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqDeleteItem(Bag, Selector, ItemIndex, CompCode, Reason);
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
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn mqGetBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        GetMsgOpts: &mut lib::MQGMO,
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqGetBag(Hconn, Hobj, MsgDesc, GetMsgOpts, Bag, CompCode, Reason);
        }
    }
    unsafe fn mqInquireBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireBag(Bag, Selector, ItemIndex, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqInquireByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        ByteStringLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireByteString(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        ByteStringLength: &mut lib::MQLONG,
        Operator: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireByteStringFilter(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireInteger(Bag, Selector, ItemIndex, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqInquireInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireInteger64(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQLONG,
        Operator: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireIntegerFilter(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        OutSelector: &mut lib::MQLONG,
        ItemType: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireItemInfo(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        StringLength: &mut lib::MQLONG,
        CodedCharSetId: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireString(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        StringLength: &mut lib::MQLONG,
        CodedCharSetId: &mut lib::MQLONG,
        Operator: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqInquireStringFilter(
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
        String: lib::PMQCHAR,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqPad(String, BufferLength, Buffer, CompCode, Reason);
        }
    }
    unsafe fn mqPutBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        PutMsgOpts: &mut lib::MQPMO,
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqPutBag(Hconn, Hobj, MsgDesc, PutMsgOpts, Bag, CompCode, Reason);
        }
    }
    unsafe fn mqSetByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetByteString(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetByteStringFilter(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetInteger(Bag, Selector, ItemIndex, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqSetInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetInteger64(Bag, Selector, ItemIndex, ItemValue, CompCode, Reason);
        }
    }
    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetIntegerFilter(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetString(
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqSetStringFilter(
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
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        String: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqTrim(BufferLength, Buffer, String, CompCode, Reason);
        }
    }
    unsafe fn mqTruncateBag(
        &self,
        Bag: lib::MQHBAG,
        ItemCount: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::mqTruncateBag(Bag, ItemCount, CompCode, Reason);
        }
    }
}
impl crate::Mqi for LinkedMq {
    unsafe fn MQBACK(
        &self,
        Hconn: lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQBACK(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQBEGIN(
        &self,
        Hconn: lib::MQHCONN,
        BeginOptions: Option<&mut lib::MQBO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQBEGIN(Hconn, BeginOptions, CompCode, Reason);
        }
    }
    unsafe fn MQBUFMH(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        BufMsgHOpts: &lib::MQBMHO,
        MsgDesc: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQBUFMH(
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
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        CallbackDesc: Option<&lib::MQCBD>,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        GetMsgOpts: Option<&lib::MQGMO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCB(
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
        Hconn: lib::MQHCONN,
        Hobj: &mut lib::MQHOBJ,
        Options: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCLOSE(Hconn, Hobj, Options, CompCode, Reason);
        }
    }
    unsafe fn MQCMIT(
        &self,
        Hconn: lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCMIT(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCONN(
        &self,
        QMgrName: &lib::MQCHAR48,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCONN(QMgrName, Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCONNX(
        &self,
        QMgrName: &lib::MQCHAR48,
        ConnectOpts: &mut lib::MQCNO,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCONNX(QMgrName, ConnectOpts, Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCRTMH(
        &self,
        Hconn: lib::MQHCONN,
        CrtMsgHOpts: &lib::MQCMHO,
        Hmsg: &mut lib::MQHMSG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCRTMH(Hconn, CrtMsgHOpts, Hmsg, CompCode, Reason);
        }
    }
    unsafe fn MQCTL(
        &self,
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        ControlOpts: &lib::MQCTLO,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQCTL(Hconn, Operation, ControlOpts, CompCode, Reason);
        }
    }
    unsafe fn MQDISC(
        &self,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQDISC(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQDLTMH(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: &mut lib::MQHMSG,
        DltMsgHOpts: &lib::MQDMHO,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQDLTMH(Hconn, Hmsg, DltMsgHOpts, CompCode, Reason);
        }
    }
    unsafe fn MQDLTMP(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        DltPropOpts: &lib::MQDMPO,
        Name: &lib::MQCHARV,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQDLTMP(Hconn, Hmsg, DltPropOpts, Name, CompCode, Reason);
        }
    }
    unsafe fn MQGET(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        GetMsgOpts: &mut lib::MQGMO,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQGET(
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
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        SelectorCount: lib::MQLONG,
        Selectors: lib::PMQLONG,
        IntAttrCount: lib::MQLONG,
        IntAttrs: lib::PMQLONG,
        CharAttrLength: lib::MQLONG,
        CharAttrs: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQINQ(
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
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        InqPropOpts: &mut lib::MQIMPO,
        Name: &lib::MQCHARV,
        PropDesc: &mut lib::MQPD,
        Type: &mut lib::MQLONG,
        ValueLength: lib::MQLONG,
        Value: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQINQMP(
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
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        MsgHBufOpts: &lib::MQMHBO,
        Name: &lib::MQCHARV,
        MsgDesc: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQMHBUF(
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
        Hconn: lib::MQHCONN,
        ObjDesc: &mut lib::MQOD,
        Options: lib::MQLONG,
        Hobj: &mut lib::MQHOBJ,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQOPEN(Hconn, ObjDesc, Options, Hobj, CompCode, Reason);
        }
    }
    unsafe fn MQPUT(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        PutMsgOpts: &mut lib::MQPMO,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQPUT(
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
        Hconn: lib::MQHCONN,
        ObjDesc: &mut lib::MQOD,
        MsgDesc: lib::PMQVOID,
        PutMsgOpts: &mut lib::MQPMO,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQPUT1(
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
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        SelectorCount: lib::MQLONG,
        Selectors: lib::PMQLONG,
        IntAttrCount: lib::MQLONG,
        IntAttrs: lib::PMQLONG,
        CharAttrLength: lib::MQLONG,
        CharAttrs: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQSET(
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
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        SetPropOpts: &lib::MQSMPO,
        Name: &lib::MQCHARV,
        PropDesc: &mut lib::MQPD,
        Type: lib::MQLONG,
        ValueLength: lib::MQLONG,
        Value: lib::PMQVOID,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQSETMP(
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
        Hconn: lib::MQHCONN,
        Type: lib::MQLONG,
        Status: &mut lib::MQSTS,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQSTAT(Hconn, Type, Status, CompCode, Reason);
        }
    }
    unsafe fn MQSUB(
        &self,
        Hconn: lib::MQHCONN,
        SubDesc: &mut lib::MQSD,
        Hobj: Option<&mut lib::MQHOBJ>,
        Hsub: &mut lib::MQHOBJ,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQSUB(Hconn, SubDesc, Hobj, Hsub, CompCode, Reason);
        }
    }
    unsafe fn MQSUBRQ(
        &self,
        Hconn: lib::MQHCONN,
        Hsub: lib::MQHOBJ,
        Action: lib::MQLONG,
        SubRqOpts: Option<&mut lib::MQSRO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQSUBRQ(Hconn, Hsub, Action, SubRqOpts, CompCode, Reason);
        }
    }
    unsafe fn MQXCNVC(
        &self,
        Hconn: lib::MQHCONN,
        Options: lib::MQLONG,
        SourceCCSID: lib::MQLONG,
        SourceLength: lib::MQLONG,
        SourceBuffer: lib::PMQCHAR,
        TargetCCSID: lib::MQLONG,
        TargetLength: lib::MQLONG,
        TargetBuffer: lib::PMQCHAR,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            lib::MQXCNVC(
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
