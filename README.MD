# Money Manager

CLI em Rust para controlar contas bancárias e saldos no terminal.

## 🎯 Funcionalidades

- Cadastrar contas bancárias com saldo
- Listar contas e saldos
- Persistência em SQLite
- Valores com até 2 casas decimais

## 🚀 Instalação

```bash
cargo install --path .
cargo install --path . --force  # atualizar
```

## 📖 Comandos

### add - Cadastrar Conta

```bash
money-manager add --name "Conta Corrente BB" --value 1500.00
money-manager add -n "Poupança Itaú" -v 5000,50
```

### list - Listar Contas

```bash
money-manager list
```

Saída:

``` txt
#1 | Conta Corrente BB | 1500.00
#2 | Poupança Itaú | 5000.50
```

## 📝 Notas

- Saldo pode ser negativo: `-500.00`
- Aceita ponto ou vírgula: `100.50` ou `100,50`
- Nomes com espaços: use aspas
- Dados salvos em `~/.money-manager/money_manager.db`
