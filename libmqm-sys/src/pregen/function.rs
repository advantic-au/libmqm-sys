use crate::lib;
#[cfg(feature = "exits")]
pub trait Exits {
    /// Register Entry Point
    ///
    /// # Arguments
    /// * `Hconfig`: Configuration handle
    /// * `ExitReason`: Exit reason
    /// * `Function`: Function identifier
    /// * `EntryPoint`: Exit function entry point
    /// * `ExitOpts`: Options that control the action of [`MQXEP`](Self::MQXEP)
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=reference-exit-entry-point-registration-call-mqxep)
    unsafe fn MQXEP(
        &self,
        Hconfig: lib::MQHCONFIG,
        ExitReason: lib::MQLONG,
        Function: lib::MQLONG,
        EntryPoint: lib::PMQFUNC,
        ExitOpts: Option<&lib::MQXEPO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Cluster Workload Navigate Records
    ///
    /// # Arguments
    /// * `ExitParms` (Input/Output): Exit parameter structure
    /// * `CurrentRecord`: Address of current record
    /// * `NextOffset`: Offset of next record
    /// * `NextRecord` (Output): Address of next record or structure
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=structures-mqxclwln-navigate-cluster-workload-records)
    unsafe fn MQXCLWLN(
        &self,
        ExitParms: &mut lib::MQWXP,
        CurrentRecord: lib::MQPTR,
        NextOffset: lib::MQLONG,
        NextRecord: &mut lib::MQPTR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Convert Message Data
    ///
    /// # Arguments
    /// * `DataConvExitParms` (Input/Output): Data-conversion exit parameter block
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `InBufferLength`: Length in bytes of `InBuffer`
    /// * `InBuffer`: Buffer containing the unconverted message
    /// * `OutBufferLength`: Length in bytes of `OutBuffer`
    /// * `OutBuffer` (Output): Buffer containing the converted message
    unsafe fn MQXDX(
        &self,
        DataConvExitParms: &mut lib::MQDXP,
        MsgDesc: lib::PMQMD,
        InBufferLength: lib::MQLONG,
        InBuffer: lib::PMQVOID,
        OutBufferLength: lib::MQLONG,
        OutBuffer: lib::PMQVOID,
    );
    /// Add Component Entry Point
    ///
    /// # Arguments
    /// * `Hconfig`: Configuration handle
    /// * `Function`: Function identifier
    /// * `EntryPoint`: Function entry point
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=information-mqzep-add-component-entry-point)
    unsafe fn MQZEP(
        &self,
        Hconfig: lib::MQHCONFIG,
        Function: lib::MQLONG,
        EntryPoint: lib::PMQFUNC,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
}
#[cfg(feature = "mqai")]
pub trait Mqai {
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbag)
    unsafe fn mqAddBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbytestring)
    unsafe fn mqAddByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddbytestringfilter)
    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinquiry)
    unsafe fn mqAddInquiry(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinteger)
    unsafe fn mqAddInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddinteger64)
    unsafe fn mqAddInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddintegerfilter)
    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddstring)
    unsafe fn mqAddString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqaddstringfilter)
    unsafe fn mqAddStringFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbagtobuffer)
    unsafe fn mqBagToBuffer(
        &self,
        OptionsBag: lib::MQHBAG,
        DataBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataLength: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbuffertobag)
    unsafe fn mqBufferToBag(
        &self,
        OptionsBag: lib::MQHBAG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQVOID,
        DataBag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Delete All Items in Bag
    ///
    /// # Arguments
    /// * `Bag`: Bag handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqclearbag)
    unsafe fn mqClearBag(
        &self,
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcountitems)
    unsafe fn mqCountItems(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemCount: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcreatebag)
    unsafe fn mqCreateBag(
        &self,
        Options: lib::MQLONG,
        Bag: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Delete Bag
    ///
    /// # Arguments
    /// * `Bag` (Input/Output): Bag handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdeletebag)
    unsafe fn mqDeleteBag(
        &self,
        Bag: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdeleteitem)
    unsafe fn mqDeleteItem(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqexecute)
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqgetbag)
    unsafe fn mqGetBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        GetMsgOpts: &mut lib::MQGMO,
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebag)
    unsafe fn mqInquireBag(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebytestring)
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirebytestringfilter)
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireinteger)
    unsafe fn mqInquireInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireinteger64)
    unsafe fn mqInquireInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireintegerfilter)
    unsafe fn mqInquireIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: &mut lib::MQLONG,
        Operator: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquireiteminfo)
    unsafe fn mqInquireItemInfo(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        OutSelector: &mut lib::MQLONG,
        ItemType: &mut lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirestring)
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinquirestringfilter)
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqpad)
    unsafe fn mqPad(
        &self,
        String: lib::PMQCHAR,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqputbag)
    unsafe fn mqPutBag(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: lib::MQHOBJ,
        MsgDesc: lib::PMQVOID,
        PutMsgOpts: &mut lib::MQPMO,
        Bag: lib::MQHBAG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetbytestring)
    unsafe fn mqSetByteString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQBYTE,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetbytestringfilter)
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetinteger)
    unsafe fn mqSetInteger(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetinteger64)
    unsafe fn mqSetInteger64(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQINT64,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetintegerfilter)
    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        ItemValue: lib::MQLONG,
        Operator: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetstring)
    unsafe fn mqSetString(
        &self,
        Bag: lib::MQHBAG,
        Selector: lib::MQLONG,
        ItemIndex: lib::MQLONG,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetstringfilter)
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqtrim)
    unsafe fn mqTrim(
        &self,
        BufferLength: lib::MQLONG,
        Buffer: lib::PMQCHAR,
        String: lib::PMQCHAR,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
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
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqtruncatebag)
    unsafe fn mqTruncateBag(
        &self,
        Bag: lib::MQHBAG,
        ItemCount: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
}
pub trait Mqi {
    /// Back Out Changes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqback-back-out-changes)
    unsafe fn MQBACK(
        &self,
        Hconn: lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Begin Unit of Work
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `BeginOptions` (Input/Output): Options that control the action of [`MQBEGIN`](Self::MQBEGIN)
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbegin-begin-unit-work)
    unsafe fn MQBEGIN(
        &self,
        Hconn: lib::MQHCONN,
        BeginOptions: Option<&mut lib::MQBO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Buffer To Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `BufMsgHOpts`: Options that control the action of [`MQBUFMH`](Self::MQBUFMH)
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `Buffer` (Input/Output): Area to contain the message buffer
    /// * `DataLength` (Output): Length of the output buffer
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbufmh-convert-buffer-into-message-handle)
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
    );
    /// Register Message consumer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Operation`: Operation
    /// * `CallbackDesc`: Callback descriptor
    /// * `Hobj`: Object handle
    /// * `MsgDesc`: Message Descriptor
    /// * `GetMsgOpts`: Get options
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcb-manage-callback)
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
    );
    /// Close Object
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj` (Input/Output): Object handle
    /// * `Options`: Options that control the action of [`MQCLOSE`](Self::MQCLOSE)
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqclose-close-object)
    unsafe fn MQCLOSE(
        &self,
        Hconn: lib::MQHCONN,
        Hobj: &mut lib::MQHOBJ,
        Options: lib::MQLONG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Commit Changes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcmit-commit-changes)
    unsafe fn MQCMIT(
        &self,
        Hconn: lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Connect Queue Manager
    ///
    /// # Arguments
    /// * `QMgrName`: Name of queue manager
    /// * `Hconn` (Output): Connection handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqconn-connect-queue-manager)
    unsafe fn MQCONN(
        &self,
        QMgrName: &lib::MQCHAR48,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Connect Queue Manager (Extended)
    ///
    /// # Arguments
    /// * `QMgrName`: Name of queue manager
    /// * `ConnectOpts` (Input/Output): Options that control the action of [`MQCONNX`](Self::MQCONNX)
    /// * `Hconn` (Output): Connection handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqconnx-connect-queue-manager-extended)
    unsafe fn MQCONNX(
        &self,
        QMgrName: &lib::MQCHAR48,
        ConnectOpts: &mut lib::MQCNO,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Create Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `CrtMsgHOpts`: Options that control the action of [`MQCRTMH`](Self::MQCRTMH)
    /// * `Hmsg` (Output): Message handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcrtmh-create-message-handle)
    unsafe fn MQCRTMH(
        &self,
        Hconn: lib::MQHCONN,
        CrtMsgHOpts: &lib::MQCMHO,
        Hmsg: &mut lib::MQHMSG,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Control Consumer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Operation`: Operation
    /// * `ControlOpts`: Control options
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqctl-control-callbacks)
    unsafe fn MQCTL(
        &self,
        Hconn: lib::MQHCONN,
        Operation: lib::MQLONG,
        ControlOpts: &lib::MQCTLO,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Disconnect Queue Manager
    ///
    /// # Arguments
    /// * `Hconn` (Input/Output): Connection handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdisc-disconnect-queue-manager)
    unsafe fn MQDISC(
        &self,
        Hconn: &mut lib::MQHCONN,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Delete Message Handle
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg` (Input/Output): Message handle
    /// * `DltMsgHOpts`: Options that control the action of [`MQDLTMH`](Self::MQDLTMH)
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdltmh-delete-message-handle)
    unsafe fn MQDLTMH(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: &mut lib::MQHMSG,
        DltMsgHOpts: &lib::MQDMHO,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Delete Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `DltPropOpts`: Options that control the action of [`MQDLTMP`](Self::MQDLTMP)
    /// * `Name`: Property name
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdltmp-delete-message-property)
    unsafe fn MQDLTMP(
        &self,
        Hconn: lib::MQHCONN,
        Hmsg: lib::MQHMSG,
        DltPropOpts: &lib::MQDMPO,
        Name: &lib::MQCHARV,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Get Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `GetMsgOpts` (Input/Output): Options that control the action of [`MQGET`](Self::MQGET)
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `Buffer` (Output): Area to contain the message data
    /// * `DataLength` (Output): Length of the message
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqget-get-message)
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
    );
    /// Inquire Object Attributes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `SelectorCount`: Count of selectors
    /// * `Selectors`: Array of attribute selectors
    /// * `IntAttrCount`: Count of integer attributes
    /// * `IntAttrs` (Output): Array of integer attributes
    /// * `CharAttrLength`: Length of character attributes buffer
    /// * `CharAttrs` (Output): Character attributes
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinq-inquire-object-attributes)
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
    );
    /// Inquire Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `InqPropOpts`: Options that control the action of [`MQINQMP`](Self::MQINQMP)
    /// * `Name`: Property name
    /// * `PropDesc` (Output): Property descriptor
    /// * `Type` (Input/Output): Property data type
    /// * `ValueLength`: Length in bytes of the Value area
    /// * `Value` (Output): Property value
    /// * `DataLength` (Output): Length of the property value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinqmp-inquire-message-property)
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
    );
    /// Message Handle To Buffer
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `MsgHBufOpts`: Options that control the action of [`MQMHBUF`](Self::MQMHBUF)
    /// * `Name`: Property name
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `BufferLength`: Length in bytes of the Buffer area
    /// * `Buffer` (Output): Area to contain the properties
    /// * `DataLength` (Output): Length of the properties
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqmhbuf-convert-message-handle-into-buffer)
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
    );
    /// Open Object
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `ObjDesc` (Input/Output): Object descriptor
    /// * `Options`: Options that control the action of [`MQOPEN`](Self::MQOPEN)
    /// * `Hobj` (Output): Object handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqopen-open-object)
    unsafe fn MQOPEN(
        &self,
        Hconn: lib::MQHCONN,
        ObjDesc: &mut lib::MQOD,
        Options: lib::MQLONG,
        Hobj: &mut lib::MQHOBJ,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Put Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `PutMsgOpts` (Input/Output): Options that control the action of [`MQPUT`](Self::MQPUT)
    /// * `BufferLength`: Length of the message in Buffer
    /// * `Buffer`: Message data
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqput-put-message)
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
    );
    /// Put One Message
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `ObjDesc` (Input/Output): Object descriptor
    /// * `MsgDesc` (Input/Output): Message descriptor
    /// * `PutMsgOpts` (Input/Output): Options that control the action of [`MQPUT1`](Self::MQPUT1)
    /// * `BufferLength`: Length of the message in Buffer
    /// * `Buffer`: Message data
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqput1-put-one-message)
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
    );
    /// Set Object Attributes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hobj`: Object handle
    /// * `SelectorCount`: Count of selectors
    /// * `Selectors`: Array of attribute selectors
    /// * `IntAttrCount`: Count of integer attributes
    /// * `IntAttrs`: Array of integer attributes
    /// * `CharAttrLength`: Length of character attributes buffer
    /// * `CharAttrs`: Character attributes
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqset-set-object-attributes)
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
    );
    /// Set Message Property
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hmsg`: Message handle
    /// * `SetPropOpts`: Options that control the action of [`MQSETMP`](Self::MQSETMP)
    /// * `Name`: Property name
    /// * `PropDesc` (Input/Output): Property descriptor
    /// * `Type`: Property data type
    /// * `ValueLength`: Length of the Value area
    /// * `Value`: Property value
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetmp-set-message-property)
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
    );
    /// Get Status Information
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Type`: Status information type
    /// * `Status` (Input/Output): Status information
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqstat-retrieve-status-information)
    unsafe fn MQSTAT(
        &self,
        Hconn: lib::MQHCONN,
        Type: lib::MQLONG,
        Status: &mut lib::MQSTS,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Subscribe to topic
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `SubDesc` (Input/Output): Subscription descriptor
    /// * `Hobj` (Input/Output): Object handle for queue
    /// * `Hsub` (Output): Subscription object handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsub-register-subscription)
    unsafe fn MQSUB(
        &self,
        Hconn: lib::MQHCONN,
        SubDesc: &mut lib::MQSD,
        Hobj: Option<&mut lib::MQHOBJ>,
        Hsub: &mut lib::MQHOBJ,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Subscription Request
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Hsub`: Subscription handle
    /// * `Action`: Action requested on the subscription
    /// * `SubRqOpts` (Input/Output): Subscription Request Options
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsubrq-subscription-request)
    unsafe fn MQSUBRQ(
        &self,
        Hconn: lib::MQHCONN,
        Hsub: lib::MQHOBJ,
        Action: lib::MQLONG,
        SubRqOpts: Option<&mut lib::MQSRO>,
        CompCode: &mut lib::MQLONG,
        Reason: &mut lib::MQLONG,
    );
    /// Convert Characters
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `Options`: Options that control the action of [`MQXCNVC`](Self::MQXCNVC)
    /// * `SourceCCSID`: Coded character set identifier of string before conversion
    /// * `SourceLength`: Length of string before conversion
    /// * `SourceBuffer`: String to be converted
    /// * `TargetCCSID`: Coded character set identifier of string after conversion
    /// * `TargetLength`: Length of output buffer
    /// * `TargetBuffer` (Output): String after conversion
    /// * `DataLength` (Output): Length of output string
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM Documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=exit-mqxcnvc-convert-characters)
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
    );
}
