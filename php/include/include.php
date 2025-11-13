<?php

//include "func_somar.php";

// Caso o arquivo 'importado' não exista ou não esteja funcionando, o 'include' vai tentar rodar o codigo mesmo assim
// Utilizado para 'importar' funções nativas do php
// 'include remoto' utilizado patra pegar arquivos de outros dominios, extremamente não recomendado devido a comprometer a segurança
// O 'include' funciona praticamente igual ao 'import' do Python
// O diretorio do arquivo e procurado a partir do local de onde se deu o 'include'
// Caso o arquivo estiver em outro diretorio o caminho devera ser especificado (caminho de rede se aplica)

require "include/func_somar.php";
// No 'require' obriga a existencia e funcionalidade do arquivo 'importado'
// Caso contrario sera gerado um erro fatal, a partir do php 7 erros fatais não interrompem a execução da pagina gerando uma exeção, podendo ser usado 'try'
// Muito mais simples e seguro

// 'require_once' e 'include_once' é utilizado para evitar a chamar a mesma função mais de uma vez


$resultado = somar(13,22);

echo($resultado);

?> 