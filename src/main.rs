#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos::logging::log;
    use leptos_axum::{LeptosRoutes, generate_route_list_with_ssg};
    use pv_root_cause::{App, shell};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let (routes, _) = generate_route_list_with_ssg(App);

    if std::env::var_os("PV_EXPORT_STATIC").is_some() {
        let options = leptos_options.clone();
        let (_, static_routes) = generate_route_list_with_ssg(move || shell(options.clone()));
        static_routes.generate(&leptos_options).await;
        return;
    }

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Hydration entry point is exported from lib.rs for cargo-leptos.
}
