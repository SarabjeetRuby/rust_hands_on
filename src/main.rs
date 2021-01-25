#[path = "models/ticket.rs"]
mod ticket;

fn main() {
    let ticket = ticket::Ticket{ id:12, title:"My laptop not working",description:"My laptop not working" };
    println!("My ticket id is {} {} {}",ticket.id,ticket.title,ticket.description);
}
