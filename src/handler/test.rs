use crate::db::a::a::dsl::*;
use crate::model::a_model::A;
use crate::connect;

use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use salvo::{prelude::*, Error};

#[handler]
pub async fn test(res: &mut Response) ->Result<(),Error> {
    let mut conn = connect().unwrap();
    let result = a.filter(account.eq("hezhe")).first::<A>(&mut conn);
    match result {
        Ok(data) => {
            res.render(Json(&data));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}

#[handler]
pub async fn test2(res: &mut Response) ->Result<(),Error> {
    let mut conn = connect().unwrap();
    let new_a = A {
        account: "tcirtsa".to_string(),
        psd: "123456".to_string(),
    };
    let result = diesel::insert_into(a).values(&new_a).execute(&mut conn);
    match result {
        Ok(data) => {
            res.render(format!("success insert {}",data));
            Ok(())
        }
        Err(e) => {
            res.render(Json(&e.to_string()));
            Ok(())
        }
    }
}