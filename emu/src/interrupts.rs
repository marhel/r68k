pub trait InterruptController
{
    fn reset_external_devices(&mut self); // triggered by RESET instruction
    fn highest_priority(&self) -> u8;
    fn acknowledge_interrupt(&mut self, priority: u8) -> Option<u8>;
}

#[allow(dead_code)]
pub const UNINITIALIZED_INTERRUPT: u8 = 0x0F;
pub const SPURIOUS_INTERRUPT: u8 = 0x18;
const AUTOVECTOR_BASE: u8 = 0x18;

#[derive(Default)]
pub struct AutoInterruptController {
    level: u8
}
impl AutoInterruptController {
    pub fn new() -> AutoInterruptController {
        AutoInterruptController { level: 0 }
    }

    pub fn request_interrupt(&mut self, irq: u8) -> u8
    {
        assert!(irq > 0 && irq < 8);
        self.level |= 1 << (irq - 1);
        self.level
    }
}
impl InterruptController for AutoInterruptController {
    fn reset_external_devices(&mut self)
    {
        // not a required effect of the RESET instruction, but assuming that
        // any external devices reset, also reset their
        // interrupt request state
        self.level = 0;
    }

    fn highest_priority(&self) -> u8 {
        (8 - self.level.leading_zeros()) as u8
    }

    fn acknowledge_interrupt(&mut self, priority: u8) -> Option<u8> {
        self.level &= !(1 << (priority - 1));
        Some(AUTOVECTOR_BASE + priority)
    }
}


#[cfg(test)]
mod tests {
    use super::{InterruptController, AutoInterruptController,
        AUTOVECTOR_BASE};

    #[test]
    fn keeps_track_of_priority() {
        let mut ctrl = AutoInterruptController { level: 0 };
        ctrl.request_interrupt(2);
        ctrl.request_interrupt(5);
        assert_eq!(5, ctrl.highest_priority());
    }
    #[test]
    fn auto_resets_on_ack() {
        let mut ctrl = AutoInterruptController { level: 0 };
        ctrl.request_interrupt(2);
        ctrl.request_interrupt(5);
        assert_eq!(Some(AUTOVECTOR_BASE + 5), ctrl.acknowledge_interrupt(5));
        assert_eq!(2, ctrl.highest_priority());
    }
    #[test]
    fn resets_irq_level_on_external_device_reset() {
        let mut ctrl = AutoInterruptController { level: 0 };
        ctrl.request_interrupt(2);
        ctrl.request_interrupt(5);
        ctrl.reset_external_devices();
        assert_eq!(0, ctrl.highest_priority());
    }
}