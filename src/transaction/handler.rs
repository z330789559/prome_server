use actix_web::{get, HttpResponse, post, Responder, Scope, web};
use serde_json::json;
use crate::AppState;
use crate::transaction::model::{TransactionModel};
use crate::utils::{FilterOptions, validate_signature};



#[get("/transaction")]
async fn get_transaction() -> impl Responder {
    const MESSAGE: &str = "ok";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}



#[post("/transaction")]
async fn create_transaction(
    body: web::Json<crate::transaction::schema::CreateTransactionSchema>,
    data: web::Data<AppState>,
) -> impl Responder {


    if !validate_signature(&body.address, &body.signature,body.0.clone().into()){
        return HttpResponse::BadRequest().json(json!({"status": "error","message": "Invalid signature"}));
    }
/**
*{"address":"5nvCpC9TVhd8AFJ2v7kVxNxM9rWHkyukRDir1yJjZYvH",
"startTs":"1552443713",
"endTs":"1552465313",
"epvToday":"1212",
"eChgToday":"1215",
"egridinToday":"1212",
"egridoutToday":"1212",
"eDischgToday":"1212",
"cobat":"1212",
"socHvs":"1212",
"sohHvs":"1212",
"mpptPower":"1212",
"latitude":"12032.3490E",
"longitude":"3120.2075N",
"signature":"66iS3TFHzW33mH11w1nT1SGG7xZHUsvmt8sQHMvQEeGL6basgDTrWZiMjdTMkfYUzshRX6d3NfH6dAvFi2U5KMef"}
 */
    let query_result = sqlx::query_as!(
        TransactionModel,
        "INSERT INTO transaction (addr,\
       start_ts,\
        end_ts,\
        epv_today,\
        e_chg_today,\
        egridin_today,\
        egridout_today,\
        e_dischg_today,\
        cobat,\
        soc_hvs,\
        soh_hvs,\
        mppt_power,\
        latitude,\
        longitude,\
         signature) VALUES ($1, $2, $3,$4, $5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15) RETURNING *",
         body.address,
            body.start_ts,
            body.end_ts,
         body.epv_today,
         body.e_chg_today,
         body.egridin_today,
            body.egridout_today,
         body.e_dischg_today,
            body.cobat,
            body.soc_hvs,
            body.soh_hvs,
            body.mppt_power,
            body.latitude,
            body.longitude,
         body.signature
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(tansanction) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "tansanction": tansanction
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}


#[get("/transactions")]
pub async fn transaction_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        TransactionModel,
        "SELECT * FROM transaction ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all note items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let transactions = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": transactions.len(),
        "transaction": transactions
    });
    HttpResponse::Ok().json(json_response)
}



pub fn config(api: Scope) -> Scope {
    //notes
    return  api.service(crate::transaction::handler::get_transaction)
        .service(crate::transaction::handler::create_transaction)
        .service(crate::transaction::handler::transaction_list_handler);

}
