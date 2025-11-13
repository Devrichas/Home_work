<?php

$qualASuaIdade = 12;

$idadeCriança = 12;
$idadeAdulto = 18;
$idadeMelhor = 65;

//A estrutura segue, if/else if (parametro) {codigo}, else {codigo}

if ($qualASuaIdade < $idadeCriança){
	echo "Criança";
}

else if ($qualASuaIdade < $idadeAdulto) {
	echo "Adolecente";
}

else if ($qualASuaIdade < $idadeMelhor){
	echo "Adulto";
}

else {
	echo "Idoso";
}

echo "<br>";

//Operador ternario, parecido com if, mas podendo apenas retornar texto
//A estrutura segue, echo (parametro)?"Texto se for TRUE":"Texto se for False"
echo ($qualASuaIdade < $idadeAdulto)?"Menor de idade":"Mairor de idade"

?>