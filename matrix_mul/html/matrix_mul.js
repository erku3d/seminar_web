window.onload = function(e) {
    //funtkioniert nicht bei jedem Browser!
    document.getElementById("mat_A").placeholder = "1,2,3\n4,5,6\n7,8,9";
    document.getElementById("mat_B").placeholder = "1,2,3\n4,5,6\n7,8,9";

};

function onlyNumbers(evt) {
  var theEvent = evt || window.event;
  var key = theEvent.keyCode || theEvent.which;
  key = String.fromCharCode( key );
  var regex = /[0-9]|\.|\,|\r/;
  if( !regex.test(key) ) {
    theEvent.returnValue = false;
    if(theEvent.preventDefault) theEvent.preventDefault();
  }
}

function getMatrixObject(rows,cols,elem){
    return {
        rows:rows,
        cols:cols,
        elem:elem
    }
}

function textareaToMatrix(id){

    //liest Textarea aus und erzeugt MatrixObject

    var str = document.getElementById(id).value.trim();

    if(str.length == 0) //falls Textarea leer
        throw "is empty";

    //erzeuge Array aus den Zeilen
    var rows = str.split("\n");
    var c_rows = rows.length; //Anzahl der Zeilen


    var mat = [];
    var c_cols; //Anzahl der Spalten

    for (i = 0; i < rows.length; i++){

        //in jeder Zeile sind die einzelnen Werte durch "," getrennt
        var row = rows[i].split(",");

        //jede Zeile der Matrix muss die gleiche Anzahl an Elementen enthalten
        if(i==0){
            c_cols = row.length;
        } else {
            if (row.length != c_cols){
                throw "rows unevenly";
            }
        }
        //entfernt Leerzeichen etc. von den einzelnen Elementen
        //TODO prüfen, ob das notwendig ist
        for(j=0; j<row.length; j++){
            row[j] = row[j].trim();
            mat.push(row[j]);
        }
    }
    return getMatrixObject(c_rows,c_cols,mat);
}

function sendRequest(){

    try{
        var mat_A = textareaToMatrix("mat_A");
    }
    catch(err){
        switch(err) {
            case "is empty":
                alert("Matrix A ist leer!");
                return;

            case "rows unevenly":
                alert("Zeilenlängen der Matrix A sind ungleich!");
                return;

            default:
                alert("Fehler: "+err);
                return;
        }
    }

    try{
        var mat_B = textareaToMatrix("mat_B");
    }
    catch(err){
        switch(err) {
            case "is empty":
                alert("Matrix B ist leer!");
                return;

            case "rows unevenly":
                alert("Zeilenlängen der Matrix B sind ungleich!");
                return;

            default:
                alert("Fehler: "+err);
                return;
        }
    }



    var op = document.getElementById("operation").value;

    if(op == "add" || op == "diff"){
        //addition, subtraktion -> A und B gleiche größe
        if((mat_A.rows != mat_B.rows) || (mat_A.cols != mat_B.cols)){
            alert("Beide Matrizen müssen dieselbe größe haben.");
            return;
        }
    } else { //mul
        //Mulltipikation ->  #spalten A = #Zeilen B
        if(mat_A.cols != mat_B.rows){
            alert("Spaltenanzhal von A muss gleich der Zeilenanzahl von B sein");
            return;
        }
    }

    //erzeuge http request body

    var body = '{"op":"'+op+'",';
    body = body + '"mat_a": ' + JSON.stringify(mat_A) +',';
    body = body + '"mat_b": ' + JSON.stringify(mat_B) +'}';


    //alert(body);


    var xhttp = new XMLHttpRequest();
      xhttp.onreadystatechange = function() {
        if (xhttp.readyState == 4 && xhttp.status == 200) {
          alert(xhttp);
        } else if ( xhttp.readyState == 4 && xhttp.status != 200){
          alert("Fehler! "+xhttp.statusText);
        }
      }

      xhttp.open("POST", "index.html", true);
      xhttp.setRequestHeader("Content-type", "application/json");
      xhttp.send(body); //sendet Text im Body

}
