#[cfg(feature = "exits")]
impl crate::Exits for crate::exits::MQIEP {
    unsafe fn MQXEP(
        &self,
        Hconfig: crate::exits::MQHCONFIG,
        ExitReason: crate::MQLONG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        ExitOpts: Option<&crate::exits::MQXEPO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self
                .MQXEP_Call
                .unwrap()(
                Hconfig,
                ExitReason,
                Function,
                EntryPoint,
                ExitOpts,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQXCLWLN(
        &self,
        ExitParms: &mut crate::exits::MQWXP,
        CurrentRecord: crate::MQPTR,
        NextOffset: crate::MQLONG,
        NextRecord: &mut crate::MQPTR,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self
                .MQXCLWLN_Call
                .unwrap()(
                ExitParms,
                CurrentRecord,
                NextOffset,
                NextRecord,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQXDX(
        &self,
        DataConvExitParms: &mut crate::exits::MQDXP,
        MsgDesc: crate::PMQMD,
        InBufferLength: crate::MQLONG,
        InBuffer: crate::PMQVOID,
        OutBufferLength: crate::MQLONG,
        OutBuffer: crate::PMQVOID,
    ) {
        unsafe {
            self
                .MQXDX_Call
                .unwrap()(
                DataConvExitParms,
                MsgDesc,
                InBufferLength,
                InBuffer,
                OutBufferLength,
                OutBuffer,
            );
        }
    }
    unsafe fn MQZEP(
        &self,
        Hconfig: crate::exits::MQHCONFIG,
        Function: crate::MQLONG,
        EntryPoint: crate::PMQFUNC,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQZEP_Call.unwrap()(Hconfig, Function, EntryPoint, CompCode, Reason);
        }
    }
}
impl crate::Mqi for crate::exits::MQIEP {
    unsafe fn MQBACK(
        &self,
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQBACK_Call.unwrap()(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQBEGIN(
        &self,
        Hconn: crate::MQHCONN,
        BeginOptions: Option<&mut crate::MQBO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQBEGIN_Call.unwrap()(Hconn, BeginOptions, CompCode, Reason);
        }
    }
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
    ) {
        unsafe {
            self
                .MQBUFMH_Call
                .unwrap()(
                Hconn,
                Hmsg,
                BufMsgHOpts,
                MsgDesc,
                BufferLength,
                Buffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
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
    ) {
        unsafe {
            self
                .MQCB_Call
                .unwrap()(
                Hconn,
                Operation,
                CallbackDesc,
                Hobj,
                MsgDesc,
                GetMsgOpts,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQCLOSE(
        &self,
        Hconn: crate::MQHCONN,
        Hobj: &mut crate::MQHOBJ,
        Options: crate::MQLONG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQCLOSE_Call.unwrap()(Hconn, Hobj, Options, CompCode, Reason);
        }
    }
    unsafe fn MQCMIT(
        &self,
        Hconn: crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQCMIT_Call.unwrap()(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCONN(
        &self,
        QMgrName: &crate::MQCHAR48,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQCONN_Call.unwrap()(QMgrName, Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCONNX(
        &self,
        QMgrName: &crate::MQCHAR48,
        ConnectOpts: &mut crate::MQCNO,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQCONNX_Call.unwrap()(QMgrName, ConnectOpts, Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQCRTMH(
        &self,
        Hconn: crate::MQHCONN,
        CrtMsgHOpts: &crate::MQCMHO,
        Hmsg: &mut crate::MQHMSG,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQCRTMH_Call.unwrap()(Hconn, CrtMsgHOpts, Hmsg, CompCode, Reason);
        }
    }
    unsafe fn MQCTL(
        &self,
        Hconn: crate::MQHCONN,
        Operation: crate::MQLONG,
        ControlOpts: &crate::MQCTLO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQCTL_Call.unwrap()(Hconn, Operation, ControlOpts, CompCode, Reason);
        }
    }
    unsafe fn MQDISC(
        &self,
        Hconn: &mut crate::MQHCONN,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQDISC_Call.unwrap()(Hconn, CompCode, Reason);
        }
    }
    unsafe fn MQDLTMH(
        &self,
        Hconn: crate::MQHCONN,
        Hmsg: &mut crate::MQHMSG,
        DltMsgHOpts: &crate::MQDMHO,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQDLTMH_Call.unwrap()(Hconn, Hmsg, DltMsgHOpts, CompCode, Reason);
        }
    }
    unsafe fn MQDLTMP(
        &self,
        Hconn: crate::MQHCONN,
        Hmsg: crate::MQHMSG,
        DltPropOpts: &crate::MQDMPO,
        Name: &crate::MQCHARV,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQDLTMP_Call.unwrap()(Hconn, Hmsg, DltPropOpts, Name, CompCode, Reason);
        }
    }
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
    ) {
        unsafe {
            self
                .MQGET_Call
                .unwrap()(
                Hconn,
                Hobj,
                MsgDesc,
                GetMsgOpts,
                BufferLength,
                Buffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
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
    ) {
        unsafe {
            self
                .MQINQ_Call
                .unwrap()(
                Hconn,
                Hobj,
                SelectorCount,
                Selectors,
                IntAttrCount,
                IntAttrs,
                CharAttrLength,
                CharAttrs,
                CompCode,
                Reason,
            );
        }
    }
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
    ) {
        unsafe {
            self
                .MQINQMP_Call
                .unwrap()(
                Hconn,
                Hmsg,
                InqPropOpts,
                Name,
                PropDesc,
                Type,
                ValueLength,
                Value,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
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
    ) {
        unsafe {
            self
                .MQMHBUF_Call
                .unwrap()(
                Hconn,
                Hmsg,
                MsgHBufOpts,
                Name,
                MsgDesc,
                BufferLength,
                Buffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQOPEN(
        &self,
        Hconn: crate::MQHCONN,
        ObjDesc: &mut crate::MQOD,
        Options: crate::MQLONG,
        Hobj: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQOPEN_Call.unwrap()(Hconn, ObjDesc, Options, Hobj, CompCode, Reason);
        }
    }
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
    ) {
        unsafe {
            self
                .MQPUT_Call
                .unwrap()(
                Hconn,
                Hobj,
                MsgDesc,
                PutMsgOpts,
                BufferLength,
                Buffer,
                CompCode,
                Reason,
            );
        }
    }
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
    ) {
        unsafe {
            self
                .MQPUT1_Call
                .unwrap()(
                Hconn,
                ObjDesc,
                MsgDesc,
                PutMsgOpts,
                BufferLength,
                Buffer,
                CompCode,
                Reason,
            );
        }
    }
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
    ) {
        unsafe {
            self
                .MQSET_Call
                .unwrap()(
                Hconn,
                Hobj,
                SelectorCount,
                Selectors,
                IntAttrCount,
                IntAttrs,
                CharAttrLength,
                CharAttrs,
                CompCode,
                Reason,
            );
        }
    }
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
    ) {
        unsafe {
            self
                .MQSETMP_Call
                .unwrap()(
                Hconn,
                Hmsg,
                SetPropOpts,
                Name,
                PropDesc,
                Type,
                ValueLength,
                Value,
                CompCode,
                Reason,
            );
        }
    }
    unsafe fn MQSTAT(
        &self,
        Hconn: crate::MQHCONN,
        Type: crate::MQLONG,
        Status: &mut crate::MQSTS,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQSTAT_Call.unwrap()(Hconn, Type, Status, CompCode, Reason);
        }
    }
    unsafe fn MQSUB(
        &self,
        Hconn: crate::MQHCONN,
        SubDesc: &mut crate::MQSD,
        Hobj: Option<&mut crate::MQHOBJ>,
        Hsub: &mut crate::MQHOBJ,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQSUB_Call.unwrap()(Hconn, SubDesc, Hobj, Hsub, CompCode, Reason);
        }
    }
    unsafe fn MQSUBRQ(
        &self,
        Hconn: crate::MQHCONN,
        Hsub: crate::MQHOBJ,
        Action: crate::MQLONG,
        SubRqOpts: Option<&mut crate::MQSRO>,
        CompCode: &mut crate::MQLONG,
        Reason: &mut crate::MQLONG,
    ) {
        unsafe {
            self.MQSUBRQ_Call.unwrap()(Hconn, Hsub, Action, SubRqOpts, CompCode, Reason);
        }
    }
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
    ) {
        unsafe {
            self
                .MQXCNVC_Call
                .unwrap()(
                Hconn,
                Options,
                SourceCCSID,
                SourceLength,
                SourceBuffer,
                TargetCCSID,
                TargetLength,
                TargetBuffer,
                DataLength,
                CompCode,
                Reason,
            );
        }
    }
}
