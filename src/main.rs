use actix_web::{get, post, Responder, web::Json, App, HttpRequest, HttpResponse, HttpServer};
use actix_files::Files;

// use std::{fs, result::Result};

use mysql::prelude::Queryable;
use serde::{Serialize, Deserialize};

const DB_USER_NAME:& 'static str = "root";
const DB_PASSWORD:& 'static str = "c2p16";
const DB_IP:& 'static str = "localhost";
const DB_PORT:u16 = 3306;
const DB_NAME:& 'static str = "movie";

const QUARY_TABLE:& 'static str = "info";
const QUARY_COLUME:& 'static str = "Rate";
const QUARY_ENG_NAME:& 'static str = "ENG_Title";
const QUARY_LIM:u32 = 10;

#[derive(Debug, Serialize, Deserialize)]
struct MOVIE{
    ko : String,
    en : String,
    poster : String,
    genre : String,
    release : String,
    runtime : String,
    age : String,
    rate : f32,
    ott : String,
}

async fn movie_info(query:&str) -> Vec<MOVIE> {
    let url = format!("mysql://{}:{}@{}:{}/{}", DB_USER_NAME, DB_PASSWORD, DB_IP, DB_PORT, DB_NAME);
    //let db_url =  // &*url or // url.as_str();
    let pool = mysql::Pool::new(url.as_str()).expect("연결 실패");
    
    let mut conn = pool.get_conn().expect("연결 실패");
    let return_info = conn.query_map(
        query,|
        (ko, en, poster, genre, release, runtime, age, rate, ott)| {
            MOVIE {ko, en, poster, genre, release, runtime, age, rate, ott}
        },
    );

    match return_info{
        Ok(movies) => {
            return movies;
        }
        Err(_) => {
            let error = vec![
                MOVIE {
                    ko : String::from("NONE"),
                    en : String::from("NONE"),
                    poster : String::from("NONE"),
                    genre : String::from("NONE"),
                    release : String::from("NONE"),
                    runtime : String::from("NONE"),
                    age : String::from("NONE"),
                    rate : 0.0,
                    ott : String::from("NONE"),
                },
            ];
            return error;
        }
    }
}

// #[get("/")]
// async fn index() -> HttpResponse {
//     let html_content = fs::read_to_string("src/post_test/helloDB.html").unwrap();
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(html_content)
// }

#[post("/movies")]
async fn handle_post(req: HttpRequest) -> HttpResponse {
    println!("{:?}",req);
    let query = format!("SELECT * FROM {} order by {} DESC limit {}", QUARY_TABLE, QUARY_COLUME, QUARY_LIM);
    let result = movie_info(query.as_str()).await;
    HttpResponse::Ok().json(result)
}

#[post("/search")]
async fn get_info(req: HttpRequest, info:Json<String>) -> HttpResponse {
    println!("{:?}",req);
    println!("{:?}",info);
    let query = format!("SELECT * FROM {} WHERE {} = '{}' ", QUARY_TABLE, QUARY_ENG_NAME, info);//info.as_str()
    let result = movie_info(query.as_str()).await;
    println!("{:?}",result);
    HttpResponse::Ok().json(result)
}

//Interstellar, Signal
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(in5dex)
            .service(handle_post)
            .service(get_info)
            .service(Files::new("/", "src/Web_content").index_file("split_img.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ####################################################################################################
// #[derive(Debug, Serialize, Deserialize)] // 내가 임의로 만든 DB 테이블 파일
// struct MOVIE {
//     제목 : String,
//     장르 : String,
//     구분 : String,
//     개봉연도 : u32,
//     평점 : f32,
//     나이제한 : String,
//     위치 : String,
// }
// 
// fn main(){
//     let url = format!("mysql://{}:{}@{}:{}/{}", DB_USER_NAME, DB_PASSWORD, DB_IP, DB_PORT, DB_NAME);
//     //let db_url =  // &*url or // url.as_str();
//     let pool = mysql::Pool::new(url.as_str()).expect("연결 실패");
//     let query = "SELECT * FROM info order by Rate DESC limit 6";
//     let mut conn = pool.get_conn().expect("연결 실패");
//     let movie_name: Result<Vec<MOVIE>, mysql::Error> = conn
//         .query_map(
//             query,
//             |(KOR_Title,ENG_Title, Poster, Ganre, Release, Runtime, Rated, Rate, OTT)| {
//                 MOVIE {KOR_Title,ENG_Title, Poster, Ganre, Release, Runtime, Rated, Rate, OTT}
//             },
//         );
//     match movie_name {
//         Ok(movies) => {
//             for movie in movies {
//                 println!("{:#?}", movie);
//             }
//         }
//         Err(err) => {
//             println!("Error: {}", err);
//         }
//     }
// }
// query.as_str(),|
//         (제목, 장르, 구분, 개봉연도, 평점, 나이제한, 위치)| {
//             MOVIE {제목, 장르, 구분, 개봉연도, 평점, 나이제한, 위치}
//         },

// #[get("/hello")]
// async fn hello() -> impl Responder {
//     // 데이터베이스에서 MOVIE 테이블의 title, rate, runtime, age 컬럼을
//     // rate를 기준으로 내림차순 정렬하여 최대 6개 행을 조회하는 쿼리
//     let query = "SELECT title, rate, runtime, age FROM MOVIE ORDER BY rate DESC LIMIT 6";
//     // 데이터베이스 연결
//     let mut conn = Conn::new(Opts::default()).unwrap();
//     // 쿼리 실행 및 결과를 Vec<Movie> 형태로 변환
//     let result: Vec<Movie> = conn
//         .query_map(query, |(KOR_Title, Ganre, Rate, Rated)| Movie {
//             KOR_Title,
//             Ganre,
//             Rate,
//             Rated,
//         })
//         .unwrap();
//     // JSON 형태로 결과 반환
//     HttpResponse::Ok().json(result)
// }

// // 소켓 프로그래밍 
// // use std::io::prelude::*;
// // use std::net::TcpStream;
// // use std::net::TcpListener;
// // use std::fs::File;
// // fn main() {
// //     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
// //     for stream in listener.incoming() {
// //         let stream = stream.unwrap();
// //         handle_connection(stream);
// //     }
// // }
// // fn handle_connection(mut stream: TcpStream) {
// //     let mut buffer = [0; 512];
// //     stream.read(&mut buffer).unwrap();
// //     let mut file = File::open("src/split_img.html").unwrap();
// //     let mut contents = String::new();
// //     file.read_to_string(&mut contents).unwrap();
// //     let response = format!(
// //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
// //         contents.len(),
// //         contents
// //     );
// //     stream.write(response.as_bytes()).unwrap();
// //     stream.flush().unwrap();
// // }
