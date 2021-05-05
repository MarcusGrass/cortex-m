//! Trace Port Interface Unit;
//!
//! *NOTE* Not available on Armv6-M.

use volatile_register::{RO, RW, WO};

use crate::peripheral::TPIU;
use bitfield::bitfield;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Supported Parallel Port Sizes
    pub sspsr: RO<u32>,
    /// Current Parallel Port Size
    pub cspsr: RW<u32>,
    reserved0: [u32; 2],
    /// Asynchronous Clock Prescaler
    pub acpr: RW<u32>,
    reserved1: [u32; 55],
    /// Selected Pin Control
    pub sppr: RW<u32>,
    reserved2: [u32; 132],
    /// Formatter and Flush Control
    pub ffcr: RW<Ffcr>,
    reserved3: [u32; 810],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
    reserved4: [u32; 4],
    /// TPIU Type
    pub _type: RO<Type>,
}

bitfield! {
    /// Formatter and flush control register.
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Ffcr(u32);
    get_enfcont, set_enfcont: 1;
}

bitfield! {
    /// Type Register.
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Type(u32);
    u8, fifosz, _: 8, 6;
    ptinvalid, _: 9;
    mancvalid, _: 10;
    nrzvalid, _: 11;
}

/// The available protocols for the trace output.
pub enum TraceProtocol {
    /// Parallel trace port mode
    Parallel = 0b00,
    /// Asynchronous SWO, using Manchester encoding
    AsyncSWOManchester = 0b01,
    /// Asynchronous SWO, using NRZ encoding
    AsyncSWONRZ = 0b10,
}

/// The SWO options supported by the TPIU.
#[allow(dead_code)]
pub struct SWOSupports {
    /// Whether UART/NRZ encoding is supported for SWO.
    nrz_encoding: bool,
    /// Whether Manchester encoding is supported for SWO.
    manchester_encoding: bool,
    /// Whether parallel trace port operation is supported.
    parallel_operation: bool,
    /// The minimum implemented FIFO queue size of the TPIU for trace data.
    min_queue_size: u8,
}

impl TPIU {
    /// Sets the prescaler value for a wanted baud rate of the Serial
    /// Wire Output (SWO) in relation to a given asynchronous refernce
    /// clock rate.
    #[inline]
    pub fn set_swo_baud_rate(&mut self, ref_clk_rate: u32, baud_rate: u32) {
        unsafe {
            self.acpr.write((ref_clk_rate / baud_rate) - 1);
        }
    }

    /// Sets the used protocol for the trace output.
    #[inline]
    pub fn set_trace_output_protocol(&mut self, proto: TraceProtocol) {
        unsafe { self.sppr.write(proto as u32) }
    }

    /// Whether to enable the formatter. If disabled, only ITM and DWT
    /// trace sources are passed through. Data from the ETM is
    /// discarded.
    #[inline]
    pub fn enable_continuous_formatting(&mut self, bit: bool) {
        unsafe {
            self.ffcr.modify(|mut r| {
                r.set_enfcont(bit);
                r
            });
        }
    }

    /// Reads the supported trace output modes and the minimum size of
    /// the TPIU FIFO queue for trace data.
    #[inline]
    pub fn get_swo_supports() -> SWOSupports {
        let _type = unsafe { (*Self::ptr())._type.read() };
        SWOSupports {
            nrz_encoding: _type.nrzvalid(),
            manchester_encoding: _type.mancvalid(),
            parallel_operation: !_type.ptinvalid(),
            min_queue_size: _type.fifosz(),
        }
    }
}
