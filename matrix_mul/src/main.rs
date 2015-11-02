extern crate iron;
extern crate bodyparser;
extern crate persistent;
extern crate rustc_serialize;


//use persistent::Read;

use iron::status;
use iron::prelude::*;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

#[derive(Debug, Clone, RustcDecodable)]
struct MyStructure {
    age: Vec<Vec<i32>>,
    name: Option<String>,
}


fn generate_response(req: &Request) -> Response{
	
	let path_vec = &req.url.path;
	
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

fn parse_body(req: &mut Request)-> IronResult<Response>{
	
    let struct_body = req.get::<bodyparser::Struct<MyStructure>>();
    match struct_body {
        Ok(Some(struct_body)) => println!("Parsed body:\n{:?} {:?}", struct_body, struct_body.age[0][1]),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(Response::with(status::Ok))
    
}



fn process_request(req: &mut Request) -> IronResult<Response> {
	
	//hat der Request einen Body?
	let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => println!("Read body:\n{}", body),
        Ok(None) => println!("No body"),
        Err(err) => panic!("Error: {:?}", err)
    }
	
	generate_response(&req);
	
	parse_body(req)	
	
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

//curl -i "localhost:3000/" -H "application/json" -d '{"name":"jason","age": [["1","2","3"],["1","2","3"]]}'
// and check out the printed json in your terminal.
fn main() {
    let mut chain = Chain::new(process_request);
    chain.link_before(persistent::Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();
}
