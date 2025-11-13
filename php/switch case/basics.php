<?php

$diaDaSemana = date("w");


//Switch recebe o valor de uma variavel, e executa um codigo dependendo do valor
switch ($diaDaSemana) {
	case 0:
	echo "Domingo";
	break;

	case 1:
	echo "Segunda";
	break;

	case 2:
	echo "Terça";
	break;

	case 3:
	echo "Quarta";
	break;

	case 4:
	echo "Quinta";
	break;

	case 5:
	echo "Sexta";
	break;

	case 6:
	echo "Sabado";
	break;

//O default serve como 'opção padrao'
	default:
	echo "Data invalida";
	break;

}

echo "<br>";

echo $diaDaSemana;

?>