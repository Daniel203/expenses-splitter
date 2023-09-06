use cfg_if::cfg_if;
use leptos::use_context;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::{LeptosOptions, ServerFnError, Scope};
        use sqlx::SqlitePool;
        use axum::extract::FromRef;
        use leptos_router::RouteListing;

        #[derive(FromRef, Debug, Clone)]
        pub struct AppState{
            pub leptos_options: LeptosOptions,
            pub pool: SqlitePool,
            pub routes: Vec<RouteListing>,

        }

        pub fn pool(cx: Scope) -> Result<SqlitePool, ServerFnError> {
            return use_context::<SqlitePool>(cx)
                .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()));
        }
    }
}
