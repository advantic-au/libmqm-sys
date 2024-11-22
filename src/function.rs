use crate::lib as mqsys;

/// IBM® MQ Interface (MQI) function calls
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments, non_snake_case)]
pub trait Mqi {
    /// Connects with extended options to a queue manager
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqconnx-connect-queue-manager-extended)
    unsafe fn MQCONNX(
        &self,
        pQMgrName: mqsys::PMQCHAR,
        pConnectOpts: mqsys::PMQCNO,
        pHconn: mqsys::PMQHCONN,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Connects an application to a queue manager
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqconn-connect-queue-manager)
    unsafe fn MQCONN(
        &self,
        pQMgrName: mqsys::PMQCHAR,
        pHconn: mqsys::PMQHCONN,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Disconnects from a queue manager
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdisc-disconnect-queue-manager)
    unsafe fn MQDISC(&self, pHconn: mqsys::PMQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG);

    /// Opens a queue or topic for access
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqopen-open-object)
    unsafe fn MQOPEN(
        &self,
        Hconn: mqsys::MQHCONN,
        pObjDesc: mqsys::PMQVOID,
        Options: mqsys::MQLONG,
        pHobj: mqsys::PMQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Puts a single message on a queue (combines [`MQOPEN`], [`MQPUT`], and [`MQCLOSE`])
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqput1-put-one-message)
    unsafe fn MQPUT1(
        &self,
        Hconn: mqsys::MQHCONN,
        pObjDesc: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Closes a queue or topic
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqclose-close-object)
    unsafe fn MQCLOSE(
        &self,
        Hconn: mqsys::MQHCONN,
        pHobj: mqsys::PMQHOBJ,
        Options: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Commit changes
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcmit-commit-changes)
    unsafe fn MQCMIT(&self, Hconn: mqsys::MQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG);

    /// Gets a message from a queue
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqget-get-message)
    unsafe fn MQGET(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Puts a message on a queue
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqput-put-message)
    unsafe fn MQPUT(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Inquire about object attributes
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinq-inquire-object-attributes)
    unsafe fn MQINQ(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        SelectorCount: mqsys::MQLONG,
        pSelectors: mqsys::PMQLONG,
        IntAttrCount: mqsys::MQLONG,
        pIntAttrs: mqsys::PMQLONG,
        CharAttrLength: mqsys::MQLONG,
        pCharAttrs: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Create a subscription to a topic
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsub-register-subscription)
    unsafe fn MQSUB(
        &self,
        Hconn: mqsys::MQHCONN,
        pSubDesc: mqsys::PMQVOID,
        pHobj: mqsys::PMQHOBJ,
        pHsub: mqsys::PMQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Request a retained publication
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsubrq-subscription-request)
    unsafe fn MQSUBRQ(
        &self,
        Hconn: mqsys::MQHCONN,
        Hsub: mqsys::MQHOBJ,
        Action: mqsys::MQLONG,
        pSubRqOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Begin a unit of work
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbegin-begin-unit-work)
    unsafe fn MQBEGIN(
        &self,
        Hconn: mqsys::MQHCONN,
        pBeginOptions: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Back out changes
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqback-back-out-changes)
    unsafe fn MQBACK(&self, Hconn: mqsys::MQHCONN, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG);

    /// Create a message handle
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcrtmh-create-message-handle)
    unsafe fn MQCRTMH(
        &self,
        Hconn: mqsys::MQHCONN,
        pCrtMsgHOpts: mqsys::PMQVOID,
        pHmsg: mqsys::PMQHMSG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Delete a message handle and is the inverse of the [`MQCRTMH`] call.
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdltmh-delete-message-handle)
    unsafe fn MQDLTMH(
        &self,
        Hconn: mqsys::MQHCONN,
        pHmsg: mqsys::PMQHMSG,
        pDltMsgHOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Convert a message handle into a buffer and is the inverse of the MQBUFMH call
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqmhbuf-convert-message-handle-into-buffer)
    unsafe fn MQMHBUF(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pMsgHBufOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Convert a buffer into a message handle and is the inverse of the MQMHBUF call
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqbufmh-convert-buffer-into-message-handle)
    unsafe fn MQBUFMH(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pBufMsgHOpts: mqsys::PMQVOID,
        pMsgDesc: mqsys::PMQVOID,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Manage callback
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqcb-manage-callback)
    unsafe fn MQCB(
        &self,
        Hconn: mqsys::MQHCONN,
        Operation: mqsys::MQLONG,
        pCallbackDesc: mqsys::PMQVOID,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Perform controlling actions on callbacks and the object handles opened for a connection
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqctl-control-callbacks)
    unsafe fn MQCTL(
        &self,
        Hconn: mqsys::MQHCONN,
        Operation: mqsys::MQLONG,
        pControlOpts: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Set object attributes
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqset-set-object-attributes)
    unsafe fn MQSET(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        SelectorCount: mqsys::MQLONG,
        pSelectors: mqsys::PMQLONG,
        IntAttrCount: mqsys::MQLONG,
        pIntAttrs: mqsys::PMQLONG,
        CharAttrLength: mqsys::MQLONG,
        pCharAttrs: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Set or modify a property of a message handle
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqsetmp-set-message-property)
    unsafe fn MQSETMP(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pSetPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pPropDesc: mqsys::PMQVOID,
        Type: mqsys::MQLONG,
        ValueLength: mqsys::MQLONG,
        pValue: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Retrieve status information
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqstat-retrieve-status-information)
    unsafe fn MQSTAT(
        &self,
        Hconn: mqsys::MQHCONN,
        Type: mqsys::MQLONG,
        pStatus: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Return the value of a property of a message
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqinqmp-inquire-message-property)
    unsafe fn MQINQMP(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pInqPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pPropDesc: mqsys::PMQVOID,
        pType: mqsys::PMQLONG,
        ValueLength: mqsys::MQLONG,
        pValue: mqsys::PMQVOID,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Delete a property from a message handle and is the inverse of the [`MQSETMP`] call
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=calls-mqdltmp-delete-message-property)
    unsafe fn MQDLTMP(
        &self,
        Hconn: mqsys::MQHCONN,
        Hmsg: mqsys::MQHMSG,
        pDltPropOpts: mqsys::PMQVOID,
        pName: mqsys::PMQVOID,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );

    /// Converts characters from one character set to another
    ///
    /// References
    /// * [IBM documentation](https://www.ibm.com/docs/en/ibm-mq/latest?topic=exit-mqxcnvc-convert-characters)
    unsafe fn MQXCNVC(
        &self,
        Hconn: mqsys::MQHCONN,
        Options: mqsys::MQLONG,
        SourceCCSID: mqsys::MQLONG,
        SourceLength: mqsys::MQLONG,
        pSourceBuffer: mqsys::PMQCHAR,
        TargetCCSID: mqsys::MQLONG,
        TargetLength: mqsys::MQLONG,
        pTargetBuffer: mqsys::PMQCHAR,
        pDataLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
}

/// IBM® MQ Administration Interface (MQAI) function calls
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments, non_snake_case)]
#[cfg(feature = "mqai")]
pub trait Mqai {
    unsafe fn mqCreateBag(
        &self,
        Options: mqsys::MQLONG,
        pBag: mqsys::PMQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqClearBag(&self, Bag: mqsys::MQHBAG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG);
    unsafe fn mqDeleteBag(&self, pBag: mqsys::PMQHBAG, pCompCode: mqsys::PMQLONG, pReason: mqsys::PMQLONG);
    unsafe fn mqGetBag(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pGetMsgOpts: mqsys::PMQVOID,
        Bag: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqPutBag(
        &self,
        Hconn: mqsys::MQHCONN,
        Hobj: mqsys::MQHOBJ,
        pMsgDesc: mqsys::PMQVOID,
        pPutMsgOpts: mqsys::PMQVOID,
        Bag: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqTruncateBag(
        &self,
        Bag: mqsys::MQHBAG,
        ItemCount: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddInquiry(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqDeleteItem(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddInteger(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddIntegerFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddInteger64(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddByteString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddByteStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqSetInteger(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqSetIntegerFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQLONG,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqSetInteger64(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        ItemValue: mqsys::MQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqAddBag(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemValue: mqsys::MQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqSetString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqSetStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqSetByteString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqSetByteStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        Operator: mqsys::MQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqInquireInteger(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqInquireIntegerFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqInquireInteger64(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQINT64,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqInquireByteString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pByteStringLength: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqInquireString(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pStringLength: mqsys::PMQLONG,
        pCodedCharSetId: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqInquireStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQCHAR,
        pStringLength: mqsys::PMQLONG,
        pCodedCharSetId: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqInquireByteStringFilter(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        BufferLength: mqsys::MQLONG,
        pBuffer: mqsys::PMQBYTE,
        pByteStringLength: mqsys::PMQLONG,
        pOperator: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqInquireBag(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        ItemIndex: mqsys::MQLONG,
        pItemValue: mqsys::PMQHBAG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqCountItems(
        &self,
        Bag: mqsys::MQHBAG,
        Selector: mqsys::MQLONG,
        pItemCount: mqsys::PMQLONG,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
    unsafe fn mqExecute(
        &self,
        Hconn: mqsys::MQHCONN,
        Command: mqsys::MQLONG,
        OptionsBag: mqsys::MQHBAG,
        AdminBag: mqsys::MQHBAG,
        ResponseBag: mqsys::MQHBAG,
        AdminQ: mqsys::MQHOBJ,
        ResponseQ: mqsys::MQHOBJ,
        pCompCode: mqsys::PMQLONG,
        pReason: mqsys::PMQLONG,
    );
}
