use actix_web::{self, http::header, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sso_ui_jwt::{
    ticket::{validate_ticket, ValidateTicketError},
    token::{create_token, decode_token, TokenType},
    SSOJWTConfig,
};

#[derive(Deserialize)]
struct LoginQuery {
    ticket: Option<String>,
}

#[derive(Serialize)]
struct TokenRes {
    token: String,
}

async fn login(config: web::Data<SSOJWTConfig>, query: web::Query<LoginQuery>) -> HttpResponse {
    if let Some(ticket) = &query.ticket {
        let res = validate_ticket(&**config, &ticket).await;

        match res {
            Ok(res) => {
                let token = create_token(&config, TokenType::AccessToken, res.clone()).unwrap();

                let creds = TokenRes { token };
                HttpResponse::Ok().json(creds)
            }
            Err(ValidateTicketError::AuthenticationFailed) => {
                HttpResponse::Unauthorized().body("Authentication failed")
            }
            _ => HttpResponse::InternalServerError().body("Internal server error"),
        }
    } else {
        HttpResponse::TemporaryRedirect()
            .insert_header((
                header::LOCATION,
                format!("{}/login?service={}", config.cas_url, config.service_url),
            ))
            .finish()
    }
}

async fn logout(config: web::Data<SSOJWTConfig>) -> HttpResponse {
    let url = format!("{}/logout?url={}", config.cas_url, config.service_url);
    HttpResponse::TemporaryRedirect()
        .insert_header((header::LOCATION, url))
        .finish()
}

async fn get_self(config: web::Data<SSOJWTConfig>, req: HttpRequest) -> HttpResponse {
    match req.headers().get("Authorization") {
        Some(header) => match header.to_str() {
            Ok(header) => {
                if header.starts_with("Bearer") {
                    let header_vec: Vec<&str> = header.split_whitespace().collect();
                    let token = header_vec.get(1);

                    if let Some(token) = token {
                        match decode_token(&**config, TokenType::AccessToken, &token) {
                            Ok(data) => HttpResponse::Ok().json(data.claims),
                            Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
                        }
                    } else {
                        HttpResponse::Unauthorized().body("Token not found")
                    }
                } else {
                    HttpResponse::Unauthorized().body("Not a bearer token")
                }
            }
            Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
        },
        None => HttpResponse::Unauthorized().body("Unauthorized request"),
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SSOJWTConfig::new(
        120,
        120,
        String::from("access secret"),
        String::from("refresh secret"),
        String::from("http://localhost:7700/login"),
        String::from("http://localhost:7700"),
    );

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(config.clone())).service(
            web::scope("")
                .service(web::resource("/login").route(web::get().to(login)))
                .service(web::resource("/logout").route(web::get().to(logout)))
                .service(web::resource("/self").route(web::get().to(get_self))),
        )
    })
    .bind("127.0.0.1:7700")?
    .run()
    .await?;

    Ok(())
}
