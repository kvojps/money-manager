# Database Migrations

Este projeto usa **refinery** para gerenciar migrações do banco de dados SQLite.

## Como adicionar uma nova migração

1. Crie um arquivo SQL na pasta `migrations/` seguindo o padrão de nomeação:

   ``` txt
   V<número>__<descrição>.sql
   ```

   Exemplos:
   - `V002__create_transactions_table.sql`
   - `V003__add_index_accounts.sql`
   - `V004__alter_accounts_add_column.sql`

2. Escreva seu SQL no arquivo:

   ```sql
   CREATE TABLE IF NOT EXISTS transactions (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       account_id INTEGER NOT NULL,
       amount_cents INTEGER NOT NULL,
       description TEXT,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
       FOREIGN KEY(account_id) REFERENCES accounts(id)
   );
   ```

3. A migração será automaticamente executada na próxima vez que o aplicativo iniciar.

## Rastreamento de migrações

O refinery automaticamente rastreia quais migrações foram executadas em uma tabela interna `refinery_schema_history`. Você pode verificar qual versão do banco está rodando consultando essa tabela.

## Notas importantes

- Os números das migrações devem ser sequenciais e crescentes
- Use nomes descritivos para fácil compreensão do que cada migração faz
- Migrações são imutáveis - nunca modifique uma migração já executada
- Se precisa desfazer algo, crie uma nova migração reversa
