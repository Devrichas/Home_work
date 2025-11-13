<?php

$frase = "Farmando aura no betinha.";

$aura = "aura";

$query = strpos($frase, $aura);
// encontra a posiçâo que a palavra começa, nesse caso seria 9

$texto = substr($frase, 0, $query);
// retorna tudo que esta antes de um ponto, (onde devo procurar, onde devo começar, onde devo parar)

var_dump($texto);

$texto2 = substr($frase, $query + strlen($aura), strlen($frase));
// strlen retorna o tamanho de uma especifica string

echo "<br>";

var_dump($texto2)

?>