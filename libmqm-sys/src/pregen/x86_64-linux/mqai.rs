/* Generated with MQ client version 9.4.3.0 */

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
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089150_.html)
    pub fn mqAddBag(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQHBAG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Add Byte String to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddByteString` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089160_.html)
    pub fn mqAddByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQBYTE,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Add Byte String Filter to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing item value
    /// * `Operator`: Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddByteStringFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089170_.html)
    pub fn mqAddByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQBYTE,
        Operator: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Add an Inquiry Item to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Attribute selector
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddInquiry` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089180_.html)
    pub fn mqAddInquiry(
        Bag: MQHBAG,
        Selector: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Add Integer to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddInteger` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089190_.html)
    pub fn mqAddInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Add 64-bit Integer to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddInteger64` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089200_.html)
    pub fn mqAddInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQINT64,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Add Integer Filter to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `Operator`: Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddIntegerFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089210_.html)
    pub fn mqAddIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQLONG,
        Operator: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Add String to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddString` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089220_.html)
    pub fn mqAddString(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQCHAR,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Add String Filter to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing item value
    /// * `Operator`: Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqAddStringFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089230_.html)
    pub fn mqAddStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQCHAR,
        Operator: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Convert Bag to PCF
    ///
    /// # Arguments
    /// * `OptionsBag`: Handle of options bag
    /// * `DataBag`: Handle of data bag
    /// * `BufferLength`: Length of buffer
    /// * `Buffer` (Output): Buffer to contain PCF
    /// * `DataLength` (Output): Length of PCF returned in buffer
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqBagToBuffer` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089240_.html)
    pub fn mqBagToBuffer(
        OptionsBag: MQHBAG,
        DataBag: MQHBAG,
        BufferLength: MQLONG,
        Buffer: PMQVOID,
        DataLength: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Convert PCF to Bag
    ///
    /// # Arguments
    /// * `OptionsBag`: Handle of options bag
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing PCF
    /// * `DataBag` (Input/Output): Handle of bag to contain data
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqBufferToBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089250_.html)
    pub fn mqBufferToBag(
        OptionsBag: MQHBAG,
        BufferLength: MQLONG,
        Buffer: PMQVOID,
        DataBag: MQHBAG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Delete All Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqClearBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089260_.html)
    pub fn mqClearBag(Bag: MQHBAG, CompCode: &mut MQLONG, Reason: &mut MQLONG);
    /// Count Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemCount` (Output): Number of items
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqCountItems` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089270_.html)
    pub fn mqCountItems(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemCount: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Create Bag
    ///
    /// # Arguments
    /// * `Options`: Bag options
    /// * `Bag` (Output): Handle of bag created
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqCreateBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089280_.html)
    pub fn mqCreateBag(
        Options: MQLONG,
        Bag: &mut MQHBAG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Delete Bag
    ///
    /// # Arguments
    /// * `Bag` (Input/Output): Bag handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqDeleteBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089290_.html)
    pub fn mqDeleteBag(Bag: &mut MQHBAG, CompCode: &mut MQLONG, Reason: &mut MQLONG);
    /// Delete Item in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqDeleteItem` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089300_.html)
    pub fn mqDeleteItem(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Send Admin Command and Receive Reponse
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Command`: Command identifier
    /// * `OptionsBag`: Handle of options bag
    /// * `AdminBag`: Handle of admin bag
    /// * `ResponseBag`: Handle of response bag
    /// * `AdminQ`: Handle of admin queue
    /// * `ResponseQ`: Handle of response queue
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqExecute` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089310_.html)
    pub fn mqExecute(
        Hconn: MQHCONN,
        Command: MQLONG,
        OptionsBag: MQHBAG,
        AdminBag: MQHBAG,
        ResponseBag: MQHBAG,
        AdminQ: MQHOBJ,
        ResponseQ: MQHOBJ,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Receive PCF Message into Bag
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Queue handle
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `GetMsgOpts` (Input/Output): Get-message options
    /// * `Bag` (Input/Output): Handle of bag to contain message
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqGetBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089320_.html)
    pub fn mqGetBag(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        MsgDesc: PMQVOID,
        GetMsgOpts: &mut MQGMO,
        Bag: MQHBAG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire Handle in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue` (Output): Item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089330_.html)
    pub fn mqInquireBag(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: &mut MQHBAG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire Byte String in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `Buffer` (Output): Buffer to contain string
    /// * `ByteStringLength` (Output): Length of byte string returned
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireByteString` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089340_.html)
    pub fn mqInquireByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQBYTE,
        ByteStringLength: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire Byte String Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `Buffer` (Output): Buffer to contain string
    /// * `ByteStringLength` (Output): Length of byte string returned
    /// * `Operator` (Output): Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireByteStringFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089350_.html)
    pub fn mqInquireByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQBYTE,
        ByteStringLength: &mut MQLONG,
        Operator: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire Integer in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue` (Output): Item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireInteger` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089360_.html)
    pub fn mqInquireInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire 64-bit Integer in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue` (Output): Item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireInteger64` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089370_.html)
    pub fn mqInquireInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: &mut MQINT64,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire Integer Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue` (Output): Item value
    /// * `Operator` (Output): Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireIntegerFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089380_.html)
    pub fn mqInquireIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: &mut MQLONG,
        Operator: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire Attributes of Item in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `OutSelector` (Output): Selector of item
    /// * `ItemType` (Output): Data type of item
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireItemInfo` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089390_.html)
    pub fn mqInquireItemInfo(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        OutSelector: &mut MQLONG,
        ItemType: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire String in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `Buffer` (Output): Buffer to contain string
    /// * `StringLength` (Output): Length of string returned
    /// * `CodedCharSetId` (Output): Character-set identifier of string
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireString` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089400_.html)
    pub fn mqInquireString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQCHAR,
        StringLength: &mut MQLONG,
        CodedCharSetId: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Inquire String Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `Buffer` (Output): Buffer to contain string
    /// * `StringLength` (Output): Length of string returned
    /// * `CodedCharSetId` (Output): Character-set identifier of string
    /// * `Operator` (Output): Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqInquireStringFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089410_.html)
    pub fn mqInquireStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQCHAR,
        StringLength: &mut MQLONG,
        CodedCharSetId: &mut MQLONG,
        Operator: &mut MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Pad Null-terminated String with Blanks
    ///
    /// # Arguments
    /// * `String`: Null-terminated string to be padded
    /// * `BufferLength`: Length of buffer
    /// * `Buffer` (Output): Buffer to contain padded string
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqPad` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089420_.html)
    pub fn mqPad(
        String: PMQCHAR,
        BufferLength: MQLONG,
        Buffer: PMQCHAR,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Send Bag as PCF Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Queue handle
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `PutMsgOpts` (Input/Output): Put-message options
    /// * `Bag`: Handle of bag containing message data
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqPutBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089430_.html)
    pub fn mqPutBag(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        MsgDesc: PMQVOID,
        PutMsgOpts: &mut MQPMO,
        Bag: MQHBAG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Modify Byte String in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqSetByteString` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089440_.html)
    pub fn mqSetByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQBYTE,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Modify Byte String Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing item value
    /// * `Operator`: Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqSetByteStringFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089450_.html)
    pub fn mqSetByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQBYTE,
        Operator: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Modify Integer in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue`: Item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqSetInteger` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089460_.html)
    pub fn mqSetInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Modify 64-bit Integer in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue`: Item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqSetInteger64` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089470_.html)
    pub fn mqSetInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQINT64,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Modify Integer Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue`: Item value
    /// * `Operator`: Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqSetIntegerFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089480_.html)
    pub fn mqSetIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQLONG,
        Operator: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Modify String in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing item value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqSetString` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089490_.html)
    pub fn mqSetString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQCHAR,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Modify String Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing item value
    /// * `Operator`: Item operator
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqSetStringFilter` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089500_.html)
    pub fn mqSetStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        Buffer: PMQCHAR,
        Operator: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Replace Trailing Blanks with Null Character
    ///
    /// # Arguments
    /// * `BufferLength`: Length of buffer
    /// * `Buffer`: Buffer containing blank-padded string
    /// * `String` (Output): String with blanks discarded
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqTrim` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089510_.html)
    pub fn mqTrim(
        BufferLength: MQLONG,
        Buffer: PMQCHAR,
        String: PMQCHAR,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
    /// Delete Trailing Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `ItemCount`: Number of items to remain in bag
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `mqTruncateBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089520_.html)
    pub fn mqTruncateBag(
        Bag: MQHBAG,
        ItemCount: MQLONG,
        CompCode: &mut MQLONG,
        Reason: &mut MQLONG,
    );
}
