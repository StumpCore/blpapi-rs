use crate::core::BLPAPI_DEFAULT_SEATTYPE_BPS;
use crate::service::Service;
use crate::Error;
use crate::{core::BLPAPI_DEFAULT_SEATTYPE_NONBPS, element::Element};
use blpapi_sys::{
    blpapi_Identity_getSeatType, blpapi_Identity_hasEntitlements, blpapi_Identity_isAuthorized,
    blpapi_Identity_release, blpapi_Identity_t, BLPAPI_SEATTYPE_INVALID_SEAT,
};
use core::ffi::c_int;
use std::{iter::FromFn, ptr};

pub enum SeatType {
    InvalidSeat,
    Bps,
    NonBps,
    Unknown,
}

impl From<c_int> for SeatType {
    fn from(arg: c_int) -> Self {
        match arg {
            BLPAPI_SEATTYPE_INVALID_SEAT => SeatType::InvalidSeat,
            BLPAPI_DEFAULT_SEATTYPE_NONBPS => SeatType::NonBps,
            BLPAPI_DEFAULT_SEATTYPE_BPS => SeatType::Bps,
            _ => SeatType::Unknown,
        }
    }
}

impl From<SeatType> for c_int {
    fn from(arg: SeatType) -> Self {
        match arg {
            SeatType::InvalidSeat => BLPAPI_SEATTYPE_INVALID_SEAT,
            SeatType::Bps => BLPAPI_DEFAULT_SEATTYPE_BPS,
            SeatType::NonBps => BLPAPI_DEFAULT_SEATTYPE_NONBPS,
            SeatType::Unknown => 112,
        }
    }
}

#[derive(Default)]
pub struct IdentityBuilder {
    ptr: Option<*mut blpapi_Identity_t>,
    valid: Option<bool>,
    seat_type: Option<SeatType>,
}

impl IdentityBuilder {
    pub fn ptr(mut self, ptr: *mut blpapi_Identity_t) -> Self {
        self.ptr = Some(ptr);
        self
    }

    pub fn valid(mut self, valid: bool) -> Self {
        self.valid = Some(valid);
        self
    }

    pub fn seat_type(mut self, seat_type: SeatType) -> Self {
        self.seat_type = Some(seat_type);
        self
    }

    pub fn build(self) -> Result<Identity, Error> {
        let ptr = self.ptr.expect("Set pointer first.");
        let valid = self.valid.expect("Set if valid first.");
        let seat_type = self.seat_type.expect("Set SeatType first.");
        match seat_type {
            SeatType::Unknown => Err(Error::Identity),
            _ => Ok(Identity {
                ptr,
                valid,
                seat_type: seat_type.into(),
            }),
        }
    }
}

pub struct Identity {
    pub(crate) ptr: *mut blpapi_Identity_t,
    pub valid: bool,
    pub seat_type: i32,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            ptr: ptr::null_mut(),
            valid: false,
            seat_type: BLPAPI_SEATTYPE_INVALID_SEAT,
        }
    }
}

impl Drop for Identity {
    fn drop(&mut self) {
        unsafe {
            blpapi_Identity_release(self.ptr);
        }
        self.valid = false;
    }
}

impl Clone for Identity {
    fn clone(&self) -> Self {
        let id_ = self.ptr;
        Identity {
            ptr: id_,
            valid: self.valid,
            seat_type: self.seat_type,
        }
    }
}

impl Identity {
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn get_seat_type(&mut self) -> Result<SeatType, Error> {
        let id: *const blpapi_Identity_t = self.ptr;
        let mut st_ind: c_int = 0;
        let st = unsafe { blpapi_Identity_getSeatType(id, &mut st_ind) } as i32;
        match st == 0 {
            true => {
                self.seat_type = st;
                Ok(st.into())
            }
            false => Err(Error::Identity),
        }
    }

    pub fn is_authorized(&self, service: Service) -> Result<bool, Error> {
        let res = unsafe { blpapi_Identity_isAuthorized(self.ptr, service.ptr) } as i32;
        match res {
            0 => Ok(true),
            _ => Err(Error::Identity),
        }
    }

    pub fn has_entitlement(
        &self,
        service: Service,
        ele: Element,
        ent_id: i64,
        ent_num: usize,
        ent_failed: i64,
        ent_count: i64,
    ) -> Result<bool, Error> {
        // TO-DO! Check if the providing *mut need to be pointer?
        // Are the used/called from other functions or do they can be created
        // in the function ?
        let res = unsafe {
            blpapi_Identity_hasEntitlements(
                self.ptr,
                service.ptr,
                ele.ptr,
                ent_id as *const c_int,
                ent_num,
                ent_failed as *mut c_int,
                ent_count as *mut c_int,
            )
        } as i32;
        match res {
            0 => Ok(true),
            _ => Err(Error::Identity),
        }
    }
}
