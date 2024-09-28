use crate::connect;
use crate::db::schema::likes::dsl::*;
use crate::model::like_model::Like;

use diesel::prelude::*;
use salvo::{prelude::*, Error};

#[handler]
pub async fn get_like_count(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let p_id = req.param::<i32>("post_id").unwrap();
    let result = likes.filter(post_id.eq(p_id)).count().first::<i64>(&mut conn);
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

#[handler]
pub async fn add_like(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let mut new_like = req.parse_json::<Like>().await?;
    new_like.like_date = Some(chrono::Utc::now().naive_utc() + chrono::Duration::hours(8));
    let result = diesel::insert_into(likes)
        .values(&new_like)
        .execute(&mut conn);
    match result {
        Ok(_) => {
            res.render(Json("success add like"));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}

#[handler]
pub async fn delete_like(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let p_id = req.param::<i32>("post_id").unwrap();
    let u_id = req.param::<i32>("user_id").unwrap();
    let result = diesel::delete(likes.filter(post_id.eq(p_id)).filter(user_id.eq(u_id))).execute(&mut conn);
    match result {
        Ok(_) => {
            res.render(Json("success delete like"));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}