use crate::model::user_model::NewUser;
use crate::db::schema::users::dsl::*;
use crate::connect;

use diesel::prelude::*;
use salvo::{prelude::*, Error};

#[handler]
pub async fn register(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let mut conn = connect().unwrap();
    let mut new_user = req.parse_json::<NewUser>().await?;
    new_user.created_at = Some(chrono::Utc::now().naive_utc());
    let result = diesel::insert_into(users).values(&new_user).execute(&mut conn);
    match result {
        Ok(_) => {
            res.render(format!("success register user {} at {}",new_user.username,new_user.created_at.unwrap()));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}