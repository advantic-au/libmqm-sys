use crate::lib;
use ::dlopen2::wrapper::WrapperApi;
#[derive(::dlopen2::wrapper::WrapperApi, Debug)]
pub struct MqWrapper {
    #[cfg(feature = "mqai")]
    mqAddBag: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddByteString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddByteStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInquiry: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInteger: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddInteger64: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddIntegerFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqAddStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqBagToBuffer: unsafe extern "C" fn(
        OptionsBag: lib::MQHBAG,
        DataBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqBufferToBag: unsafe extern "C" fn(
        OptionsBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        DataBag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqClearBag: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqCountItems: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        pItemCount: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqCreateBag: unsafe extern "C" fn(
        Options: lib::MQLONG,
        pBag: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqDeleteBag: unsafe extern "C" fn(
        pBag: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqDeleteItem: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqGetBag: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: &mut lib::MQGMO,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireBag: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireByteString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pByteStringLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireByteStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pByteStringLength: &mut lib::MQLONG,
        pOperator: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireInteger: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireInteger64: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireIntegerFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQLONG,
        pOperator: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireItemInfo: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pOutSelector: &mut lib::MQLONG,
        pItemType: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pStringLength: &mut lib::MQLONG,
        pCodedCharSetId: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqInquireStringFilter: unsafe extern "C" fn(
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
    ),
    #[cfg(feature = "mqai")]
    mqPad: unsafe extern "C" fn(
        pString: lib::PMQCHAR,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqPutBag: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: &mut lib::MQPMO,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetByteString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetByteStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetInteger: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetInteger64: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetIntegerFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetString: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqSetStringFilter: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqTrim: unsafe extern "C" fn(
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pString: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    #[cfg(feature = "mqai")]
    mqTruncateBag: unsafe extern "C" fn(
        Bag: lib::MQHBAG,
        ItemCount: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQBACK: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQBEGIN: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pBeginOptions: Option<&mut lib::MQBO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQBUFMH: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pBufMsgHOpts: &lib::MQBMHO,
        pMsgDesc: lib::PMQVOID,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQCB: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        pCallbackDesc: Option<&lib::MQCBD>,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: Option<&lib::MQGMO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQCLOSE: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pHobj: &mut lib::MQHOBJ,
        Options: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQCMIT: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQCONN: unsafe extern "C" fn(
        pQMgrName: &lib::MQCHAR48,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQCONNX: unsafe extern "C" fn(
        pQMgrName: &lib::MQCHAR48,
        pConnectOpts: &mut lib::MQCNO,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQCRTMH: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pCrtMsgHOpts: &lib::MQCMHO,
        pHmsg: &mut lib::MQHMSG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQCTL: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        pControlOpts: &lib::MQCTLO,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQDISC: unsafe extern "C" fn(
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQDLTMH: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pHmsg: &mut lib::MQHMSG,
        pDltMsgHOpts: &lib::MQDMHO,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQDLTMP: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pDltPropOpts: &lib::MQDMPO,
        pName: &lib::MQCHARV,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQGET: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: &mut lib::MQGMO,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQINQ: unsafe extern "C" fn(
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
    ),
    MQINQMP: unsafe extern "C" fn(
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
    ),
    MQMHBUF: unsafe extern "C" fn(
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
    ),
    MQOPEN: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pObjDesc: &mut lib::MQOD,
        Options: lib::MQLONG,
        pHobj: &mut lib::MQHOBJ,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQPUT: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: &mut lib::MQPMO,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQPUT1: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pObjDesc: &mut lib::MQOD,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: &mut lib::MQPMO,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQSET: unsafe extern "C" fn(
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
    ),
    MQSETMP: unsafe extern "C" fn(
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
    ),
    MQSTAT: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Type: lib::MQLONG,
        pStatus: &mut lib::MQSTS,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQSUB: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        pSubDesc: &mut lib::MQSD,
        pHobj: Option<&mut lib::MQHOBJ>,
        pHsub: &mut lib::MQHOBJ,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQSUBRQ: unsafe extern "C" fn(
        Hconn: lib::MQHCONN,
        Hsub: lib::MQHOBJ,
        Action: lib::MQLONG,
        pSubRqOpts: Option<&mut lib::MQSRO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ),
    MQXCNVC: unsafe extern "C" fn(
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
    ),
}
#[cfg(feature = "mqai")]
impl crate::Mqai for ::dlopen2::wrapper::Container<MqWrapper> {
    unsafe fn mqAddBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe {
            MqWrapper::mqAddBag(self, Bag, Selector, ItemValue, pCompCode, pReason)
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
            MqWrapper::mqAddByteString(
                self,
                Bag,
                Selector,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqAddByteStringFilter(
                self,
                Bag,
                Selector,
                BufferLength,
                pBuffer,
                Operator,
                pCompCode,
                pReason,
            )
        }
    }
    unsafe fn mqAddInquiry(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::mqAddInquiry(self, Bag, Selector, pCompCode, pReason) }
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
            MqWrapper::mqAddInteger(self, Bag, Selector, ItemValue, pCompCode, pReason)
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
            MqWrapper::mqAddInteger64(self, Bag, Selector, ItemValue, pCompCode, pReason)
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
            MqWrapper::mqAddIntegerFilter(
                self,
                Bag,
                Selector,
                ItemValue,
                Operator,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqAddString(
                self,
                Bag,
                Selector,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqAddStringFilter(
                self,
                Bag,
                Selector,
                BufferLength,
                pBuffer,
                Operator,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqBagToBuffer(
                self,
                OptionsBag,
                DataBag,
                BufferLength,
                pBuffer,
                pDataLength,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqBufferToBag(
                self,
                OptionsBag,
                BufferLength,
                pBuffer,
                DataBag,
                pCompCode,
                pReason,
            )
        }
    }
    unsafe fn mqClearBag(
        &self,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::mqClearBag(self, Bag, pCompCode, pReason) }
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
            MqWrapper::mqCountItems(self, Bag, Selector, pItemCount, pCompCode, pReason)
        }
    }
    unsafe fn mqCreateBag(
        &self,
        Options: lib::MQLONG,
        pBag: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::mqCreateBag(self, Options, pBag, pCompCode, pReason) }
    }
    unsafe fn mqDeleteBag(
        &self,
        pBag: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::mqDeleteBag(self, pBag, pCompCode, pReason) }
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
            MqWrapper::mqDeleteItem(self, Bag, Selector, ItemIndex, pCompCode, pReason)
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
            )
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
            MqWrapper::mqGetBag(
                self,
                Hconn,
                Hobj,
                pMsgDesc,
                pGetMsgOpts,
                Bag,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqInquireBag(
                self,
                Bag,
                Selector,
                ItemIndex,
                pItemValue,
                pCompCode,
                pReason,
            )
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
            )
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
            )
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
            MqWrapper::mqInquireInteger(
                self,
                Bag,
                Selector,
                ItemIndex,
                pItemValue,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqInquireInteger64(
                self,
                Bag,
                Selector,
                ItemIndex,
                pItemValue,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqInquireIntegerFilter(
                self,
                Bag,
                Selector,
                ItemIndex,
                pItemValue,
                pOperator,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqInquireItemInfo(
                self,
                Bag,
                Selector,
                ItemIndex,
                pOutSelector,
                pItemType,
                pCompCode,
                pReason,
            )
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
            )
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
            )
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
            MqWrapper::mqPad(self, pString, BufferLength, pBuffer, pCompCode, pReason)
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
            MqWrapper::mqPutBag(
                self,
                Hconn,
                Hobj,
                pMsgDesc,
                pPutMsgOpts,
                Bag,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqSetByteString(
                self,
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            )
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
            )
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
            MqWrapper::mqSetInteger(
                self,
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqSetInteger64(
                self,
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqSetIntegerFilter(
                self,
                Bag,
                Selector,
                ItemIndex,
                ItemValue,
                Operator,
                pCompCode,
                pReason,
            )
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
            MqWrapper::mqSetString(
                self,
                Bag,
                Selector,
                ItemIndex,
                BufferLength,
                pBuffer,
                pCompCode,
                pReason,
            )
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
            )
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
            MqWrapper::mqTrim(self, BufferLength, pBuffer, pString, pCompCode, pReason)
        }
    }
    unsafe fn mqTruncateBag(
        &self,
        Bag: lib::MQHBAG,
        ItemCount: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::mqTruncateBag(self, Bag, ItemCount, pCompCode, pReason) }
    }
}
impl crate::Mqi for ::dlopen2::wrapper::Container<MqWrapper> {
    unsafe fn MQBACK(
        &self,
        Hconn: lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::MQBACK(self, Hconn, pCompCode, pReason) }
    }
    unsafe fn MQBEGIN(
        &self,
        Hconn: lib::MQHCONN,
        pBeginOptions: Option<&mut lib::MQBO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::MQBEGIN(self, Hconn, pBeginOptions, pCompCode, pReason) }
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
            )
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
            )
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
        unsafe { MqWrapper::MQCLOSE(self, Hconn, pHobj, Options, pCompCode, pReason) }
    }
    unsafe fn MQCMIT(
        &self,
        Hconn: lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::MQCMIT(self, Hconn, pCompCode, pReason) }
    }
    unsafe fn MQCONN(
        &self,
        pQMgrName: &lib::MQCHAR48,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::MQCONN(self, pQMgrName, pHconn, pCompCode, pReason) }
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
            MqWrapper::MQCONNX(self, pQMgrName, pConnectOpts, pHconn, pCompCode, pReason)
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
            MqWrapper::MQCRTMH(self, Hconn, pCrtMsgHOpts, pHmsg, pCompCode, pReason)
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
            MqWrapper::MQCTL(self, Hconn, Operation, pControlOpts, pCompCode, pReason)
        }
    }
    unsafe fn MQDISC(
        &self,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    ) {
        unsafe { MqWrapper::MQDISC(self, pHconn, pCompCode, pReason) }
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
            MqWrapper::MQDLTMH(self, Hconn, pHmsg, pDltMsgHOpts, pCompCode, pReason)
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
            MqWrapper::MQDLTMP(
                self,
                Hconn,
                Hmsg,
                pDltPropOpts,
                pName,
                pCompCode,
                pReason,
            )
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
            )
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
            )
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
            )
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
            )
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
            MqWrapper::MQOPEN(self, Hconn, pObjDesc, Options, pHobj, pCompCode, pReason)
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
            )
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
            )
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
            )
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
            )
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
        unsafe { MqWrapper::MQSTAT(self, Hconn, Type, pStatus, pCompCode, pReason) }
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
            MqWrapper::MQSUB(self, Hconn, pSubDesc, pHobj, pHsub, pCompCode, pReason)
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
            MqWrapper::MQSUBRQ(self, Hconn, Hsub, Action, pSubRqOpts, pCompCode, pReason)
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
            )
        }
    }
}
