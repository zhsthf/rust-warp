use auth::{create_jwt, with_auth, Role};
use bcrypt::{hash, verify, DEFAULT_COST};
use dotenv::dotenv;
use error::Error::*;
use mongodb::{
    bson::{doc, uuid},
    options::ClientOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use warp::{http::StatusCode, reject, reply, Filter, Rejection, Reply};

mod auth;
mod error;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type MongoDbClient = Client;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[tokio::main]
async fn main() {
    let client = connect_to_mongo().await.expect("MongoDB connection failed");
    let users_collection_pointer = client.database("my_app").collection::<User>("users");

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(with_collection(users_collection_pointer.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);

    let admin_route = warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let signup_route = warp::path!("signup")
        .and(warp::post())
        .and(with_collection(users_collection_pointer.clone()))
        .and(warp::body::json())
        .and_then(signup_handler);

    let routes = login_route
        .or(signup_route)
        .or(user_route)
        .or(admin_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

pub async fn connect_to_mongo() -> mongodb::error::Result<MongoDbClient> {
    dotenv().ok();

    let username =
        env::var("MONGO_INITDB_ROOT_USERNAME").expect("MONGO_INITDB_ROOT_USERNAME must be set");
    let password =
        env::var("MONGO_INITDB_ROOT_PASSWORD").expect("MONGO_INITDB_ROOT_PASSWORD must be set");
    let mongo_uri = format!("mongodb://{}:{}@localhost:27017", username, password);

    let mut client_options = ClientOptions::parse(&mongo_uri).await?;
    client_options.app_name = Some("MyApp".to_string());

    Client::with_options(client_options)
}

fn with_collection(
    collection: Collection<User>,
) -> impl Filter<Extract = (Collection<User>,), Error = Infallible> + Clone {
    warp::any().map(move || collection.clone())
}

pub async fn signup_handler(
    users_collection: Collection<User>,
    body: SignupRequest,
) -> WebResult<impl Reply> {
    let existing_user = users_collection
        .find_one(doc! {"email": &body.email}, None)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    if existing_user.is_some() {
        return Err(reject::custom(UserAlreadyExistsError));
    }

    let hashed_pw =
        hash(body.pw, DEFAULT_COST).map_err(|_| reject::custom(PasswordHashingError))?;

    let new_user = User {
        uid: uuid::Uuid::new().to_string(),
        email: body.email,
        pw: hashed_pw,
        role: body.role,
    };

    users_collection
        .insert_one(new_user, None)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    Ok(reply::with_status(
        "User created successfully",
        StatusCode::CREATED,
    ))
}

pub async fn login_handler(
    users_collection: Collection<User>,
    body: LoginRequest,
) -> WebResult<impl Reply> {
    let user = users_collection
        .find_one(doc! {"email": &body.email}, None)
        .await
        .map_err(|_| reject::custom(DatabaseError))?;

    if let Some(user_data) = user {
        let is_password_correct = verify(&body.pw, &user_data.pw)
            .map_err(|_| reject::custom(PasswordVerificationError))?;

        if is_password_correct {
            let token = create_jwt(&user_data.uid, &Role::from_str(&user_data.role))
                .map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        } else {
            Err(reject::custom(WrongCredentialsError))
        }
    } else {
        Err(reject::custom(WrongCredentialsError))
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}
