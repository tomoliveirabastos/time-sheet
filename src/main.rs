slint::include_modules!();
use chrono::{Local};
use chrono::DateTime;
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;
use std::io::Error;

struct Historico<'a> {
    usuario_nome: &'a str,
    descricao: &'a str,
};

fn registrar_inicio() -> Result<(), mysql::Error> {
    let url = "mysql://root:123Mudar@localhost:3306/time_sheet";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let historico: Vec<Historico> = vec![
        Historico {
            usuario_nome: "Tom",
            descricao: "asdasda",
        }
    ]

    conn.exec_batch(
        r"INSERT INTO historico (usuario_nome, descricao, inicio) VALUES (:usuario_nome, :descricao, :inicio)",
          historico.iter().map(|p| params! {
            "usuario_nome" => p.usuario_nome,
            "descricao" => p.descricao,
            "inicio" => chrono::Local::now().naive_local(),
        })
    )?;

    Ok(())
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_callback_values({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let text = ui.get_descricao_texto();
            println!("{:?}", text);
        }
    });

    ui.run()
}
