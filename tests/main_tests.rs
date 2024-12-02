// tests/main_tests.rs

use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_main_unknown_option() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    // Executar o programa com uma opção desconhecida
    cmd.arg("--unknown")
        .assert()
        .success()
        .stdout(contains("Opção de linha de comando desconhecida"));

    // Também podemos verificar se o programa sugere usar '--help'
    cmd.assert()
        .stdout(contains("Use '--help' para ver as opções disponíveis."));
}

#[test]
fn test_main_help_option() {
    let mut cmd = Command::cargo_bin("nft_manager").unwrap();

    // Executar o programa com a opção '--help'
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("Gerenciador de NFTs"))
        .stdout(contains("Uso:"))
        .stdout(contains("Opções:"));
}
