# potigol-interpreter
## Sobre
O repositório inclui um lexer e parser, além de 2 opções de interpretador, um que percorre a AST e outro que compila a AST para bytecode e roda ele.

O interpretador e parser ainda estão em desenvolvimento e estou usando exemplos simples como referência para implementar a lingua.

O binário disponível recebe o caminho de um arquivo como argumento e interpreta ele percorrendo a AST gerada.
```bash
cargo run -- <path/to/file>
```

## Funcionalidades
- [x] Literais
- [x] Strings
- [x] F-strings
- [x] Variáveis com var(semi funcionais)
- [x] Funções `leia_*`
- [x] `se`, `senão` e`senãose`
- [x] `enquanto` e `para`
- [ ] Tipagem
- [ ]  Listas e tuplas
- [ ] Definição de função
- [ ] POO
- [ ] Métodos
- [ ] Aspectos de programação funcional
- [ ] `gere`
- [ ] ...
