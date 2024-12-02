// tests/cli_tests.rs

use assert_cmd::Command;
use predicates::str::contains;
use std::str;
use tempfile::tempdir;

#[test]
fn test_read_empty_storage() {
    use nft_manager::cli::commands::read_nft;
    use nft_manager::storage::file_storage::FileStorage;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("empty_nfts.db");
    let db_path_str = db_path.to_str().unwrap();

    let mut storage = FileStorage::new(db_path_str);

    // Certifique-se de que o banco de dados está vazio
    let nfts = storage.load_all().unwrap_or_else(|_| Vec::new());
    println!("Número de NFTs no banco de dados: {}", nfts.len());
    assert_eq!(nfts.len(), 0);

    // Testa a função `read_nft` com o banco de dados vazio
    let result = read_nft(db_path_str);
    assert!(result.is_ok());
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("Gerenciador de NFTs"));
}

/* #[test]
fn test_cli_create_nft_invalid_data() {
    use assert_cmd::Command;
    use tempfile::tempdir;
    use std::str;
    use predicates::str::contains;
    use std::time::Duration;

    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("nfts_test.db");
    let db_path_str = db_path.to_str().unwrap();

    cmd.env("DB_PATH", db_path_str);

    // Input com dados inválidos e válidos
    let input = "\
1
\n  // Token ID vazio (inválido)
valid_token_id  // Token ID válido
abc  // Owner ID inválido
123  // Owner ID válido
invalid-date  // Data inválida
2023-10-22  // Data válida
\n  // Categoria vazia (inválida)
Arte  // Categoria válida
5
";

    // Adicionar uma opção de saída após retornar ao menu
    let extended_input = format!("{}\n5\n", input);

    cmd
        .timeout(Duration::from_secs(5))
        .write_stdin(extended_input)
        .assert()
        .success()
        .stdout(contains("Token ID não pode ser vazio"))
        .stdout(contains("Owner ID inválido. Por favor, insira um número inteiro"))
        .stdout(contains("Data inválida. Formato esperado: AAAA-MM-DD"))
        .stdout(contains("Categoria não pode ser vazia"))
        .stdout(contains("Saindo..."));
} */

#[test]
fn test_cli_update_nft_nonexistent() {
    use assert_cmd::Command;
    use tempfile::tempdir;
    use std::str;

    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("nfts_test.db");
    let db_path_str = db_path.to_str().unwrap();

    cmd.env("DB_PATH", db_path_str);

    // Tentar atualizar um NFT inexistente
    let input = "\
3
nonexistent_token
123
5
";

    let assert = cmd
        .write_stdin(input)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = str::from_utf8(&output.stdout).unwrap();

    // Verificar se a mensagem de erro é exibida
    assert!(stdout.contains("Erro ao atualizar NFT"));
    assert!(stdout.contains("NFT com Token ID 'nonexistent_token' não encontrado."));
}

#[test]
fn test_cli_delete_nft_nonexistent() {
    use assert_cmd::Command;
    use tempfile::tempdir;
    use std::str;

    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("nfts_test.db");
    let db_path_str = db_path.to_str().unwrap();

    cmd.env("DB_PATH", db_path_str);

    // Tentar deletar um NFT inexistente
    let input = "\
4
nonexistent_token
5
";

    let assert = cmd
        .write_stdin(input)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = str::from_utf8(&output.stdout).unwrap();

    // Verificar se a mensagem de erro é exibida
    assert!(stdout.contains("Erro ao deletar NFT"));
    assert!(stdout.contains("NFT com Token ID 'nonexistent_token' não encontrado."));
}

#[test]
fn test_read_nft_corrupted_db() {
    use assert_cmd::Command;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    use predicates::str::contains;

    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("corrupted_nfts.db");
    let db_path_str = db_path.to_str().unwrap();

    // Criar um arquivo de banco de dados corrompido
    let mut file = File::create(&db_path).unwrap();
    writeln!(file, "dados inválidos").unwrap();

    cmd.env("DB_PATH", db_path_str);

    // Tentar listar NFTs
    let input = "\
2
5
";

    cmd
        .write_stdin(input)
        .assert()
        .success()
        .stdout(contains("Erro ao carregar NFTs"));
}

#[test]
fn test_cli_create_nft() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    // Cria um diretório temporário
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("nfts_test.db");
    let db_path_str = db_path.to_str().unwrap();

    // Define a variável de ambiente para o caminho do banco de dados
    cmd.env("DB_PATH", db_path_str);

    // Concatena todas as entradas em uma única string
    let input = "\
1
token_cli_test
123
2023-11-05
Arte
5
";

    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(contains("NFT salvo com sucesso!"));
}

#[test]
fn test_cli_list_nfts() {
    use std::str;
    use tempfile::tempdir;

    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    // Cria um diretório temporário
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("nfts_test.db");
    let db_path_str = db_path.to_str().unwrap();

    // Define a variável de ambiente para o caminho do banco de dados
    cmd.env("DB_PATH", db_path_str);

    // Concatena todas as entradas em uma única string
    let input = "\
1
token_list_test
456
2023-11-06
Colecionável
2
5
";

    let assert = cmd.write_stdin(input).assert().success();

    // Captura as saídas
    let output = assert.get_output();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Verifica se a saída contém o token do NFT que foi criado
    assert!(stdout.contains("token_list_test"));
}

#[test]
fn test_cli_update_nft() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("nfts_test.db");
    let db_path_str = db_path.to_str().unwrap();

    cmd.env("DB_PATH", db_path_str);

    let input = "\
1
token_update_test
123
2023-11-05
Arte
3
token_update_test
456
5
";

    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(contains("NFT atualizado com sucesso!"));
}

#[test]
fn test_cli_delete_nft() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    let dir = tempdir().unwrap();
    let db_path = dir.path().join("nfts_test.db");
    let db_path_str = db_path.to_str().unwrap();

    cmd.env("DB_PATH", db_path_str);

    let input = "\
1
token_delete_test
123
2023-11-05
Arte
4
token_delete_test
5
";

    let assert = cmd.write_stdin(input).assert().success();

    let output = assert.get_output();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(stdout.contains("NFT deletado com sucesso!"));
}
