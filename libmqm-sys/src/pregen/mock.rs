#![allow(warnings)] // reason = no control on generated code

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
            EntryPoint: lib::PMQFUNC,
            ExitOpts: Option<&'a lib::MQXEPO>,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQXCLWLN<'d, 'c, 'b, 'a>(
            &self,
            ExitParms: &'a mut lib::MQWXP,
            CurrentRecord: lib::MQPTR,
            NextOffset: lib::MQLONG,
            NextRecord: &'b mut lib::MQPTR,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQXDX<'a>(
            &self,
            DataConvExitParms: &'a mut lib::MQDXP,
            MsgDesc: lib::PMQMD,
            InBufferLength: lib::MQLONG,
            InBuffer: lib::PMQVOID,
            OutBufferLength: lib::MQLONG,
            OutBuffer: lib::PMQVOID,
        ) {}
        unsafe fn MQZEP<'b, 'a>(
            &self,
            Hconfig: lib::MQHCONFIG,
            Function: lib::MQLONG,
            EntryPoint: lib::PMQFUNC,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
    }
    #[cfg(feature = "mqai")]
    impl crate::Mqai for Mq {
        unsafe fn mqAddBag<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemValue: lib::MQHBAG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddByteString<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQBYTE,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddByteStringFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQBYTE,
            Operator: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddInquiry<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddInteger<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemValue: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddInteger64<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemValue: lib::MQINT64,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddIntegerFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemValue: lib::MQLONG,
            Operator: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddString<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQCHAR,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqAddStringFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQCHAR,
            Operator: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqBagToBuffer<'c, 'b, 'a>(
            &self,
            OptionsBag: lib::MQHBAG,
            DataBag: lib::MQHBAG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQVOID,
            DataLength: &'a mut lib::MQLONG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqBufferToBag<'b, 'a>(
            &self,
            OptionsBag: lib::MQHBAG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQVOID,
            DataBag: lib::MQHBAG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqClearBag<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqCountItems<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemCount: &'a mut lib::MQLONG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqCreateBag<'c, 'b, 'a>(
            &self,
            Options: lib::MQLONG,
            Bag: &'a mut lib::MQHBAG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqDeleteBag<'c, 'b, 'a>(
            &self,
            Bag: &'a mut lib::MQHBAG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqDeleteItem<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
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
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqGetBag<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            MsgDesc: lib::PMQVOID,
            GetMsgOpts: &'a mut lib::MQGMO,
            Bag: lib::MQHBAG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireBag<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: &'a mut lib::MQHBAG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireByteString<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQBYTE,
            ByteStringLength: &'a mut lib::MQLONG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireByteStringFilter<'d, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQBYTE,
            ByteStringLength: &'a mut lib::MQLONG,
            Operator: &'b mut lib::MQLONG,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireInteger<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: &'a mut lib::MQLONG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireInteger64<'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: &'a mut lib::MQINT64,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireIntegerFilter<'d, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: &'a mut lib::MQLONG,
            Operator: &'b mut lib::MQLONG,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireItemInfo<'d, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            OutSelector: &'a mut lib::MQLONG,
            ItemType: &'b mut lib::MQLONG,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireString<'d, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQCHAR,
            StringLength: &'a mut lib::MQLONG,
            CodedCharSetId: &'b mut lib::MQLONG,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn mqInquireStringFilter<'e, 'd, 'c, 'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQCHAR,
            StringLength: &'a mut lib::MQLONG,
            CodedCharSetId: &'b mut lib::MQLONG,
            Operator: &'c mut lib::MQLONG,
            CompCode: &'d mut lib::MQLONG,
            Reason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn mqPad<'b, 'a>(
            &self,
            String: lib::PMQCHAR,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQCHAR,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqPutBag<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            MsgDesc: lib::PMQVOID,
            PutMsgOpts: &'a mut lib::MQPMO,
            Bag: lib::MQHBAG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn mqSetByteString<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQBYTE,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetByteStringFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQBYTE,
            Operator: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetInteger<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetInteger64<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: lib::MQINT64,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetIntegerFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            ItemValue: lib::MQLONG,
            Operator: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetString<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQCHAR,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqSetStringFilter<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            Selector: lib::MQLONG,
            ItemIndex: lib::MQLONG,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQCHAR,
            Operator: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqTrim<'b, 'a>(
            &self,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQCHAR,
            String: lib::PMQCHAR,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn mqTruncateBag<'b, 'a>(
            &self,
            Bag: lib::MQHBAG,
            ItemCount: lib::MQLONG,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
    }
    impl crate::Mqi for Mq {
        unsafe fn MQBACK<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn MQBEGIN<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            BeginOptions: Option<&'a mut lib::MQBO>,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQBUFMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            BufMsgHOpts: &'a lib::MQBMHO,
            MsgDesc: lib::PMQVOID,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQVOID,
            DataLength: &'b mut lib::MQLONG,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQCB<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Operation: lib::MQLONG,
            CallbackDesc: Option<&'a lib::MQCBD>,
            Hobj: lib::MQHOBJ,
            MsgDesc: lib::PMQVOID,
            GetMsgOpts: Option<&'b lib::MQGMO>,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQCLOSE<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: &'a mut lib::MQHOBJ,
            Options: lib::MQLONG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQCMIT<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn MQCONN<'d, 'c, 'b, 'a>(
            &self,
            QMgrName: &'a lib::MQCHAR48,
            Hconn: &'b mut lib::MQHCONN,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQCONNX<'e, 'd, 'c, 'b, 'a>(
            &self,
            QMgrName: &'a lib::MQCHAR48,
            ConnectOpts: &'b mut lib::MQCNO,
            Hconn: &'c mut lib::MQHCONN,
            CompCode: &'d mut lib::MQLONG,
            Reason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn MQCRTMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            CrtMsgHOpts: &'a lib::MQCMHO,
            Hmsg: &'b mut lib::MQHMSG,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQCTL<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Operation: lib::MQLONG,
            ControlOpts: &'a lib::MQCTLO,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQDISC<'c, 'b, 'a>(
            &self,
            Hconn: &'a mut lib::MQHCONN,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQDLTMH<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: &'a mut lib::MQHMSG,
            DltMsgHOpts: &'b lib::MQDMHO,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQDLTMP<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            DltPropOpts: &'a lib::MQDMPO,
            Name: &'b lib::MQCHARV,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQGET<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            MsgDesc: lib::PMQVOID,
            GetMsgOpts: &'a mut lib::MQGMO,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQVOID,
            DataLength: &'b mut lib::MQLONG,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQINQ<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            SelectorCount: lib::MQLONG,
            Selectors: lib::PMQLONG,
            IntAttrCount: lib::MQLONG,
            IntAttrs: lib::PMQLONG,
            CharAttrLength: lib::MQLONG,
            CharAttrs: lib::PMQCHAR,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn MQINQMP<'g, 'f, 'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            InqPropOpts: &'a mut lib::MQIMPO,
            Name: &'b lib::MQCHARV,
            PropDesc: &'c mut lib::MQPD,
            Type: &'d mut lib::MQLONG,
            ValueLength: lib::MQLONG,
            Value: lib::PMQVOID,
            DataLength: &'e mut lib::MQLONG,
            CompCode: &'f mut lib::MQLONG,
            Reason: &'g mut lib::MQLONG,
        ) {}
        unsafe fn MQMHBUF<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            MsgHBufOpts: &'a lib::MQMHBO,
            Name: &'b lib::MQCHARV,
            MsgDesc: lib::PMQVOID,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQVOID,
            DataLength: &'c mut lib::MQLONG,
            CompCode: &'d mut lib::MQLONG,
            Reason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn MQOPEN<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            ObjDesc: &'a mut lib::MQOD,
            Options: lib::MQLONG,
            Hobj: &'b mut lib::MQHOBJ,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQPUT<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            MsgDesc: lib::PMQVOID,
            PutMsgOpts: &'a mut lib::MQPMO,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQVOID,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQPUT1<'d, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            ObjDesc: &'a mut lib::MQOD,
            MsgDesc: lib::PMQVOID,
            PutMsgOpts: &'b mut lib::MQPMO,
            BufferLength: lib::MQLONG,
            Buffer: lib::PMQVOID,
            CompCode: &'c mut lib::MQLONG,
            Reason: &'d mut lib::MQLONG,
        ) {}
        unsafe fn MQSET<'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hobj: lib::MQHOBJ,
            SelectorCount: lib::MQLONG,
            Selectors: lib::PMQLONG,
            IntAttrCount: lib::MQLONG,
            IntAttrs: lib::PMQLONG,
            CharAttrLength: lib::MQLONG,
            CharAttrs: lib::PMQCHAR,
            CompCode: &'a mut lib::MQLONG,
            Reason: &'b mut lib::MQLONG,
        ) {}
        unsafe fn MQSETMP<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hmsg: lib::MQHMSG,
            SetPropOpts: &'a lib::MQSMPO,
            Name: &'b lib::MQCHARV,
            PropDesc: &'c mut lib::MQPD,
            Type: lib::MQLONG,
            ValueLength: lib::MQLONG,
            Value: lib::PMQVOID,
            CompCode: &'d mut lib::MQLONG,
            Reason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn MQSTAT<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Type: lib::MQLONG,
            Status: &'a mut lib::MQSTS,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQSUB<'e, 'd, 'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            SubDesc: &'a mut lib::MQSD,
            Hobj: Option<&'b mut lib::MQHOBJ>,
            Hsub: &'c mut lib::MQHOBJ,
            CompCode: &'d mut lib::MQLONG,
            Reason: &'e mut lib::MQLONG,
        ) {}
        unsafe fn MQSUBRQ<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Hsub: lib::MQHOBJ,
            Action: lib::MQLONG,
            SubRqOpts: Option<&'a mut lib::MQSRO>,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
        unsafe fn MQXCNVC<'c, 'b, 'a>(
            &self,
            Hconn: lib::MQHCONN,
            Options: lib::MQLONG,
            SourceCCSID: lib::MQLONG,
            SourceLength: lib::MQLONG,
            SourceBuffer: lib::PMQCHAR,
            TargetCCSID: lib::MQLONG,
            TargetLength: lib::MQLONG,
            TargetBuffer: lib::PMQCHAR,
            DataLength: &'a mut lib::MQLONG,
            CompCode: &'b mut lib::MQLONG,
            Reason: &'c mut lib::MQLONG,
        ) {}
    }

}
