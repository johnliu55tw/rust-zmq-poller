extern crate zmq;

fn main() {
    let mut ctx = zmq::Context::new();
    let mut socket1 = match ctx.socket(zmq::PULL) {
        Ok(socket) => socket,
        Err(e) => panic!(e),
    };
    let mut socket2 = match ctx.socket(zmq::PULL) {
        Ok(socket) => socket,
        Err(e) => panic!(e),
    };

    match socket1.bind("ipc:///tmp/socket1.ipc") {
        Ok(()) => (),
        Err(e) => panic!(e)
    }
    match socket2.bind("ipc:///tmp/socket2.ipc") {
        Ok(()) => (),
        Err(e) => panic!(e)
    }

    loop {
        // Create a vector for storing revents for each socket
        let revent_list = {
            /*
             * This is the reason why we need another scope:
             * Here the as_poll_item immutably borrowed the socket, so we cannot
             * use socket for receiving the msg because the recv() method
             * requires mutable borrowing.
             */
            let mut poll_list = [socket1.as_poll_item(zmq::POLLIN),
                                 socket2.as_poll_item(zmq::POLLIN)];
            let mut revent_list = Vec::new();

            println!("Polling...");
            match zmq::poll(&mut poll_list, -1) {
                Ok(_) => {
                    for poll_item in poll_list.into_iter() {
                        revent_list.push(poll_item.get_revents());
                    }
                },
                Err(e) => {
                    println!("Error on polling: {}", e)
                },
            }
            revent_list
        };
        // Examine return events
        if revent_list[0] & zmq::POLLIN != 0 {
            let msg = socket1.recv_msg(0).unwrap();
            println!("socket1 recv: {}", msg.as_str().unwrap());
        }
        if revent_list[1] & zmq::POLLIN != 0 {
            let msg = socket2.recv_msg(0).unwrap();
            println!("socket2 recv: {}", msg.as_str().unwrap());
        }
    }
}
