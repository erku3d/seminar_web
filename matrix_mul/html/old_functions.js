function textareaToMatrix(id){
    //Matrix A

    var str = document.getElementById(id).value.trim();

    if(str.length == 0)
        throw "is empty";

    //erzeuge Array aus den Zeilen
    var rows = str.split("\n");

    var mat = [];
    var row_len;

    for (i = 0; i < rows.length; i++){

        //in jeder Zeile sind die einzelnen Werte durch "," getrennt
        var row = rows[i].split(",");

        //jede Zeile der Matrix muss die gleiche Anzahl an Elementen haben
        if(i==0){
            row_len = row.length;
        } else {
            if (row.length != row_len){
                throw "rows unevenly";
            }
        }
        //entfernt Leerzeichen etc. von den einzelnen Elementen
        //TODO prüfen, ob das notwendig ist
        //TODO prüfen ob zahl
        for(j=0; j<row.length; j++)
            row[j] = row[j].trim();

        mat.push(row);
    }
    return mat;
}
