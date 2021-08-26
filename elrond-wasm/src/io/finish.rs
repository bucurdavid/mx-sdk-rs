use alloc::boxed::Box;
use elrond_codec::TryStaticCast;

use crate::api::{EndpointFinishApi, ErrorApi, ManagedTypeApi};
use crate::elrond_codec::{EncodeError, TopEncode, TopEncodeOutput};
use crate::types::ManagedBuffer;
use crate::Vec;

struct ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
{
    api: FA,
}

impl<FA> ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
{
    #[inline]
    fn new(api: FA) -> Self {
        ApiOutputAdapter { api }
    }
}

impl<FA> TopEncodeOutput for ApiOutputAdapter<FA>
where
    FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
{
    type NestedBuffer = ManagedBuffer<FA>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.api.finish_slice_u8(bytes);
    }

    fn set_u64(self, value: u64) {
        self.api.finish_u64(value);
    }

    fn set_i64(self, value: i64) {
        self.api.finish_i64(value);
    }

    #[inline]
    fn set_unit(self) {
        // nothing: no result produced
    }

    #[inline]
    fn set_specialized<T: TryStaticCast, F: FnOnce() -> Box<[u8]>>(self, value: &T, else_bytes: F) {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<FA>>() {
            self.api.finish_managed_buffer_raw(managed_buffer.handle);
        } else {
            self.set_boxed_bytes(else_bytes());
        }
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        ManagedBuffer::new_empty(self.api.clone())
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        self.api.finish_managed_buffer_raw(nb.handle);
    }

    #[inline]
    fn set_big_int_handle_or_bytes<F: FnOnce() -> Vec<u8>>(self, handle: i32, _else_bytes: F) {
        self.api.finish_big_int_raw(handle);
    }

    #[inline]
    fn set_big_uint_handle_or_bytes<F: FnOnce() -> Vec<u8>>(self, handle: i32, _else_bytes: F) {
        self.api.finish_big_uint_raw(handle);
    }
}

/// All types that are returned from endpoints need to implement this trait.
pub trait EndpointResult: Sized {
    /// Indicates how the result of the endpoint can be interpreted when called via proxy.
    /// `Self` for most types.
    type DecodeAs;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static;
}

/// All serializable objects can be used as smart contract function result.
impl<T> EndpointResult for T
where
    T: TopEncode,
{
    type DecodeAs = Self;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        self.top_encode_or_exit(ApiOutputAdapter::new(api.clone()), api, finish_exit);
    }
}

#[inline(always)]
fn finish_exit<FA>(api: FA, en_err: EncodeError) -> !
where
    FA: ManagedTypeApi + EndpointFinishApi + ErrorApi + 'static,
{
    api.signal_error(en_err.message_bytes())
}
