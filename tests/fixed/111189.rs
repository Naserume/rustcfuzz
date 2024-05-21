struct Example {}

enum CryptoStoreError {}

struct RoomSettings {}

impl Example {
    /// Get the stored room settings, such as the encryption algorithm or
    /// whether to encrypt only for trusted devices.
    ///
    /// #[rustfmt::skip]
    /// These settings can be modified via
    /// [set_room_algorithm()](#method.set_room_algorithm) and
    /// [set_room_only_allow_trusted_devices()](#method.
    /// set_room_only_allow_trusted_devices) methods.
    pub fn get_room_settings(
        &self,
        _room_id: String,
    ) -> Result<Option<RoomSettings>, CryptoStoreError> {
        Ok(None)
    }

    fn set_room_only_allow_trusted_devices(&self) {}
}

fn main() {
    let e = Example {};
    let _ = e.get_room_settings(String::new());
}
