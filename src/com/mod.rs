use rtt_target::{
    UpChannel,
    DownChannel,
};

pub struct Com {
    req_spd: DownChannel,
    ramp_ti: DownChannel,
    req_cfg: DownChannel,
    act_spd: UpChannel,
    act_cfg: UpChannel,
    old_spd: u16,
    old_cfg: u8,
}

impl Com {
    /// Creates a new `Com` communication structure to read
    /// information from the host.
    pub fn new() -> Com {
        let ch = rtt_target::rtt_init! {
            up: {
                0: {
                    size: 2
                    mode: NoBlockSkip
                    name: "ActSpd"
                }
                1: {
                    size: 1
                    mode: NoBlockSkip
                    name: "ActCfg"
                }
            }
            down: {
                0: {
                    size: 2
                    mode: NoBlockSkip
                    name: "ReqSpd"
                }
                1: {
                    size: 2
                    mode: NoBlockSkip
                    name: "RmpTi"
                }
                2: {
                    size: 1
                    mode: NoBlockSkip
                    name: "ReqCfg"
                }
            }
        };

        Com {
            req_spd: ch.down.0,
            ramp_ti: ch.down.1,
            req_cfg: ch.down.2,
            act_spd: ch.up.0,
            act_cfg: ch.up.1,
            old_spd: 0,
            old_cfg: 0,
        }
    }

    /// Reads a requested speed to setup
    /// 
    /// Arguments
    /// * None
    /// 
    /// Returned
    /// * `u16` - speed in RPM_U16_BIN0 if available
    pub fn get_req_spd(&mut self) -> Option<u16> {
        let mut buf = [0;2];
        if self.req_spd.read(&mut buf) != 2 {
            return None;
        } else {
            return Some((buf[0] as u16) << 8 + (buf[1] as u16));
        }
    }

    /// Reads a configuration identifier to setup
    /// 
    /// Arguments
    /// * None
    /// 
    /// Returned
    /// * `u8` - numerical identifier for a configuration
    pub fn get_req_cfg(&mut self) -> Option<u8> {
        let mut buf = [0;1];
        if self.req_cfg.read(&mut buf) != 1 {
            None
        } else {
            Some(buf[0])
        }
    }

    /// Reads a ramp for the next speed change if available
    /// 
    /// Arguments
    /// * None
    /// 
    /// Returned
    /// * `u16` - time in MSEC_U16_BIN0 if available
    pub fn get_ramp_ti(&mut self) -> Option<u16> {
        let mut buf = [0; 2];
        if self.ramp_ti.read(&mut buf) != 2 {
            None
        } else {
            Some((buf[0] as u16) << 8 + (buf[1] as u16))
        }
    }

    /// Provides currently simulated speed
    /// 
    /// Arguments
    /// * `spd` - speed in RPM_U16_BIN0
    /// 
    /// Returned
    /// * `Ok(())` - speed correctly written to channel
    /// * `Err(())` - speed not written to channel
    pub fn set_act_spd(&mut self, spd: u16) -> Result<(), ()> {
        let mut buf = [0; 2];
        buf[0] = (spd >> 8) as u8;
        buf[1] = (spd & 0xFF) as u8;
        if self.act_spd.write(&buf) != 2 {
            Err(())
        } else {
            Ok(())
        }
    }

    /// Provides actual configuration to host
    ///
    /// Arguments
    /// * `cfg` - numerical identifier of the current configuration
    ///
    /// Returned
    /// * `Ok(())` - configuration correctly written to channel
    /// * `Err(())` - configuration not written to channel
    pub fn set_act_cfg(&mut self, cfg: u8) -> Result<(), ()> {
        let mut buf = [0; 1];
        buf[0] = cfg;
        if self.act_spd.write(&buf) != 1 {
            Err(())
        } else {
            Ok(())
        }
    }
}