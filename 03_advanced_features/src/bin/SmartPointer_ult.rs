//the powerful combo Rc + RefCell (before tokio and threads >:)
//soo hmmmm we have a botnet (C2 Server) and a couples of bots
// for each one i want to send to the server and edit the logs
// the combo Rc : to give each bot a copy of the server     RefCell: so can he edit

use std::rc::Rc;
use std::cell::RefCell;

struct C2Server {
    stolen_creds: Vec<String>,
}

struct Bot {
    id: u32,
    server: Rc<RefCell<C2Server>>, // the combo
}

impl Bot {
    fn steal_data(&self, data: &str) {
        //here the magic happen wooooow
        self.server.borrow_mut().stolen_creds.push(format!("Bot#{}: {}", self.id, data));
    }
}

fn main() {
    let central_server = Rc::new(RefCell::new(C2Server {
        stolen_creds: vec![],
    }));
    //creat bots
    let bot1 = Bot { id: 101, server: Rc::clone(&central_server) };
    let bot2 = Bot { id: 102, server: Rc::clone(&central_server) };

    //bots do hmmm work?
    bot1.steal_data("password123");
    bot2.steal_data("admin_key_xyz");
    bot1.steal_data("cookie_session");


    //mmm print
    println!("--- C2 Server Logs ---");
    for cred in &central_server.borrow().stolen_creds {
        println!("Stolen: {}", cred);
    }

}