use crate::lib;
use ::dlopen2::wrapper::WrapperApi;
#[derive(WrapperApi, Debug)]
pub struct MqWrapper {
    #[cfg(feature = "mqai")]
    mqAddBag: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddByteString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddByteStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInquiry: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInteger: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInteger64: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddIntegerFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqBagToBuffer: unsafe extern "C" fn(
        OptionsBag: lib::MQHBAG,
        DataBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqBufferToBag: unsafe extern "C" fn(
        OptionsBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataBag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqClearBag: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqCountItems: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemCount: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqCreateBag: unsafe extern "C" fn(
        Options: lib::MQLONG,
        Bag: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqDeleteBag: unsafe extern "C" fn(
        Bag: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqDeleteItem: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqExecute: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Command: lib::MQLONG,
        OptionsBag: lib::MQHBAG,
        AdminBag: lib::MQHBAG,
        ResponseBag: lib::MQHBAG,
        AdminQ: lib::MQHOBJ,
        ResponseQ: lib::MQHOBJ,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqGetBag: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        GetMsgOpts: &mut lib::MQGMO,
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireBag: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireByteString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        ByteStringLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireByteStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        ByteStringLength: &mut lib::MQLONG,
        Operator: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireInteger: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireInteger64: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireIntegerFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQLONG,
        Operator: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireItemInfo: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        OutSelector: &mut lib::MQLONG,
        ItemType: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        StringLength: &mut lib::MQLONG,
        CodedCharSetId: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireStringFilter: unsafe extern "C" fn(
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
    ),
    #[cfg(feature = "mqai")]
    mqPad: unsafe extern "C" fn(
        String: lib::PMQCHAR,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqPutBag: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        PutMsgOpts: &mut lib::MQPMO,
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetByteString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetByteStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetInteger: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetInteger64: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetIntegerFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqTrim: unsafe extern "C" fn(
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        String: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqTruncateBag: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        ItemCount: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQBACK: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQBEGIN: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        BeginOptions: Option<&mut lib::MQBO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQBUFMH: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        BufMsgHOpts: &lib::MQBMHO,
        MsgDesc: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQCB: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        CallbackDesc: Option<&lib::MQCBD>,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        GetMsgOpts: Option<&lib::MQGMO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQCLOSE: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: &mut lib::MQHOBJ,
        Options: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQCMIT: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQCONN: unsafe extern "C" fn(
        QMgrName: &lib::MQCHAR48,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQCONNX: unsafe extern "C" fn(
        QMgrName: &lib::MQCHAR48,
        ConnectOpts: &mut lib::MQCNO,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQCRTMH: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        CrtMsgHOpts: &lib::MQCMHO,
        Hmsg: &mut lib::MQHMSG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQCTL: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        ControlOpts: &lib::MQCTLO,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQDISC: unsafe extern "C" fn(
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQDLTMH: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hmsg: &mut lib::MQHMSG,
        DltMsgHOpts: &lib::MQDMHO,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQDLTMP: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        DltPropOpts: &lib::MQDMPO,
        Name: &lib::MQCHARV,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQGET: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        GetMsgOpts: &mut lib::MQGMO,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQINQ: unsafe extern "C" fn(
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
    ),
    MQINQMP: unsafe extern "C" fn(
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
    ),
    MQMHBUF: unsafe extern "C" fn(
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
    ),
    MQOPEN: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        ObjDesc: &mut lib::MQOD,
        Options: lib::MQLONG,
        Hobj: &mut lib::MQHOBJ,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQPUT: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        PutMsgOpts: &mut lib::MQPMO,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQPUT1: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        ObjDesc: &mut lib::MQOD,
        MsgDesc: lib::PMQVOID,
        PutMsgOpts: &mut lib::MQPMO,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQSET: unsafe extern "C" fn(
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
    ),
    MQSETMP: unsafe extern "C" fn(
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
    ),
    MQSTAT: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Type: lib::MQLONG,
        Status: &mut lib::MQSTS,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQSUB: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        SubDesc: &mut lib::MQSD,
        Hobj: Option<&mut lib::MQHOBJ>,
        Hsub: &mut lib::MQHOBJ,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQSUBRQ: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hsub: lib::MQHOBJ,
        Action: lib::MQLONG,
        SubRqOpts: Option<&mut lib::MQSRO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ),
    MQXCNVC: unsafe extern "C" fn(
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
    ),
}
#[cfg(feature = "mqai")]
impl crate::Mqai for ::dlopen2::wrapper::Container<MqWrapper> {
    unsafe fn mqAddBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddBag(self, Bag, Selector, ItemValue, CompCode, Reason);
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
            MqWrapper::mqAddByteString(
                self,
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddByteStringFilter(
                self,
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
            MqWrapper::mqAddInquiry(self, Bag, Selector, CompCode, Reason);
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
            MqWrapper::mqAddInteger(self, Bag, Selector, ItemValue, CompCode, Reason);
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
            MqWrapper::mqAddInteger64(self, Bag, Selector, ItemValue, CompCode, Reason);
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
            MqWrapper::mqAddIntegerFilter(
                self,
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
            MqWrapper::mqAddString(
                self,
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddStringFilter(
                self,
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
            MqWrapper::mqBagToBuffer(
                self,
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
            MqWrapper::mqBufferToBag(
                self,
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
            MqWrapper::mqClearBag(self, Bag, CompCode, Reason);
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
            MqWrapper::mqCountItems(self, Bag, Selector, ItemCount, CompCode, Reason);
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
            MqWrapper::mqCreateBag(self, Options, Bag, CompCode, Reason);
        }
    }
    unsafe fn mqDeleteBag(
        &self,
        Bag: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqDeleteBag(self, Bag, CompCode, Reason);
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
            MqWrapper::mqDeleteItem(self, Bag, Selector, ItemIndex, CompCode, Reason);
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
            MqWrapper::mqExecute(
                self,
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
            MqWrapper::mqGetBag(
                self,
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireBag(
                self,
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
            MqWrapper::mqInquireByteString(
                self,
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
            MqWrapper::mqInquireByteStringFilter(
                self,
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
            MqWrapper::mqInquireInteger(
                self,
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqInquireInteger64(
                self,
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
            MqWrapper::mqInquireIntegerFilter(
                self,
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
            MqWrapper::mqInquireItemInfo(
                self,
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
            MqWrapper::mqInquireString(
                self,
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
            MqWrapper::mqInquireStringFilter(
                self,
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
            MqWrapper::mqPad(self, String, BufferLength, Buffer, CompCode, Reason);
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
            MqWrapper::mqPutBag(
                self,
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetByteString(
                self,
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
            MqWrapper::mqSetByteStringFilter(
                self,
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
            MqWrapper::mqSetInteger(
                self,
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetInteger64(
                self,
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
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqSetIntegerFilter(
                self,
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
            MqWrapper::mqSetString(
                self,
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
            MqWrapper::mqSetStringFilter(
                self,
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
            MqWrapper::mqTrim(self, BufferLength, Buffer, String, CompCode, Reason);
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
            MqWrapper::mqTruncateBag(self, Bag, ItemCount, CompCode, Reason);
        }
    }
}
impl crate::Mqi for ::dlopen2::wrapper::Container<MqWrapper> {
    unsafe fn MQBACK(
        &self,
        Hconn: lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQBACK(self, Hconn, CompCode, Reason);
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
            MqWrapper::MQBEGIN(self, Hconn, BeginOptions, CompCode, Reason);
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
            MqWrapper::MQBUFMH(
                self,
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
            MqWrapper::MQCB(
                self,
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
            MqWrapper::MQCLOSE(self, Hconn, Hobj, Options, CompCode, Reason);
        }
    }
    unsafe fn MQCMIT(
        &self,
        Hconn: lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQCMIT(self, Hconn, CompCode, Reason);
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
            MqWrapper::MQCONN(self, QMgrName, Hconn, CompCode, Reason);
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
            MqWrapper::MQCONNX(self, QMgrName, ConnectOpts, Hconn, CompCode, Reason);
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
            MqWrapper::MQCRTMH(self, Hconn, CrtMsgHOpts, Hmsg, CompCode, Reason);
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
            MqWrapper::MQCTL(self, Hconn, Operation, ControlOpts, CompCode, Reason);
        }
    }
    unsafe fn MQDISC(
        &self,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQDISC(self, Hconn, CompCode, Reason);
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
            MqWrapper::MQDLTMH(self, Hconn, Hmsg, DltMsgHOpts, CompCode, Reason);
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
            MqWrapper::MQDLTMP(self, Hconn, Hmsg, DltPropOpts, Name, CompCode, Reason);
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
            MqWrapper::MQGET(
                self,
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
            MqWrapper::MQINQ(
                self,
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
            MqWrapper::MQINQMP(
                self,
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
            MqWrapper::MQMHBUF(
                self,
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
            MqWrapper::MQOPEN(self, Hconn, ObjDesc, Options, Hobj, CompCode, Reason);
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
            MqWrapper::MQPUT(
                self,
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
            MqWrapper::MQPUT1(
                self,
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
            MqWrapper::MQSET(
                self,
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
            MqWrapper::MQSETMP(
                self,
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
            MqWrapper::MQSTAT(self, Hconn, Type, Status, CompCode, Reason);
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
            MqWrapper::MQSUB(self, Hconn, SubDesc, Hobj, Hsub, CompCode, Reason);
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
            MqWrapper::MQSUBRQ(self, Hconn, Hsub, Action, SubRqOpts, CompCode, Reason);
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
            MqWrapper::MQXCNVC(
                self,
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
