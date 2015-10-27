extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;


use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;


use std::collections::HashMap;
use urlencoded::UrlEncodedQuery;

// fn print_query(hashmap: &&HashMap<String,Vec<String>>){
//
//     println!("Parsed GET request query string:\n {:?}", hashmap);
//
// }

fn print_query(hashmap: &&HashMap<String,Vec<String>>)-> Response{
	/*
     println!("Parsed GET request query string:\n {:?}", hashmap);
     
     let mut v: Vec<String> = vec![];
     
     let str_vec = &hashmap.get("vec").unwrap()[0];
     
     println!{"vec: {:?}",str_vec};
    
    let tmp = str_vec.split(","); 		
	println!{"vec: {:?}",tmp};
     */
     
	
     let try_file= match hashmap.get("file") {
         // The `description` method of `io::Error` returns a string that
         // describes the error
         None => {
             println!("wrong Question");
             return Response::with((status::BadRequest, "Falsche frage"))
         }
         Some(name) => name //fals erfolgreich, gib die Datei zurück
     };
	
    //erzeuge einen String
    let filename = String::from("html/".to_string() + &try_file[0]);
	
	//let filename = String::from("html/cd_catalog.xml");
	
    //öffne die angefragte Datei
    let path = Path::new(&filename);
    let display = path.display();
    let extension = path.extension();

    println!("display: {:?}", display);


     let mut file = match File::open(&path) {
         // The `description` method of `io::Error` returns a string that
         // describes the error
         Err(why) => {
             println!("couldn't open {}: {}", display, Error::description(&why));
             return Response::with((status::NotFound, "Seite nicht gefunden!"))
         }
         Ok(file) => file //fals erfolgreich, gib die Datei zurück
     };

     let mut s = String::new();


    //erzeuen einer Response mit OK und dem Dateiinhalt im body
     let mut res = match file.read_to_string(&mut s) {
         Err(why) => panic!("couldn't read {}: {}", display,
                                                    Error::description(&why)),
         Ok(_) => Response::with((status::Ok,s)),
     };

    //erzeuge String für den content-type auf basis der Dateiendung
     //unwarap führt zum Absturtz falls datei ohne Endung existiert und aufgerufen wird
     let ext = "text/".to_string()+extension.unwrap().to_str().unwrap();

     res.headers.set_raw("content-type", vec![ext.as_bytes().to_vec()]);

     return res
    // Response::with((status::Ok, "Hello!"))
}

fn generate_response(path_vec: &Vec<String>) -> Response{

    //wird aufgerufen, falls nur eine Datei (index.html u.a.) aufgerufen wird
    //es soll nur eine Seite geladen werden -> kein GET Request

    // Create a path to the desired file
    let s = if path_vec[0] == ""{ //falls keien Datei angegben -> default index.html
        "html/index.html".to_string()
    } else {
        "html/".to_string() + &path_vec[path_vec.len()-1]
    };
    let path = Path::new(&s);
    let display = path.display();

    println!("display: {:?}", display);
    println!(" len: {:?}", path_vec.len());



    //versuche die Datei zu öffnen
    //falls unbekannt gib 404 als Respons
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => {
            println!("couldn't open {}: {}", display, Error::description(&why));
            return Response::with((status::NotFound, "Seite nicht gefunden!"))
        }
        Ok(file) => file //fals erfolgreich, gib die Datei zurück
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();


    let mut res = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => Response::with((status::Ok,s)),
    };

    //setzen des content-type -> Annahme das immer html
    res.headers.set_raw("content-type", vec![b"text/html".to_vec()]);

    println!("status: {:?}",res.status);
    println!("headers: {:?}",res.headers);

    return res;
    // `file` goes out of scope, and gets closed
    //Response::with((status::Ok, "Hello!"))
}

fn log_params(req: &mut Request) -> IronResult<Response> {

    println!(" ", );
    println!("-----------------------------------------", );
    println!("-----------------------------------------", );
    //Was steht alles in req drin
    println!("url: {}",req.url);
    println!(" scheme: {}",req.url.scheme);
    println!(" Host: {}",req.url.host);
    println!(" port: {}",req.url.port);
    println!(" Path: ");
    for s in &req.url.path{
        println!("  {}", s);
    }
    println!(" username: {:?}",req.url.username);
    println!(" password: {:?}",req.url.password);
    println!(" query: {:?}",req.url.query);
    println!(" fragment: {:?}",req.url.fragment);
    println!("Method: {:?}",req.method);
    println!("Headers: {:?}",req.headers);
    println!(" ");

    let mut s = String::new();

    match req.body.read_to_string(&mut s){
        Err(why) => panic!("couldn't read Body: {}",Error::description(&why)),
        Ok(_) => println!("Body: {:?}",s),
    };

    //brauch ich glaub ich nicht
    //println!(" Extensions: {:?}",req.extensions);

    println!(" ", );

    println!("remote_addr: {}",req.remote_addr);
    println!("local_addr: {}",req.local_addr);

    // println!("-----------------------------------------", );
    // println!(" ", );

    // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
    //auswerten des Get Request String
    // match req.get_ref::<UrlEncodedQuery>() {
    //     Ok(ref hashmap) => print_query(hashmap),//println!("Parsed GET request query string:\n {:?}", hashmap),
    //     Err(ref e) => println!("{:?}", e)
    // };

    // match req.get_ref::<UrlEncodedQuery>() {
    //     Ok(ref hashmap) => print_query(hashmap),//println!("Parsed GET request query string:\n {:?}", hashmap),
    //     Err(ref e) => println!("{:?}",e)
    // };

    println!(" ", );
    println!("-----------------------------------------", );
    println!("-----------------------------------------", );
    //
    // let mut resp = match generate_response(&req.url.query, &req.url.path) {
    //     Err(_) => Response::with((status::Ok, "Hello!")),
    //     Ok(r) => r
    // };

    //überprüfe, ob url Query
    if req.url.query.is_some(){

        // let resp = match req.get_ref::<UrlEncodedQuery>() {
        //     Ok(ref hashmap) => print_query(hashmap),//println!("Parsed GET request query string:\n {:?}", hashmap),
        //     Err(_) => Response::with((status::Ok, "Hello!"))
        // };

        //erzeugt hashmap
        let hashmap = req.get_ref::<UrlEncodedQuery>().unwrap();
        let resp = print_query(&hashmap);  //auswerten der Query -> erzeugt Response

        Ok(resp)
    } else { //falls keine Query -> es soll nur seite geladen werden
        Ok(generate_response(&req.url.path))
    }

    //Ok(generate_response(&req.url.query, &req.url.path))

}

// Test out the server with `curl -i "http://localhost:3000/?name=franklin&name=trevor"  127.0.0.1`
fn main() {
    Iron::new(log_params).http("127.0.0.1:3000").unwrap();
}
