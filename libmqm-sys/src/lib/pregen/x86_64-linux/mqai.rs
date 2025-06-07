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
    /// Add Nested Bag to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemValue`: I: Item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddBag(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Add Byte String to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Add Byte String Filter to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing item value
    /// * `Operator`: I: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Add an Inquiry Item to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Attribute selector
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddInquiry(
        Bag: MQHBAG,
        Selector: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Add Integer to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemValue`: I: Item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Add 64-bit Integer to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemValue`: I: Item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQINT64,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Add Integer Filter to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemValue`: I: Item value
    /// * `Operator`: I: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQLONG,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Add String to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddString(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Add String Filter to Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing item value
    /// * `Operator`: I: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqAddStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Convert Bag to PCF
    /// # Arguments
    /// * `OptionsBag`: I: Handle of options bag
    /// * `DataBag`: I: Handle of data bag
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: OB: Buffer to contain PCF
    /// * `pDataLength`: OL: Length of PCF returned in buffer
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqBagToBuffer(
        OptionsBag: MQHBAG,
        DataBag: MQHBAG,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Convert PCF to Bag
    /// # Arguments
    /// * `OptionsBag`: I: Handle of options bag
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing PCF
    /// * `DataBag`: IO: Handle of bag to contain data
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqBufferToBag(
        OptionsBag: MQHBAG,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        DataBag: MQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Delete All Items in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqClearBag(Bag: MQHBAG, pCompCode: PMQLONG, pReason: PMQLONG);
    /// Count Items in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `pItemCount`: O: Number of items
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqCountItems(
        Bag: MQHBAG,
        Selector: MQLONG,
        pItemCount: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Create Bag
    /// # Arguments
    /// * `Options`: I: Bag options
    /// * `pBag`: O: Handle of bag created
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqCreateBag(
        Options: MQLONG,
        pBag: PMQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Delete Bag
    /// # Arguments
    /// * `pBag`: IO: Bag handle
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqDeleteBag(pBag: PMQHBAG, pCompCode: PMQLONG, pReason: PMQLONG);
    /// Delete Item in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqDeleteItem(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Send Admin Command and Receive Reponse
    /// # Arguments
    /// * `Hconn`: I: Connection handle
    /// * `Command`: I: Command identifier
    /// * `OptionsBag`: I: Handle of options bag
    /// * `AdminBag`: I: Handle of admin bag
    /// * `ResponseBag`: I: Handle of response bag
    /// * `AdminQ`: I: Handle of admin queue
    /// * `ResponseQ`: I: Handle of response queue
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
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
    /// Receive PCF Message into Bag
    /// # Arguments
    /// * `Hconn`: I: Connection handle
    /// * `Hobj`: I: Queue handle
    /// * `pMsgDesc`: IO: Message descriptor
    /// * `pGetMsgOpts`: IO: Get-message options
    /// * `Bag`: IO: Handle of bag to contain message
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqGetBag(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: PMQVOID,
        Bag: MQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Inquire Handle in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `pItemValue`: O: Item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqInquireBag(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: PMQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Inquire Byte String in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: OB: Buffer to contain string
    /// * `pByteStringLength`: O: Length of byte string returned
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
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
    /// Inquire Byte String Filter in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: OB: Buffer to contain string
    /// * `pByteStringLength`: O: Length of byte string returned
    /// * `pOperator`: O: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
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
    /// Inquire Integer in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `pItemValue`: O: Item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqInquireInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Inquire 64-bit Integer in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `pItemValue`: O: Item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqInquireInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: PMQINT64,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Inquire Integer Filter in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `pItemValue`: O: Item value
    /// * `pOperator`: O: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqInquireIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: PMQLONG,
        pOperator: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Inquire Attributes of Item in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `pOutSelector`: O: Selector of item
    /// * `pItemType`: O: Data type of item
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqInquireItemInfo(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pOutSelector: PMQLONG,
        pItemType: PMQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Inquire String in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: OB: Buffer to contain string
    /// * `pStringLength`: O: Length of string returned
    /// * `pCodedCharSetId`: O: Character-set identifier of string
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
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
    /// Inquire String Filter in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: OB: Buffer to contain string
    /// * `pStringLength`: O: Length of string returned
    /// * `pCodedCharSetId`: O: Character-set identifier of string
    /// * `pOperator`: O: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
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
    /// Pad Null-terminated String with Blanks
    /// # Arguments
    /// * `pString`: I: Null-terminated string to be padded
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: OB: Buffer to contain padded string
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqPad(
        pString: PMQCHAR,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Send Bag as PCF Message
    /// # Arguments
    /// * `Hconn`: I: Connection handle
    /// * `Hobj`: I: Queue handle
    /// * `pMsgDesc`: IO: Message descriptor
    /// * `pPutMsgOpts`: IO: Put-message options
    /// * `Bag`: I: Handle of bag containing message data
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqPutBag(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: PMQVOID,
        Bag: MQHBAG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Modify Byte String in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `BufferLength`: I: Length of buffer
    /// * `pBuffer`: I: Buffer containing item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqSetByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Modify Byte String Filter in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing item value
    /// * `Operator`: I: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
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
    /// Modify Integer in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `ItemValue`: I: Item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqSetInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Modify 64-bit Integer in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `ItemValue`: I: Item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqSetInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQINT64,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Modify Integer Filter in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `ItemValue`: I: Item value
    /// * `Operator`: I: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqSetIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQLONG,
        Operator: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Modify String in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing item value
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqSetString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Modify String Filter in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `Selector`: I: Item selector
    /// * `ItemIndex`: I: Item index
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing item value
    /// * `Operator`: I: Item operator
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
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
    /// Replace Trailing Blanks with Null Character
    /// # Arguments
    /// * `BufferLength`: IL: Length of buffer
    /// * `pBuffer`: IB: Buffer containing blank-padded string
    /// * `pString`: O: String with blanks discarded
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqTrim(
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pString: PMQCHAR,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
    /// Delete Trailing Items in Bag
    /// # Arguments
    /// * `Bag`: I: Bag handle
    /// * `ItemCount`: I: Number of items to remain in bag
    /// * `pCompCode`: OC: Completion code
    /// * `pReason`: OR: Reason code qualifying CompCode
    pub fn mqTruncateBag(
        Bag: MQHBAG,
        ItemCount: MQLONG,
        pCompCode: PMQLONG,
        pReason: PMQLONG,
    );
}
