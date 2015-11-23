
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
        for(j=0; j<row.length; j++){
            if(row[j].indexOf('.') > -1 || isNaN(row[j])) //falls keine Zahl
                throw "Not an Integer";


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
        document.getElementById("mat_A").focus();
        switch(err) {
            case "is empty":
                alert("Matrix A ist leer!");
                return;

            case "rows unevenly":
                alert("Zeilenlängen der Matrix A sind ungleich!");
                return;

            case "Not an Integer":
                alert("Matrix A darf nur aus ganzen Zahlen bestehen!");
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
        document.getElementById("mat_B").focus();
        switch(err) {
            case "is empty":
                alert("Matrix B ist leer!");
                return;

            case "rows unevenly":
                alert("Zeilenlängen der Matrix B sind ungleich!");
                return;

            case "Not an Integer":
                alert("Matrix B darf nur aus ganzen Zahlen bestehen!");
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

    var body = '{"operation":"'+op+'",';
    body = body + '"mat_a": ' + JSON.stringify(mat_A) +',';
    body = body + '"mat_b": ' + JSON.stringify(mat_B) +'}';

    //alert(body);


    var xhttp = new XMLHttpRequest();
      xhttp.onreadystatechange = function() {
        if (xhttp.readyState == 4 && xhttp.status == 200) {
          showResult(xhttp.responseText);
        } else if ( xhttp.readyState == 4 && xhttp.status != 200){
          alert("Fehler! "+xhttp.statusText+"\n"+xhttp.responseText);
        }
      }

      xhttp.open("POST", "index.html", true);
      xhttp.setRequestHeader("Content-type", "application/json");
      xhttp.send(body); //sendet Text im Body
      
      document.getElementById("mat_C").value = "Matrix wird berechnet ...";

}

function showResult(resp){
    var mat = JSON.parse(resp);

    //alert(mat.rows + "\n" + mat.cols +"\n"+ mat.elem);

    var val = mat.elem[0];

    for(i=1; i<mat.elem.length; i++){

        if(i % mat.cols == 0)
            val = val + "\n";
        else
            val = val + ",";

        val = val + mat.elem[i];
    }

    //alert(val);
    document.getElementById("mat_C").value = val;

}

function clearTextArea(){
	
	document.getElementById("mat_A").value = "";
	document.getElementById("mat_B").value = "";
	document.getElementById("mat_C").value = "";
	document.getElementById("mat_A").focus();
}

function randomMat(id){
	
	var s = ""
	
	for(i=0;i<200;i++){
		for(j=0;j<200;j++){
			var r = Math.floor((Math.random() * 100) + 1);
			
			if(j>0)
				s = s + "," + r;
			else
				s = s + r;
			
		}
		s=s+"\n";
	}
	
	
	
	
	document.getElementById(id).value = s;
}
