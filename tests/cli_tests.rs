// tests/cli_tests.rs

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::NamedTempFile;

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

    // Cria um arquivo temporário para o banco de dados
    let db_file = NamedTempFile::new().unwrap();
    let db_path = db_file.path().to_str().unwrap();

    // Define a variável de ambiente para o caminho do banco de dados
    cmd.env("DB_PATH", db_path);

    cmd.write_stdin("1\n") // Seleciona "Criar NFT"
        .write_stdin("token_cli_test\n") // Token ID
        .write_stdin("123\n") // Owner ID
        .write_stdin("2023-11-05\n") // Data de Criação
        .write_stdin("Arte\n") // Categoria
        .write_stdin("5\n") // Seleciona "Sair"
        .assert()
        .success()
        .stdout(contains("NFT salvo com sucesso!"));
}

#[test]
fn test_cli_list_nfts() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    // Cria um arquivo temporário para o banco de dados
    let db_file = NamedTempFile::new().unwrap();
    let db_path = db_file.path().to_str().unwrap();

    // Define a variável de ambiente para o caminho do banco de dados
    cmd.env("DB_PATH", db_path);

    // Primeiro, cria um NFT
    cmd.write_stdin("1\n")
        .write_stdin("token_list_test\n")
        .write_stdin("456\n")
        .write_stdin("2023-11-06\n")
        .write_stdin("Colecionável\n")
        .write_stdin("2\n") // Seleciona "Listar NFTs"
        .write_stdin("5\n") // Seleciona "Sair"
        .assert()
        .success()
        .stdout(contains("token_list_test"));
}

#[test]
fn test_cli_update_nft() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    // Cria um arquivo temporário para o banco de dados
    let db_file = NamedTempFile::new().unwrap();
    let db_path = db_file.path().to_str().unwrap();

    // Define a variável de ambiente para o caminho do banco de dados
    cmd.env("DB_PATH", db_path);

    cmd.write_stdin("1\n") // Criar NFT
        .write_stdin("token_update_test\n") // Token ID
        .write_stdin("123\n") // Owner ID
        .write_stdin("2023-11-05\n") // Data de Criação
        .write_stdin("Arte\n") // Categoria
        .write_stdin("3\n") // Atualizar NFT
        .write_stdin("token_update_test\n") // Token ID a ser atualizado
        .write_stdin("456\n") // Novo Owner ID
        .write_stdin("5\n") // Sair
        .assert()
        .success()
        .stdout(contains("NFT atualizado com sucesso!"));
}

#[test]
fn test_cli_delete_nft() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    // Cria um arquivo temporário para o banco de dados
    let db_file = NamedTempFile::new().unwrap();
    let db_path = db_file.path().to_str().unwrap();

    // Define a variável de ambiente para o caminho do banco de dados
    cmd.env("DB_PATH", db_path);

    cmd.write_stdin("1\n") // Criar NFT
        .write_stdin("token_delete_test\n") // Token ID
        .write_stdin("123\n") // Owner ID
        .write_stdin("2023-11-05\n") // Data de Criação
        .write_stdin("Arte\n") // Categoria
        .write_stdin("4\n") // Deletar NFT
        .write_stdin("token_delete_test\n") // Token ID a ser deletado
        .write_stdin("5\n") // Sair
        .assert()
        .success()
        .stdout(contains("NFT deletado com sucesso!"));
}
