use actix_web::{get, Responder};
use serde_json::{json, Value};

#[get("/package.json")]
pub async fn package_list() -> impl Responder {
    let packages = json!({
        "packages": json!({
            "toolslibrary/uuid": json!({
                "1.0.0": json!({
                    "name": Value::Null,
                    "version": "1.0.0",
                    "dist":json!({
                        "url":"",
                        "type": "zip"
                        })
                }) 
            })
        })
    });

    return actix_web::web::Json(packages);
}
