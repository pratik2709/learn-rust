#[derive(Debug)]
struct CubeSat{
    id: u64,
    mailbox: MailBox
}

#[derive(Debug)]
struct MailBox{
    messages: Vec<Message>
}

type Message = String;

struct GroundStation;

impl GroundStation{
    fn send(&self, to: &mut CubeSat, msg: Message){
        to.mailbox.messages.push(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat{
        CubeSat{id: sat_id, mailbox: MailBox{messages: vec![]}}
    }
}

impl CubeSat{
    fn recv(&mut self) -> Option<Message>{
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
enum StatusMessage{
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat{
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn fetch_sat_ids() -> Vec<u64>{
    vec![1,2,3]
}

fn main() {
    let base = GroundStation{};

    let mut sat_a: CubeSat = CubeSat{ id: 0, mailbox: MailBox {messages: vec![]}};
    let sat_b = CubeSat{ id: 1, mailbox: MailBox {messages: vec![]}};
    let sat_c = CubeSat{ id: 2, mailbox: MailBox {messages: vec![]}};

    base.send(&mut sat_a,Message::from("hello world"));
    base.send(&mut sat_a,Message::from("hello world2"));
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("{:?}", msg);

}
