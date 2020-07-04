use heapless::{Vec, consts::U10};

#[derive(Copy, Clone)]
pub enum SpdCmd {
    /// Jump to a fixed speed, in RPM
    FixedSpd(u32),
    /// Ramp to 1. destination speed [RPM] in 2. milliseconds
    Ramp(u32, u32),
    /// Increment the current speed by the specified RPMs
    Inc(u32),
    /// Decrement the current speed by the specified RPMs
    Dec(u32),
}

pub struct SpdMngr {
    cur_spd: u32,
    cmd_list: Vec<SpdCmd, U10>,
    cmd_spd: i32,
    rem_ti: u32,
    ti_res_ms: u32,
    in_ramp: bool,
}

impl SpdMngr {
    pub fn new() -> SpdMngr {
        SpdMngr{
            cur_spd: 0,
            cmd_list: Vec::new(),
            cmd_spd: 0,
            rem_ti: 0,
            ti_res_ms: 10,
            in_ramp: false,
        }
    }

    pub fn add_cmd(&mut self, cmd: SpdCmd) -> Result<(), SpdCmd>{
        self.cmd_list.push(cmd)
    }

    pub fn rst_and_force_cmd(&mut self, cmd: SpdCmd) {
        self.cmd_list.clear();
        let _ = self.cmd_list.push(cmd).ok(); // Ok here, the list has just been cleared...
    }

    /// Returns the speed to be set now and the status of the command from which results this speed. 
    pub fn get_next_speed(&mut self) -> u32 {
        if self.rem_ti == 0 {
            if let Some(nxt) = self.cmd_list.pop() {
                match nxt {
                    SpdCmd::Ramp(dest, ti) => {
                        self.cmd_spd = dest as i32; 
                        self.rem_ti = ti;
                        self.in_ramp = true;
                    },
                    SpdCmd::Inc(inc) => {
                        self.cmd_spd = inc as i32;
                        self.rem_ti = 1;
                        self.in_ramp = false;
                    },
                    SpdCmd::Dec(dec) => {
                        self.cmd_spd = -(dec as i32);
                        self.rem_ti = 1;
                        self.in_ramp = false;
                    },
                    SpdCmd::FixedSpd(dest) => {
                        self.cmd_spd = dest as i32;
                        self.rem_ti = 0;
                        self.in_ramp = false;
                    },
                }
            }
        };

        match self.rem_ti {
            // if the remaining time for ramp is 0, stay at the same speed, do nothing
            _ if self.rem_ti == 0 => {
                self.in_ramp = false;
            },
            // if the remaining time is 1, the speed shall be dec/incremented by the commanded speed
            _ if self.rem_ti == 1 && self.in_ramp == false => {
                let spd_dif = self.cur_spd as i32 + self.cmd_spd;
                self.cur_spd = if spd_dif >= 0 { spd_dif as u32 } else { 0 };
                self.cmd_spd = self.cur_spd as i32;
                self.rem_ti = 0;
                self.in_ramp = false;
            },
            // if the command was a ramp and is not currently finished, then 
            // calculate new current speed based on the remaining time and 
            // the time resolution fixed for the ramps.
            // If the remaining time reaches 0, then the command is not longer
            // a ramp but a fixed speed, as the target speed was reached. 
            _ => {
                let rem_ti = self.rem_ti as i32 - self.ti_res_ms as i32;
                let rem_ti = if rem_ti < 0 { 0 } else { rem_ti as u32 };
                let spd_step = self.cmd_spd / (self.rem_ti / self.ti_res_ms) as i32;
                self.cur_spd = ((self.cur_spd as i32 + spd_step)).abs() as u32;
                self.rem_ti = rem_ti;
                if self.rem_ti == 0 { self.in_ramp = false; }
            },
        };

        self.cur_spd
    }
}