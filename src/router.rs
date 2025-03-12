use crate::config::{create_cors_handler, create_jwt_auth};
use crate::handler;
use salvo::prelude::*;

pub fn create_router() -> Router {
    let auth_handler = create_jwt_auth();

    Router::new()
        .push(
            Router::with_path("/auth")
                .goal(handler::user::auth)
                .hoop(auth_handler),
        )
        .push(Router::with_path("/ws").goal(handler::ws::user_connected))
        .push(Router::with_path("/register").post(handler::user::register))
        .push(
            Router::with_path("/<username>")
                .get(handler::user::get_user)
                .push(Router::with_path("/updata_head").post(handler::user::updata_head))
                .push(Router::with_path("/get_user_posts").post(handler::post::get_user_posts))
                .push(Router::with_path("/collect_voice").post(handler::voice::collect_voice_fingerprint))
                .push(Router::with_path("/text_to_speech").post(handler::voice::text_to_speech)),
        )
        .push(
            Router::with_path("/posts")
                .get(handler::post::all_posts)
                .push(Router::with_path("/add_post").post(handler::post::add_post))
                .push(
                    Router::with_path("/<post_id>")
                        .post(handler::post::get_post)
                        .push(Router::with_path("/like_count").post(handler::like::get_like_count))
                        .push(
                            Router::with_path("/like/<user_id>")
                                .post(handler::like::add_like)
                                .delete(handler::like::delete_like),
                        )
                        .push(
                            Router::with_path("/comment")
                                .post(handler::comment::get_all_comment)
                                .push(
                                    Router::with_path("/comment_count")
                                        .post(handler::comment::get_comment_count),
                                )
                                .push(
                                    Router::with_path("/<comment_id>")
                                        .post(handler::comment::get_comment),
                                )
                                .push(
                                    Router::with_path("/add_comment")
                                        .post(handler::comment::add_comment),
                                ),
                        ),
                ),
        )
}

pub fn create_service() -> Service {
    Service::new(create_router()).hoop(create_cors_handler())
}