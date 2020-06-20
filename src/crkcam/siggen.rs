use super::cam;
use super::cmn;
use super::crk;

pub trait CrkCamSigGen {
    fn initialize(&mut self, cam : cam::CamSigGen, crk : crk::CrkSigGen);
    fn set_speed_rpm(&mut self, spd: u32);
    fn set_next_crk_ev(&mut self);
    fn set_next_cam_ev(&mut self);
    fn start(&self);
}
