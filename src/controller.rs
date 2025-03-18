use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    entities::requests::{BlockList, Config, Pack},
    service::Service,
};

pub fn controller() -> Router {
    Router::new()
        .route("/inventory/blocks", get(get_inventory).post(add_inventory))
        .route("/order", get(get_orders).post(submit_order))
        .route("/pack", post(add_pack))
        .route(
            "/config/inventory/order",
            get(get_strategy).put(set_strategy),
        )
        .with_state(Service::default())
}

async fn get_inventory(State(st): State<Service>) -> impl IntoResponse {
    Json(st.get_blocks())
}

async fn add_inventory(State(st): State<Service>, Json(block_list): Json<BlockList>) {
    st.add_blocks(block_list.blocks);
}

async fn get_strategy(State(st): State<Service>) -> impl IntoResponse {
    Json(st.get_strategy())
}

async fn set_strategy(State(st): State<Service>, Json(config): Json<Config>) {
    st.set_strategy(config.algorithm);
}

async fn get_orders(State(st): State<Service>) -> impl IntoResponse {
    Json(st.get_orders())
}

async fn submit_order(State(st): State<Service>, Json(block_list): Json<BlockList>) {
    st.submit_order(block_list.blocks);
}

async fn add_pack(State(st): State<Service>, Json(pack): Json<Pack>) {
    st.add_pack(pack);
}
