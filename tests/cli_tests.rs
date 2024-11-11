// tests/cli_tests.rs

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::NamedTempFile;
use tempfile::tempdir;
use std::str;


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

    let assert = cmd.write_stdin(input)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(stdout.contains("NFT deletado com sucesso!"));
}

