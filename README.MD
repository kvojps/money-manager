# Money Manager

CLI em Rust para controlar contas bancárias e saldos no terminal.

## 🎯 Funcionalidades

- Cadastrar contas bancárias com saldo;

    ```bash
    money-manager account add "Conta Corrente BB" 1500.00
    money-manager acc add "Poupança Itaú" 5000.50
    ```

- Listar contas e saldos;

    ```bash
    money-manager account list
    money-manager account ls
    money-manager acc ls
    ```

- Registrar transações de entrada;

    ```bash
    money-manager tx in 1 500.00
    money-manager tx in "Conta Corrente BB" 500.00
    ```

- Registrar transações de saída;

    ```bash
    money-manager tx out 1 250.00
    money-manager tx out 1 250.00 -d "Compra no mercado"
    ```

## 🚀 Instalação

```bash
cargo install --path .
cargo install --path . --force  # atualizar
```

## 📝 Notas

- Dados salvos em `~/.money-manager/money_manager.db`
