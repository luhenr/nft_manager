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
