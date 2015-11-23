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
use std::io::prelude::*;


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
#[derive(Debug, Clone, RustcDecodable)]
pub struct MyBody{
    operation: Option<String>,
    mat_a: MyMatrix,
    mat_b: MyMatrix,
}

/// Erzeugt eine HTTP-Response auf eine Anfrage ohne Body. (GET Request)
///
/// Die Übergebenen Anfrage wird ausgewertet und die angegebene Datei (falls diese exisitiert) in eine Response gepackt.
/// Wird eine Datei nicht gefunden, so wird ein entsprechender HTTP-Statuscode in die Response gepackt.
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
    let mut file = match File::open(&path) {
        Err(why) => {
            println!("Die Datei {} konnte nicht geöffnet werden.\n {}", display, Error::description(&why));
            return Response::with((status::NotFound, "Seite nicht gefunden!"))
        }
        Ok(file) => file //fals erfolgreich, gib die Datei zurück
    };

    let mut s = String::new();

    //den Inhalt der Datei in einen String umwandeln und falls erfolgreich Response erzeugen
    let mut res = match file.read_to_string(&mut s) {
        Err(why) => panic!("Datei {} konnte nicht gelesen werden.\n {}", display,
                                                   Error::description(&why)),
        Ok(_) => Response::with((status::Ok,s)),
    };

    //Dateiendung
    let extension = path.extension().unwrap().to_str().unwrap();

    //content-type auf Basis der Dateiendung setzen

    match extension{
        "css" => {res.headers.set_raw("content-type", vec![b"text/css".to_vec()]); res},
        "js" => {res.headers.set_raw("content-type", vec![b"text/javascript".to_vec()]); res},
        _ => {res.headers.set_raw("content-type", vec![b"text/html".to_vec()]); res},
    }
    //println!("status: {:?}",res.status);
    //println!("headers: {:?}",res.headers);


    //return res;
}

/// Erzeugt eine HTTP-Response auf eine Anfrage mit Body. (POST Request)
///
/// Wertet den übergeben Body aus und ruft die entsprechenden Methoden zur Berechnung der Ergebnissmatrix auf
/// Die Ergebnissmatrix wird anschließend in JSON codiert und in eine Response gepackt. Tritt ein Fehler auf, so wird ein
/// entsprechender Fehlercode in die Response gepackt.
///
/// # Argumente
///
/// * `body` - der in den Struct `MyBody` geparste Body des Requests
pub fn post_response(body: &MyBody) -> Response{

    let mat_c = match body.operation{ //erzeugt Option<Vec<i32>> falls eine Matrix erzeugt wurde
        Some(ref op) => { //ref op -> Pointer auf den Wert von op
            match &op[..]{ //Welche Operation soll ausgeführt werden
                "add" => { //Addition
                     match vector_add(&body.mat_a.elem, //Matrixaddition
                                     &body.mat_b.elem)
                    {
                        Some(c) => Some( MyMatrix{    //Falls erfolgreich Struct anlegen
                                            rows: body.mat_a.rows,
                                            cols: body.mat_a.cols,
                                            elem: c }),
                        None => {println!("Matrizen haben nicht die gleiche Größe"); None}
                    }
                },

                "mull" => {
                    match matrix_mul(&body.mat_a.elem, //Matrixmultiplikation
                                     &body.mat_b.elem,
                                     body.mat_a.rows as usize,
                                     body.mat_a.cols as usize,
                                     body.mat_b.cols as usize)
                    {
                        Some(c) => Some( MyMatrix{                  //Falls erfolgreich Struct anlegen
                                            rows: body.mat_a.rows,
                                            cols: body.mat_b.cols,
                                            elem: c}),
                        None => {println!("Matrizen sind falsch dimensioniert für Multiplikation"); None}
                    }
                },
                _ => {println!("Operation existiert nicht"); None}
            }
        }
        None => {println!("keine Operation angegeben"); None},
    };

    match mat_c{ // Falls Berechnung erfolgreich
        Some(c) => {
                Response::with((status::Ok, json::encode(&c).unwrap())) //Matrix als JSON codieren
            },
        None => {println!("Fehler"); Response::with((status::InternalServerError,"Operation konnten mit den Matrizen nicht ausgeführt werden"))}
    }
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
