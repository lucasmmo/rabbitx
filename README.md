# RABBITX

RABBITX é uma ferramenta de linha de comando desenvolvida em Rust, projetada para procurar por secrets (segredos sensíveis, como senhas ou tokens) dentro de arquivos presentes em um diretório.

## Requisitos

O projeto possui uma única dependência:

- [Regex](https://crates.io/crates/regex) (gerenciamento de expressões regulares) - usada para identificar padrões que podem conter secrets.

## Instalação

Para buildar o projeto, você precisará do Rust e do Cargo instalados em sua máquina. Caso ainda não os tenha instalados, siga as instruções [aqui](https://www.rust-lang.org/tools/install).

### Build

1. Clone o repositório:

    ```bash
    git clone https://github.com/seu-usuario/rabbitx.git
    cd rabbitx
    ```

2. Compile o projeto:

    ```bash
    cargo build --release
    ```

3. O binário será gerado no diretório `target/release`.

## Uso

RABBITX precisa de permissões de superusuário para ser executado corretamente. Para rodar a ferramenta no diretório atual, utilize o seguinte comando:

```bash
sudo ./target/release/rabbitx
```

Este comando irá procurar por padrões que possam indicar a presença de secrets em todos os arquivos no diretório atual e seus subdiretórios.

`Caso você queira usar em algum diretorio expecifico, você precisará mover o binário para o diretorio desejado.`
## Contribuição

Se você encontrar algum problema ou quiser contribuir com melhorias, fique à vontade para abrir uma issue ou enviar um pull request.



