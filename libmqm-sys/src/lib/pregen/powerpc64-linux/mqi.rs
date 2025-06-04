pub type MQCD = tagMQCD;
pub type PMQCD = *mut MQCD;
pub type PPMQCD = *mut PMQCD;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagMQCD {
    pub ChannelName: [MQCHAR; 20usize],
    pub Version: MQLONG,
    pub ChannelType: MQLONG,
    pub TransportType: MQLONG,
    pub Desc: [MQCHAR; 64usize],
    pub QMgrName: [MQCHAR; 48usize],
    pub XmitQName: [MQCHAR; 48usize],
    pub ShortConnectionName: [MQCHAR; 20usize],
    pub MCAName: [MQCHAR; 20usize],
    pub ModeName: [MQCHAR; 8usize],
    pub TpName: [MQCHAR; 64usize],
    pub BatchSize: MQLONG,
    pub DiscInterval: MQLONG,
    pub ShortRetryCount: MQLONG,
    pub ShortRetryInterval: MQLONG,
    pub LongRetryCount: MQLONG,
    pub LongRetryInterval: MQLONG,
    pub SecurityExit: [MQCHAR; 128usize],
    pub MsgExit: [MQCHAR; 128usize],
    pub SendExit: [MQCHAR; 128usize],
    pub ReceiveExit: [MQCHAR; 128usize],
    pub SeqNumberWrap: MQLONG,
    pub MaxMsgLength: MQLONG,
    pub PutAuthority: MQLONG,
    pub DataConversion: MQLONG,
    pub SecurityUserData: [MQCHAR; 32usize],
    pub MsgUserData: [MQCHAR; 32usize],
    pub SendUserData: [MQCHAR; 32usize],
    pub ReceiveUserData: [MQCHAR; 32usize],
    pub UserIdentifier: [MQCHAR; 12usize],
    pub Password: [MQCHAR; 12usize],
    pub MCAUserIdentifier: [MQCHAR; 12usize],
    pub MCAType: MQLONG,
    pub ConnectionName: [MQCHAR; 264usize],
    pub RemoteUserIdentifier: [MQCHAR; 12usize],
    pub RemotePassword: [MQCHAR; 12usize],
    pub MsgRetryExit: [MQCHAR; 128usize],
    pub MsgRetryUserData: [MQCHAR; 32usize],
    pub MsgRetryCount: MQLONG,
    pub MsgRetryInterval: MQLONG,
    pub HeartbeatInterval: MQLONG,
    pub BatchInterval: MQLONG,
    pub NonPersistentMsgSpeed: MQLONG,
    pub StrucLength: MQLONG,
    pub ExitNameLength: MQLONG,
    pub ExitDataLength: MQLONG,
    pub MsgExitsDefined: MQLONG,
    pub SendExitsDefined: MQLONG,
    pub ReceiveExitsDefined: MQLONG,
    pub MsgExitPtr: MQPTR,
    pub MsgUserDataPtr: MQPTR,
    pub SendExitPtr: MQPTR,
    pub SendUserDataPtr: MQPTR,
    pub ReceiveExitPtr: MQPTR,
    pub ReceiveUserDataPtr: MQPTR,
    pub ClusterPtr: MQPTR,
    pub ClustersDefined: MQLONG,
    pub NetworkPriority: MQLONG,
    pub LongMCAUserIdLength: MQLONG,
    pub LongRemoteUserIdLength: MQLONG,
    pub LongMCAUserIdPtr: MQPTR,
    pub LongRemoteUserIdPtr: MQPTR,
    pub MCASecurityId: MQBYTE40,
    pub RemoteSecurityId: MQBYTE40,
    pub SSLCipherSpec: [MQCHAR; 32usize],
    pub SSLPeerNamePtr: MQPTR,
    pub SSLPeerNameLength: MQLONG,
    pub SSLClientAuth: MQLONG,
    pub KeepAliveInterval: MQLONG,
    pub LocalAddress: [MQCHAR; 48usize],
    pub BatchHeartbeat: MQLONG,
    pub HdrCompList: [MQLONG; 2usize],
    pub MsgCompList: [MQLONG; 16usize],
    pub CLWLChannelRank: MQLONG,
    pub CLWLChannelPriority: MQLONG,
    pub CLWLChannelWeight: MQLONG,
    pub ChannelMonitoring: MQLONG,
    pub ChannelStatistics: MQLONG,
    pub SharingConversations: MQLONG,
    pub PropertyControl: MQLONG,
    pub MaxInstances: MQLONG,
    pub MaxInstancesPerClient: MQLONG,
    pub ClientChannelWeight: MQLONG,
    pub ConnectionAffinity: MQLONG,
    pub BatchDataLimit: MQLONG,
    pub UseDLQ: MQLONG,
    pub DefReconnect: MQLONG,
    pub CertificateLabel: [MQCHAR; 64usize],
    pub SPLProtection: MQLONG,
}
pub const MQCD_VERSION_1: MQLONG = 1;
pub const MQCD_VERSION_2: MQLONG = 2;
pub const MQCD_VERSION_3: MQLONG = 3;
pub const MQCD_VERSION_4: MQLONG = 4;
pub const MQCD_VERSION_5: MQLONG = 5;
pub const MQCD_VERSION_6: MQLONG = 6;
pub const MQCD_VERSION_7: MQLONG = 7;
pub const MQCD_VERSION_8: MQLONG = 8;
pub const MQCD_VERSION_9: MQLONG = 9;
pub const MQCD_VERSION_10: MQLONG = 10;
pub const MQCD_VERSION_11: MQLONG = 11;
pub const MQCD_VERSION_12: MQLONG = 12;
pub const MQCD_CURRENT_VERSION: MQLONG = 12;
pub const MQCD_LENGTH_1: usize = 984;
pub const MQCD_LENGTH_2: usize = 1312;
pub const MQCD_LENGTH_3: usize = 1480;
pub const MQCD_LENGTH_4: usize = 1568;
pub const MQCD_LENGTH_5: usize = 1584;
pub const MQCD_LENGTH_6: usize = 1688;
pub const MQCD_LENGTH_7: usize = 1792;
pub const MQCD_LENGTH_8: usize = 1888;
pub const MQCD_LENGTH_9: usize = 1912;
pub const MQCD_LENGTH_10: usize = 1920;
pub const MQCD_LENGTH_11: usize = 1984;
pub const MQCD_LENGTH_12: usize = 1992;
pub const MQCD_CURRENT_LENGTH: usize = 1992;
