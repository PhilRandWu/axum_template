use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::routes::user::AuthenticateResponse;
use crate::tests::setup::use_app;
use crate::tests::utils::create_user;

#[test]
fn post_user_route() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        name: String,
        email: String,
        password: String,
    }

    let body = Body {
        name: "Nahuel".to_owned(),
        email: "nahuel@gmail.com".to_owned(),
        password: "Password1!".to_owned(),
    };
    

    let body = Body {
        name: "Nahuel".to_owned(),
        email: "nahuel@gmail.com".to_owned(),
        password: "Password1!".to_owned(),
    };

    use_app(async move {
        let client = reqwest::Client::new();
        let res = client
            .post("http://127.0.0.1:8088/users")
            .json(&body)
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let actual = status_code;
        let expected = StatusCode::CREATED;
        assert_eq!(actual, expected);

        let body = res.json::<AuthenticateResponse>().await.unwrap();
        assert_eq!(body.user.name, "Nahuel");
        assert_eq!(body.user.email, "nahuel@gmail.com");
    })
}

#[test]
fn authenticate_user_route() {
    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        email: String,
        password: String,
    }

    let request_body = RequestBody {
        email: "nahuel@gmail.com".to_owned(),
        password: "Password1!".to_owned(),
    };

    use_app(async move {
        create_user("nahuel@gmail.com").await.unwrap();

        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:8088/users/authenticate")
            .json(&request_body)
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let actual = status_code;
        let expected = StatusCode::OK;
        assert_eq!(actual, expected);

        let body = res.json::<AuthenticateResponse>().await.unwrap();
        assert_eq!(body.user.email, "nahuel@gmail.com");
    })
}