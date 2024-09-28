use crate::connect;
use crate::db::schema::posts::dsl::*;
use crate::model::post_model::{NewPost, Post};

use diesel::prelude::*;
use salvo::{prelude::*, Error};

#[handler]
pub async fn add_post(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let mut new_post = req.parse_json::<NewPost>().await?;
    new_post.created_at = Some(chrono::Utc::now().naive_utc() + chrono::Duration::hours(8));
    let result = diesel::insert_into(posts)
        .values(&new_post)
        .execute(&mut conn);
    match result {
        Ok(_) => {
            res.render(Json("success add post"));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}

#[handler]
pub async fn get_post(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let id = req.param::<i32>("post_id").unwrap();
    let result = posts.filter(post_id.eq(id)).first::<Post>(&mut conn);
    match result {
        Ok(post) => {
            res.render(Json(post));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}

#[handler]
pub async fn get_user_posts(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let id = req.query::<i32>("user_id").unwrap();
    let result = posts.filter(user_id.eq(id)).load::<Post>(&mut conn);
    match result {
        Ok(data) => {
            res.render(Json(data));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}
