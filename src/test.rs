use actix::Actor;

use crate::websocket::Server;

// update get_service to include the websocket server
pub async fn get_service(
) -> impl Service<Request = Request, Response = ServiceResponse<Body>, Error = Error> {
    test::init_service(
        App::new()
            .data(db::new_pool())
            .data(Server::new().start())
            .configure(routes),
    )
    .await
}