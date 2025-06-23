use crate::lib;
mockall::mock! {
    pub Mq {}
    #[cfg(feature = "exits")]
    impl crate::Exits for Mq {
        unsafe fn MQXEP<'c, 'b, 'a>(
            &self,
            Hconfig: lib::MQHCONFIG,
            ExitReason: lib::MQLONG,
            Function: lib::MQLONG,
            pEntryPoint: lib::PMQFUNC,
            pExitOpts: Option<&'a lib::MQXEPO>,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQXCLWLN<'d, 'c, 'b, 'a>(
            &self,
            pExitParms: &'a mut lib::MQWXP,
            CurrentRecord: lib::MQPTR,
            NextOffset: lib::MQLONG,
            pNextRecord: &'b mut lib::MQPTR,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQXDX<'a>(
            &self,
            pDataConvExitParms: &'a mut lib::MQDXP,
            pMsgDesc: lib::PMQMD,
            InBufferLength: lib::MQLONG,
            pInBuffer: lib::PMQVOID,
            OutBufferLength: lib::MQLONG,
            pOutBuffer: lib::PMQVOID,
        ) {}
        unsafe fn MQZEP<'b, 'a>(
            &self,
            Hconfig: lib::MQHCONFIG,
            Function: lib::MQLONG,
            pEntryPoint: lib::PMQFUNC,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
    }
    #[cfg(feature = "mqai")]
    impl crate::Mqai for Mq {
        unsafe fn mqAddBag<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemValue: lib::MQHBAG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddByteString<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQBYTE,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddByteStringFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQBYTE,
            Operator: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddInquiry<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddInteger<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemValue: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddInteger64<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemValue: lib::MQINT64,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddIntegerFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemValue: lib::MQLONG,
            Operator: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddString<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQCHAR,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddStringFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQCHAR,
            Operator: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqBagToBuffer<'c, 'b, 'a>(
            &self,
            OptionsBag: lib::MQHBAG,
            DataBag: lib::MQHBAG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQVOID,
            pDataLength: &'a mut lib::MQLONG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqBufferToBag<'b, 'a>(
            &self,
            OptionsBag: lib::MQHBAG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQVOID,
            DataBag: lib::MQHBAG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqClearBag<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqCountItems<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            pItemCount: &'a mut lib::MQLONG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqCreateBag<'c, 'b, 'a>(
            &self,
            Options: lib::MQLONG,
            pBag: &'a mut lib::MQHBAG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqDeleteBag<'c, 'b, 'a>(
            &self,
            pBag: &'a mut lib::MQHBAG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqDeleteItem<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqExecute<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Command: lib::MQLONG,
            OptionsBag: lib::MQHBAG,
            AdminBag: lib::MQHBAG,
            ResponseBag: lib::MQHBAG,
            AdminQ: lib::MQHOBJ,
            ResponseQ: lib::MQHOBJ,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqGetBag<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            pMsgDesc: lib::PMQVOID,
            pGetMsgOpts: &'a mut lib::MQGMO,
            Bag: lib::MQHBAG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireBag<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            pItemValue: &'a mut lib::MQHBAG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireByteString<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQBYTE,
            pByteStringLength: &'a mut lib::MQLONG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireByteStringFilter<'d, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQBYTE,
            pByteStringLength: &'a mut lib::MQLONG,
            pOperator: &'b mut lib::MQLONG,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireInteger<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            pItemValue: &'a mut lib::MQLONG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireInteger64<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            pItemValue: &'a mut lib::MQINT64,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireIntegerFilter<'d, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            pItemValue: &'a mut lib::MQLONG,
            pOperator: &'b mut lib::MQLONG,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireItemInfo<'d, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            pOutSelector: &'a mut lib::MQLONG,
            pItemType: &'b mut lib::MQLONG,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireString<'d, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQCHAR,
            pStringLength: &'a mut lib::MQLONG,
            pCodedCharSetId: &'b mut lib::MQLONG,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireStringFilter<'e, 'd, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQCHAR,
            pStringLength: &'a mut lib::MQLONG,
            pCodedCharSetId: &'b mut lib::MQLONG,
            pOperator: &'c mut lib::MQLONG,
            pCompCode: &'d mut lib::MQLONG,
            pReason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn mqPad<'b, 'a>(
            &self,
            pString: lib::PMQCHAR,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQCHAR,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqPutBag<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            pMsgDesc: lib::PMQVOID,
            pPutMsgOpts: &'a mut lib::MQPMO,
            Bag: lib::MQHBAG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqSetByteString<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQBYTE,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetByteStringFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQBYTE,
            Operator: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetInteger<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetInteger64<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: lib::MQINT64,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetIntegerFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: lib::MQLONG,
            Operator: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetString<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQCHAR,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetStringFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQCHAR,
            Operator: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqTrim<'b, 'a>(
            &self,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQCHAR,
            pString: lib::PMQCHAR,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqTruncateBag<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            ItemCount: lib::MQLONG,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
    }
    impl crate::Mqi for Mq {
        unsafe fn MQBACK<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn MQBEGIN<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pBeginOptions: Option<&'a mut lib::MQBO>,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQBUFMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            pBufMsgHOpts: &'a lib::MQBMHO,
            pMsgDesc: lib::PMQVOID,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQVOID,
            pDataLength: &'b mut lib::MQLONG,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQCB<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Operation: lib::MQLONG,
            pCallbackDesc: Option<&'a lib::MQCBD>,
            Hobj: lib::MQHOBJ,
            pMsgDesc: lib::PMQVOID,
            pGetMsgOpts: Option<&'b lib::MQGMO>,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQCLOSE<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pHobj: &'a mut lib::MQHOBJ,
            Options: lib::MQLONG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQCMIT<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn MQCONN<'d, 'c, 'b, 'a>(
            &self,
            pQMgrName: &'a lib::MQCHAR48,
            pHconn: &'b mut lib::MQHCONN,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQCONNX<'e, 'd, 'c, 'b, 'a>(
            &self,
            pQMgrName: &'a lib::MQCHAR48,
            pConnectOpts: &'b mut lib::MQCNO,
            pHconn: &'c mut lib::MQHCONN,
            pCompCode: &'d mut lib::MQLONG,
            pReason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn MQCRTMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pCrtMsgHOpts: &'a lib::MQCMHO,
            pHmsg: &'b mut lib::MQHMSG,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQCTL<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Operation: lib::MQLONG,
            pControlOpts: &'a lib::MQCTLO,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQDISC<'c, 'b, 'a>(
            &self,
            pHconn: &'a mut lib::MQHCONN,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQDLTMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pHmsg: &'a mut lib::MQHMSG,
            pDltMsgHOpts: &'b lib::MQDMHO,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQDLTMP<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            pDltPropOpts: &'a lib::MQDMPO,
            pName: &'b lib::MQCHARV,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQGET<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            pMsgDesc: lib::PMQVOID,
            pGetMsgOpts: &'a mut lib::MQGMO,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQVOID,
            pDataLength: &'b mut lib::MQLONG,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQINQ<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            SelectorCount: lib::MQLONG,
            pSelectors: lib::PMQLONG,
            IntAttrCount: lib::MQLONG,
            pIntAttrs: lib::PMQLONG,
            CharAttrLength: lib::MQLONG,
            pCharAttrs: lib::PMQCHAR,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn MQINQMP<'g, 'f, 'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            pInqPropOpts: &'a mut lib::MQIMPO,
            pName: &'b lib::MQCHARV,
            pPropDesc: &'c mut lib::MQPD,
            pType: &'d mut lib::MQLONG,
            ValueLength: lib::MQLONG,
            pValue: lib::PMQVOID,
            pDataLength: &'e mut lib::MQLONG,
            pCompCode: &'f mut lib::MQLONG,
            pReason: &'g mut lib::MQLONG,
        ) {}
        unsafe fn MQMHBUF<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            pMsgHBufOpts: &'a lib::MQMHBO,
            pName: &'b lib::MQCHARV,
            pMsgDesc: lib::PMQVOID,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQVOID,
            pDataLength: &'c mut lib::MQLONG,
            pCompCode: &'d mut lib::MQLONG,
            pReason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn MQOPEN<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pObjDesc: &'a mut lib::MQOD,
            Options: lib::MQLONG,
            pHobj: &'b mut lib::MQHOBJ,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQPUT<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            pMsgDesc: lib::PMQVOID,
            pPutMsgOpts: &'a mut lib::MQPMO,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQVOID,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQPUT1<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pObjDesc: &'a mut lib::MQOD,
            pMsgDesc: lib::PMQVOID,
            pPutMsgOpts: &'b mut lib::MQPMO,
            BufferLength: lib::MQLONG,
            pBuffer: lib::PMQVOID,
            pCompCode: &'c mut lib::MQLONG,
            pReason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQSET<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            SelectorCount: lib::MQLONG,
            pSelectors: lib::PMQLONG,
            IntAttrCount: lib::MQLONG,
            pIntAttrs: lib::PMQLONG,
            CharAttrLength: lib::MQLONG,
            pCharAttrs: lib::PMQCHAR,
            pCompCode: &'a mut lib::MQLONG,
            pReason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn MQSETMP<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            pSetPropOpts: &'a lib::MQSMPO,
            pName: &'b lib::MQCHARV,
            pPropDesc: &'c mut lib::MQPD,
            Type: lib::MQLONG,
            ValueLength: lib::MQLONG,
            pValue: lib::PMQVOID,
            pCompCode: &'d mut lib::MQLONG,
            pReason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn MQSTAT<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Type: lib::MQLONG,
            pStatus: &'a mut lib::MQSTS,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQSUB<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            pSubDesc: &'a mut lib::MQSD,
            pHobj: Option<&'b mut lib::MQHOBJ>,
            pHsub: &'c mut lib::MQHOBJ,
            pCompCode: &'d mut lib::MQLONG,
            pReason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn MQSUBRQ<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hsub: lib::MQHOBJ,
            Action: lib::MQLONG,
            pSubRqOpts: Option<&'a mut lib::MQSRO>,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQXCNVC<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Options: lib::MQLONG,
            SourceCCSID: lib::MQLONG,
            SourceLength: lib::MQLONG,
            pSourceBuffer: lib::PMQCHAR,
            TargetCCSID: lib::MQLONG,
            TargetLength: lib::MQLONG,
            pTargetBuffer: lib::PMQCHAR,
            pDataLength: &'a mut lib::MQLONG,
            pCompCode: &'b mut lib::MQLONG,
            pReason: &'c mut lib::MQLONG,
        ) {}
    }

}
