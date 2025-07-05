use ::dlopen2::wrapper::WrapperApi;
#[derive(WrapperApi, Debug)]
pub struct MqWrapper {
    #[cfg(feature = "mqai")]
    mqAddBag: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddByteString: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddByteStringFilter: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInquiry: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInteger: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInteger64: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddIntegerFilter: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQLONG,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddString: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddStringFilter: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqBagToBuffer: unsafe extern "C" fn(
        OptionsBag: crate::mqai::MQHBAG,
        DataBag: crate::mqai::MQHBAG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqBufferToBag: unsafe extern "C" fn(
        OptionsBag: crate::mqai::MQHBAG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataBag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqClearBag: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqCountItems: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemCount: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqCreateBag: unsafe extern "C" fn(
        Options: crate::MQLONG,
        Bag: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqDeleteBag: unsafe extern "C" fn(
        Bag: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqDeleteItem: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqExecute: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Command: crate::MQLONG,
        OptionsBag: crate::mqai::MQHBAG,
        AdminBag: crate::mqai::MQHBAG,
        ResponseBag: crate::mqai::MQHBAG,
        AdminQ: crate::MQHOBJ,
        ResponseQ: crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqGetBag: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: &mut crate::MQGMO,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireBag: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireByteString: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        ByteStringLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireByteStringFilter: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        ByteStringLength: &mut crate::MQLONG,
        Operator: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireInteger: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireInteger64: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireIntegerFilter: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQLONG,
        Operator: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireItemInfo: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        OutSelector: &mut crate::MQLONG,
        ItemType: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireString: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        StringLength: &mut crate::MQLONG,
        CodedCharSetId: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireStringFilter: unsafe extern "C" fn(
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
    ),
    #[cfg(feature = "mqai")]
    mqPad: unsafe extern "C" fn(
        String: crate::PMQCHAR,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqPutBag: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetByteString: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetByteStringFilter: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetInteger: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetInteger64: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetIntegerFilter: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQLONG,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetString: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetStringFilter: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqTrim: unsafe extern "C" fn(
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        String: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqTruncateBag: unsafe extern "C" fn(
        Bag: crate::mqai::MQHBAG,
        ItemCount: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQBACK: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQBEGIN: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        BeginOptions: Option<&mut crate::MQBO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQBUFMH: unsafe extern "C" fn(
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
    MQCB: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Operation: crate::MQLONG,
        CallbackDesc: Option<&crate::MQCBD>,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: Option<&crate::MQGMO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQCLOSE: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        Options: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQCMIT: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQCONN: unsafe extern "C" fn(
        QMgrName: &crate::MQCHAR48,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQCONNX: unsafe extern "C" fn(
        QMgrName: &crate::MQCHAR48,
        ConnectOpts: &mut crate::MQCNO,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQCRTMH: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        CrtMsgHOpts: &crate::MQCMHO,
        Hmsg: &mut crate::MQHMSG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQCTL: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Operation: crate::MQLONG,
        ControlOpts: &crate::MQCTLO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQDISC: unsafe extern "C" fn(
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQDLTMH: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hmsg: &mut crate::MQHMSG,
        DltMsgHOpts: &crate::MQDMHO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQDLTMP: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        DltPropOpts: &crate::MQDMPO,
        Name: &crate::MQCHARV,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQGET: unsafe extern "C" fn(
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
    MQINQ: unsafe extern "C" fn(
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
    MQINQMP: unsafe extern "C" fn(
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
    MQMHBUF: unsafe extern "C" fn(
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
    MQOPEN: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        Options: crate::MQLONG,
        Hobj: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQPUT: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQPUT1: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQSET: unsafe extern "C" fn(
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
    MQSETMP: unsafe extern "C" fn(
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
    MQSTAT: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Type: crate::MQLONG,
        Status: &mut crate::MQSTS,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQSUB: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        SubDesc: &mut crate::MQSD,
        Hobj: Option<&mut crate::MQHOBJ>,
        Hsub: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQSUBRQ: unsafe extern "C" fn(
        Hconn: crate::MQHCONN,
        Hsub: crate::MQHOBJ,
        Action: crate::MQLONG,
        SubRqOpts: Option<&mut crate::MQSRO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ),
    MQXCNVC: unsafe extern "C" fn(
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
}
#[cfg(feature = "mqai")]
impl crate::Mqai for ::dlopen2::wrapper::Container<MqWrapper> {
    unsafe fn mqAddBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddBag(self, Bag, Selector, ItemValue, CompCode, Reason);
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddInquiry(self, Bag, Selector, CompCode, Reason);
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
            MqWrapper::mqAddInteger(self, Bag, Selector, ItemValue, CompCode, Reason);
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
            MqWrapper::mqAddInteger64(self, Bag, Selector, ItemValue, CompCode, Reason);
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        OptionsBag: crate::mqai::MQHBAG,
        DataBag: crate::mqai::MQHBAG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        OptionsBag: crate::mqai::MQHBAG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataBag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqClearBag(self, Bag, CompCode, Reason);
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
            MqWrapper::mqCountItems(self, Bag, Selector, ItemCount, CompCode, Reason);
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
            MqWrapper::mqCreateBag(self, Options, Bag, CompCode, Reason);
        }
    }
    unsafe fn mqDeleteBag(
        &self,
        Bag: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqDeleteBag(self, Bag, CompCode, Reason);
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
            MqWrapper::mqDeleteItem(self, Bag, Selector, ItemIndex, CompCode, Reason);
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
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: &mut crate::MQGMO,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQLONG,
        Operator: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        OutSelector: &mut crate::MQLONG,
        ItemType: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        String: crate::PMQCHAR,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqPad(self, String, BufferLength, Buffer, CompCode, Reason);
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQLONG,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        String: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqTrim(self, BufferLength, Buffer, String, CompCode, Reason);
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
            MqWrapper::mqTruncateBag(self, Bag, ItemCount, CompCode, Reason);
        }
    }
}
impl crate::Mqi for ::dlopen2::wrapper::Container<MqWrapper> {
    unsafe fn MQBACK(
        &self,
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQBACK(self, Hconn, CompCode, Reason);
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
            MqWrapper::MQBEGIN(self, Hconn, BeginOptions, CompCode, Reason);
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
        Hconn: crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        Options: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQCLOSE(self, Hconn, Hobj, Options, CompCode, Reason);
        }
    }
    unsafe fn MQCMIT(
        &self,
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQCMIT(self, Hconn, CompCode, Reason);
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
            MqWrapper::MQCONN(self, QMgrName, Hconn, CompCode, Reason);
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
            MqWrapper::MQCONNX(self, QMgrName, ConnectOpts, Hconn, CompCode, Reason);
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
            MqWrapper::MQCRTMH(self, Hconn, CrtMsgHOpts, Hmsg, CompCode, Reason);
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
            MqWrapper::MQCTL(self, Hconn, Operation, ControlOpts, CompCode, Reason);
        }
    }
    unsafe fn MQDISC(
        &self,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQDISC(self, Hconn, CompCode, Reason);
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
            MqWrapper::MQDLTMH(self, Hconn, Hmsg, DltMsgHOpts, CompCode, Reason);
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
            MqWrapper::MQDLTMP(self, Hconn, Hmsg, DltPropOpts, Name, CompCode, Reason);
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
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        Options: crate::MQLONG,
        Hobj: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQOPEN(self, Hconn, ObjDesc, Options, Hobj, CompCode, Reason);
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
        Hconn: crate::MQHCONN,
        Type: crate::MQLONG,
        Status: &mut crate::MQSTS,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            MqWrapper::MQSTAT(self, Hconn, Type, Status, CompCode, Reason);
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
            MqWrapper::MQSUB(self, Hconn, SubDesc, Hobj, Hsub, CompCode, Reason);
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
            MqWrapper::MQSUBRQ(self, Hconn, Hsub, Action, SubRqOpts, CompCode, Reason);
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
