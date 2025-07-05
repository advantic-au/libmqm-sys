mockall::mock! {
    pub Mq {}
    #[cfg(feature = "exits")]
    impl crate::Exits for Mq {
        unsafe fn MQXEP<'c, 'b, 'a>(
            &self,
            Hconfig: crate::exits::MQHCONFIG,
            ExitReason: crate::MQLONG,
            Function: crate::MQLONG,
            EntryPoint: crate::PMQFUNC,
            ExitOpts: Option<&'a crate::exits::MQXEPO>,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn MQXCLWLN<'d, 'c, 'b, 'a>(
            &self,
            ExitParms: &'a mut crate::exits::MQWXP,
            CurrentRecord: crate::MQPTR,
            NextOffset: crate::MQLONG,
            NextRecord: &'b mut crate::MQPTR,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQXDX<'a>(
            &self,
            DataConvExitParms: &'a mut crate::exits::MQDXP,
            MsgDesc: crate::PMQMD,
            InBufferLength: crate::MQLONG,
            InBuffer: crate::PMQVOID,
            OutBufferLength: crate::MQLONG,
            OutBuffer: crate::PMQVOID,
        ) {}
        unsafe fn MQZEP<'b, 'a>(
            &self,
            Hconfig: crate::exits::MQHCONFIG,
            Function: crate::MQLONG,
            EntryPoint: crate::PMQFUNC,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
    }
    #[cfg(feature = "mqai")]
    impl crate::Mqai for Mq {
        unsafe fn mqAddBag<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemValue: crate::mqai::MQHBAG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqAddByteString<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQBYTE,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqAddByteStringFilter<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQBYTE,
            Operator: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqAddInquiry<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqAddInteger<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemValue: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqAddInteger64<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemValue: crate::MQINT64,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqAddIntegerFilter<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemValue: crate::MQLONG,
            Operator: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqAddString<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQCHAR,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqAddStringFilter<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQCHAR,
            Operator: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqBagToBuffer<'c, 'b, 'a>(
            &self,
            OptionsBag: crate::mqai::MQHBAG,
            DataBag: crate::mqai::MQHBAG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQVOID,
            DataLength: &'a mut crate::MQLONG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqBufferToBag<'b, 'a>(
            &self,
            OptionsBag: crate::mqai::MQHBAG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQVOID,
            DataBag: crate::mqai::MQHBAG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqClearBag<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqCountItems<'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemCount: &'a mut crate::MQLONG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqCreateBag<'c, 'b, 'a>(
            &self,
            Options: crate::MQLONG,
            Bag: &'a mut crate::mqai::MQHBAG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqDeleteBag<'c, 'b, 'a>(
            &self,
            Bag: &'a mut crate::mqai::MQHBAG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqDeleteItem<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqExecute<'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Command: crate::MQLONG,
            OptionsBag: crate::mqai::MQHBAG,
            AdminBag: crate::mqai::MQHBAG,
            ResponseBag: crate::mqai::MQHBAG,
            AdminQ: crate::MQHOBJ,
            ResponseQ: crate::MQHOBJ,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqGetBag<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hobj: crate::MQHOBJ,
            MsgDesc: crate::PMQVOID,
            GetMsgOpts: &'a mut crate::MQGMO,
            Bag: crate::mqai::MQHBAG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireBag<'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            ItemValue: &'a mut crate::mqai::MQHBAG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireByteString<'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQBYTE,
            ByteStringLength: &'a mut crate::MQLONG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireByteStringFilter<'d, 'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQBYTE,
            ByteStringLength: &'a mut crate::MQLONG,
            Operator: &'b mut crate::MQLONG,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireInteger<'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            ItemValue: &'a mut crate::MQLONG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireInteger64<'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            ItemValue: &'a mut crate::MQINT64,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireIntegerFilter<'d, 'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            ItemValue: &'a mut crate::MQLONG,
            Operator: &'b mut crate::MQLONG,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireItemInfo<'d, 'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            OutSelector: &'a mut crate::MQLONG,
            ItemType: &'b mut crate::MQLONG,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireString<'d, 'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQCHAR,
            StringLength: &'a mut crate::MQLONG,
            CodedCharSetId: &'b mut crate::MQLONG,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn mqInquireStringFilter<'e, 'd, 'c, 'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQCHAR,
            StringLength: &'a mut crate::MQLONG,
            CodedCharSetId: &'b mut crate::MQLONG,
            Operator: &'c mut crate::MQLONG,
            CompCode: &'d mut crate::MQLONG,
            Reason: &'e mut crate::MQLONG,
        ) {}
        unsafe fn mqPad<'b, 'a>(
            &self,
            String: crate::PMQCHAR,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQCHAR,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqPutBag<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hobj: crate::MQHOBJ,
            MsgDesc: crate::PMQVOID,
            PutMsgOpts: &'a mut crate::MQPMO,
            Bag: crate::mqai::MQHBAG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn mqSetByteString<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQBYTE,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqSetByteStringFilter<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQBYTE,
            Operator: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqSetInteger<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            ItemValue: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqSetInteger64<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            ItemValue: crate::MQINT64,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqSetIntegerFilter<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            ItemValue: crate::MQLONG,
            Operator: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqSetString<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQCHAR,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqSetStringFilter<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            Selector: crate::MQLONG,
            ItemIndex: crate::MQLONG,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQCHAR,
            Operator: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqTrim<'b, 'a>(
            &self,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQCHAR,
            String: crate::PMQCHAR,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn mqTruncateBag<'b, 'a>(
            &self,
            Bag: crate::mqai::MQHBAG,
            ItemCount: crate::MQLONG,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
    }
    impl crate::Mqi for Mq {
        unsafe fn MQBACK<'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn MQBEGIN<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            BeginOptions: Option<&'a mut crate::MQBO>,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn MQBUFMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hmsg: crate::MQHMSG,
            BufMsgHOpts: &'a crate::MQBMHO,
            MsgDesc: crate::PMQVOID,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQVOID,
            DataLength: &'b mut crate::MQLONG,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQCB<'d, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Operation: crate::MQLONG,
            CallbackDesc: Option<&'a crate::MQCBD>,
            Hobj: crate::MQHOBJ,
            MsgDesc: crate::PMQVOID,
            GetMsgOpts: Option<&'b crate::MQGMO>,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQCLOSE<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hobj: &'a mut crate::MQHOBJ,
            Options: crate::MQLONG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn MQCMIT<'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn MQCONN<'d, 'c, 'b, 'a>(
            &self,
            QMgrName: &'a crate::MQCHAR48,
            Hconn: &'b mut crate::MQHCONN,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQCONNX<'e, 'd, 'c, 'b, 'a>(
            &self,
            QMgrName: &'a crate::MQCHAR48,
            ConnectOpts: &'b mut crate::MQCNO,
            Hconn: &'c mut crate::MQHCONN,
            CompCode: &'d mut crate::MQLONG,
            Reason: &'e mut crate::MQLONG,
        ) {}
        unsafe fn MQCRTMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            CrtMsgHOpts: &'a crate::MQCMHO,
            Hmsg: &'b mut crate::MQHMSG,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQCTL<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Operation: crate::MQLONG,
            ControlOpts: &'a crate::MQCTLO,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn MQDISC<'c, 'b, 'a>(
            &self,
            Hconn: &'a mut crate::MQHCONN,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn MQDLTMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hmsg: &'a mut crate::MQHMSG,
            DltMsgHOpts: &'b crate::MQDMHO,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQDLTMP<'d, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hmsg: crate::MQHMSG,
            DltPropOpts: &'a crate::MQDMPO,
            Name: &'b crate::MQCHARV,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQGET<'d, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hobj: crate::MQHOBJ,
            MsgDesc: crate::PMQVOID,
            GetMsgOpts: &'a mut crate::MQGMO,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQVOID,
            DataLength: &'b mut crate::MQLONG,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQINQ<'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hobj: crate::MQHOBJ,
            SelectorCount: crate::MQLONG,
            Selectors: crate::PMQLONG,
            IntAttrCount: crate::MQLONG,
            IntAttrs: crate::PMQLONG,
            CharAttrLength: crate::MQLONG,
            CharAttrs: crate::PMQCHAR,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn MQINQMP<'g, 'f, 'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hmsg: crate::MQHMSG,
            InqPropOpts: &'a mut crate::MQIMPO,
            Name: &'b crate::MQCHARV,
            PropDesc: &'c mut crate::MQPD,
            Type: &'d mut crate::MQLONG,
            ValueLength: crate::MQLONG,
            Value: crate::PMQVOID,
            DataLength: &'e mut crate::MQLONG,
            CompCode: &'f mut crate::MQLONG,
            Reason: &'g mut crate::MQLONG,
        ) {}
        unsafe fn MQMHBUF<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hmsg: crate::MQHMSG,
            MsgHBufOpts: &'a crate::MQMHBO,
            Name: &'b crate::MQCHARV,
            MsgDesc: crate::PMQVOID,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQVOID,
            DataLength: &'c mut crate::MQLONG,
            CompCode: &'d mut crate::MQLONG,
            Reason: &'e mut crate::MQLONG,
        ) {}
        unsafe fn MQOPEN<'d, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            ObjDesc: &'a mut crate::MQOD,
            Options: crate::MQLONG,
            Hobj: &'b mut crate::MQHOBJ,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQPUT<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hobj: crate::MQHOBJ,
            MsgDesc: crate::PMQVOID,
            PutMsgOpts: &'a mut crate::MQPMO,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQVOID,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn MQPUT1<'d, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            ObjDesc: &'a mut crate::MQOD,
            MsgDesc: crate::PMQVOID,
            PutMsgOpts: &'b mut crate::MQPMO,
            BufferLength: crate::MQLONG,
            Buffer: crate::PMQVOID,
            CompCode: &'c mut crate::MQLONG,
            Reason: &'d mut crate::MQLONG,
        ) {}
        unsafe fn MQSET<'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hobj: crate::MQHOBJ,
            SelectorCount: crate::MQLONG,
            Selectors: crate::PMQLONG,
            IntAttrCount: crate::MQLONG,
            IntAttrs: crate::PMQLONG,
            CharAttrLength: crate::MQLONG,
            CharAttrs: crate::PMQCHAR,
            CompCode: &'a mut crate::MQLONG,
            Reason: &'b mut crate::MQLONG,
        ) {}
        unsafe fn MQSETMP<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hmsg: crate::MQHMSG,
            SetPropOpts: &'a crate::MQSMPO,
            Name: &'b crate::MQCHARV,
            PropDesc: &'c mut crate::MQPD,
            Type: crate::MQLONG,
            ValueLength: crate::MQLONG,
            Value: crate::PMQVOID,
            CompCode: &'d mut crate::MQLONG,
            Reason: &'e mut crate::MQLONG,
        ) {}
        unsafe fn MQSTAT<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Type: crate::MQLONG,
            Status: &'a mut crate::MQSTS,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn MQSUB<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            SubDesc: &'a mut crate::MQSD,
            Hobj: Option<&'b mut crate::MQHOBJ>,
            Hsub: &'c mut crate::MQHOBJ,
            CompCode: &'d mut crate::MQLONG,
            Reason: &'e mut crate::MQLONG,
        ) {}
        unsafe fn MQSUBRQ<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Hsub: crate::MQHOBJ,
            Action: crate::MQLONG,
            SubRqOpts: Option<&'a mut crate::MQSRO>,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
        unsafe fn MQXCNVC<'c, 'b, 'a>(
            &self,
            Hconn: crate::MQHCONN,
            Options: crate::MQLONG,
            SourceCCSID: crate::MQLONG,
            SourceLength: crate::MQLONG,
            SourceBuffer: crate::PMQCHAR,
            TargetCCSID: crate::MQLONG,
            TargetLength: crate::MQLONG,
            TargetBuffer: crate::PMQCHAR,
            DataLength: &'a mut crate::MQLONG,
            CompCode: &'b mut crate::MQLONG,
            Reason: &'c mut crate::MQLONG,
        ) {}
    }

}
