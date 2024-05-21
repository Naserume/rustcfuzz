pub trait ModemTrait: Sized + Send + Sync + 'static {
    fn new() -> impl Future<Output = Result<Self, HalError>> + Send;
    fn get_modem_info(&self) -> impl Future<Output = Result<ModemInfo, HalError>> + Send + '_;
}

pub struct ModemController {
    conn: Connection,
}

impl ModemTrait for ModemController {
    async fn new() -> Result<Self, HalError> {
        let conn = Connection::system().await?;

        let ret = Self { conn };

        Ok(ret)
    }

    fn get_modem_info(&self) -> impl Future<Output = Result<ModemInfo, HalError>> + Send + '_ {
        async {
            let proxy = PppDbusProxy::new(&self.conn).await?;

            // Ok(ModemInfo::default())

            Ok(ModemInfo {
                sim_pin_status: proxy.get_sim_pin_status().await?,
                initialized: proxy.is_initialized().await?,
                online: proxy.is_online().await?,
                modem_manufacturer: proxy.get_modem_manufacturer().await?,
                modem_model: proxy.get_modem_model().await?,
                modem_imei: proxy.get_modem_imei().await?,
                modem_firmware_version: proxy.get_modem_firmware_version().await?,
                iccid: proxy.get_ccid().await?,
                network_type: proxy.get_network_type().await?,
                network_operator: proxy.get_network_operator().await?,
                signal_power: proxy.get_signal_power().await?,
                signal_power_condition: proxy.get_signal_power_condition().await?,
                signal_quality_rating: proxy.get_signal_quality_rating().await?,
                signal_quality_source: proxy.get_signal_quality_source().await?,
                mobile_network_operator_profile: proxy
                    .get_mobile_network_operators_profile()
                    .await?,
                ppp_connection_start: proxy.get_ppp_connection_start_time().await?,
            })
        }
    }
}
