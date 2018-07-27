pub const MASK_OUT_X_EA: u32 = 0b1111_0001_1100_0000; // masks out X and Y register bits, plus mode (????xxx???mmmyyy)
pub const MASK_OUT_EA: u32 = 0b1111111111000000;   // masks out Y register bits, plus mode (??????????mmmyyy)

pub const EA_DATA_REGISTER_DIRECT: u16 =      0b1000_0000_0000;
pub const EA_ADDRESS_REGISTER_DIRECT: u16 =   0b0100_0000_0000;
pub const EA_ADDRESS_REGISTER_INDIRECT: u16 = 0b0010_0000_0000;
pub const EA_ARI_POSTINCREMENT: u16 =         0b0001_0000_0000;
pub const EA_ARI_PREDECREMENT: u16 =          0b0000_1000_0000;
pub const EA_ARI_DISPLACEMENT: u16 =          0b0000_0100_0000;
pub const EA_ARI_INDEX: u16 =                 0b0000_0010_0000;
pub const EA_ABSOLUTE_SHORT: u16 =            0b0000_0001_0000;
pub const EA_ABSOLUTE_LONG: u16 =             0b0000_0000_1000;
pub const EA_IMMEDIATE: u16 =                 0b0000_0000_0100;
pub const EA_PC_DISPLACEMENT: u16 =           0b0000_0000_0010;
pub const EA_PC_INDEX: u16 =                  0b0000_0000_0001;

pub const EA_ALL: u16 = 0xfff;
pub const EA_ALL_EXCEPT_AN: u16 = EA_ALL & !EA_ADDRESS_REGISTER_DIRECT;
// despite what is claimed in MC68000PRM section 2.3 EFFECTIVE ADDRESSING MODE SUMMARY,
// Absolute Short and Long modes are in fact alterable, as is evident when looking the allowed
// addressing modes of any instruction stating only alterable (or data/memory alterable)
// such as ADDQ.
pub const EA_ALTERABLE: u16 = EA_DATA_REGISTER_DIRECT
                        | EA_ADDRESS_REGISTER_DIRECT
                        | EA_ADDRESS_REGISTER_INDIRECT
                        | EA_ARI_POSTINCREMENT
                        | EA_ARI_PREDECREMENT
                        | EA_ARI_DISPLACEMENT
                        | EA_ARI_INDEX
                        | EA_ABSOLUTE_SHORT
                        | EA_ABSOLUTE_LONG;
pub const EA_CONTROL: u16 = EA_ADDRESS_REGISTER_INDIRECT
                        | EA_ARI_DISPLACEMENT
                        | EA_ARI_INDEX
                        | EA_ABSOLUTE_SHORT
                        | EA_ABSOLUTE_LONG
                        | EA_PC_DISPLACEMENT
                        | EA_PC_INDEX;
pub const EA_CONTROL_ALTERABLE_OR_PD: u16 = EA_CONTROL & EA_ALTERABLE | EA_ARI_PREDECREMENT;
pub const EA_CONTROL_OR_PI: u16 = EA_CONTROL | EA_ARI_POSTINCREMENT;
pub const EA_DATA: u16 = EA_ALL & !EA_ADDRESS_REGISTER_DIRECT;
pub const EA_DATA_ALTERABLE: u16 = EA_DATA & EA_ALTERABLE;
pub const EA_MEMORY: u16 = EA_ALL & !(EA_DATA_REGISTER_DIRECT | EA_ADDRESS_REGISTER_DIRECT);
pub const EA_MEMORY_ALTERABLE: u16 = EA_MEMORY & EA_ALTERABLE;
#[allow(dead_code)]
pub const EA_NONE: u16 = 0x000;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ea_all_except_an() {
        assert_eq!(EA_ALL_EXCEPT_AN & EA_ADDRESS_REGISTER_DIRECT, 0);
    }
    #[test]
    fn ea_alterable() {
        assert_eq!(EA_ALTERABLE & (EA_IMMEDIATE|EA_PC_DISPLACEMENT|EA_PC_INDEX), 0);
    }
    #[test]
    fn ea_control() {
        assert_eq!(EA_CONTROL, 0x27b);
    }
    #[test]
    fn ea_control_alterable_or_pd() {
        assert_eq!(EA_CONTROL_ALTERABLE_OR_PD & EA_ARI_PREDECREMENT, EA_ARI_PREDECREMENT);
    }
    #[test]
    fn ea_control_or_pi() {
        assert_eq!(EA_CONTROL_OR_PI & EA_ARI_POSTINCREMENT, EA_ARI_POSTINCREMENT);
    }
    #[test]
    fn ea_data() {
        assert_eq!(EA_DATA & (EA_ADDRESS_REGISTER_DIRECT), 0);
    }
    #[test]
    fn ea_data_alterable() {
        assert_eq!(EA_DATA_ALTERABLE, EA_DATA & EA_ALTERABLE);
    }
    #[test]
    fn ea_memory_alterable() {
        assert_eq!(EA_MEMORY_ALTERABLE & (EA_DATA_REGISTER_DIRECT | EA_ADDRESS_REGISTER_DIRECT), 0);
    }
}