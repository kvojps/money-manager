# Money Manager

CLI em Rust para controlar contas bancárias e saldos no terminal.

## 🎯 Funcionalidades

- Cadastrar contas bancárias com saldo;

    ```bash
    money-manager add --name "Conta Corrente BB" --value 1500.00
    money-manager add -n "Poupança Itaú" -v 5000,50
    ```

- Listar contas e saldos;

    ```bash
    money-manager list
    ```

- Registrar transações;

    ```bash
    money-manager tx add --account-id 1 --tx-type add --value 500.00
    money-manager tx add --account-id 1 --tx-type subtract --value 250.00 --description "Compra no mercado"
    ```

## 🚀 Instalação

```bash
cargo install --path .
cargo install --path . --force  # atualizar
```

## 📝 Notas

- Dados salvos em `~/.money-manager/money_manager.db`
