use actix_web::{post, web, Error, HttpResponse, Responder, Scope};
use crate::account::model::AccountModel;
use crate::account::schema::CreateAccountSchema;
use crate::AppState;


#[post("/register")]
pub  async fn create_account_handler(
    body: web::Json<CreateAccountSchema>,
    data: web::Data<AppState>,
) -> impl Responder  {
    let query_result = sqlx::query_as!(
        AccountModel,
        "INSERT INTO account (user_name,address) VALUES ($1, $2) RETURNING *",
        body.user_name,
        body.address
    )
        .fetch_one(&data.db)
        .await;

     match query_result {
        Ok(account) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "account": account
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
            }

             HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))
        }
    }
}

pub  async fn login_handler(
    body: web::Json<CreateAccountSchema>,
    data: web::Data<AppState>,
) -> impl Responder  {
    let query_result = sqlx::query_as!(
        AccountModel,
        "SELECT * FROM account WHERE user_name = $1 AND address = $2",
        body.user_name,
        body.address
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(account) => {
            let token = crate::auth::generate_token(&account.id.to_string(), &account.address);
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": account.address,
                "token": token
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(e) => {
            HttpResponse::Unauthorized()
                .json(serde_json::json!({"status": "fail","message": format!("{:?}", e)}))
        }
    }
}
pub fn config(api: Scope) -> Scope {
    //notes
    return  api.service(create_account_handler);
}
