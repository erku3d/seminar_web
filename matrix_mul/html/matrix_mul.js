
function getMatrixObject(rows,cols,elem){
    var mat = {
        rows:rows,
        cols:cols,
        elem:elem
    }
    return mat;
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

    for (var i = 0; i < rows.length; i++){

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
        for(var j=0; j<row.length; j++){
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

    //erzeuge http request body

    var body = '{"mat_a": ' + JSON.stringify(mat_A) + ',' + '"mat_b": ' + JSON.stringify(mat_B) +'}';

    var xhttp = new XMLHttpRequest();

    xhttp.onreadystatechange = function() {
        if (xhttp.readyState == 4 && xhttp.status == 200) {
            showResult(xhttp.responseText);
        } else if ( xhttp.readyState == 4 && xhttp.status != 200){
            alert("Fehler! "+xhttp.statusText+"\n"+xhttp.responseText+"\n"+ xhttp.status);
        }
    }

    xhttp.open("POST", "index.html", true);
    xhttp.setRequestHeader("Content-type", "application/json");
    xhttp.send(body); //sendet Text im Body

    document.getElementById("mat_add").value = "Matrix wird berechnet ...";
    document.getElementById("mat_mul").value = "Matrix wird berechnet ...";

}

function myMatrixToString(mat){

    if(mat.elem.length < 1)
        throw "empty"

    var str = mat.elem[0];

    if (str < 10){
        str = '  '+str;
    }
    else if (str < 100){
        str = ' '+str;
    }

    for(var i=1; i<mat.elem.length; i++){

        if(i % mat.cols == 0)
            str = str + "\n";
        else
            str = str + ",";

        var el = mat.elem[i];

        if (el < 10){
            el = '  '+el;
        }
        else if (el < 100){
            el = ' '+el;
        }

        str = str + el;
    }

    return str;
}


function showResult(resp){

    var body = JSON.parse(resp);

    try{
        document.getElementById("mat_add").wrap="off";
        document.getElementById("mat_add").value = myMatrixToString(body.mat_a);
    }
    catch(err){
        document.getElementById("mat_add").wrap="hard";
        document.getElementById("mat_add").value = body.message;
    }

    try{
        document.getElementById("mat_mul").wrap="off";
        document.getElementById("mat_mul").value = myMatrixToString(body.mat_b);
    }
    catch(err){
        document.getElementById("mat_mul").wrap="hard";
        document.getElementById("mat_mul").value = body.message;
    }

}

function clearTextArea(){

	document.getElementById("mat_A").value = "";
	document.getElementById("mat_B").value = "";
	document.getElementById("mat_add").value = "";
	document.getElementById("mat_mul").value = "";

    document.getElementById("a_r").value = "";
    document.getElementById("a_c").value = "";

    document.getElementById("b_r").value = "";
    document.getElementById("b_c").value = "";

    document.getElementById("mat_A").focus();
}

function getRandMat(r,c){
    var s = "";

    for(var i=0;i<r;i++){
		for(var j=0;j<c;j++){
			var randInt = Math.floor((Math.random() * 10) + 1);

            if(randInt < 10)
                randInt = " "+randInt

			if(j>0)
				s = s + "," + randInt;
			else
				s = s + randInt;

		}
		s=s+"\n";
	}

    return s;

}


function randomMat(){

    var r = document.getElementById("a_r").value;
    var c = document.getElementById("a_c").value;

    if(r>199){
        r = 200
        document.getElementById("a_r").value = 200
    }

    if(c>199){
        c = 200
        document.getElementById("a_c").value = 200
    }

	document.getElementById('mat_A').value = getRandMat(r,c);

    r = document.getElementById("b_r").value;
    c = document.getElementById("b_c").value;

    if(r>199){
        r = 200
        document.getElementById("b_r").value = 200
    }

    if(c>199){
        c = 200
        document.getElementById("b_c").value = 200
    }

	document.getElementById('mat_B').value = getRandMat(r,c);

}
