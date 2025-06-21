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
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbag)
    pub fn mqAddBag(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQHBAG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Add Byte String to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbytestring)
    pub fn mqAddByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Add Byte String Filter to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing item value
    /// * `Operator`: Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbytestringfilter)
    pub fn mqAddByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        Operator: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Add an Inquiry Item to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Attribute selector
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinquiry)
    pub fn mqAddInquiry(
        Bag: MQHBAG,
        Selector: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Add Integer to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinteger)
    pub fn mqAddInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Add 64-bit Integer to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinteger64)
    pub fn mqAddInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQINT64,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Add Integer Filter to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `Operator`: Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddintegerfilter)
    pub fn mqAddIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemValue: MQLONG,
        Operator: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Add String to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddstring)
    pub fn mqAddString(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Add String Filter to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing item value
    /// * `Operator`: Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddstringfilter)
    pub fn mqAddStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        Operator: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Convert Bag to PCF
    ///
    /// # Arguments
    /// * `OptionsBag`: Handle of options bag
    /// * `DataBag`: Handle of data bag
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer` (Output): Buffer to contain PCF
    /// * `pDataLength` (Output): Length of PCF returned in buffer
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbagtobuffer)
    pub fn mqBagToBuffer(
        OptionsBag: MQHBAG,
        DataBag: MQHBAG,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        pDataLength: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Convert PCF to Bag
    ///
    /// # Arguments
    /// * `OptionsBag`: Handle of options bag
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing PCF
    /// * `DataBag` (Input/Output): Handle of bag to contain data
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbuffertobag)
    pub fn mqBufferToBag(
        OptionsBag: MQHBAG,
        BufferLength: MQLONG,
        pBuffer: PMQVOID,
        DataBag: MQHBAG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Delete All Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqclearbag)
    pub fn mqClearBag(Bag: MQHBAG, pCompCode: &mut MQLONG, pReason: &mut MQLONG);
    /// Count Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `pItemCount` (Output): Number of items
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcountitems)
    pub fn mqCountItems(
        Bag: MQHBAG,
        Selector: MQLONG,
        pItemCount: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Create Bag
    ///
    /// # Arguments
    /// * `Options`: Bag options
    /// * `pBag` (Output): Handle of bag created
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcreatebag)
    pub fn mqCreateBag(
        Options: MQLONG,
        pBag: &mut MQHBAG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Delete Bag
    ///
    /// # Arguments
    /// * `pBag` (Input/Output): Bag handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdeletebag)
    pub fn mqDeleteBag(pBag: &mut MQHBAG, pCompCode: &mut MQLONG, pReason: &mut MQLONG);
    /// Delete Item in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdeleteitem)
    pub fn mqDeleteItem(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
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
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqexecute)
    pub fn mqExecute(
        Hconn: MQHCONN,
        Command: MQLONG,
        OptionsBag: MQHBAG,
        AdminBag: MQHBAG,
        ResponseBag: MQHBAG,
        AdminQ: MQHOBJ,
        ResponseQ: MQHOBJ,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Receive PCF Message into Bag
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Queue handle
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `pGetMsgOpts` (Input/Output): Get-message options
    /// * `Bag` (Input/Output): Handle of bag to contain message
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqgetbag)
    pub fn mqGetBag(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pGetMsgOpts: &mut MQGMO,
        Bag: MQHBAG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire Handle in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `pItemValue` (Output): Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebag)
    pub fn mqInquireBag(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: &mut MQHBAG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire Byte String in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer` (Output): Buffer to contain string
    /// * `pByteStringLength` (Output): Length of byte string returned
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebytestring)
    pub fn mqInquireByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pByteStringLength: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire Byte String Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer` (Output): Buffer to contain string
    /// * `pByteStringLength` (Output): Length of byte string returned
    /// * `pOperator` (Output): Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebytestringfilter)
    pub fn mqInquireByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pByteStringLength: &mut MQLONG,
        pOperator: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire Integer in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `pItemValue` (Output): Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireinteger)
    pub fn mqInquireInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire 64-bit Integer in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `pItemValue` (Output): Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireinteger64)
    pub fn mqInquireInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: &mut MQINT64,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire Integer Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `pItemValue` (Output): Item value
    /// * `pOperator` (Output): Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireintegerfilter)
    pub fn mqInquireIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pItemValue: &mut MQLONG,
        pOperator: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire Attributes of Item in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `pOutSelector` (Output): Selector of item
    /// * `pItemType` (Output): Data type of item
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireiteminfo)
    pub fn mqInquireItemInfo(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        pOutSelector: &mut MQLONG,
        pItemType: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire String in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer` (Output): Buffer to contain string
    /// * `pStringLength` (Output): Length of string returned
    /// * `pCodedCharSetId` (Output): Character-set identifier of string
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirestring)
    pub fn mqInquireString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pStringLength: &mut MQLONG,
        pCodedCharSetId: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Inquire String Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer` (Output): Buffer to contain string
    /// * `pStringLength` (Output): Length of string returned
    /// * `pCodedCharSetId` (Output): Character-set identifier of string
    /// * `pOperator` (Output): Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirestringfilter)
    pub fn mqInquireStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pStringLength: &mut MQLONG,
        pCodedCharSetId: &mut MQLONG,
        pOperator: &mut MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Pad Null-terminated String with Blanks
    ///
    /// # Arguments
    /// * `pString`: Null-terminated string to be padded
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer` (Output): Buffer to contain padded string
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqpad)
    pub fn mqPad(
        pString: PMQCHAR,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Send Bag as PCF Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Queue handle
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `pPutMsgOpts` (Input/Output): Put-message options
    /// * `Bag`: Handle of bag containing message data
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqputbag)
    pub fn mqPutBag(
        Hconn: MQHCONN,
        Hobj: MQHOBJ,
        pMsgDesc: PMQVOID,
        pPutMsgOpts: &mut MQPMO,
        Bag: MQHBAG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Modify Byte String in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetbytestring)
    pub fn mqSetByteString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Modify Byte String Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing item value
    /// * `Operator`: Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetbytestringfilter)
    pub fn mqSetByteStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQBYTE,
        Operator: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Modify Integer in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue`: Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetinteger)
    pub fn mqSetInteger(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Modify 64-bit Integer in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue`: Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetinteger64)
    pub fn mqSetInteger64(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQINT64,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Modify Integer Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `ItemValue`: Item value
    /// * `Operator`: Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetintegerfilter)
    pub fn mqSetIntegerFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        ItemValue: MQLONG,
        Operator: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Modify String in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetstring)
    pub fn mqSetString(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Modify String Filter in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing item value
    /// * `Operator`: Item operator
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetstringfilter)
    pub fn mqSetStringFilter(
        Bag: MQHBAG,
        Selector: MQLONG,
        ItemIndex: MQLONG,
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        Operator: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Replace Trailing Blanks with Null Character
    ///
    /// # Arguments
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing blank-padded string
    /// * `pString` (Output): String with blanks discarded
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqtrim)
    pub fn mqTrim(
        BufferLength: MQLONG,
        pBuffer: PMQCHAR,
        pString: PMQCHAR,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
    /// Delete Trailing Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `ItemCount`: Number of items to remain in bag
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqtruncatebag)
    pub fn mqTruncateBag(
        Bag: MQHBAG,
        ItemCount: MQLONG,
        pCompCode: &mut MQLONG,
        pReason: &mut MQLONG,
    );
}
