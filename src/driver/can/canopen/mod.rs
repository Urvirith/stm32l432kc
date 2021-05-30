/* CANOpen */
/* Standard Industrial Protocol */
/* Uses Include - Robotics, Remote I/O, Safety I/O */
/* General Functions */

pub mod sdo;
pub mod nmt;

/* State Commands For CANOpen */
const BOOTUP:           u8 = 0x00;      // Boot up (Initialising)
const STOPPED:          u8 = 0x04;      // Stopped State
const OPERATIONAL:      u8 = 0x05;      // Operationall State
const PREOPERATION:     u8 = 0x7F;      // Pre-Operational State
const UNKNOWN:          u8 = 0xFF;      // Unknown State

/* Function Codes For CANOpen */
const NMT:              u32 = 0x0000;   // Network Management 
const SYNC:             u32 = 0x0080;   // Synchronization
const EMCY:             u32 = 0x0080;   // Emergency
const TIME:             u32 = 0x0100;   // Timestamp
const TPDO1:            u32 = 0x0180;   // Process Data Object
const RPDO1:            u32 = 0x0200;   // Process Data Object
const TPDO2:            u32 = 0x0280;   // Process Data Object
const RPDO2:            u32 = 0x0300;   // Process Data Object
const TPDO3:            u32 = 0x0380;   // Process Data Object
const RPDO3:            u32 = 0x0400;   // Process Data Object
const TPDO4:            u32 = 0x0480;   // Process Data Object
const RPDO4:            u32 = 0x0500;   // Process Data Object
const TSDO:             u32 = 0x0580;   // Service Data Object
const RSDO:             u32 = 0x0600;   // Service Data Object
const HEARTBEAT:        u32 = 0x0700;   // Node Monitoring (Heartbeat)

/* Function Code And Node Masks */
const NODE_MASK:        u32 = 0x007F;   // Standard ID, Node Mask (Not Extended)
const FC_MASK:          u32 = 0x0780;   // Standard ID, Function Code Mask

pub struct CANOpen {
    node:   u32,                        /* Internal Node Address Set By Program 1 - 127 */
    state:  CanOpenState,               /* Internal State Of The Node */
    toggle: bool                        /* Internal Bit For NMT Node Guarding */
}

#[derive(Clone, Copy)]
pub enum CanOpenState {Bootup, Stopped, Operational, PreOperational, Unknown}

pub fn canopen_state_val(state: CanOpenState) -> u8 {
    return match state {
        CanOpenState::Bootup            => BOOTUP,
        CanOpenState::Stopped           => STOPPED,
        CanOpenState::Operational       => OPERATIONAL,
        CanOpenState::PreOperational    => PREOPERATION,
        CanOpenState::Unknown           => UNKNOWN
    };
}

pub fn canopen_state(state: u8) -> CanOpenState {
    return match state {
        BOOTUP                          =>  CanOpenState::Bootup,
        STOPPED                         =>  CanOpenState::Stopped,
        OPERATIONAL                     =>  CanOpenState::Operational,
        PREOPERATION                    =>  CanOpenState::PreOperational,
        _                               =>  CanOpenState::Unknown
    };
}

impl CANOpen {
    pub fn init(node: u32) -> CANOpen {
        return CANOpen {
            node:   node,
            state:  CanOpenState::Bootup,
            toggle: false
        };
    }

    pub fn get_node(&self) -> u32 {
        return self.node;
    }

    pub fn set_node(&mut self, node: u32) {
        self.node = node;
    }

    pub fn get_state(&self) -> CanOpenState {
        return self.state;
    }

    pub fn set_state(&mut self, state: CanOpenState) {
        self.state = state;
    }

    /* Client Tx, Server Rx */
    pub fn get_rsdo(&self) -> u32 {
        return RSDO;
    }
}
