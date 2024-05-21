#![feature(return_type_notation)]

trait HealthCheck {
    async fn check<const N: usize>() -> bool;
}

async fn do_health_check_par<HC>(hc: HC)
where
    HC: HealthCheck<check(): Send> + Send + 'static,
{
}
