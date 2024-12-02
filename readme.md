Aplicacao CLI para consulta e analise de resultados de sorteios das loterias da Caixa.

API: https://github.com/guto-alves/loterias-api

Para a manutencao da hospedagem da API utilizada, considere fazer uma DOACAO para o autor da API através do link acima.

Caso o arquivo executavel não esteja presente no `PATH` do sistema, navegue no terminal até o diretório do arquivo executavel e preceda os comando com `./`. Ex: `./loto --help` ou `./loto.exe --help` no Windows.

```bash
loto --help
```

```bash
Usage: loto <COMMAND>

Commands:
  consultar  Consultar ultimo sorteio e comparar com aposta(s).
  historico  Mostrar historico de sorteios.
  analisar   Analisar uma aposta em sorteios passados.
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

EXEMPLOS:
loto consultar megasena
loto consultar megasena 04-08-15-16-23-42
loto consultar megasena < input.txt > output.txt

loto historico megasena
loto historico megasena --quantidate 50 --atualizar

loto analisar megasena 04-08-15-16-23-42
loto analisar megasena 04-08-15-16-23-42 -q 50 -a

Para ajuda em cada tipo de operacao, use --help.
loto consultar --help
loto historico --help
loto analisar --help

* Os numeros de uma aposta podem ser separados por espaco, ',' ou '-'.
* Arquivos de input devem conter em cada linha uma sequencia de numeros referentes a uma aposta.
* Atualizacao de dados feita a cada 12h.
```