use crate::lib;
mockall::mock! {
    pub Mq {} impl crate ::Exits for Mq { unsafe fn MQXEP < 'b, 'a > (& self, Hconfig :
    lib::MQHCONFIG, ExitReason : lib::MQLONG, Function : lib::MQLONG, pEntryPoint :
    lib::PMQFUNC, pExitOpts : lib::PMQXEPO, pCompCode : & 'a mut lib::MQLONG, pReason : &
    'b mut lib::MQLONG,) {} unsafe fn MQXCLWLN < 'b, 'a > (& self, pExitParms :
    lib::PMQWXP, CurrentRecord : lib::MQPTR, NextOffset : lib::MQLONG, pNextRecord :
    lib::PMQPTR, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {}
    unsafe fn MQXCNVC < 'c, 'b, 'a > (& self, Hconn : lib::MQHCONN, Options :
    lib::MQLONG, SourceCCSID : lib::MQLONG, SourceLength : lib::MQLONG, pSourceBuffer :
    lib::PMQCHAR, TargetCCSID : lib::MQLONG, TargetLength : lib::MQLONG, pTargetBuffer :
    lib::PMQCHAR, pDataLength : & 'a mut lib::MQLONG, pCompCode : & 'b mut lib::MQLONG,
    pReason : & 'c mut lib::MQLONG,) {} unsafe fn MQXDX(& self, pDataConvExitParms :
    lib::PMQDXP, pMsgDesc : lib::PMQMD, InBufferLength : lib::MQLONG, pInBuffer :
    lib::PMQVOID, OutBufferLength : lib::MQLONG, pOutBuffer : lib::PMQVOID,) {} unsafe fn
    MQZEP < 'b, 'a > (& self, Hconfig : lib::MQHCONFIG, Function : lib::MQLONG,
    pEntryPoint : lib::PMQFUNC, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG) {} } impl crate ::Mqai for Mq { unsafe fn mqAddBag < 'b, 'a > (& self,
    Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemValue : lib::MQHBAG, pCompCode : & 'a
    mut lib::MQLONG, pReason : & 'b mut lib::MQLONG) {} unsafe fn mqAddByteString < 'b,
    'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG, BufferLength : lib::MQLONG,
    pBuffer : lib::PMQBYTE, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG,) {} unsafe fn mqAddByteStringFilter < 'b, 'a > (& self, Bag :
    lib::MQHBAG, Selector : lib::MQLONG, BufferLength : lib::MQLONG, pBuffer :
    lib::PMQBYTE, Operator : lib::MQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : &
    'b mut lib::MQLONG,) {} unsafe fn mqAddInquiry < 'b, 'a > (& self, Bag : lib::MQHBAG,
    Selector : lib::MQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG) {} unsafe fn mqAddInteger < 'b, 'a > (& self, Bag : lib::MQHBAG,
    Selector : lib::MQLONG, ItemValue : lib::MQLONG, pCompCode : & 'a mut lib::MQLONG,
    pReason : & 'b mut lib::MQLONG) {} unsafe fn mqAddInteger64 < 'b, 'a > (& self, Bag :
    lib::MQHBAG, Selector : lib::MQLONG, ItemValue : lib::MQINT64, pCompCode : & 'a mut
    lib::MQLONG, pReason : & 'b mut lib::MQLONG) {} unsafe fn mqAddIntegerFilter < 'b, 'a
    > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemValue : lib::MQLONG,
    Operator : lib::MQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG,) {} unsafe fn mqAddString < 'b, 'a > (& self, Bag : lib::MQHBAG,
    Selector : lib::MQLONG, BufferLength : lib::MQLONG, pBuffer : lib::PMQCHAR, pCompCode
    : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn
    mqAddStringFilter < 'b, 'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG,
    BufferLength : lib::MQLONG, pBuffer : lib::PMQCHAR, Operator : lib::MQLONG, pCompCode
    : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn mqBagToBuffer <
    'c, 'b, 'a > (& self, OptionsBag : lib::MQHBAG, DataBag : lib::MQHBAG, BufferLength :
    lib::MQLONG, pBuffer : lib::PMQVOID, pDataLength : & 'a mut lib::MQLONG, pCompCode :
    & 'b mut lib::MQLONG, pReason : & 'c mut lib::MQLONG,) {} unsafe fn mqBufferToBag <
    'b, 'a > (& self, OptionsBag : lib::MQHBAG, BufferLength : lib::MQLONG, pBuffer :
    lib::PMQVOID, DataBag : lib::MQHBAG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b
    mut lib::MQLONG,) {} unsafe fn mqClearBag < 'b, 'a > (& self, Bag : lib::MQHBAG,
    pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG) {} unsafe fn
    mqCountItems < 'b, 'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG,
    pItemCount : lib::PMQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG) {} unsafe fn mqCreateBag < 'b, 'a > (& self, Options : lib::MQLONG, pBag
    : lib::PMQHBAG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG) {}
    unsafe fn mqDeleteBag < 'b, 'a > (& self, pBag : lib::PMQHBAG, pCompCode : & 'a mut
    lib::MQLONG, pReason : & 'b mut lib::MQLONG) {} unsafe fn mqDeleteItem < 'b, 'a > (&
    self, Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemIndex : lib::MQLONG, pCompCode :
    & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG) {} unsafe fn mqExecute < 'b, 'a
    > (& self, Hconn : lib::MQHCONN, Command : lib::MQLONG, OptionsBag : lib::MQHBAG,
    AdminBag : lib::MQHBAG, ResponseBag : lib::MQHBAG, AdminQ : lib::MQHOBJ, ResponseQ :
    lib::MQHOBJ, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {}
    unsafe fn mqGetBag < 'b, 'a > (& self, Hconn : lib::MQHCONN, Hobj : lib::MQHOBJ,
    pMsgDesc : lib::PMQVOID, pGetMsgOpts : lib::PMQVOID, Bag : lib::MQHBAG, pCompCode : &
    'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn mqInquireBag < 'b,
    'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemIndex : lib::MQLONG,
    pItemValue : lib::PMQHBAG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG,) {} unsafe fn mqInquireByteString < 'b, 'a > (& self, Bag : lib::MQHBAG,
    Selector : lib::MQLONG, ItemIndex : lib::MQLONG, BufferLength : lib::MQLONG, pBuffer
    : lib::PMQBYTE, pByteStringLength : lib::PMQLONG, pCompCode : & 'a mut lib::MQLONG,
    pReason : & 'b mut lib::MQLONG,) {} unsafe fn mqInquireByteStringFilter < 'b, 'a > (&
    self, Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemIndex : lib::MQLONG,
    BufferLength : lib::MQLONG, pBuffer : lib::PMQBYTE, pByteStringLength : lib::PMQLONG,
    pOperator : lib::PMQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG,) {} unsafe fn mqInquireInteger < 'b, 'a > (& self, Bag : lib::MQHBAG,
    Selector : lib::MQLONG, ItemIndex : lib::MQLONG, pItemValue : lib::PMQLONG, pCompCode
    : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn
    mqInquireInteger64 < 'b, 'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG,
    ItemIndex : lib::MQLONG, pItemValue : lib::PMQINT64, pCompCode : & 'a mut
    lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn mqInquireIntegerFilter <
    'b, 'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemIndex : lib::MQLONG,
    pItemValue : lib::PMQLONG, pOperator : lib::PMQLONG, pCompCode : & 'a mut
    lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn mqInquireItemInfo < 'b, 'a
    > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemIndex : lib::MQLONG,
    pOutSelector : lib::PMQLONG, pItemType : lib::PMQLONG, pCompCode : & 'a mut
    lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn mqInquireString < 'b, 'a >
    (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemIndex : lib::MQLONG,
    BufferLength : lib::MQLONG, pBuffer : lib::PMQCHAR, pStringLength : lib::PMQLONG,
    pCodedCharSetId : lib::PMQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG,) {} unsafe fn mqInquireStringFilter < 'b, 'a > (& self, Bag :
    lib::MQHBAG, Selector : lib::MQLONG, ItemIndex : lib::MQLONG, BufferLength :
    lib::MQLONG, pBuffer : lib::PMQCHAR, pStringLength : lib::PMQLONG, pCodedCharSetId :
    lib::PMQLONG, pOperator : lib::PMQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : &
    'b mut lib::MQLONG,) {} unsafe fn mqPad < 'b, 'a > (& self, pString : lib::PMQCHAR,
    BufferLength : lib::MQLONG, pBuffer : lib::PMQCHAR, pCompCode : & 'a mut lib::MQLONG,
    pReason : & 'b mut lib::MQLONG) {} unsafe fn mqPutBag < 'b, 'a > (& self, Hconn :
    lib::MQHCONN, Hobj : lib::MQHOBJ, pMsgDesc : lib::PMQVOID, pPutMsgOpts :
    lib::PMQVOID, Bag : lib::MQHBAG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG,) {} unsafe fn mqSetByteString < 'b, 'a > (& self, Bag : lib::MQHBAG,
    Selector : lib::MQLONG, ItemIndex : lib::MQLONG, BufferLength : lib::MQLONG, pBuffer
    : lib::PMQBYTE, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {}
    unsafe fn mqSetByteStringFilter < 'b, 'a > (& self, Bag : lib::MQHBAG, Selector :
    lib::MQLONG, ItemIndex : lib::MQLONG, BufferLength : lib::MQLONG, pBuffer :
    lib::PMQBYTE, Operator : lib::MQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : &
    'b mut lib::MQLONG,) {} unsafe fn mqSetInteger < 'b, 'a > (& self, Bag : lib::MQHBAG,
    Selector : lib::MQLONG, ItemIndex : lib::MQLONG, ItemValue : lib::MQLONG, pCompCode :
    & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn mqSetInteger64 <
    'b, 'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG, ItemIndex : lib::MQLONG,
    ItemValue : lib::MQINT64, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG,) {} unsafe fn mqSetIntegerFilter < 'b, 'a > (& self, Bag : lib::MQHBAG,
    Selector : lib::MQLONG, ItemIndex : lib::MQLONG, ItemValue : lib::MQLONG, Operator :
    lib::MQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {}
    unsafe fn mqSetString < 'b, 'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG,
    ItemIndex : lib::MQLONG, BufferLength : lib::MQLONG, pBuffer : lib::PMQCHAR,
    pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn
    mqSetStringFilter < 'b, 'a > (& self, Bag : lib::MQHBAG, Selector : lib::MQLONG,
    ItemIndex : lib::MQLONG, BufferLength : lib::MQLONG, pBuffer : lib::PMQCHAR, Operator
    : lib::MQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {}
    unsafe fn mqTrim < 'b, 'a > (& self, BufferLength : lib::MQLONG, pBuffer :
    lib::PMQCHAR, pString : lib::PMQCHAR, pCompCode : & 'a mut lib::MQLONG, pReason : &
    'b mut lib::MQLONG) {} unsafe fn mqTruncateBag < 'b, 'a > (& self, Bag : lib::MQHBAG,
    ItemCount : lib::MQLONG, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG) {} } impl crate ::Mqi for Mq { unsafe fn MQBACK < 'b, 'a > (& self,
    Hconn : lib::MQHCONN, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG) {} unsafe fn MQBEGIN < 'c, 'b, 'a > (& self, Hconn : lib::MQHCONN,
    pBeginOptions : Option < & 'a mut lib::MQBO >, pCompCode : & 'b mut lib::MQLONG,
    pReason : & 'c mut lib::MQLONG) {} unsafe fn MQBUFMH < 'd, 'c, 'b, 'a > (& self,
    Hconn : lib::MQHCONN, Hmsg : lib::MQHMSG, pBufMsgHOpts : & 'a lib::MQBMHO, pMsgDesc :
    lib::PMQVOID, BufferLength : lib::MQLONG, pBuffer : lib::PMQVOID, pDataLength : & 'b
    mut lib::MQLONG, pCompCode : & 'c mut lib::MQLONG, pReason : & 'd mut lib::MQLONG,)
    {} unsafe fn MQCB < 'd, 'c, 'b, 'a > (& self, Hconn : lib::MQHCONN, Operation :
    lib::MQLONG, pCallbackDesc : Option < & 'a lib::MQCBD >, Hobj : lib::MQHOBJ, pMsgDesc
    : lib::PMQVOID, pGetMsgOpts : Option < & 'b lib::MQGMO >, pCompCode : & 'c mut
    lib::MQLONG, pReason : & 'd mut lib::MQLONG,) {} unsafe fn MQCLOSE < 'c, 'b, 'a > (&
    self, Hconn : lib::MQHCONN, pHobj : & 'a mut lib::MQHOBJ, Options : lib::MQLONG,
    pCompCode : & 'b mut lib::MQLONG, pReason : & 'c mut lib::MQLONG) {} unsafe fn MQCMIT
    < 'b, 'a > (& self, Hconn : lib::MQHCONN, pCompCode : & 'a mut lib::MQLONG, pReason :
    & 'b mut lib::MQLONG) {} unsafe fn MQCONN < 'd, 'c, 'b, 'a > (& self, pQMgrName : &
    'a lib::MQCHAR48, pHconn : & 'b mut lib::MQHCONN, pCompCode : & 'c mut lib::MQLONG,
    pReason : & 'd mut lib::MQLONG) {} unsafe fn MQCONNX < 'e, 'd, 'c, 'b, 'a > (& self,
    pQMgrName : & 'a lib::MQCHAR48, pConnectOpts : & 'b mut lib::MQCNO, pHconn : & 'c mut
    lib::MQHCONN, pCompCode : & 'd mut lib::MQLONG, pReason : & 'e mut lib::MQLONG) {}
    unsafe fn MQCRTMH < 'c, 'b, 'a > (& self, Hconn : lib::MQHCONN, pCrtMsgHOpts :
    lib::PMQVOID, pHmsg : & 'a mut lib::MQHMSG, pCompCode : & 'b mut lib::MQLONG, pReason
    : & 'c mut lib::MQLONG) {} unsafe fn MQCTL < 'b, 'a > (& self, Hconn : lib::MQHCONN,
    Operation : lib::MQLONG, pControlOpts : lib::PMQVOID, pCompCode : & 'a mut
    lib::MQLONG, pReason : & 'b mut lib::MQLONG) {} unsafe fn MQDISC < 'c, 'b, 'a > (&
    self, pHconn : & 'a mut lib::MQHCONN, pCompCode : & 'b mut lib::MQLONG, pReason : &
    'c mut lib::MQLONG) {} unsafe fn MQDLTMH < 'c, 'b, 'a > (& self, Hconn :
    lib::MQHCONN, pHmsg : & 'a mut lib::MQHMSG, pDltMsgHOpts : lib::PMQVOID, pCompCode :
    & 'b mut lib::MQLONG, pReason : & 'c mut lib::MQLONG) {} unsafe fn MQDLTMP < 'b, 'a >
    (& self, Hconn : lib::MQHCONN, Hmsg : lib::MQHMSG, pDltPropOpts : lib::PMQVOID, pName
    : lib::PMQVOID, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG) {}
    unsafe fn MQGET < 'c, 'b, 'a > (& self, Hconn : lib::MQHCONN, Hobj : lib::MQHOBJ,
    pMsgDesc : lib::PMQVOID, pGetMsgOpts : lib::PMQVOID, BufferLength : lib::MQLONG,
    pBuffer : lib::PMQVOID, pDataLength : & 'a mut lib::MQLONG, pCompCode : & 'b mut
    lib::MQLONG, pReason : & 'c mut lib::MQLONG,) {} unsafe fn MQINQ < 'b, 'a > (& self,
    Hconn : lib::MQHCONN, Hobj : lib::MQHOBJ, SelectorCount : lib::MQLONG, pSelectors :
    lib::PMQLONG, IntAttrCount : lib::MQLONG, pIntAttrs : lib::PMQLONG, CharAttrLength :
    lib::MQLONG, pCharAttrs : lib::PMQCHAR, pCompCode : & 'a mut lib::MQLONG, pReason : &
    'b mut lib::MQLONG,) {} unsafe fn MQINQMP < 'c, 'b, 'a > (& self, Hconn :
    lib::MQHCONN, Hmsg : lib::MQHMSG, pInqPropOpts : lib::PMQVOID, pName : lib::PMQVOID,
    pPropDesc : lib::PMQVOID, pType : lib::PMQLONG, ValueLength : lib::MQLONG, pValue :
    lib::PMQVOID, pDataLength : & 'a mut lib::MQLONG, pCompCode : & 'b mut lib::MQLONG,
    pReason : & 'c mut lib::MQLONG,) {} unsafe fn MQMHBUF < 'c, 'b, 'a > (& self, Hconn :
    lib::MQHCONN, Hmsg : lib::MQHMSG, pMsgHBufOpts : lib::PMQVOID, pName : lib::PMQVOID,
    pMsgDesc : lib::PMQVOID, BufferLength : lib::MQLONG, pBuffer : lib::PMQVOID,
    pDataLength : & 'a mut lib::MQLONG, pCompCode : & 'b mut lib::MQLONG, pReason : & 'c
    mut lib::MQLONG,) {} unsafe fn MQOPEN < 'c, 'b, 'a > (& self, Hconn : lib::MQHCONN,
    pObjDesc : lib::PMQVOID, Options : lib::MQLONG, pHobj : & 'a mut lib::MQHOBJ,
    pCompCode : & 'b mut lib::MQLONG, pReason : & 'c mut lib::MQLONG) {} unsafe fn MQPUT
    < 'b, 'a > (& self, Hconn : lib::MQHCONN, Hobj : lib::MQHOBJ, pMsgDesc :
    lib::PMQVOID, pPutMsgOpts : lib::PMQVOID, BufferLength : lib::MQLONG, pBuffer :
    lib::PMQVOID, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {}
    unsafe fn MQPUT1 < 'b, 'a > (& self, Hconn : lib::MQHCONN, pObjDesc : lib::PMQVOID,
    pMsgDesc : lib::PMQVOID, pPutMsgOpts : lib::PMQVOID, BufferLength : lib::MQLONG,
    pBuffer : lib::PMQVOID, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut
    lib::MQLONG,) {} unsafe fn MQSET < 'b, 'a > (& self, Hconn : lib::MQHCONN, Hobj :
    lib::MQHOBJ, SelectorCount : lib::MQLONG, pSelectors : lib::PMQLONG, IntAttrCount :
    lib::MQLONG, pIntAttrs : lib::PMQLONG, CharAttrLength : lib::MQLONG, pCharAttrs :
    lib::PMQCHAR, pCompCode : & 'a mut lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {}
    unsafe fn MQSETMP < 'b, 'a > (& self, Hconn : lib::MQHCONN, Hmsg : lib::MQHMSG,
    pSetPropOpts : lib::PMQVOID, pName : lib::PMQVOID, pPropDesc : lib::PMQVOID, Type :
    lib::MQLONG, ValueLength : lib::MQLONG, pValue : lib::PMQVOID, pCompCode : & 'a mut
    lib::MQLONG, pReason : & 'b mut lib::MQLONG,) {} unsafe fn MQSTAT < 'b, 'a > (& self,
    Hconn : lib::MQHCONN, Type : lib::MQLONG, pStatus : lib::PMQVOID, pCompCode : & 'a
    mut lib::MQLONG, pReason : & 'b mut lib::MQLONG) {} unsafe fn MQSUB < 'c, 'b, 'a > (&
    self, Hconn : lib::MQHCONN, pSubDesc : lib::PMQVOID, pHobj : Option < & 'a mut
    lib::MQHOBJ >, pHsub : lib::PMQHOBJ, pCompCode : & 'b mut lib::MQLONG, pReason : & 'c
    mut lib::MQLONG) {} unsafe fn MQSUBRQ < 'b, 'a > (& self, Hconn : lib::MQHCONN, Hsub
    : lib::MQHOBJ, Action : lib::MQLONG, pSubRqOpts : lib::PMQVOID, pCompCode : & 'a mut
    lib::MQLONG, pReason : & 'b mut lib::MQLONG) {} }
}
