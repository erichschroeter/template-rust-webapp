use log::info;

use super::Command;


pub struct RunCommand;

impl Command for RunCommand {
    fn execute(&self) -> Result<(), Box<dyn super::FixmeError>> {
        info!("executing command 'run'");
        // let server = HttpServer::new(|| {
        //     App::new()
        //         .route("/", web::get().to(index))
        //         // .route("/generate-manifest", web::get().to(generate_manifest))
        // })
        // .bind("127.0.0.1:8080")?

        // match server {
        //     Ok(server) => {
        //         if let Err(e) = server.run().await {
        //             eprintln!("Server error: {}", e);
        //         }
        //     }
        //     Err(e) => eprintln!("Failed to bind server: {}", e),
        // }
        Ok(())
        // todo!()
    }
}
