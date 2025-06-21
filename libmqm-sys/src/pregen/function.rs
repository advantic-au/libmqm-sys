use crate::lib;
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments, non_snake_case)]
#[cfg(feature = "exits")]
pub trait Exits {
    /// Register Entry Point
    ///
    /// # Arguments
    /// * `Hconfig`: Configuration handle
    /// * `ExitReason`: Exit reason
    /// * `Function`: Function identifier
    /// * `pEntryPoint`: Exit function entry point
    /// * `pExitOpts`: Options that control the action of MQXEP
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQXEP(
        &self,
        Hconfig: lib::MQHCONFIG,
        ExitReason: lib::MQLONG,
        Function: lib::MQLONG,
        pEntryPoint: lib::PMQFUNC,
        pExitOpts: Option<&lib::MQXEPO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Cluster Workload Navigate Records
    ///
    /// # Arguments
    /// * `pExitParms` (Input/Output): Exit parameter structure
    /// * `CurrentRecord`: Address of current record
    /// * `NextOffset`: Offset of next record
    /// * `pNextRecord` (Output): Address of next record or structure
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQXCLWLN(
        &self,
        pExitParms: &mut lib::MQWXP,
        CurrentRecord: lib::MQPTR,
        NextOffset: lib::MQLONG,
        pNextRecord: &mut lib::MQPTR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Convert Message Data
    ///
    /// # Arguments
    /// * `pDataConvExitParms` (Input/Output): Data-conversion exit parameter block
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `InBufferLength`: Length in bytes of InBuffer
    /// * `pInBuffer`: Buffer containing the unconverted message
    /// * `OutBufferLength`: Length in bytes of OutBuffer
    /// * `pOutBuffer` (Output): Buffer containing the converted message
    unsafe fn MQXDX(
        &self,
        pDataConvExitParms: &mut lib::MQDXP,
        pMsgDesc: lib::PMQMD,
        InBufferLength: lib::MQLONG,
        pInBuffer: lib::PMQVOID,
        OutBufferLength: lib::MQLONG,
        pOutBuffer: lib::PMQVOID,
    );
    /// Add Component Entry Point
    ///
    /// # Arguments
    /// * `Hconfig`: Configuration handle
    /// * `Function`: Function identifier
    /// * `pEntryPoint`: Function entry point
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQZEP(
        &self,
        Hconfig: lib::MQHCONFIG,
        Function: lib::MQLONG,
        pEntryPoint: lib::PMQFUNC,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
}
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments, non_snake_case)]
#[cfg(feature = "mqai")]
pub trait Mqai {
    /// Add Nested Bag to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqAddBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqAddByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Add an Inquiry Item to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Attribute selector
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqAddInquiry(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Add Integer to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqAddInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Add 64-bit Integer to Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemValue`: Item value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqAddInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqAddString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqAddStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqBagToBuffer(
        &self,
        OptionsBag: lib::MQHBAG,
        DataBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        pDataLength: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqBufferToBag(
        &self,
        OptionsBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQVOID,
        DataBag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Delete All Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqClearBag(
        &self,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Count Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `pItemCount` (Output): Number of items
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqCountItems(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        pItemCount: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Create Bag
    ///
    /// # Arguments
    /// * `Options`: Bag options
    /// * `pBag` (Output): Handle of bag created
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqCreateBag(
        &self,
        Options: lib::MQLONG,
        pBag: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Delete Bag
    ///
    /// # Arguments
    /// * `pBag` (Input/Output): Bag handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqDeleteBag(
        &self,
        pBag: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Delete Item in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `Selector`: Item selector
    /// * `ItemIndex`: Item index
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqDeleteItem(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqGetBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pGetMsgOpts: &mut lib::MQGMO,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqInquireBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqInquireInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqInquireInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqInquireIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pItemValue: &mut lib::MQLONG,
        pOperator: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqInquireItemInfo(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        pOutSelector: &mut lib::MQLONG,
        pItemType: &mut lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    );
    /// Pad Null-terminated String with Blanks
    ///
    /// # Arguments
    /// * `pString`: Null-terminated string to be padded
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer` (Output): Buffer to contain padded string
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqPad(
        &self,
        pString: lib::PMQCHAR,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqPutBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        pMsgDesc: lib::PMQVOID,
        pPutMsgOpts: &mut lib::MQPMO,
        Bag: lib::MQHBAG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqSetByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQBYTE,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqSetInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqSetInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQINT64,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    unsafe fn mqSetString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
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
    );
    /// Replace Trailing Blanks with Null Character
    ///
    /// # Arguments
    /// * `BufferLength`: Length of buffer
    /// * `pBuffer`: Buffer containing blank-padded string
    /// * `pString` (Output): String with blanks discarded
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqTrim(
        &self,
        BufferLength: lib::MQLONG,
        pBuffer: lib::PMQCHAR,
        pString: lib::PMQCHAR,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Delete Trailing Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `ItemCount`: Number of items to remain in bag
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn mqTruncateBag(
        &self,
        Bag: lib::MQHBAG,
        ItemCount: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
}
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments, non_snake_case)]
pub trait Mqi {
    /// Back Out Changes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQBACK(
        &self,
        Hconn: lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Begin Unit of Work
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pBeginOptions` (Input/Output): Options that control the action of MQBEGIN
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQBEGIN(
        &self,
        Hconn: lib::MQHCONN,
        pBeginOptions: Option<&mut lib::MQBO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Buffer To Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pBufMsgHOpts`: Options that control the action of MQBUFMH
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `pBuffer` (Input/Output): Area to contain the message buffer
    /// * `pDataLength` (Output): Length of the output buffer
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Register Message consumer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Operation`: Operation
    /// * `pCallbackDesc`: Callback descriptor
    /// * `Hobj`: Object handle
    /// * `pMsgDesc`: Message Descriptor
    /// * `pGetMsgOpts`: Get options
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Close Object
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pHobj` (Input/Output): Object handle
    /// * `Options`: Options that control the action of MQCLOSE
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQCLOSE(
        &self,
        Hconn: lib::MQHCONN,
        pHobj: &mut lib::MQHOBJ,
        Options: lib::MQLONG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Commit Changes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQCMIT(
        &self,
        Hconn: lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Connect Queue Manager
    ///
    /// # Arguments
    /// * `pQMgrName`: Name of queue manager
    /// * `pHconn` (Output): Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQCONN(
        &self,
        pQMgrName: &lib::MQCHAR48,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Connect Queue Manager (Extended)
    ///
    /// # Arguments
    /// * `pQMgrName`: Name of queue manager
    /// * `pConnectOpts` (Input/Output): Options that control the action of MQCONNX
    /// * `pHconn` (Output): Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQCONNX(
        &self,
        pQMgrName: &lib::MQCHAR48,
        pConnectOpts: &mut lib::MQCNO,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Create Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pCrtMsgHOpts`: Options that control the action of MQCRTMH
    /// * `pHmsg` (Output): Message handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQCRTMH(
        &self,
        Hconn: lib::MQHCONN,
        pCrtMsgHOpts: &lib::MQCMHO,
        pHmsg: &mut lib::MQHMSG,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Control Consumer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Operation`: Operation
    /// * `pControlOpts`: Control options
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQCTL(
        &self,
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        pControlOpts: &lib::MQCTLO,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Disconnect Queue Manager
    ///
    /// # Arguments
    /// * `pHconn` (Input/Output): Connection handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQDISC(
        &self,
        pHconn: &mut lib::MQHCONN,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Delete Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pHmsg` (Input/Output): Message handle
    /// * `pDltMsgHOpts`: Options that control the action of MQDLTMH
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQDLTMH(
        &self,
        Hconn: lib::MQHCONN,
        pHmsg: &mut lib::MQHMSG,
        pDltMsgHOpts: &lib::MQDMHO,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Delete Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pDltPropOpts`: Options that control the action of MQDLTMP
    /// * `pName`: Property name
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQDLTMP(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        pDltPropOpts: &lib::MQDMPO,
        pName: &lib::MQCHARV,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Get Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `pGetMsgOpts` (Input/Output): Options that control the action of MQGET
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `pBuffer` (Output): Area to contain the message data
    /// * `pDataLength` (Output): Length of the message
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Inquire Object Attributes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `SelectorCount`: Count of selectors
    /// * `pSelectors`: Array of attribute selectors
    /// * `IntAttrCount`: Count of integer attributes
    /// * `pIntAttrs` (Output): Array of integer attributes
    /// * `CharAttrLength`: Length of character attributes buffer
    /// * `pCharAttrs` (Output): Character attributes
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Inquire Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pInqPropOpts`: Options that control the action of MQINQMP
    /// * `pName`: Property name
    /// * `pPropDesc` (Output): Property descriptor
    /// * `pType` (Input/Output): Property data type
    /// * `ValueLength`: Length in bytes of the Value area
    /// * `pValue` (Output): Property value
    /// * `pDataLength` (Output): Length of the property value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Message Handle To Buffer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pMsgHBufOpts`: Options that control the action of MQMHBUF
    /// * `pName`: Property name
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `pBuffer` (Output): Area to contain the properties
    /// * `pDataLength` (Output): Length of the properties
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Open Object
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pObjDesc` (Input/Output): Object descriptor
    /// * `Options`: Options that control the action of MQOPEN
    /// * `pHobj` (Output): Object handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQOPEN(
        &self,
        Hconn: lib::MQHCONN,
        pObjDesc: &mut lib::MQOD,
        Options: lib::MQLONG,
        pHobj: &mut lib::MQHOBJ,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Put Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `pPutMsgOpts` (Input/Output): Options that control the action of MQPUT
    /// * `BufferLength`: Length of the message in Buffer
    /// * `pBuffer`: Message data
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Put One Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pObjDesc` (Input/Output): Object descriptor
    /// * `pMsgDesc` (Input/Output): Message descriptor
    /// * `pPutMsgOpts` (Input/Output): Options that control the action of MQPUT1
    /// * `BufferLength`: Length of the message in Buffer
    /// * `pBuffer`: Message data
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Set Object Attributes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `SelectorCount`: Count of selectors
    /// * `pSelectors`: Array of attribute selectors
    /// * `IntAttrCount`: Count of integer attributes
    /// * `pIntAttrs`: Array of integer attributes
    /// * `CharAttrLength`: Length of character attributes buffer
    /// * `pCharAttrs`: Character attributes
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Set Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `pSetPropOpts`: Options that control the action of MQSETMP
    /// * `pName`: Property name
    /// * `pPropDesc` (Input/Output): Property descriptor
    /// * `Type`: Property data type
    /// * `ValueLength`: Length of the Value area
    /// * `pValue`: Property value
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
    /// Get Status Information
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Type`: Status information type
    /// * `pStatus` (Input/Output): Status information
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQSTAT(
        &self,
        Hconn: lib::MQHCONN,
        Type: lib::MQLONG,
        pStatus: &mut lib::MQSTS,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Subscribe to topic
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `pSubDesc` (Input/Output): Subscription descriptor
    /// * `pHobj` (Input/Output): Object handle for queue
    /// * `pHsub` (Output): Subscription object handle
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQSUB(
        &self,
        Hconn: lib::MQHCONN,
        pSubDesc: &mut lib::MQSD,
        pHobj: Option<&mut lib::MQHOBJ>,
        pHsub: &mut lib::MQHOBJ,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Subscription Request
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hsub`: Subscription handle
    /// * `Action`: Action requested on the subscription
    /// * `pSubRqOpts` (Input/Output): Subscription Request Options
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
    unsafe fn MQSUBRQ(
        &self,
        Hconn: lib::MQHCONN,
        Hsub: lib::MQHOBJ,
        Action: lib::MQLONG,
        pSubRqOpts: Option<&mut lib::MQSRO>,
        pCompCode: &mut lib::MQLONG,
        pReason: &mut lib::MQLONG,
    );
    /// Convert Characters
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Options`: Options that control the action of MQXCNVC
    /// * `SourceCCSID`: Coded character set identifier of string before conversion
    /// * `SourceLength`: Length of string before conversion
    /// * `pSourceBuffer`: String to be converted
    /// * `TargetCCSID`: Coded character set identifier of string after conversion
    /// * `TargetLength`: Length of output buffer
    /// * `pTargetBuffer` (Output): String after conversion
    /// * `pDataLength` (Output): Length of output string
    /// * `pCompCode` (Output): Completion code
    /// * `pReason` (Output): Reason code qualifying `CompCode`
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
    );
}
