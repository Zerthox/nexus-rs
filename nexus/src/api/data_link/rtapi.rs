pub use crate::rtapi::data::RealTimeData;

/// Returns the shared [`ReadlTimeData`] pointer.
#[inline]
pub fn get_rtapi_ptr() -> *const RealTimeData {
    RealTimeData::get_ptr()
}

/// Reads the shared [`ReadlTimeData`].
#[inline]
pub fn read_rtapi() -> Option<RealTimeData> {
    RealTimeData::read()
}
