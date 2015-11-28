extern crate iron;
extern crate bodyparser;
extern crate persistent;
extern crate rustc_serialize;

pub mod matrix;
use matrix::*;

#[doc(no_inline)]
use rustc_serialize::json;
use iron::status;
use iron::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;


use std::thread;
use std::sync::Arc;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)] // Automatically generate trait implementations
/// Matrix, deren Elemente alle Zeilenweise als Vektor vorliegen
pub struct MyMatrix{
    /// Zeilenanzahl
    pub rows: i32,
    /// Spaltenanzhal
    pub cols: i32,
    ///Elemente der Matrix
    pub elem: Vec<i32>,
}
/// Struct in den das übermittelte JSON geparsed wird
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct MyBody{
    message: Option<String>,
    mat_a: MyMatrix,
    mat_b: MyMatrix,
}

/// Erzeugt eine HTTP-Response auf eine Anfrage ohne Body. (GET Request)
///
/// Die Übergebenen Anfrage wird ausgewertet und die angegebene Datei (falls diese exisitiert) in eine Response gepackt.
/// Wird eine Datei nicht gefunden, so wird ein entsprechender HTTP-Statuscode als Response zurück gegeben.
///
/// # Argumente
///
/// * `req` - vom Server empfange Anfrage
pub fn get_response(req: &Request) -> Response{

	let path_vec = &req.url.path; //liefert Pfad als Array
    //Pfad auswerten
    let s = if path_vec[0] == ""{ //falls keien Datei angegben -> default index.html
        "html/index.html".to_string()
    } else {
        "html/".to_string() + &path_vec[path_vec.len()-1] //im letzten Feld steht der Dateiname
    };
    let path = Path::new(&s);
    let display = path.display();

    //versuche die Datei zu öffnen
    //falls unbekannt gib 404 als Response
    let file = match File::open(&path) {
        Err(why) => {
            println!("Die Datei {} konnte nicht geöffnet werden.\n {}", display, Error::description(&why));
            return Response::with((status::NotFound, "Seite nicht gefunden!"))
        }
        Ok(file) => file //fals erfolgreich, gib die Datei zurück
    };

    //Response mit Datei
    let mut res = Response::with((status::Ok,file));

    //Dateiendung
    let extension = path.extension().unwrap().to_str().unwrap();

    //content-type auf Basis der Dateiendung setzen und Response zurückgeben

    match extension{
        "css" => {res.headers.set_raw("content-type", vec![b"text/css".to_vec()]); res},
        "js" => {res.headers.set_raw("content-type", vec![b"text/javascript".to_vec()]); res},
        "png" => {res.headers.set_raw("content-type", vec![b"image/png".to_vec()]); res},
        _ => {res.headers.set_raw("content-type", vec![b"text/html".to_vec()]); res},
    }
}

/// Erzeugt eine HTTP-Response auf eine Anfrage mit Body. (POST Request)
///
/// Wertet den übergeben Body aus und ruft die entsprechenden Methoden zur Berechnung der Ergebnissmatrix auf
/// Die Ergebnissmatrizen werden anschließend in JSON codiert und in eine Response gepackt. Tritt ein Fehler auf, so wird ein
/// entsprechender Fehlercode im Feld 'message' gepackt.
///
/// # Argumente
///
/// * `body` - der in den Struct `MyBody` geparste Body des Requests
pub fn post_response(body: &MyBody) -> Response{

    let body_arc =  Arc::new(body.clone()); //mehrer Threads sollen auf den body zugreifen

    let body_ref_for_add = body_arc.clone(); //neue Referenz auf den Body

    let handle_add = thread::spawn(move|| { //neuer Thread
        vector_add(&body_ref_for_add.mat_a.elem, &body_ref_for_add.mat_b.elem)//Matrixaddition
    });

    let body_ref_for_mul = body_arc.clone(); //neue Referenz auf den Body

    let handle_mul = thread::spawn(move|| { //neuer Thread
        matrix_mul(&body_ref_for_mul.mat_a.elem, //Matrixmultiplikation
                     &body_ref_for_mul.mat_b.elem,
                     body_ref_for_mul.mat_a.rows as usize,
                     body_ref_for_mul.mat_a.cols as usize,
                     body_ref_for_mul.mat_b.cols as usize)
    });

    //Ergebnisse auswerten

    let res_add = match handle_add.join(){ //warte bis Thread fertig
        Ok(result) => match result {
            Some(c) => Some(MyMatrix{    //Falls erfolgreich Struct anlegen
                                rows: body.mat_a.rows,
                                cols: body.mat_a.cols,
                                elem: c }),
            None => None // Matrizen haben nicht die gleiche Größe
        },
        Err(_) => {println!("Add ist abgestürtzt"); None}
    };

    let res_mul = match handle_mul.join(){
        Ok(result) => match result {
            Some(c) => Some(MyMatrix{
                                rows: body.mat_a.rows,
                                cols: body.mat_b.cols,
                                elem: c}),
            None => None //Matrizen sind falsch dimensioniert für Multiplikation
        },
        Err(_) => {println!("Mul ist abgestürtzt"); None}
    };

    //erzeugen von MyBody
    let resp_body = match res_add{
        Some(c_add) => {
            match res_mul{
                Some(c_mul) => MyBody{message: None, mat_a: c_add, mat_b: c_mul},
                None => MyBody{message: Some("Matrizen sind falsch dimensioniert für Multiplikation!".to_string()), mat_a: c_add, mat_b: MyMatrix{rows:0,cols:0,elem: vec![]}}
            }
        }
        None => {
            match res_mul{
                Some(c_mul) => MyBody{message: Some("Matrizen haben nicht die gleiche Größe!".to_string()), mat_a: MyMatrix{rows:0,cols:0,elem: vec![]}, mat_b: c_mul},
                None => MyBody{message: Some("Matrix konnte nicht berechnet werden! Matrizen sind vermutlich falsch dimensioniert.".to_string()), mat_a: MyMatrix{rows:0,cols:0,elem: vec![]}, mat_b: MyMatrix{rows:0,cols:0,elem: vec![]}}
            }
        }
    };

    println!("{:?}",resp_body);

    Response::with((status::Ok, json::encode(&resp_body).unwrap())) //Matrix als JSON codieren
}

/// Erhält den Request und erzeugt eine Response als IronResult<Response>.
///
/// Je nach verwendeter Methode wird die entsprechende Funktion zur Auswertung des Requests aufgerufen
/// Wird Post verwendet, wird zusätzlich der Body in den Struct `MyBody` geparset.
pub fn process_request(req: &mut Request) -> IronResult<Response> {

    match req.method{

        iron::method::Method::Post => {
            match req.get::<bodyparser::Struct<MyBody>>() {
                Ok(Some(struct_body)) => Ok(post_response(&struct_body)),
                Ok(None) => {println!("Post ohne Body");Ok(Response::with((status::InternalServerError,"Es wurde kein Body übergeben!")))},
                Err(err) => {println!("Error: {:?}", err);Ok(Response::with((status::InternalServerError,"Anfrage konnte nicht ausgewertet werden.")))},
            }
        }

        iron::method::Method::Get => Ok(get_response(req)),

        _ => Ok(Response::with((status::InternalServerError,"Request Method nicht unterstützt"))),
    }

}
const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {

    let mut chain = Chain::new(process_request);
    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();
}
