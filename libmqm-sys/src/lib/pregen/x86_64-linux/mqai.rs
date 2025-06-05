/* Generated with MQ client version 9.4.2.1 */

pub type MQHBAG = MQLONG;
pub type PMQHBAG = *mut MQHBAG;
pub const MQCBO_NONE: MQLONG = 0;
pub const MQCBO_USER_BAG: MQLONG = 0;
pub const MQCBO_ADMIN_BAG: MQLONG = 1;
pub const MQCBO_COMMAND_BAG: MQLONG = 16;
pub const MQCBO_SYSTEM_BAG: MQLONG = 32;
pub const MQCBO_GROUP_BAG: MQLONG = 64;
pub const MQCBO_LIST_FORM_ALLOWED: MQLONG = 2;
pub const MQCBO_LIST_FORM_INHIBITED: MQLONG = 0;
pub const MQCBO_REORDER_AS_REQUIRED: MQLONG = 4;
pub const MQCBO_DO_NOT_REORDER: MQLONG = 0;
pub const MQCBO_CHECK_SELECTORS: MQLONG = 8;
pub const MQCBO_DO_NOT_CHECK_SELECTORS: MQLONG = 0;
pub const MQBL_NULL_TERMINATED: MQLONG = -1;
pub const MQITEM_INTEGER: MQLONG = 1;
pub const MQITEM_STRING: MQLONG = 2;
pub const MQITEM_BAG: MQLONG = 3;
pub const MQITEM_BYTE_STRING: MQLONG = 4;
pub const MQITEM_INTEGER_FILTER: MQLONG = 5;
pub const MQITEM_STRING_FILTER: MQLONG = 6;
pub const MQITEM_INTEGER64: MQLONG = 7;
pub const MQITEM_BYTE_STRING_FILTER: MQLONG = 8;
pub const MQIT_INTEGER: MQLONG = 1;
pub const MQIT_STRING: MQLONG = 2;
pub const MQIT_BAG: MQLONG = 3;
pub const MQHA_FIRST: MQLONG = 4001;
pub const MQHA_BAG_HANDLE: MQLONG = 4001;
pub const MQHA_LAST_USED: MQLONG = 4001;
pub const MQHA_LAST: MQLONG = 6000;
pub const MQOA_FIRST: MQLONG = 1;
pub const MQOA_LAST: MQLONG = 9000;
pub const MQIASY_FIRST: MQLONG = -1;
pub const MQIASY_CODED_CHAR_SET_ID: MQLONG = -1;
pub const MQIASY_TYPE: MQLONG = -2;
pub const MQIASY_COMMAND: MQLONG = -3;
pub const MQIASY_MSG_SEQ_NUMBER: MQLONG = -4;
pub const MQIASY_CONTROL: MQLONG = -5;
pub const MQIASY_COMP_CODE: MQLONG = -6;
pub const MQIASY_REASON: MQLONG = -7;
pub const MQIASY_BAG_OPTIONS: MQLONG = -8;
pub const MQIASY_VERSION: MQLONG = -9;
pub const MQIASY_LAST_USED: MQLONG = -9;
pub const MQIASY_LAST: MQLONG = -2000;
pub const MQSEL_ANY_SELECTOR: MQLONG = -30001;
pub const MQSEL_ANY_USER_SELECTOR: MQLONG = -30002;
pub const MQSEL_ANY_SYSTEM_SELECTOR: MQLONG = -30003;
pub const MQSEL_ALL_SELECTORS: MQLONG = -30001;
pub const MQSEL_ALL_USER_SELECTORS: MQLONG = -30002;
pub const MQSEL_ALL_SYSTEM_SELECTORS: MQLONG = -30003;
pub const MQIND_NONE: MQLONG = -1;
pub const MQIND_ALL: MQLONG = -2;
pub const MQHB_UNUSABLE_HBAG: MQLONG = -1;
pub const MQHB_NONE: MQLONG = -2;
unsafe extern "C" {
    pub fn mqAddBag(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqAddByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqAddByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqAddInquiry(
        Bag: MQHBAG,
        Selector: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqAddInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqAddInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQINT64,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqAddIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQLONG,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqAddString(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqAddStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqBagToBuffer(
        OptionsBag: MQHBAG,
        DataBag: MQHBAG,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqBufferToBag(
        OptionsBag: MQHBAG,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        DataBag: MQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqClearBag(Bag: MQHBAG, pCompCode: PMQLONG, pReason: PMQLONG);
    pub fn mqCountItems(
        Bag: MQHBAG,
        Selector: MQLONG,
        pItemCount: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqCreateBag(
        Options: MQLONG,
        pBag: PMQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqDeleteBag(pBag: PMQHBAG, pCompCode: PMQLONG, pReason: PMQLONG);
    pub fn mqDeleteItem(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqExecute(
        Hconn: MQHCONN,
        Command: MQLONG,
        OptionsBag: MQHBAG,
        AdminBag: MQHBAG,
        ResponseBag: MQHBAG,
        AdminQ: MQHOBJ,
        ResponseQ: MQHOBJ,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqGetBag(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        Bag: MQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireBag(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: PMQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pByteStringLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pByteStringLength: PMQLONG,
        pOperator: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: PMQINT64,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: PMQLONG,
        pOperator: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireItemInfo(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pOutSelector: PMQLONG,
        pItemType: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pStringLength: PMQLONG,
        pCodedCharSetId: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqInquireStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pStringLength: PMQLONG,
        pCodedCharSetId: PMQLONG,
        pOperator: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqPad(
        pString: PMQCHAR,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqPutBag(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: PMQVOID,
        Bag: MQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqSetByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqSetByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqSetInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqSetInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQINT64,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqSetIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQLONG,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqSetString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqSetStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqTrim(
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pString: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    pub fn mqTruncateBag(
        Bag: MQHBAG,
        ItemCount: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
