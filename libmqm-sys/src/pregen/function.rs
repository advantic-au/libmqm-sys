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
    /// * [IBM `MQXEP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q109660_.html)
    unsafe fn MQXEP(
        &self,
        Hconfig: crate::exits::MQHCONFIG,
        ExitReason: crate::MQLONG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        ExitOpts: Option<&crate::exits::MQXEPO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQXCLWLN` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q082540_.html)
    unsafe fn MQXCLWLN(
        &self,
        ExitParms: &mut crate::exits::MQWXP,
        CurrentRecord: crate::MQPTR,
        NextOffset: crate::MQLONG,
        NextRecord: &mut crate::MQPTR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
        DataConvExitParms: &mut crate::exits::MQDXP,
        MsgDesc: crate::PMQMD,
        InBufferLength: crate::MQLONG,
        InBuffer: crate::PMQVOID,
        OutBufferLength: crate::MQLONG,
        OutBuffer: crate::PMQVOID,
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
    /// * [IBM `MQZEP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q110350_.html)
    unsafe fn MQZEP(
        &self,
        Hconfig: crate::exits::MQHCONFIG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `mqAddBag` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089150_.html)
    unsafe fn mqAddBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqAddByteString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqAddInquiry(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqAddInteger(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqAddInteger64(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemValue: crate::MQLONG,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqAddString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqAddStringFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqBagToBuffer(
        &self,
        OptionsBag: crate::mqai::MQHBAG,
        DataBag: crate::mqai::MQHBAG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataLength: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqBufferToBag(
        &self,
        OptionsBag: crate::mqai::MQHBAG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQVOID,
        DataBag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqClearBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `mqCountItems` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089270_.html)
    unsafe fn mqCountItems(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemCount: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqCreateBag(
        &self,
        Options: crate::MQLONG,
        Bag: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqDeleteBag(
        &self,
        Bag: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `mqDeleteItem` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refadmin/q089300_.html)
    unsafe fn mqDeleteItem(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqGetBag(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        GetMsgOpts: &mut crate::MQGMO,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqInquireBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqInquireInteger(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqInquireInteger64(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqInquireIntegerFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: &mut crate::MQLONG,
        Operator: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqInquireItemInfo(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        OutSelector: &mut crate::MQLONG,
        ItemType: &mut crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqPad(
        &self,
        String: crate::PMQCHAR,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqPutBag(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: crate::MQHOBJ,
        MsgDesc: crate::PMQVOID,
        PutMsgOpts: &mut crate::MQPMO,
        Bag: crate::mqai::MQHBAG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqSetByteString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQBYTE,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqSetInteger(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqSetInteger64(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQINT64,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        ItemValue: crate::MQLONG,
        Operator: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqSetString(
        &self,
        Bag: crate::mqai::MQHBAG,
        Selector: crate::MQLONG,
        ItemIndex: crate::MQLONG,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqTrim(
        &self,
        BufferLength: crate::MQLONG,
        Buffer: crate::PMQCHAR,
        String: crate::PMQCHAR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    unsafe fn mqTruncateBag(
        &self,
        Bag: crate::mqai::MQHBAG,
        ItemCount: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQBACK` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101690_.html)
    unsafe fn MQBACK(
        &self,
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQBEGIN` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101700_.html)
    unsafe fn MQBEGIN(
        &self,
        Hconn: crate::MQHCONN,
        BeginOptions: Option<&mut crate::MQBO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQBUFMH` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101710_.html)
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
    /// * [IBM `MQCB` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101720_.html)
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
    /// * [IBM `MQCLOSE` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101740_.html)
    unsafe fn MQCLOSE(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        Options: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    );
    /// Commit Changes
    ///
    /// # Arguments
    /// * `Hconn`: Connection handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `MQCMIT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101750_.html)
    unsafe fn MQCMIT(
        &self,
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQCONN` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101760_.html)
    unsafe fn MQCONN(
        &self,
        QMgrName: &crate::MQCHAR48,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQCONNX` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101770_.html)
    unsafe fn MQCONNX(
        &self,
        QMgrName: &crate::MQCHAR48,
        ConnectOpts: &mut crate::MQCNO,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQCRTMH` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101780_.html)
    unsafe fn MQCRTMH(
        &self,
        Hconn: crate::MQHCONN,
        CrtMsgHOpts: &crate::MQCMHO,
        Hmsg: &mut crate::MQHMSG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQCTL` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101790_.html)
    unsafe fn MQCTL(
        &self,
        Hconn: crate::MQHCONN,
        Operation: crate::MQLONG,
        ControlOpts: &crate::MQCTLO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    );
    /// Disconnect Queue Manager
    ///
    /// # Arguments
    /// * `Hconn` (Input/Output): Connection handle
    /// * `CompCode` (Output): Completion code
    /// * `Reason` (Output): Reason code qualifying `CompCode`
    ///
    /// # References
    /// * [IBM `MQDISC` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101800_.html)
    unsafe fn MQDISC(
        &self,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQDLTMH` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101810_.html)
    unsafe fn MQDLTMH(
        &self,
        Hconn: crate::MQHCONN,
        Hmsg: &mut crate::MQHMSG,
        DltMsgHOpts: &crate::MQDMHO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQDLTMP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101820_.html)
    unsafe fn MQDLTMP(
        &self,
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        DltPropOpts: &crate::MQDMPO,
        Name: &crate::MQCHARV,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQGET` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101830_.html)
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
    /// * [IBM `MQINQ` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101840_.html)
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
    /// * [IBM `MQINQMP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101850_.html)
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
    /// * [IBM `MQMHBUF` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101860_.html)
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
    /// * [IBM `MQOPEN` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101870_.html)
    unsafe fn MQOPEN(
        &self,
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        Options: crate::MQLONG,
        Hobj: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQPUT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101880_.html)
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
    /// * [IBM `MQPUT1` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101890_.html)
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
    /// * [IBM `MQSET` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101900_.html)
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
    /// * [IBM `MQSETMP` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101910_.html)
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
    /// * [IBM `MQSTAT` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101920_.html)
    unsafe fn MQSTAT(
        &self,
        Hconn: crate::MQHCONN,
        Type: crate::MQLONG,
        Status: &mut crate::MQSTS,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQSUB` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101930_.html)
    unsafe fn MQSUB(
        &self,
        Hconn: crate::MQHCONN,
        SubDesc: &mut crate::MQSD,
        Hobj: Option<&mut crate::MQHOBJ>,
        Hsub: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQSUBRQ` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q101940_.html)
    unsafe fn MQSUBRQ(
        &self,
        Hconn: crate::MQHCONN,
        Hsub: crate::MQHOBJ,
        Action: crate::MQLONG,
        SubRqOpts: Option<&mut crate::MQSRO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
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
    /// * [IBM `MQXCNVC` Documentation](https://www.ibm.com/docs/en/SSFKSJ_latest/refdev/q104110_.html)
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
    );
}
